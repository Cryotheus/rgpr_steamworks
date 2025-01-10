use crate::error::CallError;
use crate::steam_api::matchmaking::LobbyEnter;
use crate::steam_api::user_stats::UserStatsUnloaded;
use crate::Private;
use futures::task::AtomicWaker;
use mem::{replace, transmute};
use rgpr_steamworks_sys as sys;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::ffi::{c_int, c_void};
use std::fmt::{Debug, Formatter};
use std::future::Future;
use std::marker::PhantomData;
use std::mem;
use std::mem::{zeroed, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

#[derive(Debug)]
pub(crate) struct Callback {
	callback_fn: CallbackFn,
	listeners: HashMap<TypeId, Box<dyn Any + Send>>,
}

/// Yields the output of Steamworks functions which provide their results through callbacks instead.
#[must_use]
pub struct CallbackFuture<T: Send>(Arc<CallbackFutureChannel<T>>);

impl<T: Send> CallbackFuture<T> {
	/// # Deadlocks
	/// You **must** send a value using the sender from another thread.
	pub(crate) fn new() -> (CallbackFutureSender<T>, Self) {
		let channel = Arc::new(CallbackFutureChannel::<T> {
			waker: AtomicWaker::new(),
			data: Mutex::new(None),
		});

		(CallbackFutureSender(Arc::clone(&channel)), CallbackFuture(channel))
	}
}

impl<T: Debug + Send> Debug for CallbackFuture<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("CallbackFuture").field(&self.0).finish()
	}
}

impl<T: Send> Future for CallbackFuture<T> {
	type Output = T;

	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		let self_mut = self.get_mut();

		self_mut.0.waker.register(cx.waker());

		if let Some(value) = self_mut.0.take() {
			Poll::Ready(value)
		} else {
			Poll::Pending
		}
	}
}

/// Bridges between [CallbackFuture] and [CallbackFutureSender].
struct CallbackFutureChannel<T: Send> {
	waker: AtomicWaker,
	data: Mutex<Option<T>>,
}

impl<T: Debug + Send> Debug for CallbackFutureChannel<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("CallbackFutureChannel").field("waker", &self.waker).field("data", &self.data).finish()
	}
}

impl<T: Send> CallbackFutureChannel<T> {
	fn ready(&self) -> bool {
		self.data.lock().unwrap().is_some()
	}

	fn take(&self) -> Option<T> {
		self.data.lock().unwrap().take()
	}
}

/// Wakes and sends data to its [CallbackFuture].
#[must_use]
pub(crate) struct CallbackFutureSender<T: Send>(Arc<CallbackFutureChannel<T>>);

impl<T: Send> CallbackFutureSender<T> {
	/// Sends the value to the associated future.
	pub(crate) fn send(self, value: T) {
		*self.0.data.lock().unwrap() = Some(value);

		self.0.waker.wake();
	}
}

impl<T: Debug + Send> Debug for CallbackFutureSender<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_tuple("CallbackFutureSender").field(&self.0).finish()
	}
}

/// Yields the output of asynchronous Steamworks function calls.
#[derive(Debug)]
#[must_use]
pub struct CallFuture<T: SteamworksDispatch> {
	/// Becomes `None` if the call has finished polling.
	inner: Option<Arc<CallFutureChannel>>,
	parameters: T,
}

impl<T: SteamworksDispatch> CallFuture<T> {
	/// Returns `Some(true)` if there is only a single strong reference and no weak references for the inner [`CallFutureChannel`].
	/// Returns `Some(false)` if that is not the case.
	/// Returns `None` if the call has already been polled.
	fn is_unique(&self) -> Option<bool> {
		let Some(arc) = self.inner.as_ref() else {
			return None;
		};

		Some(Arc::strong_count(arc) == 1 && Arc::weak_count(arc) == 0)
	}

	fn waker(&self) -> &AtomicWaker {
		&self.inner.as_ref().unwrap().waker
	}
}

impl<T: SteamworksDispatch> Future for CallFuture<T> {
	type Output = Result<T::Output, CallError>;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
		self.waker().register(cx.waker());

		let mut call_state = self.inner.as_ref().unwrap().state.lock().unwrap();

		let alloc = match call_state.deref_mut() {
			CallState::Failed => return Poll::Ready(Err(CallError::Failed)),

			CallState::Pending => {
				return if self.is_unique().unwrap() {
					//if its unique, bad news: the call manager dropped the channel before setting a value
					//which means it probably crashed
					//or was suddenly shutdown
					Poll::Ready(Err(CallError::Shutdown))
				} else {
					//if the channel is shared, then it's still in-queue
					Poll::Pending
				};
			}

			CallState::Received(alloc) => unsafe { transmute::<Box<()>, Box<T::CType>>(replace(alloc, Box::new(()))) },
		};

		drop(call_state);

		let parameters = replace(&mut self.parameters, unsafe { zeroed() });
		let output = parameters.post(alloc, Private(()));

