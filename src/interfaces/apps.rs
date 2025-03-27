//! See [`AppsInterface`].

use crate::call::{Dispatch};
use crate::dt::{AppId, CsvString, DepotId, SteamId};
use crate::error::{CallError, GeneralError, SteamError, UnspecifiedError};
use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild};
use crate::iter::{SteamApiIterator, Unreliable};
use crate::util::{expect_string, some_string, success, CStrArray, CStrArrayPath, MAX_PATH};
use crate::{sys, Private};
use bitflags::bitflags;
use rgpr_steamworks_macros::callback;
use rgpr_steamworks_sys::SteamAPICall_t;
use std::ffi::{c_int, c_uint, CString};
use std::fmt::{Display, Formatter};
use std::iter::FusedIterator;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::ptr::null_mut;
use std::time::{Duration, SystemTime};

impl AsRef<AppsInterface> for super::Interfaces {
	fn as_ref(&self) -> &AppsInterface {
		&self.apps
	}
}

/// > Exposes a wide range of information and actions for applications and [Downloadable Content (DLC)].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps)
///
/// [Downloadable Content (DLC)]: https://partner.steamgames.com/doc/store/application/dlc
#[derive(Debug)]
pub struct AppsInterface {
	fip: FixedInterfacePtr<sys::ISteamApps>,
	steam: SteamChild,
}

