//! Make calls to the Steam API or react to emitted callbacks.
//!
//! All Steam API callbacks are offered in one of three ways:
//! 1. Handled entirely by this crate, available as a async function.  
//!     This is used for callbacks which behave like a Steam API call result.
//! 2. Provided as a public type, implementing [`Callback`].  
//!     Use [`CallManager::listen`]
//!
//! Your Steam app should use this module to do both
//! [callback listening](#callback-listening) and [callback handling](#callback-handling).
//!
//! # Callback Listening
//!
//! TODO!
//!
//! # Callback Handling
//!
//! TODO!
//!

use crate::error::{CallError, CallFutureError};
use crate::interfaces::{SteamChild, SteamInterface};
use crate::util::IncognitoBox;
use crate::{sys, Private};
use futures::task::AtomicWaker;
use std::alloc::Layout;
use std::any::{type_name, Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{c_int, c_void};
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdout, Write};
use std::marker::Unsize;
use std::mem::{replace, zeroed, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::slice;
use std::sync::mpsc::{channel, RecvError, SendError, Sender, TryRecvError};
use std::sync::{mpsc, Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread::{self, JoinHandle, LocalKey};
use std::time::{Duration, Instant};

//ReleaseCurrentThreadMemory
const CALL_THREAD_NAME: &'static str = "rgpr_steamworks_call_thread";

type AnySend = dyn Any + Send + Sync;

/// Simple garbage collection.
/// Does nothing if this thread thread is the [`CallThread`].
#[derive(Debug)]
pub(crate) struct ApiThreadGc(Option<ApiThreadGcInner>);

impl ApiThreadGc {
	/// Creates a new `ApiThreadGc`.
	/// Use [`get`] instead.
	///
	/// [`get`]: Self::get
	fn new() -> Self {
		if std::thread::current().name() == Some(CALL_THREAD_NAME) {
			Self(None)
		} else {
			Self(Some(ApiThreadGcInner::new()))
		}
	}

	/// Gets the thread-local instance of `ApiThreadGc`,
	/// or creates one if none exist.
	fn get() -> &'static LocalKey<RefCell<Self>> {
		thread_local! {
			static GC: RefCell<ApiThreadGc> = RefCell::new(ApiThreadGc::new());
		}

		&GC
	}

	fn exec<T>(function: impl Fn(&mut ApiThreadGcInner) -> T) -> T {
		let mut out: Option<T> = None;

		Self::get().with_borrow_mut(|instance| {
			let Some(inner) = &mut instance.0 else {
				return;
			};

			out = Some(function(inner));
		});

		out.unwrap()
	}

	/// Call whenever there is a good moment to run GC.
	/// This will only run GC if there has been enough usage to warrant doing so.
	pub(crate) fn check() -> bool {
		Self::exec(ApiThreadGcInner::check)
	}

	pub(crate) fn report() {
		Self::exec(ApiThreadGcInner::report)
	}

	pub(crate) fn should_gc() -> bool {
		Self::exec(|mut_ref| ApiThreadGcInner::should_gc(mut_ref as &_))
	}
}

///
#[derive(Debug)]
struct ApiThreadGcInner {
	first_report: Option<Instant>,
	api_call_count: u32,
}

impl ApiThreadGcInner {
	/// Run every 16 hertz.
	const MAX_DELAY: Duration = Duration::new(0, 62_500_000);

	/// Maximum report count before GC should be attempted before the delay.
	const MAX_REPORTS: u32 = 120;

	const fn new() -> Self {
		Self {
			first_report: None,
			api_call_count: 0,
		}
	}

	fn check(&mut self) -> bool {
		if !self.should_gc() {
			return false;
		}

		unsafe { sys::SteamAPI_ReleaseCurrentThreadMemory() };
		self.reset();

		true
	}

	fn reset(&mut self) {
		self.api_call_count = 0;
		self.first_report = None;
	}

	fn report(&mut self) {
		self.api_call_count = self.api_call_count.saturating_add(1);

		if self.first_report.is_none() {
			self.first_report = Some(Instant::now());
		}
	}

	fn should_gc(&self) -> bool {
		if self.api_call_count > Self::MAX_REPORTS {
			return true;
		}

		let Some(last_release) = self.first_report else {
			return false;
		};

		let time_since = Instant::now().duration_since(last_release);

		time_since > Self::MAX_DELAY
	}
}

/// Result of a call to the Steam API.
/// Upon storing the `Ok(IncognitoBox)`, the following is guaranteed:
/// - The pointer points to an allocation that is the exact memory layout of a [`CType`].
/// There is no reflection for the [`CType`], so the type must be known elsewhere.
/// - IncognitoBox: The pointer is unique, no other instances/copies of the pointer exists anywhere.
/// - The data pointed to implements [Send] + [Sync]
/// - The data pointed to does not have a thread-dependent destructor
///
/// [`CType`]: Dispatch::CType
#[doc(hidden)]
#[derive(Debug)]
#[repr(transparent)]
struct CallResult(Result<IncognitoBox<true>, CallFutureError>);

/// SAFETY: See guarantees for Ok variant above.
unsafe impl Send for CallResult {}
unsafe impl Sync for CallResult {}

type DynListeners = HashMap<TypeId, Box<AnySend>>;
type DynDispatchFn = dyn FnMut(&mut AnySend, *const c_void, Option<&mut DynListeners>) + Send + Sync;

/// Performs callback-specific tasks and optionally calls listeners.
/// Manual impls of Send + Sync are because of the raw pointer.
/// The safety of their implementation is described in [CallResult].
pub(crate) struct CallbackHandler {
	/// The type implementing CallbackRaw
	callback_impl: Box<AnySend>,

	on_callback_fn: Box<DynDispatchFn>,

	//TODO: create a contiguous unsized vec for trait objects of all the same size
	listeners: Option<HashMap<TypeId, Box<AnySend>>>,
}

impl CallbackHandler {
	// /// # Panics
	// /// If the provided callback type is not what is stored.
	// pub(crate) fn data<C: CallbackRaw>(&self) -> &C {
	// 	self.callback_impl.downcast_ref::<C>().unwrap()
	// }

	// /// # Panics
	// /// If the provided callback type is not what is stored.
	// pub(crate) fn data_mut<C: CallbackRaw>(&mut self) -> &mut C {
	// 	self.callback_impl.downcast_mut::<C>().unwrap()
	// }

	fn new_raw<C: CallbackRaw>(steam: &SteamInterface) -> Self {
		Self {
			callback_impl: Box::from(C::register(steam, Private)),
			on_callback_fn: Box::new(|any_send, void_ptr, _| {
				let callback = any_send.downcast_mut::<C>().unwrap();
				let c_data = unsafe { &*(void_ptr as *const C::CType) };

				unsafe {
					callback.on_callback(c_data, Private);
				}
			}),
			listeners: None,
		}
	}

	fn new_pub<C: Callback>(steam: &SteamInterface) -> Self
	where
		<C as CallbackRaw>::Output: Clone,
	{
		Self {
			callback_impl: Box::from(C::register(steam, Private)),
			on_callback_fn: Box::new(|any_send, void_ptr, listeners| {
				let callback = any_send.downcast_mut::<C>().unwrap(); //unwrap panicked
				let c_data = unsafe { &*(void_ptr as *const C::CType) };

				unsafe {
					let output = callback.on_callback(c_data, Private);
					let listeners = listeners.unwrap().values_mut().map(|any| any.downcast_mut::<Box<C::Fn>>().unwrap().as_mut());

					callback.call_listeners(listeners, &output, Private);
				}
			}),
			listeners: Some(HashMap::new()),
		}
	}
}

static_assertions::assert_impl_all!(CallbackHandler: Send, Sync);

impl Debug for CallbackHandler {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CallBackHandler")
			.field("callback_impl", &self.callback_impl)
			.field("callback_fn", &format!("Box<fn@{:p}>", self.on_callback_fn.as_ref() as *const _))
			.finish()
	}
}

