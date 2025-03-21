//! See [`UtilsInterface`].

use crate::dt::{AppId, ImageHandle, SteamId};
use crate::error::UnspecifiedError;
use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild, SteamInterface};
use crate::sys;
use crate::util::{expect_string, lossy_cstring, some_string, success, OptionalCString};
use cfg_if::cfg_if;
use futures::channel::oneshot;
use futures::lock::Mutex as AsyncMutex;
use rgpr_steamworks_macros::callback;
use std::ffi::{c_char, c_int, CStr};
use std::num::NonZeroU32;
use std::ops::Add;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex, Once};
use std::time::{Duration, SystemTime};

impl AsRef<UtilsInterface> for super::Interfaces {
	fn as_ref(&self) -> &UtilsInterface {
		&self.utils
	}
}

/// > Interface which provides access to a range of miscellaneous utility functions.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils)
#[derive(Debug)]
pub struct UtilsInterface {
	fip: FixedInterfacePtr<sys::ISteamUtils>,
	gamepad_text_input: Arc<GamepadTextInputLock>,

	/// `true` if [`start_vr_dashboard`] has been called.
	///
	/// [`start_vr_dashboard`]: Self::start_vr_dashboard
	vr_dashboard: AtomicBool,
}

impl UtilsInterface {
	/// > Returns the number of seconds since the application was active.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetSecondsSinceAppActive)
	#[doc(alias = "GetSecondsSinceAppActive")]
	pub fn app_active_duration(&self) -> Duration {
		Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetSecondsSinceAppActive(*self.fip) } as u64)
	}

	/// > Gets the App ID of the current process.
	///
	/// Use [`SteamInterface::app_id`] instead.
	///
	/// [`SteamInterface::app_id`]: SteamInterface::app_id
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetAppID)
	#[doc(alias = "GetAppID")]
	pub fn app_id(&self) -> AppId {
		cfg_if! {
			if #[cfg(any(debug_assertions))] {
				let start_call_count = self.ipc_call_count();
				let app_id = unsafe { AppId(sys::SteamAPI_ISteamUtils_GetAppID(*self.fip)) };
				let new_call_count = self.ipc_call_count();

				//TODO: this test V
				println!("start_call_count {start_call_count} new_call_count {new_call_count}");

				app_id
			} else {
				unsafe { AppId(sys::SteamAPI_ISteamUtils_GetAppID(*self.fip)) }
			}
		}
	}

	/// > Returns the number of seconds since the user last moved the mouse.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetSecondsSinceComputerActive)
	#[doc(alias = "GetSecondsSinceComputerActive")]
	pub fn computer_active_duration(&self) -> Duration {
		Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetSecondsSinceComputerActive(*self.fip) } as u64)
	}

	/// > Gets the current amount of battery power on the computer.
	///
	/// Power returned will be in the range of `0 ..= 100`.
	/// Returns `None` when the user is on AC power.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetCurrentBatteryPower)
	#[doc(alias = "GetCurrentBatteryPower")]
	pub fn current_battery_power(&self) -> Option<u8> {
		let power = unsafe { sys::SteamAPI_ISteamUtils_GetCurrentBatteryPower(*self.fip) };

		if power == 255 {
			None
		} else {
			Some(power)
		}
	}

	/// > Filters the provided input message, and returns the censored [`String`].
	/// Legally required filtering is always applied.
	/// dditional filtering may occur, based on the context and user settings.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#FilterText)
	#[doc(alias = "FilterText")]
	pub fn filter_text(&self, context: TextFilteringContext, steam_id: impl Into<SteamId>, str: impl AsRef<str>) -> String {
		static INIT: Once = Once::new();

		INIT.call_once(|| unsafe {
			sys::SteamAPI_ISteamUtils_InitFilterText(*self.fip, 0);
		});

		let cstring = lossy_cstring(str);
		let mut output_buffer = Vec::<u8>::with_capacity(cstring.as_bytes_with_nul().len());
		let capacity = output_buffer.capacity();

		//zero-out the buffer - this helps with resolving nul bytes later
		output_buffer.resize(capacity, 0);

		//TODO: maybe provide the return for how many chars are filtered out?
		unsafe {
			sys::SteamAPI_ISteamUtils_FilterText(
				*self.fip,
				context.into(),
				steam_id.into().0,
				cstring.as_ptr(),
				output_buffer.as_mut_ptr() as *mut c_char,
				capacity as u32,
			)
		};

		CStr::from_bytes_until_nul(&output_buffer).unwrap().to_string_lossy().to_string()
	}

	/// > Opens a floating keyboard over the game content and sends OS keyboard keys directly to the game.
	/// The text field position is specified in pixels relative the origin of the game window
	/// and is used to position the floating keyboard in a way that doesn't cover the text field.
	///
	/// `avoid_*` parameters are the position and size of a rectangle on the currently focused window which you want the virtual keyboard to avoid.
	/// Large areas may suffer overlap.
	///
	/// `avoid_position` is the top-left of the rectangle to avoid.
	///
	/// You can use the [`FloatingGamepadTextInputDismissed`] callback to know when the virtual keyboard is dismissed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowFloatingGamepadTextInput)
	#[doc(alias = "ShowFloatingGamepadTextInput")]
	pub fn floating_gamepad_text_input(&self, mode: FloatingGamepadTextInputMode, avoid_position: [i32; 2], avoid_size: [u32; 2]) -> Result<(), UnspecifiedError> {
		success(unsafe { sys::SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput(*self.fip, mode.into(), avoid_position[0], avoid_position[1], avoid_size[0] as c_int, avoid_size[1] as c_int) })
	}

	/// > Activates the Big Picture text input dialog which only supports gamepad input.
	///
	/// Only one call is ever active at a time,
	/// all other calls will have to wait for the current call to finish.
	///
	/// # Panics
	/// If `config` contains a [`String`] with nul bytes.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
	#[doc(alias = "ShowGamepadTextInput")]
	pub async fn gamepad_text_input(&self, config: &GamepadTextInputConfig) -> Result<String, GamepadTextInputError> {
		//TODO: ShowGamepadTextInput may allow 0 for un-clamped inputs
		//TODO: ShowGamepadTextInput may not allow null ptrs for messages

		let description = OptionalCString::from(&config.description);
		let existing_text = OptionalCString::from(&config.existing_text);
		let (tx, rx) = oneshot::channel::<Option<String>>();

		//get an exclusive lock
		//this ensures we are the only ones to be running the code below
		let lock = self.gamepad_text_input.async_lock.lock().await;

		//we need to provide our sender - for receiving the value
		let mut sender_guard = self.gamepad_text_input.sender.lock().unwrap();
		*sender_guard = Some(tx);

		//explicit drop for significant drop
		drop(sender_guard);

		success(unsafe {
			sys::SteamAPI_ISteamUtils_ShowGamepadTextInput(
				*self.fip,
				config.mode(),
				config.line_mode(),
				description.as_nullable_ptr(),
				config.char_max(),
				existing_text.as_nullable_ptr(),
			)
		})?;

		//retrieve the callback's response
		let result = rx.await.map_err(|_| GamepadTextInputError::Unspecified)?.ok_or(GamepadTextInputError::Dismissed);

		//explicit drop for significant drop
		drop(lock);

		result
	}

	/// > Gets the image from an image handle.  
	/// <br>
	/// This call can be somewhat expensive as it converts from the compressed type (`JPG`, `PNG`, `TGA`) and provides no internal caching of returned buffer,
	/// thus it is highly recommended to only call this once per image handle and cache the result.
	/// This function is only used for Steam Avatars and Achievement images and those are not expected to change mid game.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetImageRGBA)
	#[cfg(feature = "image")]
	#[cfg_attr(doc, doc(cfg(feature = "image")))]
	#[doc(alias = "GetImageRGBA")]
	pub fn image(&self, image_handle: ImageHandle) -> Result<image::RgbaImage, UnspecifiedError> {
		let fip = *self.fip;
		let [width, height] = self.image_size(image_handle).ok_or(UnspecifiedError)?;
		let length = width * height * 4;

		//if image_size_uncached returns a dimension of 0
		//it could be a problem
		if length == 0 {
			return Err(UnspecifiedError);
		}

		let mut buffer: Vec<u8> = Vec::new();

		//reserve and with_capacity could over-allocate
		//since we expect to only ever write once
		//this is the best we can do to avoid that
		buffer.reserve_exact(length as usize);

		unsafe {
			if sys::SteamAPI_ISteamUtils_GetImageRGBA(fip, image_handle.handle, buffer.as_mut_ptr(), length as i32) {
				buffer.set_len(length as usize);
			} else {
				return Err(UnspecifiedError);
			}
		}

		Ok(image::RgbaImage::from_raw(width, height, buffer).unwrap())
	}

	/// > Gets the size of a Steam image handle.
	///
	/// Returns the image size as `[width, height]`.
	/// If you want the actual image, use [`image`] (requires the image feature.)
	///
	/// [`image`]: Self::image
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetImageSize)
	#[doc(alias = "GetImageSize")]
	pub fn image_size(&self, image_handle: ImageHandle) -> Option<[u32; 2]> {
		match image_handle.get_cached_size() {
			None => {
				let (success, size) = self.image_size_uncached(image_handle.handle);

				if success {
					Some(size)
				} else {
					None
				}
			}

			some => some,
		}
	}

	pub(crate) fn image_size_uncached(&self, raw_handle: i32) -> (bool, [u32; 2]) {
		let mut size = [0u32; 2];
		let success = unsafe { sys::SteamAPI_ISteamUtils_GetImageSize(*self.fip, raw_handle, &mut size[0], &mut size[1]) };

		(success, size)
	}

	/// > Returns the number of IPC calls made since the last time this function was called.  
	/// <br>
	/// Used for perf debugging so you can determine how many IPC (Inter-Process Communication) calls your game makes per frame
	/// Every IPC call is at minimum a thread context switch if not a process one so you want to rate control how often you do them.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetIPCCallCount)
	#[doc(alias = "GetIPCCallCount")]
	pub fn ipc_call_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamUtils_GetIPCCallCount(*self.fip) }
	}

	/// > Returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in.
	/// e.g "US" or "UK".  
	/// <br>
	/// This is looked up via an IP-to-location database.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetIPCountry)
	#[doc(alias = "GetIPCountry")]
	pub fn ip_country(&self) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamUtils_GetIPCountry(*self.fip)) }
	}

	/// > Checks if Steam & the Steam Overlay are running in Big Picture mode.  
	/// <br>
	/// Games must be launched through the Steam client to enable the Big Picture overlay.
	/// During development, a game can be added as a non-steam game to the developer's library to test this feature.  
	/// <br>
	/// Always returns `false` if your app is not the 'game' [application type].
	///
	/// [application type]: https://partner.steamgames.com/doc/store/application#types_of_applications
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamInBigPictureMode)
	#[doc(alias = "IsSteamInBigPictureMode")]
	pub fn is_steam_big_picture(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamInBigPictureMode(*self.fip) }
	}

	/// > Returns whether the current launcher is a Steam China launcher.
	/// You can cause the client to behave as the Steam China launcher by adding `-dev -steamchina` to the command line when running Steam.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamChinaLauncher)
	#[doc(alias = "IsSteamChinaLauncher")]
	pub fn is_steam_china_launcher(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamChinaLauncher(*self.fip) }
	}

	/// > Checks if Steam is running on a Steam Deck device.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamRunningOnSteamDeck)
	#[doc(alias = "IsSteamRunningOnSteamDeck")]
	pub fn is_steam_deck(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(*self.fip) }
	}

	/// > Checks if Steam is running in VR mode.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamRunningInVR)
	#[doc(alias = "IsSteamRunningInVR")]
	pub fn is_steam_vr(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamRunningInVR(*self.fip) }
	}

	/// > Checks if the HMD view is being streamed via Steam Remote Play.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsVRHeadsetStreamingEnabled)
	#[doc(alias = "IsVRHeadsetStreamingEnabled")]
	pub fn is_vr_headset_streaming_enabled(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsVRHeadsetStreamingEnabled(*self.fip) }
	}

	/// > Checks if the [Steam Overlay] is running & the user can access it.  
	/// <br>
	/// The overlay process could take a few seconds to start & hook the game process,
	/// so this function will initially return `false` while the overlay is loading.
	///
	/// [Steam Overlay]: https://partner.steamgames.com/doc/features/overlay
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsOverlayEnabled)
	#[doc(alias = "IsOverlayEnabled")]
	pub fn overlay_enabled(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsOverlayEnabled(*self.fip) }
	}

	/// > Checks if the Overlay needs a present. Only required if using event driven render updates.  
	/// <br>
	/// Typically this call is unneeded if your game has a constantly running frame loop that calls the D3D Present API,
	/// or OGL SwapBuffers API every frame as is the case in most games.
	/// However, if you have a game that only refreshes the screen on an event driven basis then that can break the overlay,
	/// as it uses your Present/SwapBuffers calls to drive it's internal frame loop and it may also need to `Present()` to the screen any time a notification happens or when the overlay is brought up over the game by a user.
	/// You can use this API to ask the overlay if it currently need a present in that case,
	/// and then you can check for this periodically (roughly 33hz is desirable) and make sure you refresh the screen with Present or SwapBuffers to allow the overlay to do its work.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#BOverlayNeedsPresent)
	#[doc(alias = "BOverlayNeedsPresent")]
	pub fn overlay_needs_present(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_BOverlayNeedsPresent(*self.fip) }
	}

	/// > Returns the Steam server time in Unix epoch format.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetServerRealTime)
	#[doc(alias = "GetServerRealTime")]
	pub fn server_real_time(&self) -> SystemTime {
		SystemTime::UNIX_EPOCH.add(Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetServerRealTime(*self.fip) } as u64))
	}

	/// > In game launchers that don't have controller support
	/// you can call this to have Steam Input translate the controller input into mouse/kb to navigate the launcher.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#SetGameLauncherMode)
	#[doc(alias = "SetGameLauncherMode")]
	pub fn set_game_launcher_mode(&self, enable: bool) {
		unsafe { sys::SteamAPI_ISteamUtils_SetGameLauncherMode(*self.fip, enable) };
	}

	/// > Sets the inset of the overlay notification from the corner specified by [`set_overlay_notification_position`].
	/// <br>
	/// A value of (0, 0) resets the position into the corner.
	/// <br>
	/// This position is per-game and is reset each launch.
	///
	/// [`set_overlay_notification_position`]: Self::set_overlay_notification_position
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#SetOverlayNotificationInset)
	#[doc(alias = "SetOverlayNotificationInset")]
	pub fn set_overlay_notification_inset(&self, horizontal: i32, vertical: i32) {
		unsafe { sys::SteamAPI_ISteamUtils_SetOverlayNotificationInset(*self.fip, horizontal.into(), vertical.into()) }
	}

	/// > Sets which corner the Steam overlay notification popup should display itself in.
	/// <br>
	/// You can also set the distance from the specified corner by using [`set_overlay_notification_inset`].
	/// <br>
	/// This position is per-game and is reset each launch.
	///
	/// [`set_overlay_notification_inset`]: Self::set_overlay_notification_inset
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#SetOverlayNotificationPosition)
	#[doc(alias = "SetOverlayNotificationPosition")]
	pub fn set_overlay_notification_position(&self, position: NotificationPosition) {
		unsafe { sys::SteamAPI_ISteamUtils_SetOverlayNotificationPosition(*self.fip, position.into()) }
	}

	/// > Asks Steam to create and render the OpenVR dashboard.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#StartVRDashboard)
	#[doc(alias = "StartVRDashboard")]
	pub fn start_vr_dashboard(&self) {
		if self.vr_dashboard.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) == Ok(false) {
			unsafe { sys::SteamAPI_ISteamUtils_StartVRDashboard(*self.fip) };
		}
	}

	/// > Returns the language the steam client is running in.  
	/// <br>
	/// You probably want [`AppsInterface::current_language`] instead, this should only be used in very special cases.  
	/// <br>
	/// For a full list of languages see [Supported Languages].
	///
	/// This exists as the Steam UI's language can be different from the app's selected language.
	/// You should always opt for [`AppsInterface::current_language`] when selecting your localizations,
	/// as the user may want to use a language different from what they use in Steam.
	///
	/// [Supported Languages]: https://partner.steamgames.com/doc/store/localization/languages
	/// [`AppsInterface::current_language`]: super::apps::AppsInterface::current_language
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetSteamUILanguage)
	#[doc(alias = "GetSteamUILanguage")]
	pub fn steam_ui_language(&self) -> String {
		unsafe { expect_string(sys::SteamAPI_ISteamUtils_GetSteamUILanguage(*self.fip)) }
	}
}