impl AppsInterface {
	/// > Checks if a specific app is installed.
	/// The app may not actually be owned by the current user,
	/// they may have it left over from a free weekend, etc.
	/// This only works for base applications, not Downloadable Content (DLC).
	/// Use [`dlc_installed`](Self::dlc_installed) for DLC instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsAppInstalled)
	pub fn app_installed(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsAppInstalled(*self.fip, app_id.into().0) }
	}

	/// > Gets the Steam ID of the true owner of the current app.
	/// This is different from the current user if they are accessing this app via Family Sharing.
	///
	/// You can use [`subscribed_from_family_sharing`] to check if the current app is used through family sharing.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetAppOwner)
	///
	/// [`subscribed_from_family_sharing`]: Self::subscribed_from_family_sharing
	pub fn app_owner(&self) -> SteamId {
		unsafe { SteamId(sys::SteamAPI_ISteamApps_GetAppOwner(*self.fip)) }
	}

	/// > Gets a comma separated list of the languages the current app supports.
	/// For the full list of languages that may be returned see [Localization and Languages].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetAvailableGameLanguages)
	///
	/// [Localization and Languages]: https://partner.steamgames.com/doc/store/localization
	///
	/// TODO: Steam may treat an empty string as nominal, if so: use some_string instead
	pub fn available_languages(&self) -> CsvString {
		unsafe { CsvString(expect_string(sys::SteamAPI_ISteamApps_GetAvailableGameLanguages(*self.fip))) }
	}

	/// > Returns total number of known app branches (including default "public" branch)
	/// which can be iterated with [`beta_iter`].
	///
	/// [`BetaCount`] additionally contains the count of private and available beta branches.  
	/// Use [`beta_count_total`] if you only need the total amount of branches.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetNumBetas)
	///
	/// [`beta_count_total`]: Self::beta_count_total
	/// [`beta_iter`]: Self::beta_iter
	/// [`BetaCount`]: BetaCount
	pub fn beta_count(&self) -> BetaCount {
		let mut counts = BetaCount { total: 0, available: 0, private: 0 };
		counts.total = unsafe { sys::SteamAPI_ISteamApps_GetNumBetas(*self.fip, &mut counts.available as *mut u32 as *mut c_int, &mut counts.private as *mut u32 as *mut c_int) } as u32;

		counts
	}

	/// > Returns total number of known app branches (including default "public" branch)
	/// which can be iterated with [`beta_iter`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetNumBetas)
	///
	/// [`beta_iter`]: Self::beta_iter
	pub fn beta_count_total(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamApps_GetNumBetas(*self.fip, null_mut(), null_mut()) as u32 }
	}

	/// > Get details about an app beta branch like name, description and state.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetBetaInfo)
	pub fn beta_iter(&self) -> Unreliable<BetaIter> {
		BetaIter { apps_interface: &self, cursor: 0 }.wrap()
	}

	/// > Gets the buildid of this app,
	/// may change at any time based on backend updates to the game.
	///
	/// Returns `None` if you're not running a build downloaded from Steam.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetAppBuildId)
	pub fn build_id(&self) -> BuildId {
		unsafe { sys::SteamAPI_ISteamApps_GetAppBuildId(*self.fip) }.into()
	}

	/// > Checks if the user is running from a beta branch, and gets the name of the branch if they are.
	///
	/// Returns `None` if the current branch is not a beta branch.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetCurrentBetaName)
	pub fn current_beta_name(&self) -> Option<String> {
		let mut buffer = CStrArray::<128>::new();

		unsafe {
			if sys::SteamAPI_ISteamApps_GetCurrentBetaName(*self.fip, buffer.ptr(), buffer.c_len()) {
				Some(buffer.to_string())
			} else {
				None
			}
		}
	}

	/// > Gets the current language that the user has set.
	/// This falls back to the Steam UI language if the user hasn't explicitly picked a language for the title.
	/// For the full list of languages see [Supported Languages].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetCurrentGameLanguage)
	///
	/// [Supported Languages]: https://partner.steamgames.com/doc/store/localization/languages
	pub fn current_language(&self) -> String {
		unsafe { some_string(sys::SteamAPI_ISteamApps_GetCurrentGameLanguage(*self.fip)).unwrap() }
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
	pub fn dlc_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamApps_GetDLCCount(*self.fip) as u32 }
	}

	/// > Gets the download progress for optional DLC.
	///
	/// Returns `None` if the DLC doesn't exist OR the DLC is not currently downloading.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetDlcDownloadProgress)
	pub fn dlc_download_progress(&self, app_id: impl Into<AppId>) -> Option<DlcDownloadProgress> {
		let mut progress = DlcDownloadProgress::new(0, 0);

		if unsafe { sys::SteamAPI_ISteamApps_GetDlcDownloadProgress(*self.fip, app_id.into().0, &mut progress.downloaded as *mut _, &mut progress.total as *mut _) } {
			Some(progress)
		} else {
			None
		}
	}

	/// Returns an iterator
	///
	/// > If you have more than 64 DLC,
	/// you may want to setup your own internal list of DLC instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetDLCCount)
	pub fn dlc_iter(&self) -> Unreliable<DlcIter> {
		DlcIter { apps_interface: &self, cursor: 0 }.wrap()
	}

	/// > Checks if the user owns a specific DLC and if the DLC is installed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsDlcInstalled)
	pub fn dlc_installed(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsDlcInstalled(*self.fip, app_id.into().0) }
	}

	/// > Gets the time of purchase of the specified app in Unix epoch format
	/// (time since Jan 1st, 1970).
	/// This is useful for rewarding users based on their initial purchase date.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetEarliestPurchaseUnixTime)
	pub fn earliest_purchase_time(&self, app_id: impl Into<AppId>) -> SystemTime {
		let unix_time = unsafe { sys::SteamAPI_ISteamApps_GetEarliestPurchaseUnixTime(*self.fip, app_id.into().0) };

		SystemTime::UNIX_EPOCH + Duration::from_secs(unix_time as u64)
	}

	/// > Asynchronously retrieves metadata details about a specific file in the depot manifest.
	///
	/// # Panics
	/// If the provided path contains a null character.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetFileDetails)
	pub async fn file_details(&self, path: impl Into<PathBuf>) -> Result<FileDetails, CallError<GeneralError>> {
		//callback definition
		//its only ever used by this function,
		//thus the definition is only here
		#[doc(hidden)]
		struct GetFileDetails {
			file_name: CString,
			steam: SteamChild,
		}

		unsafe impl Dispatch for GetFileDetails {
			type CType = sys::FileDetailsResult_t;
			type Output = FileDetails;
			type Error = GeneralError;

			unsafe fn dispatch(&mut self, _: Private) -> SteamAPICall_t {
				let steam = self.steam.get();

				sys::SteamAPI_ISteamApps_GetFileDetails(*steam.interfaces.apps.fip, self.file_name.as_ptr())
			}

			fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
				match GeneralError::new(c_data.m_eResult) {
					None => Ok(FileDetails {
						size: c_data.m_ulFileSize.into(),
						flags: c_data.m_unFlags.into(),
						sha: c_data.m_FileSHA.into(),
					}),

					Some(general_error) => Err(general_error),
				}
			}
		}

		//function start
		let path = path.into();
		let steam = self.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(GetFileDetails {
			file_name: CString::new(path.into_os_string().into_encoded_bytes()).unwrap(),
			steam: self.steam.clone(),
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		future.await
	}

	/// > Gets a list of all installed depots for a given App ID in mount order.
	///
	/// Gets up to a maximum of `capacity` depots, the `capacity` of the vector that is allocated.
	/// Use [`installed_depots_iter`] if you are not the maximum amount of depots.
	/// Use [`installed_depot`] if you only want the first one.
	///
	/// [`installed_depot`]: Self::installed_depot
	/// [`installed_depots_iter`]: Self::installed_depots_iter
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetInstalledDepots)
	pub fn get_installed_depots(&self, app_id: impl Into<AppId>, capacity: usize) -> Vec<DepotId> {
		//SAFETY: DepotId is the same size as its sys counterpart as checked in the dt::test module
		let mut vec: Vec<DepotId> = Vec::with_capacity(capacity);

		unsafe {
			let filled = sys::SteamAPI_ISteamApps_GetInstalledDepots(*self.fip, app_id.into().0, vec.as_mut_ptr() as _, vec.capacity() as _);

			vec.set_len(filled as usize);
		}

		vec
	}

	/// > Gets the install folder for a specific AppID.
	/// This works even if the application is not installed,
	/// based on where the game would be installed with the default Steam library location.
	///
	/// On non-linux targets, this may return `None` if the install directory cannot be converted to valid UTF-8.
	/// See [`install_dir_bytes`] for an infallible version.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetAppInstallDir)
	///
	/// [`install_dir_bytes`]: Self::install_dir_bytes
	pub fn get_install_dir(&self, app_id: impl Into<AppId>) -> Option<PathBuf> {
		let mut buf = CStrArrayPath::new();

		unsafe { sys::SteamAPI_ISteamApps_GetAppInstallDir(*self.fip, app_id.into().0, buf.ptr(), MAX_PATH as u32) };

		buf.get_path().map(Path::to_owned)
	}

	/// Infallible version of [`get_install_dir`].
	///
	/// [`get_install_dir`]: Self::get_install_dir
	pub fn install_dir_bytes(&self, app_id: impl Into<AppId>) -> Vec<u8> {
		let mut buf = CStrArrayPath::new();

		unsafe { sys::SteamAPI_ISteamApps_GetAppInstallDir(*self.fip, app_id.into().0, buf.ptr(), MAX_PATH as u32) };

		Vec::from(buf.c_str().to_bytes())
	}

	/// > Allows you to install an optional DLC.
	///
	/// Triggers a [`DlcInstalled`] callback.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#InstallDLC)
	pub fn install_dlc(&self, app_id: impl Into<AppId>) {
		unsafe {
			sys::SteamAPI_ISteamApps_InstallDLC(*self.fip, app_id.into().0);
		}
	}

	/// Gets a single installed depot.
	/// Useful if you need the [DepotId] and are sure you the app has only 1 depot.
	///
	/// Returns `None` if there are no depots,
	/// typically when the app is run before its first upload.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetInstalledDepots)
	pub fn installed_depot(&self, app_id: impl Into<AppId>) -> Option<DepotId> {
		let app_id = app_id.into();
		let mut depot_id = DepotId(0);

		if unsafe { sys::SteamAPI_ISteamApps_GetInstalledDepots(*self.fip, app_id.0, &mut depot_id as *mut DepotId as _, 1) } != 0 {
			Some(depot_id)
		} else {
			None
		}
	}

	/// Creates an iterator that dynamically requests batches of the installed depot IDs.
	/// Although this is marked as [`Unreliable`], it is extremely unlikely to suffer from race conditions.
	///
	/// > Gets a list of all installed depots for a given App ID in mount order.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetInstalledDepots)
	pub fn installed_depots_iter(&self, app_id: impl Into<AppId>) -> Unreliable<DepotIter> {
		Unreliable(DepotIter {
			apps_interface: &self,
			app_id: app_id.into(),
			cursor: 0,
			depots: Vec::new(),
		})
	}

	/// > Gets the command line if the game was launched via Steam URL, e.g. `steam://run/<appid>//<command line>/`.
	/// This method is preferable to launching with a command line via the operating system,
	/// which can be a security risk.
	/// In order for rich presence joins to go through this and not be placed on the OS command line,
	/// you must enable "Use launch command line" from the **Installation** > **General** page on your app.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetLaunchCommandLine)
	pub fn launch_command_line(&self) -> String {
		let mut buffer = CStrArray::<1024>::new();

		unsafe {
			sys::SteamAPI_ISteamApps_GetLaunchCommandLine(*self.fip, buffer.ptr(), buffer.c_len());

			buffer.to_string()
		}
	}

	/// > Gets the associated launch parameter if the game is run via  
	/// `steam://run/<appid>/?param1=value1;param2=value2;param3=value3` etc.  
	/// <br>
	/// Parameter names starting with the character `'@'` are reserved for internal use and will always return an empty string.  
	/// <br>
	/// Parameter names starting with an underscore `'_'` are reserved for steam features -- they can be queried by the game,
	/// but it is advised that you not param names beginning with an underscore for your own features.  
	/// <br>
	/// Returns an empty string (`""`) if the specified key does not exist.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetLaunchQueryParam)
	pub fn launch_query_param(&self, key: impl Into<Vec<u8>>) -> Option<String> {
		let c_string = CString::new(key).unwrap();

		unsafe { some_string(sys::SteamAPI_ISteamApps_GetLaunchQueryParam(*self.fip, c_string.as_ptr())) }
	}

	/// > Checks if the license owned by the user provides low violence depots.
	/// Low violence depots are useful for copies sold in countries that have content restrictions.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsLowViolence)
	pub fn low_violence(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsLowViolence(*self.fip) }
	}

	/// > Allows you to force verify game content on next launch.
	/// If you detect the game is out-of-date (for example, by having the client detect a version mismatch with a server),
	/// you can call use `mark_content_corrupt` to force a verify, show a message to the user, and then quit.
	///
	/// You don't actually have to quit if you call this.
	/// Just make sure your client is up-to-date with
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#MarkContentCorrupt)
	pub fn mark_content_corrupt(&self, missing_files_only: bool) -> Result<(), UnspecifiedError> {
		success(unsafe { sys::SteamAPI_ISteamApps_MarkContentCorrupt(*self.fip, missing_files_only) })
	}

	/// > Select an beta branch for this app as active,
	/// might need the game to restart so Steam can update its' content that branch.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#SetActiveBeta)
	pub fn set_beta(&self, beta_name: impl Into<Vec<u8>>) -> Result<(), SteamError> {
		let c_string = CString::new(beta_name)?;

		if unsafe { sys::SteamAPI_ISteamApps_SetActiveBeta(*self.fip, c_string.as_ptr()) } {
			Ok(())
		} else {
			Err(SteamError::Unspecified)
		}
	}

	/// Not in Steamworks Docs.
	///
	/// From `isteamapps.h`:
	///
	/// > Set current DLC AppID being played (or 0 if none).
	/// Allows Steam to track usage of major DLC extensions
	pub fn set_dlc_context(&self, app_id: impl Into<AppId>) -> Result<(), UnspecifiedError> {
		success(unsafe { sys::SteamAPI_ISteamApps_SetDlcContext(*self.fip, app_id.into().0) })
	}

	/// > Checks if the active user is subscribed to the current App ID.
	///
	/// For checking if the account is subscribed to other apps,
	/// use [`subscribed_to`](Self::subscribed_to).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribed)
	pub fn subscribed(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribed(*self.fip) }
	}

	/// > Checks if the active user is subscribed to a specified AppId.
	/// Only use this if you need to check ownership of another game related to yours,
	/// a demo for example.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedApp)
	pub fn subscribed_to(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedApp(*self.fip, app_id.into().0) }
	}

	/// > Checks if the active user is accessing the current appID via a temporary
	/// Family Shared license owned by another user.
	/// If you need to determine the steamID of the permanent owner of the license,
	/// use [`app_owner`](Self::app_owner).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedFromFamilySharing)
	pub fn subscribed_from_family_sharing(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedFromFamilySharing(*self.fip) }
	}

	/// > Checks if the user is subscribed to the current appID through a free weekend.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsSubscribedFromFreeWeekend)
	pub fn subscribed_from_free_weekend(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsSubscribedFromFreeWeekend(*self.fip) }
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

		if unsafe { sys::SteamAPI_ISteamApps_BIsTimedTrial(*self.fip, &mut timed_trial.secs_allowed as *mut _, &mut timed_trial.secs_played as *mut _) } {
			Some(timed_trial)
		} else {
			None
		}
	}

	/// > Allows you to uninstall an optional DLC.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#UninstallDLC)
	pub fn uninstall_dlc(&self, app_id: impl Into<AppId>) {
		unsafe {
			sys::SteamAPI_ISteamApps_UninstallDLC(*self.fip, app_id.into().0);
		}
	}

	/// > Checks if the user has a VAC ban on their account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BIsVACBanned)
	pub fn vac_banned(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamApps_BIsVACBanned(*self.fip) }
	}
}