/// Bridge between the [`CallManager`]'s [`Dispatched`] and its [`CallFuture`]s.
#[derive(Debug)]
#[must_use]
pub(crate) struct CallChannel {
	atomic_waker: AtomicWaker,

	/// Data received from the call.
	call_result: Mutex<Option<CallResult>>,
}

impl CallChannel {
	pub(crate) fn register(&self, waker: &Waker) {
		self.atomic_waker.register(waker);
	}

	/// Attempt to get the [`CallResult`], [`Box`] it, and call [`Dispatch::post`].
	pub(crate) fn post<D: Dispatch>(&self, dispatch: &mut D) -> Poll<Result<D::Output, CallError<D::Error>>> {
		let mut guard = self.call_result.lock().unwrap();

		let Some(call_result) = guard.as_mut() else {
			return Poll::Pending;
		};

		//take ownership
		let CallResult(call_result) = replace(call_result, CallResult(Err(CallFutureError::Moved)));

		//`MutexGuard<T>` does not behave like `&mut T`
		//it doesn't know how to drop early
		//so we do it manually to allow exclusive mutable access to self
		drop(guard);

		//now that the `Box<c_void>` has been turned into a box with the same memory layout as what was allocated
		//we can be sure it is safe to drop
		let c_box = match call_result {
			Ok(incog_box) => unsafe { incog_box.identify() },
			Err(error) => return Poll::Ready(Err(error.into())),
		};

		Poll::Ready(dispatch.post(c_box, Private).map_err(|error| CallError::Specific(error)))
	}

