#![allow(private_interfaces)]

use crate::call::{CallbackListenerIter, SteamworksCallback, SteamworksDispatch};
use crate::dt::SteamId;
use crate::error::{Error, GeneralError};
use crate::{FixedInterface, Private, Steamworks};
use rgpr_steamworks_sys as sys;
use std::collections::HashSet;
use std::ffi::{c_char, c_void, CString};

/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#GetNumberOfCurrentPlayers).
#[derive(Debug)]
pub struct GetNumberOfCurrentPlayers;

impl SteamworksDispatch for GetNumberOfCurrentPlayers {
	type CType = sys::NumberOfCurrentPlayers_t;
	type Output = i32;

	fn post(self, call_result: Box<Self::CType>, _private: Private) -> Self::Output {
		assert_ne!(call_result.m_bSuccess as u8, 0u8);

		call_result.m_cPlayers as _
	}

	unsafe fn dispatch(&mut self, _private: Private) -> sys::SteamAPICall_t {
		sys::SteamAPI_ISteamUserStats_GetNumberOfCurrentPlayers(sys::SteamAPI_SteamUserStats_v013())
	}
}

/// An individual Steam user's stats.
#[derive(Clone, Copy, Debug)]
pub struct UserStats<'a> {
	steam_id: SteamId,
	user_stats_interface: &'a UserStatsInterface,
}

impl<'a> UserStats<'a> {
	/// `Stat` should either be a `i32` or `f32`.
	/// Returns `None` if one of the following is the case:
	/// - The stats for the user are not loaded
	/// - The stat does not exist for the app
	/// - The stat type specified here
	#[allow(private_bounds)]
	pub fn get_stat<T: Stat>(&self, str: impl AsRef<str>) -> Option<T> {
		let c_str = CString::new(str.as_ref()).unwrap();

		T::get(self.user_stats_interface.interface.ptr(), c_str.as_ptr(), Some(self.steam_id), Private(()))
	}

	pub fn steam_id(&self) -> SteamId {
		self.steam_id
	}
}

impl<'a> Eq for UserStats<'a> {}

impl<'a> PartialEq for UserStats<'a> {
	fn eq(&self, other: &Self) -> bool {
		self.steam_id.eq(&other.steam_id)
	}
}

/// Provides access to the current user's stats, and utilities to access other users' stats.
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats).
#[derive(Debug)]
pub struct UserStatsInterface {
	cached: HashSet<SteamId>,
	interface: FixedInterface<sys::ISteamUserStats>,
}

impl UserStatsInterface {
	/// # Safety
	/// Steamworks' globals must be initialized.
	/// Steamworks must only shut down if this is dropped, or is being dropped.
	pub(crate) unsafe fn new() -> Self {
		Self {
			cached: HashSet::new(),
			interface: FixedInterface::new(sys::SteamAPI_SteamUserStats_v013()),
		}
	}

	/// Attempts to get a cached [UserStats].
	pub fn get_cached_user(&self, steam_id: SteamId) -> Option<UserStats> {
		Some(UserStats {
			steam_id: *self.cached.get(&steam_id)?,
			user_stats_interface: self,
		})
	}

	/// Gets the currently caches [UserStats] or loads it.
	pub async fn get_or_load_user(&mut self, steam_id: SteamId) -> Result<UserStats, Error> {
		//we use contains instead of
		// `if let Some(existing) = self.stored.get(&steam_id) { ... }`
		//to prevent immutable borrowing of the data inside the statement
		//which would prevent the mutable borrow below, even if it doesn't look like it would >:(
		if self.cached.contains(&steam_id) {
			return Ok(UserStats {
				steam_id: *self.cached.get(&steam_id).unwrap(),
				user_stats_interface: self,
			});
		}

		self.load(steam_id).await
	}

