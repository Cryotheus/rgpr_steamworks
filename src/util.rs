use crate::sys;
use std::ffi::{c_char, c_int, CStr, CString};
use std::fmt::{Debug, Formatter};
use std::mem;
use std::mem::{transmute, MaybeUninit};
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Maximum path length.
/// - `"windows"` = 260
/// - `"macos"` = 1024
/// - `"linux"` = 4096

#[cfg(target_os = "windows")]
pub const MAX_PATH: usize = 260;

#[cfg(target_os = "macos")]
pub const MAX_PATH: usize = 1024;

#[cfg(target_os = "linux")]
pub const MAX_PATH: usize = 4096;

pub type CStrArrayPath = CStrArray<MAX_PATH>;

/// For C strings that the Steam API wants us to allocate.
#[derive(Clone, Debug)]
#[doc(hidden)]
#[repr(transparent)]
pub struct CStrArray<const LEN: usize>([c_char; LEN]);

impl<const LEN: usize> CStrArray<LEN> {
	pub fn new() -> Self {
		Self([c_char::default(); LEN])
	}

	pub fn c_len(&self) -> c_int {
		self.0.len() as c_int
	}

	pub fn c_str(&self) -> &CStr {
		assert_eq!(*self.0.last().unwrap(), c_char::default(), "CStr buffer did not end with a null terminator");

		unsafe { CStr::from_ptr(self.0.as_ptr()) }
	}

	pub fn c_string(&self) -> CString {
		CString::from(self.c_str())
	}

	pub fn ptr(&mut self) -> *mut c_char {
		self.0.as_mut_ptr()
	}

	pub fn path(&self) -> &Path {
		let bytes = self.c_str().to_bytes();

		#[cfg(target_os = "linux")]
		{
			std::os::unix::ffi::OsStrExt::from_bytes(bytes).as_ref()
		}

		#[cfg(not(target_os = "linux"))]
		{
			std::str::from_utf8(bytes).unwrap().as_ref()
		}
	}

	pub fn to_string(mut self) -> String {
		if self.0[0] == c_char::default() {
			return String::new();
		}

		assert_eq!(*self.0.last().unwrap(), c_char::default(), "CStr buffer did not end with a null terminator");

		unsafe { CStr::from_ptr(self.ptr()) }.to_string_lossy().to_string()
	}

	/// Same as `to_string` but returns `None` if the string is empty.
	pub fn to_some_string(mut self) -> Option<String> {
		if self.0[0] == c_char::default() {
			return None;
		}

		assert_eq!(*self.0.last().unwrap(), c_char::default(), "CStr buffer did not end with a null terminator");

		Some(unsafe { CStr::from_ptr(self.ptr()) }.to_string_lossy().to_string())
	}
}

impl<const LEN: usize> AsRef<CStr> for CStrArray<LEN> {
	fn as_ref(&self) -> &CStr {
		self.c_str()
	}
}

impl<const LEN: usize> AsRef<Path> for CStrArray<LEN> {
	fn as_ref(&self) -> &Path {
		self.path()
	}
}

impl AsMut<sys::SteamErrMsg> for CStrArray<1024> {
	fn as_mut(&mut self) -> &mut sys::SteamErrMsg {
		unsafe { transmute::<&mut Self, &mut sys::SteamErrMsg>(self) }
	}
}

impl From<CStrArray<1024>> for sys::SteamErrMsg {
	fn from(value: CStrArray<1024>) -> Self {
		//this is essentially our assertion for sizes of `CStrArray<1024>` == `sys::SteamErrMsg`
		unsafe { transmute::<CStrArray<1024>, Self>(value) }
	}
}

/// Converts a char ptr into a `String`, returning `None` if it's empty or null.
/// If there are invalid UTF-8 codepoints, they will be replaced.
///
/// # Safety
/// The pointer must point to a valid null-terminated allocation, or be null.
#[doc(hidden)]
#[inline(always)]
pub unsafe fn some_string(char_ptr: *const c_char) -> Option<String> {
	if char_ptr.is_null() || *char_ptr == c_char::default() {
		None
	} else {
		Some(CStr::from_ptr(char_ptr).to_string_lossy().to_string())
	}
}

/// Turns a bool into `Result<(), SilentFailure>`  
/// where `true` is `Ok(())`  
/// and `false` is `Err(SilentFailure)`
#[cfg(feature = "steam")]
#[doc(hidden)]
#[inline(always)]
pub fn success(success: bool) -> Result<(), crate::error::SilentFailure> {
	if success {
		Ok(())
	} else {
		Err(crate::error::SilentFailure)
	}
}

/// For dealing with a specific issue in the external APIs:
/// Some lists can be mutated during iteration causing iterators to skip items or yield duplicates.
///
/// A simple solution would be to hold a `Mutex<()>` or `RwLock<()>` which would then be used as a lock when calling the functions that cause this issue,
/// but it may be undesirable if those functions' calls should nominally be allowed to overlap without deadlocking.
/// A `ForeignLock` acts as a `RwLock<()>` with the ability to delay calls until an exclusive write lock can be made.
///
/// The `Q` data type should be used to store calls that must be made later.
pub struct ForeignLock<Q> {
	queue: Mutex<Q>,

	/// Set to `true` if the `Q` is exposed,
	/// and `false` after a flush is performed.
	needs_flush: AtomicBool,

	/// Solely to keep track of readers and writers.
	state: RwLock<()>,
}

impl<Q> ForeignLock<Q> {
	pub fn new(queue: Q) -> Self {
		Self {
			queue: Mutex::new(queue),
			needs_flush: AtomicBool::new(false),
			state: Default::default(),
		}
	}
}