impl Interface for UtilsInterface {
	type CInterface = sys::ISteamUtils;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, _steam: SteamChild) -> Self {
		Self {
			fip,
			gamepad_text_input: Arc::new(GamepadTextInputLock::new()),
			vr_dashboard: AtomicBool::new(false),
		}
	}

	fn initialize(steam: &SteamInterface) {
		let mut call_manager = steam.call_manager_lock();

		unsafe {
			call_manager.register_raw::<GamepadTextInputDismissed>();
		}
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		sys::SteamAPI_SteamUtils_v010()
	}
}

callback! {
	/// ```
	/// fn listener() {}
	/// ```
	///
	/// > Sent after the device returns from sleep/suspend mode.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#AppResumingFromSuspend_t)
	pub struct AppResumingFromSuspend;
}

/// > Controls the mode for the floating keyboard.
///
/// Used by [`UtilsInterface::floating_gamepad_text_input`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#EFloatingGamepadTextInputMode)
#[doc(alias = "EFloatingGamepadTextInputMode")]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatingGamepadTextInputMode {
	#[default]
	SingleLine,

	MultipleLines,

	Email,

	Numeric,
}

impl From<FloatingGamepadTextInputMode> for sys::EFloatingGamepadTextInputMode {
	fn from(value: FloatingGamepadTextInputMode) -> Self {
		use FloatingGamepadTextInputMode::*;

		match value {
			SingleLine => Self::k_EFloatingGamepadTextInputModeModeSingleLine,
			MultipleLines => Self::k_EFloatingGamepadTextInputModeModeMultipleLines,
			Email => Self::k_EFloatingGamepadTextInputModeModeEmail,
			Numeric => Self::k_EFloatingGamepadTextInputModeModeNumeric,
		}
	}
}

