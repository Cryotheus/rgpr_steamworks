//! Internal utilities for working with the Steam API.
//! These utilities are either unsafe, or have turbulent APIs.

use crate::sys;
use cfg_if::cfg_if;
use futures::channel::oneshot;
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};
use std::ffi::{c_char, c_int, CStr, CString};
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::mem::{transmute, MaybeUninit};
use std::path::Path;
use std::ptr::{null, NonNull};

pub const EMPTY_CSTR: &'static CStr = c"";

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

	/// Returns `None` if the path is not UTF-8 compliant on windows.
	pub fn get_path(&self) -> Option<&Path> {
		let bytes = self.c_str().to_bytes();

		cfg_if! {
			if #[cfg(target_os = "linux")] {
				Some(<OsStr as AsRef<Path>>::as_ref(std::os::unix::ffi::OsStrExt::from_bytes(bytes)))
			} else {
				std::str::from_utf8(bytes).ok().map(|str| <str as AsRef<Path>>::as_ref(str))
			}
		}
	}

	/// Also see [`get_path`].
	///
	/// # Panics
	/// On windows, if the path is not UTF-8 compliant.
	///
	/// [`get_path`]: Self::get_path
	pub fn path(&self) -> &Path {
		let bytes = self.c_str().to_bytes();

		//linux makes things easy
		//windows - not so much
		cfg_if! {
			if #[cfg(target_os = "linux")] {
				<OsStr as AsRef<Path>>::as_ref(std::os::unix::ffi::OsStrExt::from_bytes(bytes))
			} else {
				std::str::from_utf8(bytes).expect("CStrArray must be UTF-8 to support &Path on windows").as_ref()
			}
		}
	}

	pub fn ptr(&mut self) -> *mut c_char {
		self.0.as_mut_ptr()
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

/// An `Option<CString>` for optional C char pointers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptionalCString(Option<CString>);

impl OptionalCString {
	pub fn new() -> Self {
		Self(None)
	}

	fn map(bytes: impl Into<Vec<u8>>) -> CString {
		CString::new(bytes).expect("CString must not contain nul bytes")
	}

	pub fn as_ref(&self) -> Option<&CStr> {
		self.0.as_ref().map(|c_string| c_string.as_ref())
	}

	pub fn fill(&mut self, bytes: impl Into<Vec<u8>>) {
		self.0 = Some(Self::map(bytes));
	}

	pub fn fill_str(&mut self, str: impl AsRef<str>) {
		self.fill(str.as_ref().as_bytes());
	}

	/// Use [`as_ptr`] if the C char pointer cannot be null.
	pub fn as_nullable_ptr(&self) -> *const c_char {
		match &self.0 {
			None => null(),
			Some(c_string) => c_string.as_ptr(),
		}
	}

	/// Will return a raw pointer to a static empty [`CStr`] if the `OptionalCString` has not been filled.  
	/// If a null pointer is acceptable, use [`as_nullable_ptr`] instead.
	///
	/// [`as_nullable_ptr`]: Self::as_nullable_ptr
	pub fn as_ptr(&self) -> *const c_char {
		match &self.0 {
			None => EMPTY_CSTR.as_ptr(),
			Some(c_string) => c_string.as_ptr(),
		}
	}
}

impl From<Option<&str>> for OptionalCString {
	fn from(value: Option<&str>) -> Self {
		Self(value.map(Self::map))
	}
}

impl From<Option<String>> for OptionalCString {
	fn from(value: Option<String>) -> Self {
		Self(value.map(Self::map))
	}
}

impl From<Option<&String>> for OptionalCString {
	fn from(value: Option<&String>) -> Self {
		Self(value.map(|string_ref| Self::map(string_ref.as_str())))
	}
}

impl<'a, T> From<&'a Option<T>> for OptionalCString where Self: From<Option<&'a T>> {
	fn from(value: &'a Option<T>) -> Self {
		Self::from(value.as_ref())
	}
}

