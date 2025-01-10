#![allow(private_interfaces)]

use crate::call::{CallbackListenerIter, SteamworksCallback};
use crate::dt::{AppId, SteamId};
use crate::{FixedInterface, Private};
use bitflags::bitflags;
use rgpr_steamworks_sys as sys;
use std::collections::HashSet;
use std::ffi::{c_char, c_void, CString};

/// For use with [activate_game_overlay](FriendsInterface::activate_game_overlay),
/// represents the available parameters for all possible options of the following:
/// - [ActivateGameOverlay](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
/// - [ActivateGameOverlayToUser](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
/// - [ActivateGameOverlayInviteDialog](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayInviteDialog)
/// - [ActivateGameOverlayToStore](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToStore)
/// - [ActivateGameOverlayToWebPage](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToWebPage)
#[derive(Clone, Debug)]
pub enum ActivateGameOverlay {
	/// Internally `"Friends"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
	Friends,

	/// Internally `"Community"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
	Community,

	/// Internally `"Players"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
	Players,

	/// Internally `"Settings"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
	Settings,

	/// Internally `"OfficialGameGroup"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
	OfficialGameGroup,

	/// Opens the overlay web browser to the specified user's stats.  
	///
	/// For `None`: Internally `"Stats"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)  
	/// For `Some`: Internally `"stats"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	Stats(Option<SteamId>),

	/// Opens the overlay web browser to the specified user's achievements.  
	///
	/// For `None`: Internally `"Achievements"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)  
	/// For `Some`: Internally `"achievements"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	Achievements(Option<SteamId>),

	/// Internally `"chatroomgroup/nnnn"` where `nnnn` is the a Steam ID for a chat group.
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	ChatRoomGroup(SteamId),

	/// Opens the overlay web browser to the specified user or groups profile.
	/// Internally `"steamid"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	Profile(SteamId),

	/// Opens a chat window to the specified user, or joins the group chat.
	/// Internally `"chat"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	Chat(SteamId),

	/// Opens a window to a Steam Trading session that was started with the ISteamEconomy/StartTrade Web API.
	/// Internally `"jointrade"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	JoinTrade(SteamId),

	/// Opens the overlay in minimal mode prompting the user to add the target user as a friend.
	/// Internally `"friendadd"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	FriendAdd(SteamId),

	/// Opens the overlay in minimal mode prompting the user to remove the target friend.
	/// Internally `"friendremove"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	FriendRemove(SteamId),

	/// Opens the overlay in minimal mode prompting the user to accept an incoming friend invite.
	/// Internally `"friendrequestaccept"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	FriendRequestAccept(SteamId),

	/// Opens the overlay in minimal mode prompting the user to ignore an incoming friend invite.
	/// Internally `"friendrequestignore"` [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
	FriendRequestIgnore(SteamId),

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayInviteDialog)
	InviteDialog(SteamId),

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToStore)
	Store {
		/// The App ID to use for the store page.
		/// Use 0 if you want to open the front page of the Steam store.
		app_id: AppId,

		/// Set to true if you want the app to be added to the cart
		show_in_cart: bool,
	},

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToWebPage)
	WebPage {
		/// The webpage to open.
		/// Must start with protocol e.g. `https://`
		url: String,

		/// Set to `false`:
		/// > Browser will open next to all other windows that the user has open in the overlay.
		/// The window will remain open, even if the user closes then re-opens the overlay.
		///
		/// Set to `true`:
		/// > Browser will be opened in a special overlay configuration which hides all other windows that the user has open in the overlay.
		/// When the user closes the overlay, the browser window will also close.
		/// When the user closes the browser window, the overlay will automatically close.
		///
		/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EActivateGameOverlayToWebPageMode)
		modal: bool,
	},
}

