pub mod call;
pub mod dt;
pub mod error;
pub mod steam_api;
pub(crate) mod util;

use crate::dt::AppId;
use call::SteamworksCallManager;
use error::Error;
use rgpr_steamworks_sys as sys;
use std::fmt::Debug;
use std::sync::{Arc, Mutex, MutexGuard, Weak};
use steam_api::friends::FriendsInterface;
use steam_api::user_stats::UserStatsInterface;

/// Holds a weak reference to the currently attached Steamworks interface.
static STEAMWORKS: Mutex<Weak<SteamworksInner>> = Mutex::new(Weak::new());

/// Stores a raw mutable pointer to a thread-safe interface.
/// # Safety
/// - Do not use after Steamworks has been shutdown.
/// - Do not give crate users access.
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixedInterface<T>(*mut T);

impl<T> FixedInterface<T> {
	/// # Safety
	/// All uses of the inteface must be thread-safe.
	pub(crate) unsafe fn new(t_ptr: *mut T) -> Self {
		Self(t_ptr)
	}

	pub(crate) fn ptr(&self) -> *mut T {
		self.0
	}
}

unsafe impl<T> Send for FixedInterface<T> {}
unsafe impl<T> Sync for FixedInterface<T> {}

/// A portable reference to Steamworks resources.
/// Safe to clone and send to other threads.
///
/// Internal representation may change.
#[derive(Debug)]
pub struct Steamworks(Arc<SteamworksInner>);

impl Steamworks {
	/// Get the currently attached Steamworks interface.
	/// Returns `None` if Steamworks is not currently attached because:
	/// - The interface has not been initialized through the [`init`](Self::init) method.
	/// - The interface had been dropped.
	pub fn get() -> Option<Steamworks> {
		let weak = STEAMWORKS.lock().ok()?;

		weak.upgrade().map(|arc| Steamworks(arc))
	}

	pub fn init(config: SteamworksInitConfig) -> Result<Self, Error> {
		let mut guard = STEAMWORKS.lock().expect("STEAMWORKS Mutex poisoned");

		if let Some(arc) = guard.upgrade() {
			return Ok(Self(arc));
		}

		if config.restart_through_steam {
			//launches the app id through steam if the exe was not launched through steam
			if unsafe { sys::SteamAPI_RestartAppIfNecessary(config.app_id.0) } {
				return Err(Error::RestartingThroughSteam);
			}
		}

		if config.override_app_id {
			use std::env::set_var;

			let id_str = config.app_id.0.to_string();

			set_var("SteamAppId", &id_str);
			set_var("SteamGameId", id_str);
		}

		unsafe {
			let mut err_msg: sys::SteamErrMsg = [0; 1024];
			let result_enum = sys::SteamAPI_InitFlat(&mut err_msg);

			if result_enum != sys::ESteamAPIInitResult::k_ESteamAPIInitResult_OK {
				todo!();
			}

			//we will handle callbacks in our own way
			sys::SteamAPI_ManualDispatch_Init();
		}

		let interface_config = config.interfaces;
		let mut call_manager = SteamworksCallManager::new();
		let mut friends_interface: Option<FriendsInterface> = None;
		let mut user_stats_interface: Option<UserStatsInterface> = None;

		unsafe {
			if interface_config.friends {
				friends_interface = Some(FriendsInterface::new());
			}

			if interface_config.user_stats {
				user_stats_interface = Some(UserStatsInterface::new());

				//to keep the store up-to-date
				//TODO: make a user stats store optional
				call_manager.register::<steam_api::user_stats::UserStatsUnloaded>();
			}
		}

		//convenience
		fn mutex<T>(t: T) -> Mutex<T> {
			Mutex::new(t)
		}

		let arc = Arc::new(SteamworksInner {
			app_id: config.app_id,
			call_manager: Mutex::new(call_manager),
			friends_interface: friends_interface.map(mutex),
			user_stats_interface: user_stats_interface.map(mutex),
		});

		//to make the static available for use
		*guard = Arc::downgrade(&arc);

		Ok(Self(arc))
	}