/// A `Box<T>` without the known type `T` or [type id].
/// Destructors will never be called for the contained data.
///
/// [type id]: std::any::TypeId
#[derive(Debug)]
pub struct IncognitoBox {
	layout: Layout,
	pointer: NonNull<u8>,
}

impl IncognitoBox {
	/// # Safety
	/// `T` must not implement or contain any deconstructors.
	///
	/// # Panics
	/// If the layout of `T` has a size of zero.
	pub unsafe fn new<T>(x: T) -> Self {
		let layout = Layout::new::<T>();

		assert_ne!(layout.size(), 0, "given layout with zero-size");

		let pointer = NonNull::new(unsafe { alloc(layout) }).expect("failed to alloc");

		//put the owned value into the allocation
		unsafe { (pointer.as_ptr() as *mut T).write(x) };

		Self { layout, pointer }
	}

	#[must_use]
	#[inline(always)]
	pub fn as_ptr(&mut self) -> *mut u8 {
		self.pointer.as_ptr()
	}

	/// # Safety
	/// See [`new`].
	///
	/// # Panics
	/// If the layout of `T` has a size of zero.
	///
	/// [`new`]: Self::new
	pub unsafe fn from_box<T: Sized>(boxxed: Box<T>) -> Self {
		Self {
			layout: Layout::new::<T>(),
			pointer: NonNull::new(Box::into_raw(boxxed) as *mut u8).unwrap(),
		}
	}

	/// The internal allocation will be zeroed.
	///
	/// # Panics
	/// If the layout has a size of zero.
	pub fn from_layout(layout: Layout) -> Self {
		assert_ne!(layout.size(), 0, "given layout with zero-size");

		let alloc = unsafe { alloc_zeroed(layout) };

		Self {
			layout,
			pointer: NonNull::new(alloc).unwrap(),
		}
	}

	/// Turns the `IncognitoBox` into a `Box<T>` given the known type `T`.
	/// # Safety
	/// `T` provided here must match the type used in [`new`]/[`from_box`] or what was written through [`as_ptr`].
	/// `T` must be initialized; use [`identify_uninit`] if the memory could be uninit.
	///
	/// [`as_ptr`]: Self::as_ptr
	/// [`from_box`]: Self::from_box
	/// [`identify_uninit`]: Self::identify_uninit
	/// [`new`]: Self::new
	pub unsafe fn identify<T>(self) -> Box<T> {
		assert_eq!(self.layout, Layout::new::<T>(), "generic T does not have the same layout as self");

		Box::from_raw(self.pointer.as_ptr() as *mut T)
	}

	/// Turns the `IncognitoBox` into a `Box<MaybeUninit<T>>` given the known type `T`.
	///
	/// See [`identify`] for initialized data.
	///
	/// [`identify`]: Self::identify
	pub fn identify_uninit<T>(self) -> Box<MaybeUninit<T>> {
		assert_eq!(self.layout, Layout::new::<T>(), "generic T does not have the same layout as self");

		unsafe { Box::from_raw(self.pointer.as_ptr() as *mut MaybeUninit<T>) }
	}

	/// Returns the [layout] the `IngonitoBox` was created with.
	///
	/// [layout]: Layout
	pub fn layout(&self) -> Layout {
		self.layout
	}
}

impl Drop for IncognitoBox {
	fn drop(&mut self) {
		unsafe { dealloc(self.pointer.as_ptr(), self.layout) };
	}
}

static_assertions::assert_not_impl_all!(IncognitoBox: Send, Sync);

/// For queuing asynchronous requests.
///
/// - `K` **Key** - what the request is for.
/// - `R` **Request** - typically a value to check against when deciding if a request should be fulfilled.
/// - `M` **Message** - the value to provide the requester upon completion.
pub struct RequestQueue<K, R, M> {
	requests: HashMap<K, VecDeque<(R, oneshot::Sender<M>)>>,
}

impl<K, R, M> RequestQueue<K, R, M> {
	pub fn new() -> Self {
		Self { requests: HashMap::new() }
	}
}