impl<Q: ForeignLockQueue> ForeignLock<Q> {
	/// Creates a read lock forcing writes to queue.
	/// # Panics
	/// If the internal `RwLock<()>` is poisoned.
	pub fn read(&self) -> ForeignReadGuard<Q> {
		ForeignReadGuard {
			read_guard: MaybeUninit::new((&self, self.state.read().unwrap())),
		}
	}

	/// Attempts to get a [`ForeignWriteGuardExclusive`] to call [`flush`].
	/// Returns `true` if the queue was flushed.
	///
	/// # Panics
	/// If the internal `Mutex<Q>` or `RwLock<()>` is poisoned.
	///
	/// [`flush`]: ForeignWriteGuardExclusive::flush
	pub fn try_flush(&self) -> bool {
		if let Some(mut exclusive) = self.try_write_exclusive() {
			exclusive.flush();

			true
		} else {
			false
		}
	}

	/// Attempts to get a [`ForeignWriteGuardExclusive`],
	/// and returns a [`ForeignWriteGuardQueue`] upon failure.
	///
	/// # Panics
	/// If the internal `Mutex<Q>` or `RwLock<()>` is poisoned.
	pub fn try_write(&self) -> ForeignWriteGuard<Q> {
		let queue = self.queue.lock().unwrap();

		match self.state.try_write() {
			Ok(write_lock) => ForeignWriteGuard::Exclusive(ForeignWriteGuardExclusive {
				queue,
				needs_flush: &self.needs_flush,
				write_lock,
			}),
			Err(_) => {
				self.needs_flush.store(true, Ordering::SeqCst);

				ForeignWriteGuard::Queue(ForeignWriteGuardQueue { queue })
			}
		}
	}

	/// Attempts to gain an exclusive lock `Some(RwLockWriteGuard<()>)`,
	/// returning `None` if unavailable.
	///
	/// # Panics
	/// If the internal `Mutex<Q>` or `RwLock<()>` is poisoned.
	pub fn try_write_exclusive(&self) -> Option<ForeignWriteGuardExclusive<Q>> {
		let queue = self.queue.lock().unwrap();

		match self.state.try_write() {
			Ok(write_lock) => Some(ForeignWriteGuardExclusive {
				queue,
				needs_flush: &self.needs_flush,
				write_lock,
			}),
			Err(_) => None,
		}
	}
}

impl<Q: Debug> Debug for ForeignLock<Q> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForeignLock").field("queue", &self.queue).field("state", &self.state).finish()
	}
}

/// A guard that prevents exclusive access to the [`ForeignLock`],
/// instructing writes to queue.
pub struct ForeignReadGuard<'a, Q: ForeignLockQueue> {
	read_guard: MaybeUninit<(&'a ForeignLock<Q>, RwLockReadGuard<'a, ()>)>,
}

impl<'a, Q: Debug + ForeignLockQueue> Debug for ForeignReadGuard<'a, Q> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ForeignReadGuard").field("read_guard", &self.read_guard).finish()
	}
}

impl<'a, Q: ForeignLockQueue> Drop for ForeignReadGuard<'a, Q> {
	fn drop(&mut self) {
		let (queue_lock, read_guard) = unsafe { mem::replace(&mut self.read_guard, MaybeUninit::uninit()).assume_init() };

		drop(read_guard);

		if queue_lock.needs_flush.load(Ordering::SeqCst) {
			queue_lock.try_flush();
		}
	}
}

/// Provides a lock on the queue if there is a [`ForeignReadGuard`],
/// or fully exclusive access to the [`ForeignLock`].
pub enum ForeignWriteGuard<'a, Q: ForeignLockQueue> {
	Exclusive(ForeignWriteGuardExclusive<'a, Q>),
	Queue(ForeignWriteGuardQueue<'a, Q>),
}

/// Exclusive lock on the data.
/// No other locks are currently held; read, write, or queue.
pub struct ForeignWriteGuardExclusive<'a, Q: ForeignLockQueue> {
	queue: MutexGuard<'a, Q>,
	needs_flush: &'a AtomicBool,
	write_lock: RwLockWriteGuard<'a, ()>,
}

impl<'a, Q: ForeignLockQueue> ForeignWriteGuardExclusive<'a, Q> {
	pub fn flush(&mut self) {
		self.queue.flush_lock_queue();
		self.needs_flush.store(false, Ordering::SeqCst);
	}

	pub fn queue_mut(&'a mut self) -> &'a mut MutexGuard<'a, Q> {
		self.needs_flush.store(true, Ordering::SeqCst);

		&mut self.queue
	}
}

impl<'a, Q: ForeignLockQueue> Drop for ForeignWriteGuardExclusive<'a, Q> {
	fn drop(&mut self) {
		if self.needs_flush.load(Ordering::SeqCst) {
			self.flush();
		}
	}
}

/// Just the queue for queuing the operation.
pub struct ForeignWriteGuardQueue<'a, Q> {
	queue: MutexGuard<'a, Q>,
}

impl<'a, Q> Deref for ForeignWriteGuardQueue<'a, Q> {
	type Target = Q;

	fn deref(&self) -> &Self::Target {
		<MutexGuard<Q> as Deref>::deref(&self.queue)
	}
}

impl<'a, Q> DerefMut for ForeignWriteGuardQueue<'a, Q> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		<MutexGuard<Q> as DerefMut>::deref_mut(&mut self.queue)
	}
}

/// The queue for a [`ForeignLock`].
/// Implementers usually contain a `Vec` or some kind of flag to record queued writes.
///
/// `flush_lock_queue` will be called when exclusive access is gained, to perform the queued writes.
pub trait ForeignLockQueue {
	fn flush_lock_queue(&mut self);
}