		Poll::Ready(Ok(output))
	}
}

/// Shared between a [CallFuture] and its associated [Dispatch.
#[derive(Debug, Default)]
struct CallFutureChannel {
	state: Mutex<CallState>,
	waker: AtomicWaker,
}

#[derive(Debug, Default)]
enum CallState {
	#[default]
	Pending,
	Failed,
	Received(Box<()>),
}

/// Function used when a Steamworks API callback is ran.
struct CallbackFn(Box<dyn FnMut(*mut c_void, &mut HashMap<TypeId, Box<dyn Any + Send>>) + Send>);

impl Debug for CallbackFn {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:p} -> {:p}", &self.0, self.0.deref())
	}
}

impl Deref for CallbackFn {
	type Target = dyn FnMut(*mut c_void, &mut HashMap<TypeId, Box<dyn Any + Send>>);

	fn deref(&self) -> &Self::Target {
		self.0.deref()
	}
}

impl DerefMut for CallbackFn {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.deref_mut()
	}
}

#[derive(Debug)]
pub struct CallbackListenerIter<'a, F: 'static> {
	iter_mut: std::collections::hash_map::IterMut<'a, TypeId, Box<dyn Any + Send>>,
	phantom: PhantomData<F>,
}

impl<'a, F: 'static> ExactSizeIterator for CallbackListenerIter<'a, F> {}

impl<'a, F: 'static> Iterator for CallbackListenerIter<'a, F> {
	type Item = (&'a TypeId, &'a mut F);

	fn next(&mut self) -> Option<Self::Item> {
		let (type_id, any_box) = self.iter_mut.next()?;
		let listener_fn: &'a mut F = any_box.downcast_mut().unwrap();

		Some((type_id, listener_fn))
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.iter_mut.size_hint()
	}
}

#[derive(Debug)]
struct Dispatch {
	/// The allocation where the call result should be placed.
	/// The necessary amount of memory is allocated at creation time.
	alloc: Box<()>,

	/// For sending the call result and waking the future.
	channel: Arc<CallFutureChannel>,
}

//TODO: wake the Call if the Arc used here is dropped
#[derive(Debug)]
pub struct SteamworksCallManager {
	callbacks: HashMap<i32, Callback>,
	dispatched_calls: HashMap<sys::SteamAPICall_t, Dispatch>,
}

impl SteamworksCallManager {
	pub(crate) fn new() -> Self {
		Self {
			dispatched_calls: Default::default(),
			callbacks: Default::default(),
		}
	}

	/// Dispatches an asynchronous call to the Steamworks interface.
	/// You will need to [`.await`](https://doc.rust-lang.org/std/keyword.await.html) the output until it is received by [`run_callbacks`](Self::run_callbacks).
	pub fn call<T: SteamworksDispatch>(&mut self, mut parameters: T) -> CallFuture<T> {
		let call_id = unsafe { parameters.dispatch(Private(())) };
		let channel_arc = Arc::new(CallFutureChannel::default());
		let alloc = unsafe { transmute::<Box<_>, Box<()>>(Box::<MaybeUninit<T::CType>>::new_uninit()) };

		self.dispatched_calls.insert(
			call_id,
			Dispatch {
				alloc,
				channel: Arc::clone(&channel_arc),
			},
		);

		CallFuture { inner: Some(channel_arc), parameters }
	}