impl Interface for AppsInterface {
	type CInterface = sys::ISteamApps;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, steam: SteamChild) -> Self {
		Self { fip, steam }
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		sys::SteamAPI_SteamApps_v008()
	}
}

/// Represents a single beta branch.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetBetaInfo)
#[derive(Clone, Debug, Hash)]
pub struct Beta {
	flags: BetaFlags,
	build_id: BuildId,
	name: String,
	description: String,
}

impl Beta {
	/// > This branch can be selected (available)
	pub fn available(&self) -> bool {
		self.flags.bits() & BetaFlags::AVAILABLE.bits() != 0
	}

	/// Returns the build ID associated with the beta.
	/// This will match what [`AppsInterface::build_id`] returns if they are on this branch.
	pub fn build_id(&self) -> BuildId {
		self.build_id
	}

	/// > This is the default branch ("public")
	///
	/// `steamclientpublic.h` `EBetaBranchFlags`
	pub fn default_branch(&self) -> bool {
		self.flags.bits() & BetaFlags::DEFAULT.bits() != 0
	}

	/// No documentation.
	pub fn description(&self) -> &str {
		&self.description
	}

	/// > This is the currently installed branch (mounted)
	pub fn installed(&self) -> bool {
		self.flags.bits() & BetaFlags::INSTALLED.bits() != 0
	}

