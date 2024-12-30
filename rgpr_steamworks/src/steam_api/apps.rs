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

	/// > Checks if a specific app is installed.
	/// The app may not actually be owned by the current user,
	/// they may have it left over from a free weekend, etc.
	/// This only works for base applications, not Downloadable Content (DLC).
	/// Use [`dlc_installed`](Self::is_dlc_installed) for DLC instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsAppInstalled)
	pub fn app_installed(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsAppInstalled(self.interface(), app_id.into().0) }
	}

	/// > Gets the buildid of this app,
	/// may change at any time based on backend updates to the game.
	///
	/// Returns `None` if you're not running a build downloaded from Steam.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetAppBuildId)
	pub fn build_id(&self) -> Option<NonZeroI32> {
		unsafe { NonZeroI32::new(sys::SteamAPI_ISteamApps_GetAppBuildId(self.interface())) }
	}

	/// > Gets the number of DLC pieces for the current app.
	/// Note that the returned value may max out at 64,
	/// depending on how much unowned DLC the user has.
	/// If your app has a large number of DLC,
	/// you should set your own internal list of known DLC to check against.
	///
	/// Consider using [`dlc_iter`](Self::dlc_iter) to get a list of the DLC apps.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetDLCCount)
	pub fn dlc_count(&self) -> i32 {
		unsafe { sys::SteamAPI_ISteamApps_GetDLCCount(self.interface()) }
	}

	/// > Gets the download progress for optional DLC.
	///
	/// Returns `None` if the DLC doesn't exist OR the DLC is not currently downloading.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetDlcDownloadProgress)
	pub fn dlc_download_progress(&self, app_id: impl Into<AppId>) -> Option<DlcDownloadProgress> {
		let mut progress = DlcDownloadProgress { downloaded: 0, total: 0 };

		if unsafe { sys::SteamAPI_ISteamApps_GetDlcDownloadProgress(self.interface(), app_id.into().0, &mut progress.downloaded as *mut _, &mut progress.total as *mut _) } {
			Some(progress)
		} else {
			None
		}
	}

	/// Returns an iterator that calls [`get_dlc`](Self::get_dlc).
	///
	/// > If you have more than 64 DLC,
	/// you may want to setup your own internal list of DLC instead.
	/// - [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetDLCCount)
	pub fn dlc_iter(&self) -> DlcIter {
		DlcIter { apps_interface: &self, current: 0 }
	}

	/// > Checks if the user owns a specific DLC and if the DLC is installed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsDlcInstalled)
	pub fn dlc_installed(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsDlcInstalled(self.interface(), app_id.into().0) }
	}

	/// > Gets the time of purchase of the specified app in Unix epoch format
	/// (time since Jan 1st, 1970).
	/// This is useful for rewarding users based on their initial purchase date.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetEarliestPurchaseUnixTime)
	pub fn earliest_purchase_time(&self, app_id: impl Into<AppId>) -> SystemTime {
		let unix_time = unsafe { sys::SteamAPI_ISteamApps_GetEarliestPurchaseUnixTime(self.interface(), app_id.into().0) };

		SystemTime::UNIX_EPOCH + Duration::from_secs(unix_time as u64)
	}

	/// Asynchronously retrieves metadata details about a specific file in the depot manifest.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetFileDetails)
	pub fn file_details(&self) {
		todo!();
	}

	/// > Returns metadata for a DLC by index.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
	pub fn get_dlc(&self, index: impl IntoCIndex) -> Option<DlcData> {
		let mut app_id = c_uint::default();
		let mut available = false;

		//128 is what is shown in the example
		let mut name_buffer = [0u8; 128];

		//TODO: verify that this function respects adding a null terminator

		//returns true upon success
		if unsafe {
			sys::SteamAPI_ISteamApps_BGetDLCDataByIndex(
				self.interface(),
				index.into_c_index(),
				&mut app_id as *mut sys::AppId_t,
				&mut available as *mut _,
				name_buffer.as_mut_ptr() as *mut c_char,
				name_buffer.len() as i32,
			)
		} {
			//TODO: check vs CStr::from_ptr
			let name_cstr = CStr::from_bytes_until_nul(&name_buffer).unwrap();

			Some(DlcData {
				app_id: app_id.into(),
				available,
				name: name_cstr.to_string_lossy().into_owned(),
			})
		} else {
			None
		}
	}

	pub fn install_dir(&self, app_id: impl Into<AppId>) {
		let mut dir_buf = [0u8; MAX_PATH];
		let buf_ptr = dir_buf.as_mut_ptr();

		unsafe {
			sys::SteamAPI_ISteamApps_GetAppInstallDir(self.interface(), app_id.into().0, buf_ptr as _, MAX_PATH as u32);
			
		}
		

		todo!();
	}

	/// > Gets a list of all installed depots for a given App ID in mount order.
	///
	/// `count` is the amount of entries to pre-allocate for.
	/// Typically, `count` is set to match the maximum amount of installed depots.
	/// If you're not sure, just set it to your total depots.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetInstalledDepots)
	pub fn installed_depots(&self, app_id: impl Into<AppId>, count: usize) -> Vec<DepotId> {
		todo!();
	}

	/// > Checks if the license owned by the user provides low violence depots.
	/// Low violence depots are useful for copies sold in countries that have content restrictions.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsLowViolence)
	pub fn low_violence(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsLowViolence(self.interface()) }
	}

	/// > Checks if the active user is subscribed to the current App ID.
	///
	/// For checking if the account is subscribed to other apps,
	/// use [`subscribed_to`](Self::is_subscribed_to).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribed)
	pub fn subscribed(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribed(self.interface()) }
	}

	/// > Checks if the active user is subscribed to a specified AppId.
	/// Only use this if you need to check ownership of another game related to yours,
	/// a demo for example.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedApp)
	pub fn subscribed_to(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedApp(self.interface(), app_id.into().0) }
	}

	/// > Checks if the active user is accessing the current appID via a temporary
	/// Family Shared license owned by another user.
	/// If you need to determine the steamID of the permanent owner of the license,
	/// use [`app_owner`](Self::app_owner).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedFromFamilySharing)
	pub fn subscribed_from_family_sharing(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedFromFamilySharing(self.interface()) }
	}

	/// > Checks if the user is subscribed to the current appID through a free weekend.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedFromFreeWeekend)
	pub fn subscribed_from_free_weekend(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedFromFreeWeekend(self.interface()) }
	}

	/// > Checks if the user is subscribed to the current appID through a timed trial.
	/// If so, returns true and gives back the total time the timed trial is allowed to play,
	/// along with the current amount of time the user has played.
	///
	/// Returns `None` if the license is not a timed trial license.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsTimedTrial)
	pub fn timed_trial(&self) -> Option<TimedTrial> {
		let mut timed_trial = TimedTrial { secs_allowed: 0, secs_played: 0 };

		if unsafe { sys::SteamAPI_ISteamApps_BIsTimedTrial(self.interface(), &mut timed_trial.secs_allowed as *mut _, &mut timed_trial.secs_played as *mut _) } {
			Some(timed_trial)
		} else {
			None
		}
	}

	/// > Checks if the user has a VAC ban on their account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsVACBanned)
	pub fn vac_banned(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsVACBanned(self.interface()) }
	}
}