	/// Silently fails if data has already been sent.
	fn send(&self, result: CallResult) -> Result<(), CallChannelFilled> {
		let mut guard = self.call_result.lock().unwrap();

		if guard.is_some() {
			return Err(CallChannelFilled);
		}

		*guard = Some(result);

		self.atomic_waker.wake();

		Ok(())
	}
}

impl Drop for CallChannel {
	fn drop(&mut self) {
		let Ok(mut guard) = self.call_result.lock() else {
			return;
		};

		let opt_ref = guard.deref_mut();

		match opt_ref {
			//the value was still pending
			//so place an error to let the CallManager know
			//it should deallocate what was going to be a Box<Dispatch::CData>
			//if this is the IncognitoBox then it is dropped due to this
			None | Some(CallResult(Ok(_))) => *opt_ref = Some(CallResult(Err(CallFutureError::EarlyDrop))),

			//its an error, nothing special to do
			Some(CallResult(Err(_))) => {}
		}
	}
}

/// The [`CallChannel`] already had data sent.
#[derive(Debug)]
struct CallChannelFilled;

impl Display for CallChannelFilled {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("CallChannel already has a call result filled in")
	}
}

impl std::error::Error for CallChannelFilled {}

/// An asycnhronous call from the Steam API / Steamworks.
/// Use [`.await`] to get the call result.
///
/// [`.await`]: https://doc.rust-lang.org/std/keyword.await.html
#[derive(Debug)]
#[must_use]
pub struct CallFuture<D: Dispatch> {
	channel: Arc<CallChannel>,
	dispatch: D,
}

impl<D: Dispatch + Unpin> CallFuture<D> {
	/// See [`CallChannel::post`].
	pub(crate) fn post(&mut self) -> Poll<Result<D::Output, CallError<D::Error>>> {
		self.channel.post::<D>(&mut self.dispatch)
	}

	/// Register the waker so the [`CallManager`] can wake the thread once the result has arrived.
	pub(crate) fn register(&self, waker: &Waker) {
		self.channel.atomic_waker.register(waker);
	}
}

impl<D: Dispatch + Unpin> std::future::Future for CallFuture<D> {
	type Output = Result<D::Output, CallError<D::Error>>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let CallFuture { channel, dispatch } = self.deref_mut();

		channel.register(cx.waker());
		channel.post::<D>(dispatch)
	}
}

