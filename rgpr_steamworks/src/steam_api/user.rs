use crate::FixedInterface;
use rgpr_steamworks_sys as sys;

#[derive(Debug)]
pub struct UserInterface {
	interface: FixedInterface<sys::ISteamUser>,
}

impl UserInterface {
	pub unsafe fn new() -> Self {
		Self {
			interface: FixedInterface(sys::SteamAPI_SteamUser_v023()),
		}
	}

	#[doc(hidden)]
	fn interface(&self) -> *mut sys::ISteamUser {
		self.interface.ptr()
	}
}