	pub fn app_id(&self) -> AppId {
		self.0.app_id
	}

	pub fn call_manager(&self) -> MutexGuard<SteamworksCallManager> {
		self.0.call_manager.lock().unwrap()
	}

	pub fn user_stats_interface(&self) -> Option<MutexGuard<UserStatsInterface>> {
		self.0.user_stats_interface.as_ref().map(|mutex| mutex.lock().unwrap())
	}
}

impl Clone for Steamworks {
	fn clone(&self) -> Self {
		Self(Arc::clone(&self.0))
	}
}

#[derive(Clone, Debug)]
pub struct SteamworksInitConfig {
	/// The AppID of the Steam App.
	pub app_id: AppId,

	/// Force the specified app ID to be used.
	/// Don't use this on your builds that are launched through Steam.
	pub override_app_id: bool,

	/// Calls [`SteamAPI_RestartAppIfNecessary`](https://partner.steamgames.com/doc/api/steam_api#SteamAPI_RestartAppIfNecessary).
	/// Causes an error if the executable was not launched through Steam.
	/// ## If you're not sure:
	/// - in development it's fine to set it to `false` for convenience
	/// - in production you use should `true` and gracefully handle the
	/// [`RestartingThroughSteam`](Error::RestartingThroughSteam) error from [`init`](Steamworks::init).
	pub restart_through_steam: bool,

	/// Interfaces to initialized.
	pub interfaces: SteamworksInterfaceConfig,
}

impl SteamworksInitConfig {
	/// Config with preferrable settings for development.
	/// Same as [default](Default::default) with the exception of the app ID.
	pub fn development(app_id: impl Into<AppId>) -> Self {
		Self {
			app_id: app_id.into(),
			..Self::default()
		}
	}

	/// Config with preferrable settings for production.
	pub fn production(app_id: impl Into<AppId>) -> Self {
		Self {
			app_id: app_id.into(),
			override_app_id: false,
			restart_through_steam: true,
			..Self::default()
		}
	}
}

impl Default for SteamworksInitConfig {
	fn default() -> Self {
		Self {
			app_id: AppId(480),
			override_app_id: true,
			restart_through_steam: false,
			interfaces: SteamworksInterfaceConfig::default(),
		}
	}
}

#[derive(Clone, Debug, Default)]
pub struct SteamworksInterfaceConfig {
	/// Constructs a [`FriendsInterface`].
	/// Set to `true` if Steam friends integration is needed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends).
	pub friends: bool,

	pub matchmaking: bool,

	pub user: bool,

	/// Constructs a [`UserStatsInterface`] and the necessary callback handlers.
	/// Set to `true` if access to user's stats is needed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats).
	pub user_stats: bool,
}

impl SteamworksInterfaceConfig {
	/// Enable all available interfaces.
	pub fn everything() -> Self {
		Self {
			friends: true,
			matchmaking: true,
			user: true,
			user_stats: true,
		}
	}
}

#[derive(Debug)]
pub(crate) struct SteamworksInner {
	app_id: AppId,

	/// Runs callbacks and retrieves call results.
	call_manager: Mutex<SteamworksCallManager>,

	friends_interface: Option<Mutex<FriendsInterface>>,

	user_stats_interface: Option<Mutex<UserStatsInterface>>,
}

impl Drop for SteamworksInner {
	fn drop(&mut self) {
		unsafe {
			sys::SteamAPI_Shutdown();

			//personal preference here: upgrading a Weak returns earlier if it's a dangling pointer
			//we can make sure we have a Weak holding a dangling pointer by calling its new function
			//of course, this isn't essential, it's just preference
			if let Some(mut guard) = STEAMWORKS.try_lock().ok() {
				*guard = Weak::new();
			}
		}
	}
}

/// Used as a parameter on private functions that are exposed.
#[derive(Debug)]
pub(crate) struct Private(pub(crate) ());