callback! {
	/// ```
	/// fn listener() {}
	/// ```
	///
	/// > Called when the floating keyboard invoked from [`UtilsInterface::floating_gamepad_text_input`] has been closed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#FloatingGamepadTextInputDismissed_t)
	pub struct FloatingGamepadTextInputDismissed;
}

/// Configuration for calling [`UtilsInterface::gamepad_text_input`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
#[doc(alias = "ShowGamepadTextInput")]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct GamepadTextInputConfig {
	/// > The maximum number of characters that the user can input.
	///
	/// Clamped to a range of `1 ..= 536_870_911`.
	///
	/// [Steamworks Docs]()
	pub char_max: Option<NonZeroU32>,

	/// > Sets the description that should inform the user what the input dialog is for.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
	pub description: Option<String>,

	/// > Sets the preexisting text which the user can edit.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
	pub existing_text: Option<String>,

	/// Set to `true` to allow multiple lines in the input text.
	pub multi_line: bool,

	/// Hides the text during input.  
	/// Use for passwords or other sensitive text.
	pub sensitive: bool,
}

impl GamepadTextInputConfig {
	fn char_max(&self) -> u32 {
		const MAX_CHARS: u32 = 536_870_911;

		match self.char_max {
			None => MAX_CHARS,
			Some(nz) => nz.get().min(MAX_CHARS),
		}
	}

