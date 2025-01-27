#![allow(private_interfaces)]
extern crate core;

use cfg_if::cfg_if;

pub mod dt;
pub mod iter;
pub mod util;

#[cfg(feature = "steam_encrypted_app_ticket")]
pub mod encrypted_app_ticket;

cfg_if! {
	if #[cfg(feature = "steam")] {
		pub mod call;
		pub mod config;
		pub mod error;
		pub mod interfaces;
		pub mod net;
	}
}

cfg_if! {
	if #[cfg(feature = "sys")] {
		pub use rgpr_steamworks_sys as sys;
	} else {
		#[cfg(feature = "steam")]
		pub(crate) use rgpr_steamworks_sys as sys;
	}
}

/// Prevents usage of exposed private functions.
#[derive(Debug)]
pub(crate) struct Private;
