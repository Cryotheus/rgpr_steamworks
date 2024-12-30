use crate::call::{CallbackListenerIter, SteamworksCallback, SteamworksDispatch};
use crate::dt::{AppId, SteamId};
use crate::error::{ChatRoomEnterError, GeneralError};
use crate::{FixedInterface, Private, Steamworks};
use bitflags::bitflags;
use rgpr_steamworks_sys as sys;
use std::ffi::c_void;
use std::net::SocketAddrV4;

#[derive(Clone, Copy, Debug)]
pub struct CreateLobby {
	pub lobby_type: LobbyType,

	/// The maximum number of players that can join this lobby.
	/// Must not exceed 250.
	pub maximum_members: u8,
}

impl SteamworksDispatch for CreateLobby {
	#![allow(private_interfaces)]
	type CType = sys::LobbyCreated_t;
	type Output = Result<SteamId, GeneralError>;

	fn post(self, call_result: Box<Self::CType>, _private: Private) -> Self::Output {
		if let Some(error) = GeneralError::new(call_result.m_eResult) {
			Err(error)
		} else {
			Ok(SteamId(call_result.m_ulSteamIDLobby))
		}
	}

	unsafe fn dispatch(&mut self, _private: Private) -> sys::SteamAPICall_t {
		sys::SteamAPI_ISteamMatchmaking_CreateLobby(sys::SteamAPI_SteamMatchmaking_v009(), self.lobby_type.into(), self.maximum_members as i32)
	}
}

#[derive(Clone, Debug)]
pub struct GameServerRecord {
	/// The App ID of the game the game server is hosting for.
	pub app_id: Option<AppId>,

	/// The IPv4 address and port to connect to the game server.
	pub address: SocketAddrV4,

	/// The port to use for finding the server.
	pub query_port: Option<u16>,
}

impl GameServerRecord {
	/// # Panics
	/// If Steamworks is not attached.
	/// Returns integers for (AppID, Host-order IPv4, Port, Query port)
	fn params(self) -> (u32, u32, u16, u16) {
		let port = self.address.port();

		(
			self.app_id.unwrap_or_else(|| Steamworks::get().unwrap().app_id()).0,
			u32::from_be_bytes(self.address.ip().octets()),
			port,
			self.query_port.unwrap_or(port),
		)
	}
}

/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking)
pub struct MatchmakingInterface {
	interface: FixedInterface<sys::ISteamMatchmaking>,
}

impl MatchmakingInterface {
	pub(crate) unsafe fn new() -> Self {
		Self {
			interface: FixedInterface::new(sys::SteamAPI_SteamMatchmaking_v009()),
		}
	}

	#[doc(hidden)]
	fn time_stamp() -> u32 {
		//just like steam likes it
		std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32
	}

	pub fn add_historic_game_server(&self, game_server: GameServerRecord) -> i32 {
		unsafe {
			let params = game_server.params();

			sys::SteamAPI_ISteamMatchmaking_AddFavoriteGame(self.interface(), params.0, params.1, params.2, params.3, sys::k_unFavoriteFlagHistory, Self::time_stamp())
		}
	}

	pub fn favorite_game_server(&self, game_server: GameServerRecord) -> i32 {
		unsafe {
			let params = game_server.params();

			sys::SteamAPI_ISteamMatchmaking_AddFavoriteGame(self.interface(), params.0, params.1, params.2, params.3, sys::k_unFavoriteFlagFavorite, Self::time_stamp())
		}
	}

	#[doc(hidden)]
	fn interface(&self) -> *mut sys::ISteamMatchmaking {
		self.interface.ptr()
	}

	pub fn unfavorite_game_server(&self, game_server: GameServerRecord) -> bool {
		unsafe {
			let params = game_server.params();

			sys::SteamAPI_ISteamMatchmaking_RemoveFavoriteGame(self.interface(), params.0, params.1, params.2, params.3, sys::k_unFavoriteFlagFavorite)
		}
	}
}

bitflags! {
	/// Flags describing how a users lobby state has changed.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#EChatMemberStateChange)
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct ChatMemberStateChange: i32 {
		const ENTERED = sys::EChatMemberStateChange::k_EChatMemberStateChangeEntered as i32;
		const LEFT = sys::EChatMemberStateChange::k_EChatMemberStateChangeLeft as i32;
		const DISCONNECTED = sys::EChatMemberStateChange::k_EChatMemberStateChangeDisconnected as i32;
		const KICKED = sys::EChatMemberStateChange::k_EChatMemberStateChangeKicked as i32;
		const BANNED = sys::EChatMemberStateChange::k_EChatMemberStateChangeBanned as i32;
	}
}

/// Lobby search filter options.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyComparison)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LobbyComparison {
	NotEqual,
	Less,
	LessEqual,
	Equal,
	GreaterEqual,
	Greater,
}

impl From<sys::ELobbyComparison> for LobbyComparison {
	fn from(value: sys::ELobbyComparison) -> Self {
		use sys::ELobbyComparison::*;

		match value {
			//C-enum ordering preserved
			k_ELobbyComparisonEqualToOrLessThan => Self::LessEqual,
			k_ELobbyComparisonLessThan => Self::Less,
			k_ELobbyComparisonEqual => Self::Equal,
			k_ELobbyComparisonGreaterThan => Self::Greater,
			k_ELobbyComparisonEqualToOrGreaterThan => Self::GreaterEqual,
			k_ELobbyComparisonNotEqual => Self::NotEqual,
			_ => unreachable!(),
		}
	}
}

