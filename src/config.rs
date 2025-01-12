use crate::dt::AppId;
use crate::error::SteamError;
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

/// Tells [`Steam::init`] how to override the [`AppId`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OverrideAppId {
	/// Unsafe.
	Env,

	/// Creates a file to do what it needs to.
	File,

	/// Lets Steam set the app ID.
	Inherit,
}

/// Builder for configuring and building a [`SteamInterface`].
#[derive(Clone, Debug)]
pub struct SteamBuilder {
	/// See [AppId].
	pub(crate) app_id: AppId,

	pub(crate) call_thread_config: Option<CallThreadBuilder>,

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	pub(crate) override_app_id: OverrideAppId,

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
			override_app_id: OverrideAppId::Inherit,
			restart_through_steam: true,
		}
	}

	/// Initializes the Steam API and its interfaces.
	/// If all instances of [`Steam`] are dropped, the API will be shutdown.
	/// You can use [`Steam::get`] to get a reference to the API if the current context does not have one.
	pub fn build(&self) -> Result<Steam, SteamError> {
		unsafe { Steam::init(self) }
	}

	/// Lets Steam decide the [`AppId`].
	/// This only works if the app is launched through Steam.
	///
	/// This is the opposite of [`override_app_id`].
	///
	/// [`override_app_id`]: Self::override_app_id
	pub fn inherit_app_id(&mut self) -> &mut Self {
		self.override_app_id = OverrideAppId::Inherit;

		self
	}

	/// Overrides the [`AppId`] of this app.  
	/// This may create a temporary file named `steam_appid.txt`.
	///
	/// This is the opposite of [`inherit_app_id`].
	///
	/// [`inherit_app_id`]: Self::inherit_app_id
	pub fn override_app_id(&mut self) -> &mut Self {
		#[cfg(target_os = "windows")]
		unsafe {
			self.override_app_id_env();
		}

		#[cfg(not(target_os = "windows"))]
		{
			self.override_app_id = OverrideAppId::File;
		}

		self
	}

	/// Force the specified [`AppId`] to be used by overriding environment variables.
	/// Use should typically use [`override_app_id`] instead.
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
	/// [`override_app_id`]: Self::override_app_id
	pub unsafe fn override_app_id_env(&mut self) -> &mut Self {
		self.override_app_id = OverrideAppId::Env;

		self
	}

	/// Will not create a dedicated thread to run the [`CallManager`].
	/// The [`CallManager`] will have to be manually run at no less than 10Hz.
	/// this can be done with [`CallManager::run`].
	///
	/// See [`CallThreadBuilder`] for use with [`set_call_thread_config`].
	///
	/// [`CallManager`]: call::CallManager
	/// [`CallManager::run`]: call::CallManager::run
	/// [`set_call_thread_config`]: Self::set_call_thread_config
	pub fn remove_call_thread(&mut self) -> &mut Self {
		self.call_thread_config = None;

		self
	}

	/// See [`CallThreadBuilder`].
	///
	/// Use [`remove_call_thread`] if you don't want a [`CallThread`] to be created.
	///
	/// [`CallThread`]: call::CallThread
	/// [`remove_call_thread`]: Self::remove_call_thread
	pub fn set_call_thread_config(&mut self, call_thread_builder: CallThreadBuilder) -> &mut Self {
		self.call_thread_config = Some(call_thread_builder);

		self
	}

	/// Same as calling [`override_app_id`] and [`set_restart_through_steam(false)`].
	///
	/// [`override_app_id`]: Self::override_app_id
	/// [`set_restart_through_steam(false)`]: Self::set_restart_through_steam
	pub fn set_dev(&mut self) -> &mut Self {
		self.override_app_id = OverrideAppId::File;
		self.restart_through_steam = false;

		self
	}

	/// > Checks if your executable was launched through Steam and relaunches it through Steam if it wasn't.
	///
	/// Causes [`build`] to error if the executable was not launched through Steam,
	/// and relaunch the app through Steam.  
	/// You should check if the error variant is [`RestartingThroughSteam`], and gracefully terminate your app.
	///
	/// ## If you're not sure:
	/// - in development it's fine to set it to `false` for convenience
	/// - in production you use should `true` and gracefully handle the
	/// [`RestartingThroughSteam`] error from [`build`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary)
	///
	/// [`build`]: Self::build
	/// [`RestartingThroughSteam`]: crate::error::SteamError::RestartingThroughSteam
	/// [`SteamAPI_RestartAppIfNecessary`]: https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary
	pub fn set_restart_through_steam(&mut self, restart_through_steam: bool) -> &mut Self {
		self.restart_through_steam = restart_through_steam;

		self
	}

	/// Same as [`remove_call_thread`] but keeps `self` owned.
	///
	/// [`remove_call_thread`]: Self::remove_call_thread
	pub fn without_call_thread(mut self) -> Self {
		self.remove_call_thread();

		self
	}

	/// Same as [`set_dev`] but keeps `self` owned.
	///
	/// [`set_dev`]: Self::set_dev
	pub fn with_dev(mut self) -> Self {
		self.set_dev();

		self
	}

	/// Same as [`set_call_thread_config`] but keeps `self` owned.
	///
	/// [`set_call_thread_config`]: Self::set_call_thread_config
	pub fn with_call_thread_config(mut self, call_thread_builder: CallThreadBuilder) -> Self {
		self.set_call_thread_config(call_thread_builder);

		self
	}

	/// Same as [`override_app_id`] but keeps `self` owned.
	///
	/// [`override_app_id`]: Self::override_app_id
	pub fn with_override_app_id(mut self) -> Self {
		self.override_app_id();

		self
	}

	/// Same as [`set_override_app_id_env`] but keeps `self` owned.
	///
	/// # Safety
	/// Safety explained in [`set_override_app_id_env`].
	///
	/// [`set_override_app_id_env`]: Self::override_app_id_env
	pub unsafe fn with_override_app_id_env(mut self) -> Self {
		self.override_app_id();

		self
	}

	/// Same as [`set_restart_through_steam`] but keeps `self` owned.
	///
	/// [`set_restart_through_steam`]: Self::set_restart_through_steam
	pub fn with_restart_through_steam(mut self, restart: bool) -> Self {
		self.set_restart_through_steam(restart);

		self
	}
}