	fn line_mode(&self) -> sys::EGamepadTextInputLineMode {
		match self.multi_line {
			true => sys::EGamepadTextInputLineMode::k_EGamepadTextInputLineModeMultipleLines,
			false => sys::EGamepadTextInputLineMode::k_EGamepadTextInputLineModeSingleLine,
		}
	}

	fn mode(&self) -> sys::EGamepadTextInputMode {
		match self.sensitive {
			true => sys::EGamepadTextInputMode::k_EGamepadTextInputModePassword,
			false => sys::EGamepadTextInputMode::k_EGamepadTextInputModeNormal,
		}
	}
}

callback! {
	struct GamepadTextInputDismissed {
		steam: SteamChild,
	}

	new steam;

	data {
		let utils = &self.steam.get().interfaces.utils;
		let mut guard = utils.gamepad_text_input.sender.lock().unwrap();
		let sender = guard.take().expect("invalid call order");

		if !data.m_bSubmitted {
			sender.send(None).unwrap();

			return;
		}

		let mut buffer = Vec::<u8>::new();
		let c_len = data.m_unSubmittedText; //TODO: verify that this length does not include the nul terminator
		let len = c_len as usize;

		//we write once - immediately convert
		buffer.reserve_exact(len + 1);

		//TODO: verify if the C fn below writes the nul term
		buffer.set_len(len);
		buffer.push(0);

		assert!(
			sys::SteamAPI_ISteamUtils_GetEnteredGamepadTextInput(*utils.fip, buffer.as_mut_ptr() as *mut c_char, c_len),
			"GamepadTextInputDismissed called but GetEnteredGamepadTextInput failed (length {c_len})"
		);

		sender.send(Some(CStr::from_bytes_until_nul(&buffer).unwrap().to_string_lossy().to_string())).unwrap();
		drop(guard); //explicit drop for significant drop
	}
}

