//! See [`UtilsInterface`].

use crate::dt::{AppId, ImageHandle, SteamId};
use crate::error::UnspecifiedError;
use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild};
use crate::sys;
use crate::util::{expect_string, lossy_cstring, some_string, success, OptionalCString};
use cfg_if::cfg_if;
use std::ffi::{c_char, CStr};
use std::num::NonZeroU32;
use std::ops::Add;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
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

	/// `true` if [`start_vr_dashboard`] has been called.
	///
	/// [`start_vr_dashboard`]: Self::start_vr_dashboard
	vr_dashboard: AtomicBool,
}

impl UtilsInterface {
	/// > Returns the number of seconds since the application was active.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetSecondsSinceAppActive)
	pub fn app_active_duration(&self) -> Duration {
		Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetSecondsSinceAppActive(*self.fip) } as u64)
	}

	/// > Gets the App ID of the current process.
	///
	/// Use [`SteamInterface::app_id`] instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetAppID)
	///
	/// [`SteamInterface::app_id`]: super::SteamInterface::app_id
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
	pub fn computer_active_duration(&self) -> Duration {
		Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetSecondsSinceComputerActive(*self.fip) } as u64)
	}

	/// > Gets the current amount of battery power on the computer.
	///
	/// Power returned will be in the range of `0 ..= 100`.
	/// Returns `None` when the user is on AC power.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetCurrentBatteryPower)
	pub fn current_battery_power(&self) -> Option<u8> {
		let power = unsafe { sys::SteamAPI_ISteamUtils_GetCurrentBatteryPower(*self.fip) };

		if power == 255 {
			None
		} else {
			Some(power)
		}
	}

	/// > Censors `str` based on the user's preferences and local laws.
	/// Legally required filtering is always applied.
	/// Additional filtering may occur, based on the context and user settings.
	///
	/// [Steamworks Docs]()
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

	/// > Activates the Big Picture text input dialog which only supports gamepad input.
	///
	/// # Panics
	/// If `config` contains a [`String`] with nul bytes.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
	pub async fn gamepad_text_input(&self, config: &GamepadTextInputConfig) -> Result<(), UnspecifiedError> {
		//TODO: ShowGamepadTextInput may allow 0 for un-clamped inputs
		//TODO: ShowGamepadTextInput may not allow null ptrs for messages
		//TODO: make this wait for a callback

		let description = OptionalCString::from(&config.description);
		let existing_text = OptionalCString::from(&config.existing_text);

		if !unsafe {
			sys::SteamAPI_ISteamUtils_ShowGamepadTextInput(
				*self.fip,
				config.mode(),
				config.line_mode(),
				description.as_nullable_ptr(),
				config.char_max(),
				existing_text.as_nullable_ptr(),
			)
		} {
			return Err(UnspecifiedError);
		}

		todo!()
	}

	/// > Gets the image from an image handle.  
	/// <br>
	/// This call can be somewhat expensive as it converts from the compressed type (`JPG`, `PNG`, `TGA`) and provides no internal caching of returned buffer,
	/// thus it is highly recommended to only call this once per image handle and cache the result.
	/// This function is only used for Steam Avatars and Achievement images and those are not expected to change mid game.
	///
	/// Cache the result
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetImageRGBA)
	#[cfg(feature = "image")]
	#[cfg_attr(doc, doc(cfg(feature = "image")))]
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
	pub fn ipc_call_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamUtils_GetIPCCallCount(*self.fip) }
	}

	/// > Returns the 2 digit ISO 3166-1-alpha-2 format country code which client is running in.
	/// e.g "US" or "UK".  
	/// <br>
	/// This is looked up via an IP-to-location database.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetIPCountry)
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
	pub fn is_steam_big_picture(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamInBigPictureMode(*self.fip) }
	}

	/// > Returns whether the current launcher is a Steam China launcher.
	/// You can cause the client to behave as the Steam China launcher by adding `-dev -steamchina` to the command line when running Steam.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamChinaLauncher)
	pub fn is_steam_china_launcher(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamChinaLauncher(*self.fip) }
	}

	/// > Checks if Steam is running on a Steam Deck device.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamRunningOnSteamDeck)
	pub fn is_steam_deck(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(*self.fip) }
	}

	/// > Checks if Steam is running in VR mode.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsSteamRunningInVR)
	pub fn is_steam_vr(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_IsSteamRunningInVR(*self.fip) }
	}

	/// > Checks if the HMD view is being streamed via Steam Remote Play.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#IsVRHeadsetStreamingEnabled)
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
	pub fn overlay_needs_present(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamUtils_BOverlayNeedsPresent(*self.fip) }
	}

	/// > Returns the Steam server time in Unix epoch format.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#GetServerRealTime)
	pub fn server_real_time(&self) -> SystemTime {
		SystemTime::UNIX_EPOCH.add(Duration::from_secs(unsafe { sys::SteamAPI_ISteamUtils_GetServerRealTime(*self.fip) } as u64))
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
	pub fn set_overlay_notification_position(&self, position: NotificationPosition) {
		unsafe { sys::SteamAPI_ISteamUtils_SetOverlayNotificationPosition(*self.fip, position.into()) }
	}

	/// > Asks Steam to create and render the OpenVR dashboard.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#StartVRDashboard)
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
	pub fn steam_ui_language(&self) -> String {
		unsafe { expect_string(sys::SteamAPI_ISteamUtils_GetSteamUILanguage(*self.fip)) }
	}
}

impl Interface for UtilsInterface {
	type CInterface = sys::ISteamUtils;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, _steam: SteamChild) -> Self {
		Self {
			fip,
			vr_dashboard: AtomicBool::new(false),
		}
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		sys::SteamAPI_SteamUtils_v010()
	}
}

/// Configuration for calling [`UtilsInterface::gamepad_text_input`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamUtils#ShowGamepadTextInput)
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

struct GamepadTextInputDismissed;

/// > Possible positions to have the overlay show notifications in.
/// Used with [`UtilsInterface::set_overlay_notification_position`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#ENotificationPosition)
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

/// > Parameter to [`UtilsInterface::filter_text`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#ETextFilteringContext)
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