	/// The name that is shown in the game's properties in the user's Steam library.
	pub fn name(&self) -> &str {
		&self.name
	}

	/// > This is a private branch (password protected)
	pub fn private(&self) -> bool {
		self.flags.bits() & BetaFlags::PRIVATE.bits() != 0
	}

	/// > This is the currently selected branch (active)
	pub fn selected(&self) -> bool {
		self.flags.bits() & BetaFlags::SELECTED.bits() != 0
	}
}

/// The different counts of beta branches.
#[derive(Clone, Copy, Debug, Hash)]
pub struct BetaCount {
	pub total: u32,
	pub available: u32,
	pub private: u32,
}

bitflags! {
	/// `steamclientpublic.h` `EBetaBranchFlags`
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	struct BetaFlags: u32 {
		const NONE = 0;
		const DEFAULT = 1;
		const AVAILABLE = 2;
		const PRIVATE = 4;
		const SELECTED = 8;
		const INSTALLED = 16;
	}
}

/// Iterator which yields a [`Beta`] for each of the current app's beta branches.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetBetaInfo)
#[derive(Debug)]
pub struct BetaIter<'a> {
	apps_interface: &'a AppsInterface,
	cursor: c_int,
}

unsafe impl<'a> SteamApiIterator for BetaIter<'a> {
	type Item = Beta;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		let mut flags = BetaFlags::NONE;
		let mut build_id = BuildId::default();
		let mut name_buffer = CStrArray::<128>::new(); //size is as shown in steamworks example
		let mut desc_buffer = CStrArray::<1024>::new(); //size is as shown in steamworks example