/// The failure states of [`UtilsInterface::gamepad_text_input`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum GamepadTextInputError {
	/// The text input window shown to the user was dismissed,
	/// thus no text has been received.
	#[error("the GamepadTextInput was dismissed")]
	Dismissed,

	/// Failed, with no error message from the Steam API.
	#[error("failed, no error message from the Steam API is available")]
	Unspecified,

	/// The [`CallManager`] was shutdown before a response was received.
	///
	/// [`CallManager`]: crate::call::CallManager
	#[error("the CallManager is being shutdown")]
	Shutdown,
}

impl From<UnspecifiedError> for GamepadTextInputError {
	fn from(_: UnspecifiedError) -> Self {
		GamepadTextInputError::Unspecified
	}
}

/// Maintains locks ensuring responses arrive to the appropriate callers.
#[derive(Debug)]
struct GamepadTextInputLock {
	async_lock: AsyncMutex<()>,

	/// `Some(Sender<_>)` if a receiver is awaiting a response.
	/// `None` otherwise.
	sender: StdMutex<Option<oneshot::Sender<Option<String>>>>,
}

impl GamepadTextInputLock {
	fn new() -> Self {
		Self {
			async_lock: AsyncMutex::new(()),
			sender: StdMutex::new(None),
		}
	}
}