#[derive(Debug)]
pub struct CallManager {
	callbacks: HashMap<i32, CallbackHandler>,
	dispatches: HashMap<sys::SteamAPICall_t, Dispatched>,
	steam: SteamChild,
}

impl CallManager {
	pub(crate) fn new(steam: SteamChild) -> Self {
		Self {
			callbacks: HashMap::new(),
			dispatches: HashMap::new(),
			steam,
		}
	}

	/// Queues the dispatched Steam API call and returns a future which yields  
	/// `Result<Dispatch::Output, CallError<Dispatch::Error>>`.
	pub(crate) fn dispatch<D: Dispatch>(&mut self, mut dispatch: D) -> CallFuture<D> {
		CallFuture {
			channel: self.dispatch_manual::<D>(&mut dispatch),
			dispatch,
		}
	}

	/// Queues the dispatched Steam API call and returns the created channel.  
	/// This created channel has to be manually polled with [`CallChannel::post`].
	pub(crate) fn dispatch_manual<D: Dispatch>(&mut self, dispatch: &mut D) -> Arc<CallChannel> {
		let layout = Layout::new::<D::CType>();

		let channel = Arc::new(CallChannel {
			atomic_waker: AtomicWaker::new(),
			call_result: Mutex::new(None),
		});

		self.dispatches.insert(
			unsafe { dispatch.dispatch(Private) },
			Dispatched {
				channel: Arc::clone(&channel),
				layout,

				#[cfg(debug_assertions)]
				dispatch_type_name: type_name::<D>(),
			},
		);

		channel
	}

	/// # Safety
	/// This is only for types that implement [`CallbackRaw`] and not [`Callback`].
	pub(crate) unsafe fn get_or_register_raw<C: CallbackRaw>(&mut self) -> &mut CallbackHandler {
		let entry = self.callbacks.entry(C::CALLBACK_ID);

		entry.or_insert_with(|| CallbackHandler::new_raw::<C>(self.steam.get().deref()))
	}

	pub(crate) fn get_or_register_pub<C: Callback>(&mut self) -> &mut CallbackHandler
	where
		<C as CallbackRaw>::Output: Clone,
	{
		let entry = self.callbacks.entry(C::CALLBACK_ID);

		entry.or_insert_with(|| CallbackHandler::new_pub::<C>(self.steam.get().deref()))
	}

	/// Registers a function to be called everytime a callback is ran.
	/// ```rust
	/// # use rgpr_steamworks::call::CallManager;
	/// use rgpr_steamworks::interfaces::apps::DlcInstalled;
	///
	/// // Just a type to be used as an identifier.
	/// // IDs are unique to only the callback,
	/// // meaning you can re-use the ID for different callback types.
	/// struct JohnDoe;
	///
	/// # fn example_env(call_manager: &mut CallManager) {
	/// // Create a listener for the "DlcInstalled" callback, with the ID "JohnDoe"
	/// call_manager.listen::<DlcInstalled, JohnDoe>(|app_id| {
	///     println!("dlc {app_id} installed!");
	///
	///     // Celebrate, and inform the player that your 300GB DLC is done downloading
	/// });
	/// # }
	/// ```
	///
	/// Call [`remove_listener`] with the same callback and ID types to remove the listener.
	///
	/// Boxed functions will behave exactly the same as unboxed functions.
	/// There is no advantage to boxing or unboxing functions, therefore: you should leave the function as is.
	///
	/// [`remove_listener`]: Self::remove_listener
	pub fn listen<C: Callback, ID>(&mut self, listener_fn: impl Unsize<C::Fn>) -> Option<Box<C::Fn>>
	where
		ID: ?Sized + 'static,
		<C as CallbackRaw>::Output: Clone,
	{
		let boxed_fn: Box<C::Fn> = Box::new(listener_fn) as Box<C::Fn>;

		self.listen_box::<C, ID>(boxed_fn)
	}

