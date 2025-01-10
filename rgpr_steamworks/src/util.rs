use std::ffi::{c_char, c_int, CStr, CString};
use std::path::Path;

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

pub(crate) type CStrArrayPath = CStrArray<MAX_PATH>;

/// For C strings that the Steam API wants us to allocate.
#[derive(Clone, Debug)]
#[doc(hidden)]
#[repr(transparent)]
pub(crate) struct CStrArray<const LEN: usize>([c_char; LEN]);

impl<const LEN: usize> CStrArray<LEN> {
	pub(crate) fn new() -> Self {
		Self([c_char::default(); LEN])
	}

	pub(crate) fn c_len(&self) -> c_int {
		self.0.len() as c_int
	}

	pub(crate) fn c_str(&self) -> &CStr {
		assert_eq!(*self.0.last().unwrap(), c_char::default(), "CStr buffer did not end with a null terminator");

		unsafe { CStr::from_ptr(self.0.as_ptr()) }
	}

	pub(crate) fn c_string(&self) -> CString {
		CString::from(self.c_str())
	}

	pub(crate) fn ptr(&mut self) -> *mut c_char {
		self.0.as_mut_ptr()
	}

	pub(crate) fn path(&self) -> &Path {
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

	pub(crate) fn to_string(mut self) -> String {
		if self.0[0] == c_char::default() {
			return String::new();
		}

		assert_eq!(*self.0.last().unwrap(), c_char::default(), "CStr buffer did not end with a null terminator");

		unsafe { CStr::from_ptr(self.ptr()) }.to_string_lossy().to_string()
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