	/// Calls the provided function every time the specified `C` callback is ran.
	/// `S` is the type for identifying the listener.
	/// Calling `remove_listener` with the same `C` and `S` types will remove it.
	pub fn listen<C: SteamworksCallback, S: 'static>(&mut self, listen_fn: C::ListenFn) {
		self.register_private::<C>().listeners.insert(TypeId::of::<S>(), Box::from(listen_fn));
	}

	/// Registers a callback for execution, without any listeners.
	pub(crate) fn register<C: SteamworksCallback>(&mut self) {
		debug_assert!(!C::REQUIRES_LISTENERS, "Callback requires listeners but was registered without one");
		self.register_private::<C>();
	}

	/// Registers a callback for execution without any listeners and returns it for modifaction.
	fn register_private<C: SteamworksCallback>(&mut self) -> &mut Callback {
		if self.callbacks.contains_key(&C::C_ENUM) {
			return unsafe { self.callbacks.get_mut(&C::C_ENUM).unwrap_unchecked() };
		}

		self.callbacks.insert(
			C::C_ENUM,
			Callback {
				callback_fn: CallbackFn(Box::new(|void_ptr, listeners| unsafe {
					//in a closure so we can perform the expected type conversions
					C::callback(
						void_ptr,
						&mut CallbackListenerIter {
							iter_mut: listeners.iter_mut(),
							phantom: PhantomData,
						},
						Private(()),
					);
				})),
				listeners: HashMap::new(),
			},
		);

		unsafe { self.callbacks.get_mut(&C::C_ENUM).unwrap_unchecked() }
	}

	/// Removes a registered [Callback].
	pub(crate) fn remove<C: SteamworksCallback>(&mut self) {
		self.callbacks.remove(&C::C_ENUM);
	}

	/// Removes a [Callback] if it has no listeners.
	pub(crate) fn remove_empty<C: SteamworksCallback>(&mut self) {
		let Some(callback) = self.callbacks.get(&C::C_ENUM) else {
			return;
		};

		if callback.listeners.is_empty() {
			self.callbacks.remove(&C::C_ENUM);
		}
	}

	/// Removes a listener added with [listen](Self::listen).
	pub fn remove_listener<C: SteamworksCallback, S: 'static>(&mut self) -> Option<C::ListenFn> {
		let callback = self.callbacks.get_mut(&C::C_ENUM)?;
		let listener = callback.listeners.remove(&TypeId::of::<S>());

		if C::REQUIRES_LISTENERS && callback.listeners.is_empty() {
			self.callbacks.remove(&C::C_ENUM);
		}

		//unwrap:
		Some(*listener?.downcast::<C::ListenFn>().unwrap())
	}

	/// Runs Steamworks callbacks.
	/// This should be done frequently, no less than once a second, no more than 100 times a second.
	/// The less frequently the callbacks are ran, the more latency for callbacks and dispatched function calls.
	/// Typically, this is run on frame in the game loop.
	/// To prevent deadlocking of polling [CallFuture] instances, it may be necessary to put this on a separate thread.
	pub fn run_callbacks(&mut self) {
		unsafe {
			//please refer to steam_api.h around line 180 for "why"
			let pipe = sys::SteamAPI_GetHSteamPipe();
			let mut callback = zeroed::<sys::CallbackMsg_t>();

			sys::SteamAPI_ManualDispatch_RunFrame(pipe);

			while sys::SteamAPI_ManualDispatch_GetNextCallback(pipe, &mut callback) {
				//check for single-dispatch call results
				if callback.m_iCallback == sys::SteamAPICallCompleted_t_k_iCallback as i32 {
					//TODO: check if `*/ as *mut _ as *mut //` is necessary
					let call: &mut sys::SteamAPICallCompleted_t = &mut *(callback.m_pubParam as *mut sys::SteamAPICallCompleted_t);
					let call_id = call.m_hAsyncCall.clone();

					let Some(Dispatch { mut alloc, channel }) = self.dispatched_calls.remove(&call_id) else {
						unreachable!("received call result with call_id not in call manager");
					};

					let alloc_mut = alloc.as_mut();
					let mut failed = false;

					if sys::SteamAPI_ManualDispatch_GetAPICallResult(
						pipe,
						call_id as sys::SteamAPICall_t,
						alloc_mut as *mut _ as *mut c_void,
						call.m_cubParam as c_int,
						call.m_iCallback,
						&mut failed,
					) {
						let mut guard = channel.state.lock().unwrap();
						*guard = if failed { CallState::Failed } else { CallState::Received(alloc) };

						//make absolutely sure
						drop(guard);
						channel.waker.wake();
					}
				} else if let Some(Callback { callback_fn, listeners }) = self.callbacks.get_mut(&callback.m_iCallback) {
					//it's a callback
					//TODO: fancy?
					//callback_fn.call(callback.m_pubParam as *mut _);
					callback_fn(callback.m_pubParam as *mut _, listeners);
				}

				sys::SteamAPI_ManualDispatch_FreeLastCallback(pipe);
			}
		}
	}
}

/// Callbacks are typically triggered by events from Steam's resources.
/// The exception to this are some functions which use the callback system like the call result system.
/// Functions using call results are called with data types that implement [`SteamworksDispatch`] instead of `SteamworksCallback` implementers.  
///
/// Should only be implemented by the crate.
pub trait SteamworksCallback: Debug + Send + 'static {
	/// The integer enumeration of the callback template.
	#[doc(hidden)]
	const C_ENUM: i32;

	/// Set to `true` if the callback should not be called without listeners.
	const REQUIRES_LISTENERS: bool = false;

	/// The type for listener functions.
	type ListenFn: Send;

	#[allow(private_interfaces)]
	#[doc(hidden)]
	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private);
}

/// Implemented on datatypes that are used to call external functions provided by Steamworks.  
///
/// Should only be implemented by the crate.
pub trait SteamworksDispatch: Debug + Unpin + Send {
	#![allow(private_interfaces)]

	/// The type returned by the SteamAPI.
	type CType;

	/// The type provided on the Rust side, available after polling a [`CallFuture`].
	type Output: Send + Sized;

	/// # Private
	/// Convert the C data type returned by the call to a Rust data type.
	/// Called right before the [CallFuture] returns `Self::Output`.
	#[doc(hidden)]
	fn post(self, call_result: Box<Self::CType>, _private: Private) -> Self::Output;

	/// # Private
	/// Call the Steamworks function from the FFI here.
	#[doc(hidden)]
	unsafe fn dispatch(&mut self, _private: Private) -> sys::SteamAPICall_t;
}
