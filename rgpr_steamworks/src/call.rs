use crate::dt::SteamId;
use crate::error::{CallError, CallFutureError};
use crate::interfaces::{SteamChild, SteamInterface};
use crate::{sys, Private};
use futures::task::AtomicWaker;
use std::alloc::{alloc_zeroed, dealloc, Layout};
use std::any::{type_name, Any, TypeId};
use std::collections::HashMap;
use std::ffi::{c_int, c_void};
use std::fmt::{Debug, Display, Formatter};
use std::io::{stdout, Write};
use std::mem::{replace, zeroed, MaybeUninit};
use std::ops::DerefMut;
use std::pin::Pin;
use std::process::Output;
use std::sync::mpsc::{channel, Receiver, RecvError, SendError, Sender, TryRecvError};
use std::sync::{Arc, Mutex, Weak};
use std::task::{Context, Poll, Waker};
use std::thread::JoinHandle;
use std::time::{Duration, Instant, SystemTime};
use std::{slice, thread};

type AnySend = dyn Any + Send + Sync;

/// Result of a call to the Steam API.
/// Upon storing the `Ok(*mut c_void)`, the following is guaranteed:
/// - The pointer points to an allocation that is the exact memory layout of a [`CType`].
/// There is no reflection for the [`CType`], so the type must be known elsewhere.
/// - The pointer is unique, no other instances/copies of the pointer exists anywhere but this CallResult.
/// - The data pointed to implements [Send] + [Sync]
/// - The data pointed to does not have a thread-dependent destructor
///
/// [`CType`]: Dispatch::CType
#[doc(hidden)]
#[derive(Debug)]
#[repr(transparent)]
struct CallResult(Result<*mut c_void, CallFutureError>);

/// SAFETY: See guarantees for Ok variant above.
unsafe impl Send for CallResult {}
unsafe impl Sync for CallResult {}

impl From<Result<*mut c_void, CallFutureError>> for CallResult {
	fn from(value: Result<*mut c_void, CallFutureError>) -> Self {
		Self(value)
	}
}

/// Performs callback-specific tasks and optionally calls listeners.
/// Manual impls of Send + Sync are because of the raw pointer.
/// The safety of their implementation is described in [CallResult].
pub(crate) struct CallbackHandler {
	/// The type implementing CallbackRaw
	callback_impl: Box<AnySend>,

	on_callback_fn: Box<dyn FnMut(&mut AnySend, *const c_void, Option<&mut HashMap<TypeId, Box<AnySend>>>) + Send + Sync>,

	//TODO: create a contiguous unsized vec for trait objects of all the same size
	listeners: Option<HashMap<TypeId, Box<AnySend>>>,
}

impl CallbackHandler {
	/// # Panics
	/// If the provided callback type is not what is stored.
	pub(crate) fn data<C: CallbackRaw>(&self) -> &C {
		self.callback_impl.downcast_ref::<C>().unwrap()
	}

	/// # Panics
	/// If the provided callback type is not what is stored.
	pub(crate) fn data_mut<C: CallbackRaw>(&mut self) -> &mut C {
		self.callback_impl.downcast_mut::<C>().unwrap()
	}

	fn new_raw<C: CallbackRaw>(steam: &SteamInterface) -> Self {
		Self {
			callback_impl: Box::from(C::register(steam)),
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
			callback_impl: Box::from(C::register(steam)),
			on_callback_fn: Box::new(|any_send, void_ptr, listeners| {
				let callback = any_send.downcast_mut::<C>().unwrap();
				let c_data = unsafe { &*(void_ptr as *const C::CType) };

				unsafe {
					let output = callback.on_callback(c_data, Private);
					let listeners = listeners.unwrap().values_mut().map(|any| any.downcast_mut::<Box<C::Fn>>().unwrap().as_mut());

					callback.call_listeners(listeners, &output);
				}
			}),
			listeners: Some(HashMap::new()),
		}
	}
}

unsafe impl Send for CallbackHandler {}
unsafe impl Sync for CallbackHandler {}

impl Debug for CallbackHandler {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CallBackHandler")
			.field("callback_impl", &self.callback_impl)
			.field("callback_fn", &format!("Box<fn@{:p}>", self.on_callback_fn.as_ref() as *const _))
			.finish()
	}
}

/// Bridge between the [CallManager]'s [Dispatched] and its [CallFuture]s.
#[derive(Debug)]
#[must_use]
pub(crate) struct CallChannel {
	atomic_waker: AtomicWaker,

	/// Data received from the call.
	call_result: Mutex<Option<CallResult>>,

	/// The layout of `Dispatch::CType`.
	/// Necessary for deallocating the contained Box.
	layout: Layout,
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
		let CallResult(call_result) = replace(call_result, Err(CallFutureError::Moved).into());

