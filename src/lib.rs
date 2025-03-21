//! # RgPr Steamworks
//!
//! High-level Rust bindings to [Steamworks], the Steam API.
//!
//! > The Steamworks SDK provides a range of features which are designed to help ship your application or game on Steam in an efficient manner.  
//! > \- [Steamworks Documentation]
//!
//! Headers and binaries are in the [`rgpr_steamworks_sys`] crate.
//! This is purely for Steam apps that require integration with the Steam client and not the Steam Web API.
//!
//! [Steamworks]: https://partner.steamgames.com/
//! [Steamworks Documentation]: https://partner.steamgames.com/doc/sdk
//! [`rgpr_steamworks_sys`]: https://crates.io/crates/rgpr_steamworks_sys

// SAFETY: we do not Pin anything that interacts with Unsize
#![feature(unsize)]

// we do this a lot
#![allow(private_interfaces)]

// allow doc(cfg( ... )) attribute - useful for showing feature gates
#![cfg_attr(doc, feature(doc_cfg))]

use cfg_if::cfg_if;

pub mod dt;
pub mod error;
pub mod iter;
pub mod prelude;
pub mod util;

#[cfg(feature = "steam_encrypted_app_ticket")]
#[cfg_attr(doc, doc(cfg(feature = "steam_encrypted_app_ticket")))]
pub mod encrypted_app_ticket;

cfg_if! {
	if #[cfg(feature = "steam")] {
		#[cfg_attr(doc, doc(cfg(feature = "steam")))]
		pub mod call;
		
		#[cfg_attr(doc, doc(cfg(feature = "steam")))]
		pub mod config;
		
		#[cfg_attr(doc, doc(cfg(feature = "steam")))]
		pub mod interfaces;
		
		#[cfg_attr(doc, doc(cfg(feature = "steam")))]
		pub mod net;
		
		#[cfg_attr(doc, doc(cfg(feature = "steam")))]
		pub mod steam;
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

#[cfg(test)]
pub(crate) mod test {
	use crate::config::SteamBuilder;
	use crate::steam::Steam;

	pub(crate) fn setup_steam() -> Steam {
		SteamBuilder::new(480).with_dev().build().unwrap()
	}
}
