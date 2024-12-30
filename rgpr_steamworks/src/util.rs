use std::ffi::{CStr, OsStr};
use std::num::NonZeroU32;
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

fn cstr_path(c_str: &CStr) -> &Path {
	#[cfg(target_os = "linux")]
	{
		std::os::unix::ffi::OsStrExt::from_bytes(c_str.to_bytes()).as_ref()
	}

	#[cfg(not(target_os = "linux"))]
	{
		std::str::from_utf8(c_str.to_bytes()).unwrap().as_ref()
	}
}