		//`MutexGuard<T>` does not behave like `&mut T`
		//it doesn't know how to drop early
		//so we do it manually to allow exclusive mutable access to self
		drop(guard);

		//now that the `Box<c_void>` has been turned into a box with the same memory layout as what was allocated
		//we can be sure it is safe to drop
		let c_box = match call_result {
			Ok(void_ptr) => unsafe { Box::from_raw(void_ptr as *mut D::CType) },
			Err(error) => return Poll::Ready(Err(error.into())),
		};

		Poll::Ready(dispatch.post(c_box, Private).map_err(|error| CallError::Specific(error)))
	}

	/// Silently fails if data has already been sent.
	fn send(&self, result: impl Into<CallResult>) -> Result<(), CallChannelFilled> {
		let mut guard = self.call_result.lock().unwrap();

		if guard.is_some() {
			return Err(CallChannelFilled);
		}

		*guard = Some(result.into());

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
			//we had a ptr to Dispatch::CData
			//carefully deallocate it
			Some(CallResult(Ok(void_ptr))) => unsafe {
				dealloc(*void_ptr as _, self.layout);

				//then leave an error behind
				*opt_ref = Some(CallResult(Err(CallFutureError::EarlyDrop)))
			},

			//its an error, nothing special to do
			Some(CallResult(Err(_))) => {}

			//the value was still pending
			//so place an error to let the CallManager know
			//it should deallocate what was going to be a Box<Dispatch::CData>
			None => *opt_ref = Some(CallResult(Err(CallFutureError::EarlyDrop))),
		}
	}
}

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
	pub(crate) steam: Weak<SteamInterface>,
}

impl CallManager {
	pub(crate) fn new() -> Self {
		Self {
			callbacks: HashMap::new(),
			dispatches: HashMap::new(),
			steam: Weak::new(),
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
			layout,
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

		entry.or_insert_with(|| CallbackHandler::new_raw::<C>(self.steam.upgrade().unwrap().as_ref()));

		self.register_raw::<C>()
	}

	pub(crate) fn get_or_register_pub<C: Callback>(&mut self) -> &mut CallbackHandler
	where
		<C as CallbackRaw>::Output: Clone,
	{
		let entry = self.callbacks.entry(C::CALLBACK_ID);

		entry.or_insert_with(|| CallbackHandler::new_raw::<C>(self.steam.upgrade().unwrap().as_ref()));

		self.register_pub::<C>()
	}

	/// Registers a function to be called everytime a callback is ran.
	/// Requires:
	/// - The type of callback
	/// - A type as an identifier
	/// - A boxed function
	/// ```rs
	/// # use rgpr_steamworks::call::CallManager;
	/// use rgpr_steamworks::interfaces::apps::DlcInstalled;
	///
	/// // Just a type to be used as an identifier.
	/// // IDs are unique to only the callback,
	/// // meaning you can re-use the ID for different callback types.
	/// struct JohnDoe;
	///
	/// # fn example_env(call_manager: &mut CallManager) {
	/// // Create a listener for the "DlcInstalled" callback, with the ID "SomethingUnique"
	/// call_manager.listen::<DlcInstalled, JohnDoe>(Box::new(|app_id| {
	///     println!("dlc {app_id} installed!");
	///
	///     // React here!
	/// }));
	/// # }
	/// ```
	///
	/// Call [`remove_listener`] with the same callback and ID types to remove the listener.
	///
	/// *(Should not panic, but can if the callback was incorrectly registered internally.)*
	///
	/// [`remove_listener`]: Self::remove_listener
	pub fn listen<C: Callback, ID: ?Sized + 'static>(&mut self, listener_fn: Box<C::Fn>) -> Option<Box<C::Fn>>
	where
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

		Some(*listener_fns.insert(TypeId::of::<ID>(), box_box)?.downcast::<Box<C::Fn>>().unwrap())
	}

	/// Register a Callback that allows listeners.
	pub(crate) fn register_pub<C: Callback>(&mut self) -> &mut CallbackHandler
	where
		<C as CallbackRaw>::Output: Clone,
	{
		self.callbacks.insert(C::CALLBACK_ID, CallbackHandler::new_pub::<C>(self.steam.upgrade().unwrap().as_ref()));

		self.callbacks.get_mut(&C::CALLBACK_ID).unwrap()
	}