impl ActivateGameOverlay {
	/// Returns the [SteamId] inside the enum if the variant contains one.
	pub const fn steam_id(&self) -> Option<SteamId> {
		match self {
			Self::ChatRoomGroup(id)
			| Self::Profile(id)
			| Self::Chat(id)
			| Self::JoinTrade(id)
			| Self::FriendAdd(id)
			| Self::FriendRemove(id)
			| Self::FriendRequestAccept(id)
			| Self::FriendRequestIgnore(id)
			| Self::InviteDialog(id) => Some(*id),

			//already is an `Option<SteamId>`, just copy it
			Self::Stats(opt) | Self::Achievements(opt) => *opt,

			//no SteamId for parameter
			Self::OfficialGameGroup | Self::Friends | Self::Community | Self::Players | Self::Settings => None,
			Self::Store { .. } | Self::WebPage { .. } => None,
		}
	}
}

/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends)
#[derive(Debug)]
pub struct FriendsInterface {
	/// Contains the Steam IDs of users who are using steam chat in game,
	/// and should have their voice comms muted for those who would hear double otherwise.
	in_game_speaking: HashSet<SteamId>,

	interface: FixedInterface<sys::ISteamFriends>,
}

impl FriendsInterface {
	/// # Safety
	/// Steamworks' globals must be initialized.
	/// Steamworks must only shut down if this is dropped, or is being dropped.
	pub(crate) unsafe fn new() -> Self {
		Self {
			in_game_speaking: HashSet::new(),
			interface: FixedInterface::new(sys::SteamAPI_SteamFriends_v017()),
		}
	}

	pub fn activate_game_overlay(&self, instruction: ActivateGameOverlay) {
		unsafe {
			use ActivateGameOverlay::*;

			let interface = self.interface();

			match instruction {
				//ActivateGameOverlay
				Friends => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Friends".as_ptr()),
				Community => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Community".as_ptr()),
				Players => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Players".as_ptr()),
				Settings => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Settings".as_ptr()),
				OfficialGameGroup => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"OfficialGameGroup".as_ptr()),
				Stats(None) => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Stats".as_ptr()),
				Achievements(None) => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c"Achievements".as_ptr()),

				//still ActivateGameOverlay, just a little special
				ChatRoomGroup(steam_id) => {
					let string = format!("chatroomgroup/{}", steam_id.0);
					let c_string = CString::new(string).unwrap();

					sys::SteamAPI_ISteamFriends_ActivateGameOverlay(interface, c_string.as_ptr())
				}

				//ActivateGameOverlayToUser
				Profile(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"steamid".as_ptr(), steam_id.0),
				Chat(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"chat".as_ptr(), steam_id.0),
				JoinTrade(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"jointrade".as_ptr(), steam_id.0),
				Stats(Some(steam_id)) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"stats".as_ptr(), steam_id.0),
				Achievements(Some(steam_id)) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"achievements".as_ptr(), steam_id.0),
				FriendAdd(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"friendadd".as_ptr(), steam_id.0),
				FriendRemove(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"friendremove".as_ptr(), steam_id.0),
				FriendRequestAccept(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"friendrequestaccept".as_ptr(), steam_id.0),
				FriendRequestIgnore(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(interface, c"friendrequestignore".as_ptr(), steam_id.0),

				//ActivateGameOverlayInviteDialog
				InviteDialog(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialog(interface, steam_id.0),

				//ActivateGameOverlayToStore
				Store { app_id, show_in_cart } => {
					sys::SteamAPI_ISteamFriends_ActivateGameOverlayToStore(
						interface,
						app_id.0,
						if show_in_cart {
							sys::EOverlayToStoreFlag::k_EOverlayToStoreFlag_AddToCartAndShow
						} else {
							sys::EOverlayToStoreFlag::k_EOverlayToStoreFlag_None
						},
					);
				}

				//ActivateGameOverlayToWebPage
				WebPage { url, modal } => {
					let c_str = CString::new(url).unwrap();

					sys::SteamAPI_ISteamFriends_ActivateGameOverlayToWebPage(
						interface,
						c_str.as_ptr(),
						if modal {
							sys::EActivateGameOverlayToWebPageMode::k_EActivateGameOverlayToWebPageMode_Modal
						} else {
							sys::EActivateGameOverlayToWebPageMode::k_EActivateGameOverlayToWebPageMode_Default
						},
					);
				}
			}
		}
	}

	/// Clears the "In-game voice speaking" status from all users who were marked as such.
	/// Same as calling [`set_in_game_speaking`](Self::set_in_game_speaking) for each speaking user.
	pub fn clear_in_game_speaking(&mut self) {
		let interface = self.interface();

		for steam_id in self.in_game_speaking.drain() {
			unsafe {
				sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(interface, steam_id.0, false);
			}
		}
	}

	pub fn clear_rich_presence(&self) {
		unsafe { sys::SteamAPI_ISteamFriends_ClearRichPresence(self.interface()) }
	}

	#[doc(hidden)]
	fn interface(&self) -> *mut sys::ISteamFriends {
		self.interface.ptr()
	}

	/// Same as [request_user_info](Self::request_user_info) but also downloads the Avatar.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation)
	pub fn request_user_avatar(&self, steam_id: SteamId) {
		unsafe {
			sys::SteamAPI_ISteamFriends_RequestUserInformation(self.interface(), steam_id.0, false);
		}
	}

	/// > Requests the persona name of a specified user.
	///
	/// Force a [PersonaStateChange] callback to be run.
	/// Useful if you need to retrieve a username but the one you currently have is out-of-date.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation)
	pub fn request_user_info(&self, steam_id: SteamId) {
		unsafe {
			sys::SteamAPI_ISteamFriends_RequestUserInformation(self.interface(), steam_id.0, true);
		}
	}

	/// Currently not on the Steamworks docs.
	/// From `isteamfriends.h` `SetInGameVoiceSpeaking`:
	/// > User is in a game pressing the talk button (will suppress the microphone for all voice comms from the Steam friends UI)
	pub fn set_in_game_speaking(&mut self, steam_id: SteamId, speaking: bool) {
		if speaking {
			self.in_game_speaking.insert(steam_id);
		} else {
			self.in_game_speaking.remove(&steam_id);
		}

		unsafe {
			sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(self.interface(), steam_id.0, speaking);
		}
	}

	pub fn set_played_with(&self, steam_id: SteamId) {
		unsafe { sys::SteamAPI_ISteamFriends_SetPlayedWith(self.interface(), steam_id.0) }
	}

	/// Settings value to `None` will remove the key.
	/// Rich presence lets you share a
	pub fn set_rich_presence(&self, key: impl AsRef<str>, value: Option<impl AsRef<str>>) -> bool {
		let key = CString::new(key.as_ref()).unwrap();
		let interface = self.interface();

		if let Some(value_ref) = value {
			let value = CString::new(value_ref.as_ref()).unwrap();

			unsafe { sys::SteamAPI_ISteamFriends_SetRichPresence(interface, key.as_ptr(), value.as_ptr()) }
		} else {
			unsafe { sys::SteamAPI_ISteamFriends_SetRichPresence(interface, key.as_ptr(), std::ptr::null::<c_char>()) }
		}
	}
}