callback! {
	/// ```
	/// fn listener(iso_code: Option<String>) { }
	/// ```
	///
	/// > Called when the country of the user changed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IPCountry_t)
	pub struct IpCountry {
		steam: SteamChild,
	}

	sys IPCountry;
	new steam;
}

callback! {
	/// ```
	/// # use std::time::Duration;
	/// fn listener(estimated_time_left: Duration) { }
	/// ```
	///
	/// > Called when running on a laptop and less than 10 minutes of battery is left,
	/// and then fires then every minute afterwards.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#LowBatteryPower_t)
	pub struct LowBatteryPower;

	data -> Duration { Duration::from_secs(data.m_nMinutesBatteryLeft as u64 * 60) }
}

/// > Possible positions to have the overlay show notifications in.
/// Used with [`UtilsInterface::set_overlay_notification_position`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#ENotificationPosition)
#[doc(alias = "ENotificationPosition")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NotificationPosition {
	TopLeft,
	TopRight,
	BottomLeft,
	BottomRight,
}

impl From<NotificationPosition> for sys::ENotificationPosition {
	fn from(value: NotificationPosition) -> Self {
		use sys::ENotificationPosition::*;
		use NotificationPosition::*;

		match value {
			TopLeft => k_EPositionTopLeft,
			TopRight => k_EPositionTopRight,
			BottomLeft => k_EPositionBottomLeft,
			BottomRight => k_EPositionBottomRight,
		}
	}
}

callback! {
	/// > Called when Steam wants to shutdown.
	///
	/// You should try to perform a graceful shutdown here, such as saving the game.
	///
	/// ## Don't take too long
	/// You have roughly 10 seconds before Steam gives up and tries to kill your app instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#SteamShutdown_t)
	pub struct SteamShutdown;
}

/// > Parameter to [`UtilsInterface::filter_text`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#ETextFilteringContext)
#[doc(alias = "ETextFilteringContext")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TextFilteringContext {
	/// > Unknown context.
	Unknown,

	/// > Game content, only legally required filtering is performed.
	GameContent,

	/// > Chat from another player.
	Chat,

	/// > Character or item name.
	Name,
}

impl From<TextFilteringContext> for sys::ETextFilteringContext {
	fn from(value: TextFilteringContext) -> Self {
		use TextFilteringContext::*;

		match value {
			Unknown => Self::k_ETextFilteringContextUnknown,
			GameContent => Self::k_ETextFilteringContextGameContent,
			Chat => Self::k_ETextFilteringContextChat,
			Name => Self::k_ETextFilteringContextName,
		}
	}
}