	/// # Safety
	/// This is only for registering types that implement [`CallbackRaw`] and not [`Callback`].
	/// If the type implements [`Callback`], use [`register_pub`] instead.
	///
	/// If this function is used when [`register_pub`] should have been used, [`listen`] will panic.
	///
	/// [`register_pub`]: Self::register_pub
	pub(crate) unsafe fn register_raw<C: CallbackRaw + Send + Sync>(&mut self) -> &mut CallbackHandler {
		self.callbacks.insert(C::CALLBACK_ID, CallbackHandler::new_raw::<C>(self.steam.upgrade().unwrap().as_ref()));

		self.callbacks.get_mut(&C::CALLBACK_ID).unwrap()
	}

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
					let mut failed = false;

					//true if we should deallocate the allocation
					//this gets set to false if the allocation has been sent off to the future for usage
					let mut please_dealloc = true;
					let alloc = alloc_zeroed(dispatch.layout) as *mut c_void;

					assert_eq!(call.m_cubParam as usize, dispatch.layout.size(), "call result cubParam should match layout size");

					if sys::SteamAPI_ManualDispatch_GetAPICallResult(pipe, call_id, alloc, call.m_cubParam as c_int, call.m_iCallback, &mut failed) {
						//TODO: research or ask Valve for what the bool returned signifies vs the bool from the pointer
						//because it's possible the data could be good even if this if statement below fails
						//for example: the CType is a union of the good data and an error enum/message
						//and we know which based on this bool
						//because... you know...
						//Valve probably makes good decisions for steamworks since theres more than 100,000 games that use it
						//...right?
						if !failed {
							//please dealloc if we can't send
							//don't dealloc if there were no errors - because the value has been sent to the CallFuture successfully
							please_dealloc = dispatch.channel.send(Ok(alloc)).is_err();
						}

						//check if the allocation was modified even though the failed ptr is true
						#[cfg(debug_assertions)]
						{
							let alloc_bytes = slice::from_raw_parts::<u8>(alloc as _, dispatch.layout.size());
							let mut stdout = stdout();

							writeln!(
								stdout,
								"SteamAPI_ManualDispatch_GetAPICallResult alloc addr {alloc:p} was initialized zero, but contains non-zero bytes after a partial failure"
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

					if please_dealloc {
						dealloc(alloc as _, dispatch.layout);

						//if we can't send the error
						//it's because CallFutureError::EarlyDrop has already been sent
						let _ = dispatch.channel.send(Err(CallFutureError::Failed));
					}
				} else if let Some(CallbackHandler {
					callback_impl,
					on_callback_fn,
					listeners,
				}) = self.callbacks.get_mut(&callback_id)
				{
					//the data behind callback_msg.m_pubParam must be used before the end of this iteration
					on_callback_fn.as_mut()(callback_impl, callback_msg.m_pubParam as *const c_void, listeners.as_mut());
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
			let _ = channel.send(Err(CallFutureError::Shutdown));
		}
	}
}

/// Calls `CallManager::run` routinely.
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

	/// Starts running the [`CallManager`] on the interval specified in [`new`].
	/// # Panics
	/// If the thread panicked or was shutdown.
	///
	/// [`new`]: Self::new
	pub fn start(&mut self) {
		unsafe { self.thread.assume_init_mut() }.send_command(CallThreadCommand::Start).unwrap()
	}

	/// Suspends running the [`CallManager`] causing the thread to only wait for a command.
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
	fn new(interval: Duration, steam_child: SteamChild) -> Self {
		let (command_tx, command_rx) = channel::<CallThreadCommand>();

		let handle = thread::spawn(move || {
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

					let steam = steam_child.get();
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
		});

		Self { command_tx, handle }
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
	/// [CallManager] even after all of its listeners have been removed.
	const KEEP_REGISTERED: bool = false;

	type Fn: ?Sized + Any + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output);

	fn call_listeners<'a>(&mut self, listener_fns: impl Iterator<Item = &'a mut Self::Fn>, params: &Self::Output) {
		for listener_fn in listener_fns {
			self.call_listener(listener_fn, params.clone());
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
pub unsafe trait CallbackRaw: Sized + Send + Sync + 'static {
	#[doc(hidden)]
	const CALLBACK_ID: i32;

	/// A raw pointer of the type will be sent to a different thread,
	/// where the `CType` will be provided as a reference to [`on_callback`].
	///
	/// [`on_callback`]: Self::on_callback
	type CType: Send + Sync;

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

	fn register(steam: &SteamInterface) -> Self;
}

/// Implementations should initiate a call to the Steam API
/// which returns a `SteamAPICall_t`.
/// The [CallManager] will then take the result, and pass it to `post`.
///
/// # Safety
/// `Self::CType` must match the type returned from the dispatched call result.
#[doc(hidden)]
pub unsafe trait Dispatch: Send {
	/// The type that the SteamAPI provides in the call result.
	/// This will be provided to [post](Dispatch::post).
	///
	/// Although this type itself may not be sent between threads,
	/// a raw pointer to it will.
	/// This ensures
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