	/// Same as [`listen`], but for listener functions that are already [boxed](Box).
	///
	/// [`listen`]: Self::listen
	pub(crate) fn listen_box<C: Callback, ID>(&mut self, listener_fn: Box<C::Fn>) -> Option<Box<C::Fn>>
	where
		ID: ?Sized + 'static,
		<C as CallbackRaw>::Output: Clone,
	{
		let callback_handler = self.get_or_register_pub::<C>();
		let listener_fns = callback_handler
			.listeners
			.as_mut()
			.expect("report this - listen on callback without listeners collection, see CallManager::register_raw for safety concerns");
		let fn_box: Box<C::Fn> = listener_fn.into();

		//because in order to cast a type as dyn Any
		//its size needs to be known
		//the only way we can do this is Box<Box<C::Fn>>
		//which isn't toooo terrible
		//definitely not good though
		let box_box = Box::new(fn_box);

		println!("register with id {:?}, current reg: {listener_fns:?}", TypeId::of::<ID>());

		Some(*listener_fns.insert(TypeId::of::<ID>(), box_box)?.downcast().unwrap())
	}

	/// Register a Callback that allows listeners.
	/// Will override any existing registration.
	pub(crate) fn register_pub<C: Callback>(&mut self) -> &mut CallbackHandler
	where
		<C as CallbackRaw>::Output: Clone,
	{
		self.callbacks.insert(C::CALLBACK_ID, CallbackHandler::new_pub::<C>(self.steam.get().deref()));

		self.callbacks.get_mut(&C::CALLBACK_ID).unwrap()
	}

	/// Will override any existing registration.
	///
	/// # Safety
	/// This is only for registering types that implement [`CallbackRaw`] and not [`Callback`].
	/// If the type implements [`Callback`], use [`register_pub`] instead.
	///
	/// If this function is used when [`register_pub`] should have been used, [`listen`] will panic.
	///
	/// [`register_pub`]: Self::register_pub
	pub(crate) unsafe fn register_raw<C: CallbackRaw + Send + Sync>(&mut self) -> &mut CallbackHandler {
		self.callbacks.insert(C::CALLBACK_ID, CallbackHandler::new_raw::<C>(self.steam.get().deref()));

		self.callbacks.get_mut(&C::CALLBACK_ID).unwrap()
	}

	/// Removes a listener function registered with [`listen`].
	///
	/// [`listen`]: Self::listen
	pub fn remove_listener<C: Callback, ID: ?Sized + 'static>(&mut self) -> Option<Box<C::Fn>>
	where
		<C as CallbackRaw>::Output: Clone,
	{
		let callback_handler = self.get_or_register_pub::<C>();
		let listener_fns = callback_handler.listeners.as_mut().unwrap();
		let removed = listener_fns.remove(&TypeId::of::<ID>())?;

		if listener_fns.is_empty() && !C::KEEP_REGISTERED {
			self.callbacks.remove(&C::CALLBACK_ID);
		}

		Some(*removed.downcast::<Box<C::Fn>>().unwrap())
	}