	/// Gets the total number of achievements the app has.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#GetNumAchievements)
	pub fn get_num_achievements(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamUserStats_GetNumAchievements(self.interface()) }
	}

	#[allow(private_bounds)]
	pub fn get_stat<T: Stat>(&self, str: impl AsRef<str>) -> Option<T> {
		let c_str = CString::new(str.as_ref()).unwrap();

		T::get(self.interface(), c_str.as_ptr(), None, Private(()))
	}

	/// Displays a Steam notification of an achievement's progress.
	/// Although not explicitly required, you should be calling [set_stat](Self::set_stat) before this.
	/// Returns `true` upon success.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#IndicateAchievementProgress)
	pub fn indicate_achievement_progress(&self, str: impl AsRef<str>, current_progress: u32, maximum_progress: u32) -> bool {
		let c_str = CString::new(str.as_ref()).unwrap();

		unsafe { sys::SteamAPI_ISteamUserStats_IndicateAchievementProgress(self.interface(), c_str.as_ptr(), current_progress, maximum_progress) }
	}

	#[doc(hidden)]
	fn interface(&self) -> *mut sys::ISteamUserStats {
		self.interface.ptr()
	}

	/// Loads [UserStats] for the user by their [SteamId].
	/// Safe to use for refreshing stats.
	pub async fn load(&mut self, steam_id: SteamId) -> Result<UserStats, Error> {
		//dispatch a function call for the user stats
		Steamworks::get().unwrap().call_manager().call(RequestUserStats { steam_id, user_stats_interface: self }).await??;

		//the function call done above inserts the steam ID into our cache
		//so we can be confident it is available
		Ok(UserStats {
			steam_id: *self.cached.get(&steam_id).unwrap(),
			user_stats_interface: self,
		})
	}

	/// As per the docs:
	/// > This automatically calls [StoreStats](https://partner.steamgames.com/doc/api/ISteamUserStats#StoreStats) (eq: [store](Self::store)) to persist the changes to the server.
	/// This should typically only be used for testing purposes during development.
	///
	/// [RequestCurrentStats](https://partner.steamgames.com/doc/api/ISteamUserStats#RequestCurrentStats) was removed in Steamworks v1.61 and thus does not need to be used for this.
	///
	/// Returns `true` upon success.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#ResetAllStats)
	pub fn reset_all_stats(&self, achievements_too: bool) -> bool {
		unsafe { sys::SteamAPI_ISteamUserStats_ResetAllStats(self.interface(), achievements_too) }
	}

	#[allow(private_bounds)]
	pub fn set_stat<T: Stat>(&self, str: impl AsRef<str>, value: T) -> bool {
		let c_str = CString::new(str.as_ref()).unwrap();

		T::set(self.interface(), c_str.as_ptr(), value, Private(()))
	}

	/// Uploads the stats for permanent storage.
	/// Returns `true` upon success.  
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#StoreStats)
	pub fn store(&self) -> bool {
		//TODO: check UserStatsStored_t callback for errors
		unsafe { sys::SteamAPI_ISteamUserStats_StoreStats(self.interface()) }
	}

	/// Returns `true` upon success.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#UpdateAvgRateStat)
	pub fn update_avg_stat(&self, str: impl AsRef<str>, count: f32, session_length: f64) -> bool {
		let c_str = CString::new(str.as_ref()).unwrap();

		unsafe { sys::SteamAPI_ISteamUserStats_UpdateAvgRateStat(self.interface(), c_str.as_ptr(), count, session_length) }
	}
}

/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUserStats#RequestUserStats)
#[derive(Debug)]
struct RequestUserStats<'a> {
	steam_id: SteamId,
	user_stats_interface: &'a mut UserStatsInterface,
}

impl<'a> SteamworksDispatch for RequestUserStats<'a> {
	type CType = sys::UserStatsReceived_t;
	type Output = Result<(), GeneralError>;

	fn post(self, call_result: Box<Self::CType>, _private: Private) -> Self::Output {
		if let Some(error) = GeneralError::new(call_result.m_eResult) {
			Err(error)
		} else {
			self.user_stats_interface.cached.insert(self.steam_id);

			Ok(())
		}
	}

	unsafe fn dispatch(&mut self, _private: Private) -> sys::SteamAPICall_t {
		sys::SteamAPI_ISteamUserStats_RequestUserStats(sys::SteamAPI_SteamUserStats_v013(), self.steam_id.0)
	}
}

/// Keep private!
/// The unloading of user stats is handled in
#[derive(Debug)]
pub(crate) struct UserStatsUnloaded;

impl SteamworksCallback for UserStatsUnloaded {
	const C_ENUM: i32 = sys::UserStatsUnloaded_t_k_iCallback as i32;
	type ListenFn = ();

	unsafe fn callback(void_ptr: *mut c_void, _listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = unsafe { &*(void_ptr as *mut sys::UserStatsUnloaded_t) };
		let steamworks = Steamworks::get().unwrap();

		steamworks.user_stats_interface().cached.remove(&SteamId::from(data.m_steamIDUser));
	}
}

/// Implemented for `i32` and `f32` - the only types Steam offers for stats.
pub trait Stat: Copy + std::fmt::Debug + Sized + Send + Sync {
	#![allow(private_interfaces)]
	fn get(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, steam_id: Option<SteamId>, _private: Private) -> Option<Self>;
	fn set(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, value: Self, _private: Private) -> bool;
}

impl Stat for i32 {
	#![allow(private_interfaces)]
	fn get(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, steam_id: Option<SteamId>, _private: Private) -> Option<Self> {
		let mut result = 0i32;

		if unsafe {
			if let Some(steam_id) = steam_id {
				sys::SteamAPI_ISteamUserStats_GetUserStatInt32(interface, steam_id.0, stat_name, &mut result as *mut i32)
			} else {
				sys::SteamAPI_ISteamUserStats_GetStatInt32(interface, stat_name, &mut result as *mut i32)
			}
		} {
			Some(result)
		} else {
			None
		}
	}

	fn set(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, value: Self, _private: Private) -> bool {
		unsafe { sys::SteamAPI_ISteamUserStats_SetStatInt32(interface, stat_name, value) }
	}
}

impl Stat for f32 {
	#![allow(private_interfaces)]
	fn get(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, steam_id: Option<SteamId>, _private: Private) -> Option<Self> {
		let mut result = 0f32;

		if unsafe {
			if let Some(steam_id) = steam_id {
				sys::SteamAPI_ISteamUserStats_GetUserStatFloat(interface, steam_id.0, stat_name, &mut result as *mut f32)
			} else {
				sys::SteamAPI_ISteamUserStats_GetStatFloat(interface, stat_name, &mut result as *mut f32)
			}
		} {
			Some(result)
		} else {
			None
		}
	}

	fn set(interface: *mut sys::ISteamUserStats, stat_name: *const c_char, value: Self, _private: Private) -> bool {
		unsafe { sys::SteamAPI_ISteamUserStats_SetStatFloat(interface, stat_name, value) }
	}
}
