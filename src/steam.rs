//! See [`Steam`].

use std::ops::Deref;
use std::sync::Arc;
use crate::config::SteamBuilder;
use crate::error::SteamError;
use crate::interfaces::{SteamInterface, STEAM_INTERFACE};

/// Reference to the current [`SteamInterface`].
/// If none of these exist, the [`SteamInterface`] will be shutdown.
#[derive(Clone, Debug)]
pub struct Steam(pub(crate) Arc<SteamInterface>);

impl Steam {
	/// Get a reference to the currently initialized Steam API interface.
	/// Returns `None` if the Steam API is not initialized.
	pub fn get() -> Option<Steam> {
		STEAM_INTERFACE.read().unwrap().upgrade().map(|steam_interface| Steam(steam_interface))
	}

	/// Attach to the Steam API and initialize interfaces.
	///
	/// Called by [`SteamBuilder::build`].
	///
	/// [`SteamBuilder::build`]: SteamBuilder::build
	pub(crate) unsafe fn new(config: &SteamBuilder) -> Result<Steam, SteamError> {
		SteamInterface::new(config).map(|arc| Steam(arc))
	}
}

impl<T> AsRef<T> for Steam
where
	<Self as Deref>::Target: AsRef<T>,
{
	fn as_ref(&self) -> &T {
		self.deref().as_ref()
	}
}

impl Deref for Steam {
	type Target = SteamInterface;

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}