		//unfn returns true upon success
		if unsafe {
			sys::SteamAPI_ISteamApps_GetBetaInfo(
				*self.apps_interface.fip,
				index,
				&mut flags as *mut BetaFlags as *mut c_uint,
				&mut build_id as *mut BuildId as *mut c_uint,
				name_buffer.ptr(),
				name_buffer.c_len(),
				desc_buffer.ptr(),
				desc_buffer.c_len(),
			)
		} {
			Some(Beta {
				flags: BetaFlags::NONE,
				build_id,
				name: name_buffer.to_string(),
				description: desc_buffer.to_string(),
			})
		} else {
			None
		}
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamApps_GetNumBetas(*self.apps_interface.fip, null_mut(), null_mut());
	}
}

/// A build ID
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct BuildId(pub Option<NonZeroU32>);

impl BuildId {
	/// Converts the `BuildId` to a `u32`.
	#[inline(always)]
	pub fn as_u32(self) -> u32 {
		//as is used instead of to or into because it is very cheap to compute
		//all of this gets compiled away anyways
		//since we're basically doing `u32 as u32`
		match self.0 {
			None => 0,
			Some(non_zero) => non_zero.get(),
		}
	}

	/// Converts a `u32` to a `BuildId`.
	pub fn new(u32: u32) -> Self {
		Self(NonZeroU32::new(u32))
	}

