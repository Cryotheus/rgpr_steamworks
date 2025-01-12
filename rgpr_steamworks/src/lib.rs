#![allow(private_interfaces)]

pub mod call;
pub mod config;
pub mod dt;
pub mod error;
pub mod interfaces;
pub mod net;

#[cfg(feature = "steam_encrypted_app_ticket")]
pub mod encrypted_app_ticket;

#[cfg(not(feature = "sys"))]
pub(crate) mod util;

#[cfg(not(feature = "sys"))]
pub(crate) use rgpr_steamworks_sys as sys;

#[cfg(feature = "sys")]
pub mod util;

#[cfg(feature = "sys")]
pub use rgpr_steamworks_sys as sys;

#[derive(Debug)]
pub(crate) struct Private;

#[cfg(feature = "steam_apps")]
mod example {

}