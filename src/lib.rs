#![allow(private_interfaces)]

use cfg_if::cfg_if;

pub mod dt;

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
		pub mod util;
		pub use rgpr_steamworks_sys as sys;
	} else {
		#[cfg(feature = "steam")]
		pub(crate) mod util;
		pub(crate) use rgpr_steamworks_sys as sys;
	}
}

#[derive(Debug)]
pub(crate) struct Private;