/// Metadata for DLC.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
#[derive(Clone, Debug)]
pub struct DlcData {
	/// App ID of the DLC itself, not the app that owns it.
	pub app_id: AppId,

	/// > Returns whether the DLC is currently available on the Steam store.
	/// Will be false if the DLC does not have a visible store page.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
	pub available: bool,
	pub name: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct DlcDownloadProgress {
	/// The amount of bytes that have been downloaded.
	pub downloaded: u64,

	/// The amount of bytes in total, to download, including currently downloaded bytes.
	pub total: u64,
}

impl DlcDownloadProgress {
	fn new(downloaded: u64, total: u64) -> Self {
		Self { downloaded, total }
	}

	pub fn fraction(&self) -> f32 {
		self.downloaded as f32 / self.total as f32
	}
}

/// Iterator which yields a [`DlcData`] for each of the current app's DLCs.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
#[derive(Debug)]
pub struct DlcIter<'a> {
	apps_interface: &'a AppsInterface,
	current: i32,
}

impl<'a> Iterator for DlcIter<'a> {
	type Item = DlcData;

	fn next(&mut self) -> Option<Self::Item> {
		//unwrap: the function only returns None if we're out-of-bounds
		//dlc_count check above prevents that
		let dlc_opt = self.apps_interface.get_dlc(self.current);
		self.current += 1;

		dlc_opt
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let dlc_count = self.apps_interface.dlc_count() as usize;

		(dlc_count, Some(dlc_count))
	}
}

impl<'a> ExactSizeIterator for DlcIter<'a> {}

/// Provided by [`AppsInterface::is_timed_trial`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TimedTrial {
	pub secs_allowed: u32,
	pub secs_played: u32,
}

impl TimedTrial {
	/// The `secs_allowed` field but as a [`Duration`] instead of a `u32`.
	pub fn duration_allowed(&self) -> Duration {
		Duration::from_secs(self.secs_allowed as u64)
	}

	/// The `secs_played` field but as a [`Duration`] instead of a `u32`.
	pub fn duration_played(&self) -> Duration {
		Duration::from_secs(self.secs_played as u64)
	}

	/// Time left for the trial license.
	pub fn duration_left(&self) -> Duration {
		self.duration_allowed().saturating_sub(self.duration_played())
	}

	/// Time left for the trial license.
	pub fn secs_left(&self) -> u32 {
		self.secs_allowed.saturating_sub(self.secs_played)
	}
}