#[derive(Clone, Copy, Debug)]
pub struct GameLobbyJoinRequested;

impl SteamworksCallback for GameLobbyJoinRequested {
	const C_ENUM: i32 = sys::GameLobbyJoinRequested_t_k_iCallback as i32;
	const REQUIRES_LISTENERS: bool = true;
	type ListenFn = Box<dyn FnMut(SteamId, SteamId) + Send>;

	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = &*(void_ptr as *const sys::GameLobbyJoinRequested_t);
		let lobby_steam_id = SteamId::from(data.m_steamIDLobby);
		let friend_steam_id = SteamId::from(data.m_steamIDFriend);

		for (_, listener_fn) in listeners {
			listener_fn(lobby_steam_id, friend_steam_id)
		}
	}
}

/// Callback.
///
/// > Posted when the [Steam Overlay] activates or deactivates.
/// The game can use this to pause or resume single player games.
///
/// Listener signature:
/// ```
/// fn listener(active: bool, user_initiated: bool)
/// # {}
/// ```
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GameOverlayActivated_t)
/// [Steam Overlay]: https://partner.steamgames.com/doc/features/overlay
#[derive(Debug)]
pub struct GameOverlayActivated;

impl SteamworksCallback for GameOverlayActivated {
	const C_ENUM: i32 = sys::GameOverlayActivated_t_k_iCallback as i32;
	const REQUIRES_LISTENERS: bool = true;
	type ListenFn = Box<dyn FnMut(bool, bool) + Send>;

	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = &*(void_ptr as *mut sys::GameOverlayActivated_t);
		let active = data.m_bActive != 0;
		let user_initiated = data.m_bUserInitiated;