	/// Runs [callbacks] and retrieves dispatched call results.
	///
	/// [callbacks]: Callback
	pub fn run(&mut self) {
		//look at the comment in steam_api.h above SteamAPI_ManualDispatch_Init
		//line 166 of writing
		const CALL_COMPLETED_CALLBACK_ID: i32 = sys::SteamAPICallCompleted_t_k_iCallback as i32;

		unsafe {
			let pipe = sys::SteamAPI_GetHSteamPipe();
			let mut callback_msg: sys::CallbackMsg_t = zeroed();

			sys::SteamAPI_ManualDispatch_RunFrame(pipe);

			while sys::SteamAPI_ManualDispatch_GetNextCallback(pipe, &mut callback_msg as *mut _) {
				let callback_id = callback_msg.m_iCallback;

				//check for dispatched API call results
				if callback_id == CALL_COMPLETED_CALLBACK_ID {
					let call: &mut sys::SteamAPICallCompleted_t = &mut *(callback_msg.m_pubParam as *mut sys::SteamAPICallCompleted_t);
					let call_id: sys::SteamAPICall_t = call.m_hAsyncCall;

					let Some(dispatch) = self.dispatches.remove(&call_id) else {
						unreachable!("received call result with call_id {call_id} not in call manager");
					};

					//if the call to the steam API failed
					let mut failed = true;

					//we don't have the type here
					//but the endpoint does!
					let mut incog_box = IncognitoBox::from_layout(dispatch.layout);

					//true if we should deallocate the allocation
					//this gets set to false if the allocation has been sent off to the future for usage
					let mut send_err = true;

					assert_eq!(call.m_cubParam as usize, dispatch.layout.size(), "call result cubParam should match layout size");

					if sys::SteamAPI_ManualDispatch_GetAPICallResult(pipe, call_id, incog_box.as_ptr() as _, call.m_cubParam as c_int, call.m_iCallback, &mut failed) {
						//TODO: research or ask Valve for what the bool returned signifies vs the bool from the pointer
						//because it's possible the data could be good even if this if statement below fails
						//for example: the CType is a union of the good data and an error enum/message
						//and we know which based on this bool
						//because... you know...
						//Valve probably makes good decisions for steamworks since theres more than 100,000 games that use it
						//...right?
						if !failed {
							let _ = dispatch.channel.send(CallResult(Ok(incog_box)));
							send_err = false;
						} else {
							//check if the allocation was modified even though the failed ptr is true
							#[cfg(debug_assertions)]
							{
								let alloc_bytes = slice::from_raw_parts::<u8>(incog_box.as_ptr(), dispatch.layout.size());
								let mut stdout = stdout();

								writeln!(
									stdout,
									"SteamAPI_ManualDispatch_GetAPICallResult alloc addr {:p} was initialized zero, but contains non-zero bytes after a partial failure",
									incog_box.as_ptr()
								)
								.unwrap();
								writeln!(stdout, "call ID: {call_id}").unwrap();
								writeln!(stdout, "dispatch type name: {}", dispatch.dispatch_type_name).unwrap();

								for byte in alloc_bytes {
									write!(stdout, "{byte} ").unwrap();
								}

								stdout.flush().unwrap();
							}
						}
					}

					//we always need to send something
					//otherwise the CallFuture could wait forever
					if send_err {
						//if we can't send the error
						//it's because CallFutureError::EarlyDrop has already been sent
						let _ = dispatch.channel.send(CallResult(Err(CallFutureError::Failed)));
					}
				} else if let Some(CallbackHandler {
					callback_impl,
					on_callback_fn,
					listeners,
				}) = self.callbacks.get_mut(&callback_id)
				{
					//the data behind callback_msg.m_pubParam must be used before the end of this iteration
					on_callback_fn.as_mut()(callback_impl.as_mut(), callback_msg.m_pubParam as *const c_void, listeners.as_mut());
				}

				//TODO: maybe attempt to call this when unwinding from this loop?
				sys::SteamAPI_ManualDispatch_FreeLastCallback(pipe);
			}
		}
	}
}

impl Drop for CallManager {
	fn drop(&mut self) {
		for (_, Dispatched { channel, .. }) in self.dispatches.drain() {
			//its okay if this fails
			//since having CallFutureError::EarlyDrop already sent is to be expected
			let _ = channel.send(CallResult(Err(CallFutureError::Shutdown)));
		}
	}
}

/// Calls [`CallManager::run`] routinely.
#[derive(Debug)]
pub struct CallThread {
	/// # Safety
	/// Becomes uninit after [`Drop::drop`].
	thread: MaybeUninit<CallThreadInner>,
}

impl CallThread {
	/// Manages a thread for automatically running the [`CallManager`].
	pub(crate) fn new(interval: Duration, steam: SteamChild) -> Self {
		Self {
			thread: MaybeUninit::new(CallThreadInner::new(interval, steam)),
		}
	}

	/// # Panics
	/// If the thread panicked or was shutdown.
	pub fn start(&mut self) {
		unsafe { self.thread.assume_init_mut() }.send_command(CallThreadCommand::Start).unwrap()
	}

