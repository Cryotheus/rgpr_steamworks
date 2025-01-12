use crate::dt::AppId;
use crate::error::Error;
use crate::interfaces::Steam;
use std::time::Duration;

/// Settings for the creation of a [`CallThread`].
///
/// [`CallThread`]: call::CallThread
#[derive(Clone, Debug)]
pub struct CallThreadBuilder {
	pub(crate) auto_start: bool,
	pub(crate) interval: Duration,
}

impl CallThreadBuilder {
	/// Creates a builder for configuring a CallThreadBuilder.
	pub fn new() -> Self {
		Self {
			auto_start: true,
			interval: Duration::from_secs_f64(1.0 / 64.0),
		}
	}

	/// Sets how often the thread should run the [`CallManager`].
	/// Should be anywhere between 10Hz to 100Hz.
	/// Start with 60Hz if you're unsure.
	///
	/// Alternative to [`with_interval`].
	///
	/// # Panics
	/// If given zero.
	///
	/// [`CallManager`]: call::CallManager
	/// [`with_interval`]: Self::with_interval
	pub fn set_frequency(&mut self, hertz: u32) -> &mut Self {
		assert!(hertz > 0, "CallThreadBuilder::with_frequency must have hertz > 0");

		self.interval = Duration::from_secs_f64(1.0 / hertz as f64);

		self
	}

	/// Sets how often the thread should run the [`CallManager`].
	/// Should be anywhere between 10ms to 100ms.
	/// Start with 15ms if you're unsure.
	///
	/// Alternative to [`with_frequency`].
	///
	/// # Panics
	/// If given zero.
	///
	/// [`CallManager`]: call::CallManager
	/// [`with_frequency`]: Self::with_frequency
	pub fn set_interval(&mut self, interval: Duration) -> &mut Self {
		assert!(!interval.is_zero(), "CallThreadBuilder::with_interval must have interval > 0");

		self.interval = interval;

		self
	}

	/// Sets how often the thread should run the [`CallManager`].
	/// Should be anywhere between 10Hz to 100Hz.
	/// Start with 60Hz if you're unsure.
	///
	/// Alternative to [`with_interval`].
	///
	/// # Panics
	/// If given zero.
	///
	/// [`CallManager`]: call::CallManager
	/// [`with_interval`]: Self::with_interval
	pub fn with_frequency(mut self, hertz: u32) -> Self {
		self.set_frequency(hertz);

		self
	}

	/// Sets how often the thread should run the [`CallManager`].
	/// Should be anywhere between 10ms to 100ms.
	/// Start with 15ms if you're unsure.
	///
	/// Alternative to [`with_frequency`].
	///
	/// # Panics
	/// If given zero.
	///
	/// [`CallManager`]: call::CallManager
	/// [`with_frequency`]: Self::with_frequency
	pub fn with_interval(mut self, interval: Duration) -> Self {
		self.set_interval(interval);

		self
	}
}

impl Default for CallThreadBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for configuring and building a [`SteamInterface`].
#[derive(Clone, Debug)]
pub struct SteamBuilder {
	/// See [AppId].
	pub(crate) app_id: AppId,

	pub(crate) call_thread_config: Option<CallThreadBuilder>,

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	pub(crate) override_app_id: bool,

	/// See [`with_restart_through_steam`].
	///
	/// [`with_restart_through_steam`]: Self::with_restart_through_steam
	pub(crate) restart_through_steam: bool,
}

impl SteamBuilder {
	/// Attach to the Steam API and initialize interfaces.
	pub fn new(app_id: impl Into<AppId>) -> Self {
		Self {
			app_id: app_id.into(),
			call_thread_config: Some(CallThreadBuilder::new()),
			override_app_id: false,
			restart_through_steam: false,
		}
	}

	/// Initializes the Steam API and its interfaces.
	/// If all instances of [`Steam`] are dropped, the API will be shutdown.
	/// You can use [`Steam::get`] to get a reference to the API if the current context does not have one.
	pub fn build(&self) -> Result<Steam, Error> {
		unsafe { Steam::init(self) }
	}

	/// See [`CallThreadBuilder`].
	/// 
	/// Use [`without_call_thread`] if you don't want a [`CallThread`] to be created.
	///
	/// [`CallThread`]: call::CallThread
	/// [`without_call_thread`]: Self::without_call_thread
	pub fn with_call_thread_config(mut self, call_thread_config: CallThreadBuilder) -> Self {
		self.call_thread_config = Some(call_thread_config);

		self
	}

	/// Will not create a dedicated thread to run the [`CallManager`].  
	/// The [`CallManager`] will have to be manually run at no less than 10Hz.
	/// this can be done with [`CallManager::run`].
	///
	/// See [`CallThreadBuilder`] for use with [`with_call_thread_config`].
	///
	/// [`CallManager`]: call::CallManager
	/// [`CallManager::run`]: call::CallManager::run
	/// [`with_call_thread_config`]: Self::with_call_thread_config
	pub fn without_call_thread(mut self) -> Self {
		self.call_thread_config = None;

		self
	}

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	///
	/// # Safety
	/// On windows, this function is always safe.  
	///
	/// For other operating systems, when this is set to `true`:  
	/// Make sure that the environment variables are not being read or written until after [`build`] is called.  
	/// Due to the nature of environment variables, explained in [`set_var`](std::env::set_var),
	/// the best method to prevent issues is to call [`build`] from the main thread before other threads are created.
	///
	/// [`build`]: Self::build
	pub unsafe fn with_override_app_id(mut self, override_app_id: bool) -> Self {
		//TODO: use temporary steam_appid.txt file instead of env vars?
		self.override_app_id = override_app_id;

		self
	}

	/// Calls [`SteamAPI_RestartAppIfNecessary`].
	/// Causes an error if the executable was not launched through Steam.
	///
	/// ## If you're not sure:
	/// - in development it's fine to set it to `false` for convenience
	/// - in production you use should `true` and gracefully handle the
	/// [`RestartingThroughSteam`] error from [`build`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary)
	///
	/// [`build`]: Self::build
	/// [`RestartingThroughSteam`]: crate::error::Error::RestartingThroughSteam
	/// [`SteamAPI_RestartAppIfNecessary`]: https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary
	pub fn with_restart_through_steam(mut self, restart: bool) -> Self {
		self.restart_through_steam = restart;

		self
	}
}
