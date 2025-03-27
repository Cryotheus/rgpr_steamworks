#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! Generated bindings to [Steamworks](https://partner.steamgames.com/).

#[cfg_attr(target_os = "linux", path = "bindings/linux.rs")]
#[cfg_attr(target_os = "macos", path = "bindings/macos.rs")]
#[cfg_attr(target_os = "windows", path = "bindings/windows.rs")]
#[doc(hidden)]
mod sys;

pub use sys::*;