		for (_, listener_fn) in listeners {
			listener_fn(active, user_initiated);
		}
	}
}

bitflags! {
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EPersonaChange).
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct PersonaChange: i32 {
		const NAME = sys::EPersonaChange::k_EPersonaChangeName as i32;
		const STATUS = sys::EPersonaChange::k_EPersonaChangeStatus as i32;
		const COME_ONLINE = sys::EPersonaChange::k_EPersonaChangeComeOnline as i32;
		const GONE_OFFLINE = sys::EPersonaChange::k_EPersonaChangeGoneOffline as i32;
		const GAME_PLAYED = sys::EPersonaChange::k_EPersonaChangeGamePlayed as i32;
		const GAME_SERVER = sys::EPersonaChange::k_EPersonaChangeGameServer as i32;
		const AVATAR = sys::EPersonaChange::k_EPersonaChangeAvatar as i32;
		const JOINED_SOURCE = sys::EPersonaChange::k_EPersonaChangeJoinedSource as i32;
		const LEFT_SOURCE = sys::EPersonaChange::k_EPersonaChangeLeftSource as i32;
		const RELATIONSHIP_CHANGED = sys::EPersonaChange::k_EPersonaChangeRelationshipChanged as i32;
		const NAME_FIRST_SET = sys::EPersonaChange::k_EPersonaChangeNameFirstSet as i32;
		const FACEBOOK_INFO = sys::EPersonaChange::k_EPersonaChangeBroadcast as i32;
		const NICKNAME = sys::EPersonaChange::k_EPersonaChangeNickname as i32;
		const STEAM_LEVEL = sys::EPersonaChange::k_EPersonaChangeSteamLevel as i32;
		const RICH_PRESENCE = sys::EPersonaChange::k_EPersonaChangeRichPresence as i32;
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PersonaState {
	/// Friend is not currently logged on.
	Offline,

	/// Friend is logged on.
	Online,

	/// User is on, but busy.
	Busy,

	/// Auto-away feature.
	Away,

	/// Auto-away for a long time.
	Snooze,

	/// Online, trading.
	LookingToTrade,

	/// Online, wanting to play.
	LookingToPlay,

	/// Online, but appears offline to friends.  This status is never published to clients.
	Invisible,
}

impl From<sys::EPersonaState> for PersonaState {
	fn from(value: sys::EPersonaState) -> Self {
		use sys::EPersonaState::*;

		match value {
			k_EPersonaStateOffline => Self::Offline,
			k_EPersonaStateOnline => Self::Online,
			k_EPersonaStateBusy => Self::Busy,
			k_EPersonaStateAway => Self::Away,
			k_EPersonaStateSnooze => Self::Snooze,
			k_EPersonaStateLookingToTrade => Self::LookingToTrade,
			k_EPersonaStateLookingToPlay => Self::LookingToPlay,
			k_EPersonaStateInvisible => Self::Invisible,
			k_EPersonaStateMax | _ => unreachable!(),
		}
	}
}

/// Callback.
///
/// > Called whenever a friends' status changes.
///
/// Listener signature:
/// ```
/// # use rgpr_steamworks::{ dt::SteamId, steam_api::friends::PersonaChange };
/// fn listener(steam_id: SteamId, persona_change: PersonaChange)
/// # {}
/// ```
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#PersonaStateChange_t)
#[derive(Debug)]
pub struct PersonaStateChange;

impl SteamworksCallback for PersonaStateChange {
	const C_ENUM: i32 = sys::PersonaStateChange_t__bindgen_ty_1::k_iCallback as i32;
	const REQUIRES_LISTENERS: bool = true;
	type ListenFn = Box<dyn FnMut(SteamId, PersonaChange) + Send>;

	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = &*(void_ptr as *mut sys::PersonaStateChange_t);
		let steam_id = SteamId(data.m_ulSteamID);
		let persona_change = PersonaChange::from_bits_retain(data.m_nChangeFlags);

		for (_, fn_mut) in listeners {
			fn_mut(steam_id, persona_change);
		}
	}
}
