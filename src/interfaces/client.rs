use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild};
use crate::sys;

impl AsRef<ClientInterface> for super::Interfaces {
	fn as_ref(&self) -> &ClientInterface {
		&self.client
	}
}

#[derive(Debug)]
pub struct ClientInterface {
	fip: FixedInterfacePtr<sys::ISteamClient>,
}

impl Interface for ClientInterface {
	type CInterface = sys::ISteamClient;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, _steam: SteamChild) -> Self {
		Self { fip }
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		unsafe { sys::SteamClient() }
	}
}