	/// Returns `false` if the `BuildId` is `None` / `0`.
	pub fn valid(self) -> bool {
		self.0.is_some()
	}

	pub fn valid_from(value: impl Into<Self>) -> Option<Self> {
		let value = value.into();

		if value.valid() {
			Some(value)
		} else {
			None
		}
	}
}

impl Display for BuildId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.as_u32(), f)
	}
}

impl From<BuildId> for u32 {
	fn from(BuildId(opt): BuildId) -> Self {
		match opt {
			None => 0,
			Some(non_zero) => non_zero.get(),
		}
	}
}

impl From<i32> for BuildId {
	fn from(value: i32) -> Self {
		Self::new(value as u32)
	}
}

impl From<u32> for BuildId {
	fn from(value: u32) -> Self {
		Self::new(value)
	}
}

/// Iterator which yields a [`DepotId`] for each of the current app's mounted depots.
/// Returned by [`AppsInterface::installed_depots_iter`].
///
/// This list is not expected to change during the program's lifetime.
/// Steam requires launched apps to be shutdown during update installation,
/// so the [unreliability] most Steam API iterators have is only encountered in a development environment.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#GetInstalledDepots)
///
/// [unreliability]: Unreliable
#[derive(Debug)]
pub struct DepotIter<'a> {
	apps_interface: &'a AppsInterface,
	app_id: AppId,
	cursor: usize,
	depots: Vec<DepotId>,
}

impl<'a> DepotIter<'a> {
	/// How much to increase the allocation by
	const ALLOC_STEP: usize = 16;

	/// Consumes the iterator returning the current alloc of [`DepotId`]s.
	/// This includes entries already iterated over.
	pub fn collect_current(self) -> Vec<DepotId> {
		self.depots
	}
}

impl<'a> Iterator for DepotIter<'a> {
	type Item = DepotId;

	fn next(&mut self) -> Option<Self::Item> {
		//if we reached the end of the allocation
		//ask steam for more
		//we essentially keep doing this until `length < capacity`
		//at which the cursor stops incrementing
		if self.cursor == self.depots.capacity() {
			//reserve goes by po2
			//we can save on calls by starting with a bigger step
			//the current start is 16
			//meaning the sizes in order are typically 16, 32, 64, 128, etc.
			self.depots.reserve(DepotIter::ALLOC_STEP);

			unsafe {
				let filled = sys::SteamAPI_ISteamApps_GetInstalledDepots(*self.apps_interface.fip, self.app_id.0, self.depots.as_mut_ptr() as _, self.depots.capacity() as _);

				self.depots.set_len(filled as usize);
			}
		}

		//cheap to clone
		let depot_id = self.depots.get(self.cursor).cloned();

		//only progress if we had an entry
		//if this index is now out-of-bounds
		//the next iteration will fill the vec with more
		if depot_id.is_some() {
			self.cursor += 1;
		}

		depot_id
	}
}

impl<'a> FusedIterator for DepotIter<'a> {}