	/// Suspends running the [`CallManager`] causing the thread to only wait for a command.
	///
	/// # Panics
	/// If the thread panicked or was shutdown.
	pub fn stop(&mut self) {
		unsafe { self.thread.assume_init_mut() }.send_command(CallThreadCommand::Stop).unwrap()
	}
}

impl Drop for CallThread {
	fn drop(&mut self) {
		unsafe { replace(&mut self.thread, MaybeUninit::uninit()).assume_init() }.kill();
	}
}

/// Used by [`CallThreadInner`] for control flow.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CallThreadCommand {
	/// Start the callback loop.
	Start,

	/// Stop the callback loop.
	Stop,

	/// Kill the callback thread.
	/// Using [`CallThreadInner::kill`] is preferrable.
	Kill,
}

/// The thread handle and channel use by [`CallThread`].
#[derive(Debug)]
struct CallThreadInner {
	command_tx: Sender<CallThreadCommand>,
	handle: JoinHandle<()>,
}

impl CallThreadInner {
	/// # Panics
	/// If the OS fails to create the thread.
	fn new(interval: Duration, steam_child: SteamChild) -> Self {
		let (command_tx, command_rx) = channel::<CallThreadCommand>();

		let handle = thread::Builder::new()
			.name(String::from(CALL_THREAD_NAME))
			.spawn(move || Self::call_loop(interval, steam_child, command_rx))
			.expect("failed to create CallManager thread");

		Self { command_tx, handle }
	}

	fn call_loop(interval: Duration, steam_child: SteamChild, command_rx: mpsc::Receiver<CallThreadCommand>) {
		let mut run = false;
		let command_rx = command_rx;

		loop {
			if run {
				let hit_time = Instant::now() + interval;

				match command_rx.try_recv() {
					Ok(CallThreadCommand::Stop) => {
						run = false;

						continue;
					}

					Ok(CallThreadCommand::Start) | Err(TryRecvError::Empty) => {}
					Ok(CallThreadCommand::Kill) | Err(TryRecvError::Disconnected) => return,
				}

				let Some(steam) = steam_child.try_get() else {
					return;
				};

				let mut guard = steam.call_manager_lock();

				guard.run();
				drop(guard); //explicit drop for significant drop

				thread::sleep(Instant::now().saturating_duration_since(hit_time));
			} else {
				match command_rx.recv() {
					Ok(CallThreadCommand::Start) => run = true,
					Ok(CallThreadCommand::Stop) => continue,
					Ok(CallThreadCommand::Kill) | Err(RecvError) => return,
				}
			};
		}
	}

	fn kill(mut self) {
		if self.send_command(CallThreadCommand::Kill).is_ok() {
			self.handle.join().unwrap();
		}
	}

	fn send_command(&mut self, command: CallThreadCommand) -> Result<(), SendError<CallThreadCommand>> {
		//using `.send` does not require mutability,
		//but a mutable reference does rule out race conditions from being an internal issue
		self.command_tx.send(command)
	}
}

/// A single call dispatched through the Steam API.
#[derive(Debug)]
struct Dispatched {
	/// [CallChannel] to send data to a [CallFuture].
	channel: Arc<CallChannel>,

	/// Memory layout of the C type yielded from a call's result.
	layout: Layout,

	/// Type name of Dispatch implementer.
	#[cfg(debug_assertions)]
	#[doc(hidden)]
	dispatch_type_name: &'static str,
}

