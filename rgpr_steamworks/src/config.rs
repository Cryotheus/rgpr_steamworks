use crate::dt::AppId;
use crate::error::Error;
use crate::interfaces::Steam;

#[derive(Clone, Debug)]
pub struct SteamBuilder {
	/// See [AppId].
	pub(crate) app_id: AppId,

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	pub(crate) override_app_id: bool,

	/// Calls [`SteamAPI_RestartAppIfNecessary`].
	/// Causes an error if the executable was not launched through Steam.
	///
	/// ## If you're not sure:
	/// - in development it's fine to set it to `false` for convenience
	/// - in production you use should `true` and gracefully handle the
	/// [`RestartingThroughSteam`] error from [`build`](SteamBuilder::build).
	///
	/// [`init`]: Steamworks::init
	/// [`RestartingThroughSteam`]: crate::error::Error::RestartingThroughSteam
	/// [`SteamAPI_RestartAppIfNecessary`]: https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary
	pub(crate) restart_through_steam: bool,
}

impl SteamBuilder {
	/// Attach to the Steam API and initialize interfaces.
	pub fn new(app_id: impl Into<AppId>) -> Self {
		Self {
			app_id: app_id.into(),
			override_app_id: false,
			restart_through_steam: false,
		}
	}

	pub fn build(&self) -> Result<Steam, Error> {
		unsafe { Steam::init(self) }
	}

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	/// 
	/// # Safety
	/// On windows, this function is safe.  
	/// 
	/// For other operating systems:  
	/// Make sure that the environment variables are not being read or written.  
	/// Due to the nature of environment variables, explained in [set_var](std::env::set_var),  
	/// the best method to prevent issues is to call [build] from the main thread before other threads are created.
	pub unsafe fn with_override_app_id(mut self, override_app_id: bool) -> Self {
		self.override_app_id = override_app_id;

		self
	}

	pub fn with_restart_through_steam(mut self, restart: bool) -> Self {
		self.restart_through_steam = restart;

		self
	}
}