/// Metadata for DLC.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
#[derive(Clone, Debug, Hash)]
pub struct Dlc {
	/// App ID of the DLC itself, not the app that owns it.
	pub app_id: AppId,

	/// > Returns whether the DLC is currently available on the Steam store.
	/// Will be false if the DLC does not have a visible store page.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
	pub available: bool,

	/// Display name of the DLC.
	pub name: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DlcDownloadProgress {
	/// The amount of bytes that have been downloaded.
	pub downloaded: u64,

	/// The amount of bytes in total, to download, including currently downloaded bytes.
	pub total: u64,
}

impl DlcDownloadProgress {
	pub fn new(downloaded: u64, total: u64) -> Self {
		Self { downloaded, total }
	}

	/// Returns the download progress as a fraction `0f32 ..= 1f32`.
	pub fn fraction(&self) -> f32 {
		self.downloaded as f32 / self.total as f32
	}
}

callback! {
	/// Steam API callback.
	///
	/// ```rs
	/// # use rgpr_steamworks::{dt::AppId, interfaces::apps::TimedTrial};
	/// fn listener(apps_interface: &AppsInterface) { }
	/// ```
	///
	/// > Triggered after the current user gains ownership of DLC and that DLC is installed.
	///
	/// Use [`AppsInterface::install_dlc`] to trigger.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#DlcInstalled_t)
	pub struct DlcInstalled;

	data -> AppId { data.m_nAppID.into() }
}

/// Iterator which yields a [`Dlc`] for each of the current app's DLCs.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#BGetDLCDataByIndex)
#[derive(Debug)]
pub struct DlcIter<'a> {
	apps_interface: &'a AppsInterface,
	cursor: c_int,
}

unsafe impl<'a> SteamApiIterator for DlcIter<'a> {
	type Item = Dlc;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		let mut app_id = c_uint::default();
		let mut available = false;

		//128 is what is shown in the example
		let mut buffer = CStrArray::<128>::new();

		//BGetDLCDataByIndex returns true upon success
		if unsafe {
			sys::SteamAPI_ISteamApps_BGetDLCDataByIndex(
				*self.apps_interface.fip,
				index,
				&mut app_id as *mut sys::AppId_t,
				&mut available as *mut _,
				buffer.ptr(),
				buffer.c_len(),
			)
		} {
			Some(Dlc {
				app_id: app_id.into(),
				available,
				name: buffer.to_string(),
			})
		} else {
			None
		}
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamApps_GetDLCCount(*self.apps_interface.fip);
	}
}

/// Yielded from [`AppsInterface::file_details`].
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileDetails {
	size: u64,
	flags: u32,
	sha: [u8; 20],
}

callback! {
	/// > Posted after the user executes a steam url with command line or query parameters such as
	/// `steam://run/<appid>//?param1=value1;param2=value2;param3=value3;`
	/// while the game is already running.
	/// The new params can be queried with [`AppsInterface::launch_command_line`] and [`AppsInterface::launch_query_param`].
	///
	/// [`AppsInterface::launch_query_param`] is the preferred and newer method.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#NewUrlLaunchParameters_t)
	#[doc(alias = "NewLaunchQueryParameters_t")]
	pub struct NewUrlLaunchParameters {
		steam: SteamChild,
	}

	new steam;
	data => interface shared AppsInterface;
}

/// Provided by [`AppsInterface::timed_trial`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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

callback! {
	/// > Sent every minute when a [AppId] is owned via a timed trial.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamApps#TimedTrialStatus_t)
	pub struct TimedTrialStatus;

	data -> (AppId, bool, TimedTrial) {
		(
			data.m_unAppID.into(),
			data.m_bIsOffline,
			TimedTrial {
				secs_allowed: data.m_unSecondsAllowed.into(),
				secs_played: data.m_unSecondsPlayed.into(),
			},
		)
	}
}

#[cfg(test)]
mod test {
	use static_assertions::assert_eq_size;
	use std::ffi::*;

	#[test]
	fn assert_sizes() {
		assert_eq_size!(super::BuildId, c_uint);
		assert_eq_size!(super::BetaFlags, c_uint);
		assert_eq_size!(u32, c_uint); //for BetaCount
	}
}
