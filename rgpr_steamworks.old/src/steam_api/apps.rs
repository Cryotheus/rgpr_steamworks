use crate::dt::{AppId, DepotId};
use crate::util::MAX_PATH;
use crate::{dt::IntoCIndex, FixedInterface};
use rgpr_steamworks_sys as sys;
use std::ffi::{c_char, c_uint, CStr, OsStr};
use std::num::NonZeroI32;
use std::path::Path;
use std::time::{Duration, SystemTime};

#[derive(Debug)]
pub struct AppsInterface {
	interface: FixedInterface<sys::ISteamApps>,
}

impl AppsInterface {
	pub(crate) unsafe fn new() -> Self {
		Self {
			interface: FixedInterface::new(sys::SteamAPI_SteamApps_v008()),
		}
	}

	#[doc(hidden)]
	fn interface(&self) -> *mut sys::ISteamApps {
		self.interface.ptr()
	}
}