impl<K: Eq + Hash, R, M> RequestQueue<K, R, M> {
	pub fn insert(&mut self, key: K, request: R) -> oneshot::Receiver<M> {
		let (tx, rx) = oneshot::channel::<M>();
		let pair = (request, tx);

		match self.requests.entry(key) {
			Entry::Occupied(mut entry) => entry.get_mut().push_back(pair),

			Entry::Vacant(entry) => {
				let mut vec = VecDeque::new();

				vec.push_back(pair);
				entry.insert(vec);
			}
		}

		rx
	}

	/// Fulfils all requests with a message.
	pub fn fulfil_all(&mut self, key: &K, message: M)
	where
		M: Clone,
	{
		let Some(vec) = self.requests.remove(key) else {
			return;
		};

		//send a clone of the message to all requests
		for (_, tx) in vec {
			let _ = tx.send(message.clone());
		}
	}

	/// Fulfils the request if the predicate yields a message to send.
	pub fn fulfil_if(&mut self, key: &K, mut predicate: impl FnMut(&mut R) -> Option<M>) {
		let Some(vec) = self.requests.get_mut(key) else {
			return;
		};

		let mut messages = Vec::<(usize, M)>::new();

		//find what needs to be removed
		for (index, (request, _)) in vec.iter_mut().enumerate().rev() {
			if let Some(message) = predicate(request) {
				messages.push((index, message));
			}
		}

		//do the removals and sends
		for (index, message) in messages {
			let (_, tx) = vec.swap_remove_back(index).unwrap();
			let _ = tx.send(message);
		}
	}
}

impl<K: Debug, R: Debug, M: Debug> Debug for RequestQueue<K, R, M> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("RequestQueue").field("requests", &self.requests).finish()
	}
}

impl<K, R, M> Default for RequestQueue<K, R, M> {
	fn default() -> Self {
		Self::new()
	}
}

/// Key-only [`RequestQueue`].
pub type KoQueue<K> = RequestQueue<K, (), ()>;

/// Same as [`some_string`], but allows empty strings.
///
/// # Panics
/// If the pointer is null.
///
/// # Safety
/// The pointer must point to a valid null-terminated allocation.
#[inline(always)]
pub unsafe fn checked_string(char_ptr: *const c_char) -> String {
	assert!(!char_ptr.is_null());

	if *char_ptr == 0 {
		String::new()
	} else {
		CStr::from_ptr(char_ptr).to_string_lossy().to_string()
	}
}

/// Same as [`some_string`], but panics.
///
/// # Panics
/// If [`some_string`] returns `None`.
///
/// # Safety
/// The pointer must point to a valid null-terminated allocation, or be null.
#[inline(always)]
pub unsafe fn expect_string(char_ptr: *const c_char) -> String {
	some_string(char_ptr).expect("expected some C string but got none")
}

/// Converts a char ptr into a `String`, returning `None` if it's empty or null.
/// If there are invalid UTF-8 codepoints, they will be replaced.
///
/// # Safety
/// The pointer must point to a valid null-terminated allocation, or be null.
#[inline(always)]
pub unsafe fn some_string(char_ptr: *const c_char) -> Option<String> {
	if char_ptr.is_null() || *char_ptr == 0 {
		None
	} else {
		Some(CStr::from_ptr(char_ptr).to_string_lossy().to_string())
	}
}

/// Converts `str` into [`CString`] dropping nul bytes.
pub fn lossy_cstring(str: impl AsRef<str>) -> CString {
	CString::new(str.as_ref().bytes().filter(|byte| *byte != 0).collect::<Vec<_>>()).unwrap()
}

/// Turns a bool into `Result<(), SilentFailure>`  
/// where `true` is `Ok(())`  
/// and `false` is `Err(SilentFailure)`
#[cfg(feature = "steam")]
#[doc(hidden)]
#[inline(always)]
pub fn success(success: bool) -> Result<(), crate::error::UnspecifiedError> {
	if success {
		Ok(())
	} else {
		Err(crate::error::UnspecifiedError)
	}
}

pub fn empty_cstr_ptr() -> *const c_char {
	EMPTY_CSTR.as_ptr()
}
