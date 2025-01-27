mod internal;

use cfg_if::cfg_if;

cfg_if! {
	if #[cfg(feature = "sys")] {
		pub use internal::*;
	} else {
		pub(crate) use internal::*;
	}
}