/// Implemented by Steam API callbacks that allow for listeners.
/// Implmenting types can be passed to [`listen`] as a generic type.
///
/// Callbacks are events that may be paired with data.
/// If a callback occurs, the [`CallManager`] calls all of the callback's registered listeners
///
/// [`listen`]: CallManager::listen
pub trait Callback: CallbackRaw
where
	<Self as CallbackRaw>::Output: Clone,
{
	/// Set to `true` to keep the implementing type registered in the
	/// [`CallManager`] even after all of its listeners have been removed.
	#[doc(hidden)]
	const KEEP_REGISTERED: bool = false;

	/// The type of this callback's listener functions.
	type Fn: ?Sized + Any + Send + Sync;

	/// Work around for the lack of a stable [`Tuple`] trait.
	/// Upon stabilization, the `Callback` trait will be corrected to no longer need this.
	///
	/// [`Tuple`]: https://doc.rust-lang.org/std/marker/trait.Tuple.html
	#[doc(hidden)]
	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: Private);

	/// Calls [`call_listener`] for all listener functions in the iterator,
	/// cloning `params` once for each listener.
	#[doc(hidden)]
	fn call_listeners<'a>(&mut self, listener_fns: impl Iterator<Item = &'a mut Self::Fn>, params: &Self::Output, _: Private) {
		for listener_fn in listener_fns {
			self.call_listener(listener_fn, params.clone(), Private);
		}
	}
}

/// Implemented to support a specific Steam API callback.
/// These are used by rgpr_steamworks internally,
/// see [`Callback`]'s implementers for what types allow listening.
///
/// If `CallbackRaw` is implemented and [`Callback`] is not,
/// the interface defined at the top of the callback's module will likely
/// have functions that fulfil the callback's purpose.
///
/// # Safety
/// `CType` must match the type associated with the `CALLBACK_ID`.
#[doc(hidden)]
pub unsafe trait CallbackRaw: Sized + Send + Sync + 'static {
	const CALLBACK_ID: i32;

	/// A raw pointer of the type will be sent to a different thread,
	/// where the `CType` will be provided as a reference to [`on_callback`].
	///
	/// `Copy` is required strictly to ensure the data is safe to use in an [`IncognitoBox`].
	///
	/// [`on_callback`]: Self::on_callback
	type CType: Copy + Send + Sync;

	/// The type returned from `on_callback`.
	/// Used by [`Callback`] for calling listener functions.
	type Output;

	/// Called when the targetted callback was received.
	/// # Safety
	/// Ensure any data kept outside this function from the [`CType`] is copied if it is behind a raw pointer.
	/// Otherwise, the data behind the pointer may be deallocated.
	///
	/// [`CType`]: Self::CType
	unsafe fn on_callback(&mut self, c_data: &Self::CType, _: Private) -> Self::Output;

	fn register(steam: &SteamInterface, _: Private) -> Self;
}

/// Implementations should initiate a call to the Steam API
/// which returns a `SteamAPICall_t`.
/// The [`CallManager`] will then take the result, and pass it to `post`.
///
/// # Safety
/// `Self::CType` must match the type returned from the dispatched call result.
#[doc(hidden)]
pub unsafe trait Dispatch: Send {
	/// The type that the SteamAPI provides in the call result.
	/// This will be provided to [post](Dispatch::post).
	///
	/// # Safety
	/// This type must not require destructors.
	/// `Drop` implementations may not be called.
	type CType: Send + Sync;

	/// The type returned for successful calls.
	type Output;

	/// The type returned for failed calls.
	type Error: Debug + std::error::Error;

	/// For dispatching the asynchronous call.
	/// Asynchronous calls in the Steam API always return a `SteamAPICall_t`.
	/// If your call to the Steam API returns its result in a named callback instead,
	/// you will have to setup your own channel between the callback and your `async fn`.
	unsafe fn dispatch(&mut self, _: Private) -> sys::SteamAPICall_t;

	/// Copy fields from the `c_data` here for use inside the callback or for output.
	/// Although it should be safe, avoid holding onto the `CType` itself.
	/// Never serve the raw `CType` outside of the crate.
	fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error>;
}

//TODO: investigate if SteamAPI_ReleaseCurrentThreadMemory is necessary
//for game servers: SteamGameServer_ReleaseCurrentThreadMemory
//could try by attaching as many interfaces as possible,
//calling async fns and triggering callbacks from as many different threads as possible
//and recording the memory with a debugger
//then doing some manual analysis or using a heatmap
//
//if the fn does need to be called,
//it should occasionally be called in `CallFuture`'s `Future::poll` impl
