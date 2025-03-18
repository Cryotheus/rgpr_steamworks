//! Utilities for working with the Steam API.

mod internal;

use cfg_if::cfg_if;
use std::ffi::{CString, NulError};
use std::ops::Deref;

cfg_if! {
	if #[cfg(feature = "sys")] {
		pub use internal::*;
	} else {
		pub(crate) use internal::*;
	}
}

/// Trait accepting parameters that are less than a size known at compile time.
/// 
/// Implemented by all implementers of `AsRef<[u8]>` such as `String`, `&str`, and `Vec<u8>`.
pub trait FiniteBytes<const CAP: usize>: Sized {
	fn to_finite_bytes(self) -> [u8; CAP];
	fn to_finite_cstring(self) -> Result<CString, NulError>;
}

impl<const CAP: usize, T: AsRef<[u8]>> FiniteBytes<CAP> for T {
	/// # Panics
	/// If `self` is longer or equal to `BYTES`.
	fn to_finite_bytes(self) -> [u8; CAP] {
		let bytes = self.as_ref();
		let byte_count = bytes.len();

		assert!(byte_count <= CAP, "given {byte_count} for FiniteBytes<{CAP}> which is {byte_count} > {CAP}");

		std::array::from_fn(|index| bytes.get(index).cloned().unwrap_or(0u8))
	}

	/// # Panics
	/// If `self` is longer or equal to `BYTES`.
	fn to_finite_cstring(self) -> Result<CString, NulError> {
		let bytes = self.as_ref();
		let byte_count = bytes.len();

		assert!(byte_count <= CAP, "given {byte_count} for FiniteBytes<{CAP}> which is {byte_count} > {CAP}");

		CString::new(bytes)
	}
}

/// Same as [`FiniteBytes`] but strictly for types which enforce UTF-8 compliance.
pub trait FiniteStr<const CAP: usize>: FiniteBytes<CAP> {}

impl<const CAP: usize, T: Deref<Target = str> + AsRef<[u8]>> FiniteStr<CAP> for T {}