/// Lobby search distance filters when requesting the lobby list. Lobby results are sorted from closest to farthest.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyDistanceFilter)
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum LobbyDistanceFilter {
	/// > Only lobbies in the same immediate region will be returned.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyDistanceFilter)
	Close,

	/// > Only lobbies in the same region or nearby regions will be returned.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyDistanceFilter)
	#[default]
	Medium,

	/// > For games that don't have many latency requirements, will return lobbies about half-way around the globe.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyDistanceFilter)
	Far,

	/// > No filtering, will match lobbies as far as India to NY (not recommended, expect multiple seconds of latency between the clients).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyDistanceFilter)
	WorldWide,
}

impl From<sys::ELobbyDistanceFilter> for LobbyDistanceFilter {
	fn from(value: sys::ELobbyDistanceFilter) -> Self {
		use sys::ELobbyDistanceFilter::*;

		match value {
			k_ELobbyDistanceFilterClose => Self::Close,
			k_ELobbyDistanceFilterDefault => Self::Medium,
			k_ELobbyDistanceFilterFar => Self::Far,
			k_ELobbyDistanceFilterWorldwide => Self::WorldWide,
			_ => unreachable!(),
		}
	}
}

/// Callback.
///
/// > Recieved upon attempting to enter a lobby. Lobby metadata is available to use immediately after receiving this.
///
/// Listener signature:
/// ```
/// # use rgpr_steamworks::{error::ChatRoomEnterError, dt::SteamId};
/// // Upon success, `error` is `None`
/// fn listener(lobby_steam_id: SteamId, invite_only: bool, error: Option<ChatRoomEnterError>)
/// # {}
/// ```
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#LobbyEnter_t)
#[derive(Clone, Copy, Debug)]
pub struct LobbyEnter;

impl SteamworksCallback for LobbyEnter {
	const C_ENUM: i32 = sys::LobbyEnter_t_k_iCallback as i32;
	const REQUIRES_LISTENERS: bool = true;
	type ListenFn = Box<dyn FnMut(SteamId, bool, Option<ChatRoomEnterError>) + Send>;

	#[allow(private_interfaces)]
	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = &*(void_ptr as *const sys::LobbyEnter_t);
		let lobby_steam_id = SteamId(data.m_ulSteamIDLobby);
		let invite_only = data.m_bLocked;
		let error = ChatRoomEnterError::try_from(data.m_EChatRoomEnterResponse as u32).ok();

		for (_, listener_fn) in listeners {
			listener_fn(lobby_steam_id, invite_only, error)
		}
	}
}
/// Callback.
///
/// > Recieved upon attempting to enter a lobby. Lobby metadata is available to use immediately after receiving this.
///
/// Listener signature:
/// ```
/// # use rgpr_steamworks::{error::ChatRoomEnterError, dt::SteamId};
/// // Upon success, `error` is `None`
/// fn listener(user_steam_id: SteamId, lobby_steam_id: SteamId, game_id: u64)
/// # {}
/// ```
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#LobbyInvite_t)
#[derive(Clone, Copy, Debug)]
pub struct LobbyInvite;

impl SteamworksCallback for LobbyInvite {
	const C_ENUM: i32 = sys::LobbyInvite_t_k_iCallback as i32;
	const REQUIRES_LISTENERS: bool = true;
	type ListenFn = Box<dyn FnMut(SteamId, SteamId, u64) + Send>;

	#[allow(private_interfaces)]
	unsafe fn callback(void_ptr: *mut c_void, listeners: &mut CallbackListenerIter<Self::ListenFn>, _private: Private) {
		let data = &*(void_ptr as *const sys::LobbyInvite_t);
		let user_steam_id = SteamId(data.m_ulSteamIDUser);
		let lobby_steam_id = SteamId(data.m_ulSteamIDLobby);
		let game_id: u64 = data.m_ulGameID.into();

		for (_, listener_fn) in listeners {
			listener_fn(user_steam_id, lobby_steam_id, game_id);
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Specifies the lobby type.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
pub enum LobbyType {
	/// > The only way to join the lobby is from an invite.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
	Private,

	/// > Joinable by friends and invitees, but does not show up in the lobby list.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
	FriendsOnly,

	/// > Returned by search and visible to friends.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
	Public,

	/// > Returned by search, but not visible to other friends.
	/// This is useful if you want a user in two lobbies, for example matching groups together.
	/// A user can be in only one regular lobby, and up to two invisible lobbies.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
	Invisible,

	/// Private, unique and does not delete when empty - only one of these may exist per unique keypair set.
	/// These lobbies can only be created from the Web API.
	///
	/// Currently not on the docs, found in `isteammatchmaking.h`.  
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamMatchmaking#ELobbyType)
	PrivateUnique,
}

impl From<sys::ELobbyType> for LobbyType {
	fn from(value: sys::ELobbyType) -> Self {
		use sys::ELobbyType::*;

		match value {
			k_ELobbyTypePrivate => Self::Private,
			k_ELobbyTypeFriendsOnly => Self::FriendsOnly,
			k_ELobbyTypePublic => Self::Public,
			k_ELobbyTypeInvisible => Self::Invisible,
			k_ELobbyTypePrivateUnique => Self::PrivateUnique,
			_ => unreachable!(),
		}
	}
}

impl From<LobbyType> for sys::ELobbyType {
	fn from(value: LobbyType) -> Self {
		use LobbyType::*;

		match value {
			Private => Self::k_ELobbyTypePrivate,
			FriendsOnly => Self::k_ELobbyTypeFriendsOnly,
			Public => Self::k_ELobbyTypePublic,
			Invisible => Self::k_ELobbyTypeInvisible,
			PrivateUnique => Self::k_ELobbyTypePrivateUnique,
		}
	}
}
