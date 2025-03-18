//! Rust data types and traits loosely modeling what is used by Steamworks.
//!
//! [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#typedefs)

use rgpr_steamworks_sys as sys;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::num::NonZeroU64;

/// A [`SteamId`].
/// Type alias is purely to help show the usage or provision of a [`SteamId`] is meant to be a lobby.
pub type LobbyId = SteamId;

/// Steam account types.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AccountType {
	/// > Used for invalid Steam IDs.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Invalid,

	/// > Regular user account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Individual,

	/// > Multiseat (e.g. cybercafe) account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Multiseat,

	/// > Persistent (not anonymous) game server account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	GameServer,

	/// > Anonymous game server account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	AnonGameServer,

	/// > Pending.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Pending,

	/// > Valve internal content server account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	ContentServer,

	/// > Steam Group (clan).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Clan,

	/// > Steam group chat or lobby.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	Chat,

	/// > Fake Steam ID for local PSN account on PS3 or Live account on 360, etc.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	ConsoleUser,

	/// > Anonymous user account. (Used to create an account or reset a password)
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	AnonUser,
}

impl From<sys::EAccountType> for AccountType {
	fn from(value: sys::EAccountType) -> Self {
		use sys::EAccountType::*;

		match value {
			k_EAccountTypeInvalid => Self::Invalid,
			k_EAccountTypeIndividual => Self::Individual,
			k_EAccountTypeMultiseat => Self::Multiseat,
			k_EAccountTypeGameServer => Self::GameServer,
			k_EAccountTypeAnonGameServer => Self::AnonGameServer,
			k_EAccountTypePending => Self::Pending,
			k_EAccountTypeContentServer => Self::ContentServer,
			k_EAccountTypeClan => Self::Clan,
			k_EAccountTypeChat => Self::Chat,
			k_EAccountTypeConsoleUser => Self::ConsoleUser,
			k_EAccountTypeAnonUser => Self::AnonUser,
			k_EAccountTypeMax => unreachable!(),
		}
	}
}

impl TryFrom<u32> for AccountType {
	type Error = ();

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		use sys::EAccountType;

		const INVALID: u32 = EAccountType::k_EAccountTypeInvalid as u32;
		const INDIVIDUAL: u32 = EAccountType::k_EAccountTypeIndividual as u32;
		const MULTISEAT: u32 = EAccountType::k_EAccountTypeMultiseat as u32;
		const GAMESERVER: u32 = EAccountType::k_EAccountTypeGameServer as u32;
		const ANONGAMESERVER: u32 = EAccountType::k_EAccountTypeAnonGameServer as u32;
		const PENDING: u32 = EAccountType::k_EAccountTypePending as u32;
		const CONTENTSERVER: u32 = EAccountType::k_EAccountTypeContentServer as u32;
		const CLAN: u32 = EAccountType::k_EAccountTypeClan as u32;
		const CHAT: u32 = EAccountType::k_EAccountTypeChat as u32;
		const CONSOLEUSER: u32 = EAccountType::k_EAccountTypeConsoleUser as u32;
		const ANONUSER: u32 = EAccountType::k_EAccountTypeAnonUser as u32;
		const MAX: u32 = EAccountType::k_EAccountTypeMax as u32;

		Ok(match value {
			INVALID => Self::Invalid,
			INDIVIDUAL => Self::Individual,
			MULTISEAT => Self::Multiseat,
			GAMESERVER => Self::GameServer,
			ANONGAMESERVER => Self::AnonGameServer,
			PENDING => Self::Pending,
			CONTENTSERVER => Self::ContentServer,
			CLAN => Self::Clan,
			CHAT => Self::Chat,
			CONSOLEUSER => Self::ConsoleUser,
			ANONUSER => Self::AnonUser,
			MAX | _ => return Err(()),
		})
	}
}

/// > Unique identifier for an app.
/// For more information see the [Applications] documentation.
///
/// Equivalent to `AppId_t`.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#AppId_t)
///
/// [Applications]: https://partner.steamgames.com/doc/store/application
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AppId(pub u32);

impl AppId {
	pub fn new(id: u32) -> Self {
		Self(id)
	}

	/// Returns `None` if the provided value was invalid when turned into a `AppId`.
	pub fn valid_from(value: impl Into<AppId>) -> Option<Self> {
		let app_id = value.into();

		if app_id.valid() {
			Some(app_id)
		} else {
			None
		}
	}

	pub fn valid(self) -> bool {
		self.0 != 0u32
	}
}

impl Display for AppId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<u32> for AppId {
	fn from(value: u32) -> Self {
		Self(value)
	}
}

impl From<AppId> for u32 {
	fn from(AppId(value): AppId) -> Self {
		value
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct AuthTicket(pub u32);

impl From<u32> for AuthTicket {
	fn from(value: u32) -> Self {
		Self(value)
	}
}

impl From<AuthTicket> for u32 {
	fn from(AuthTicket(value): AuthTicket) -> Self {
		value
	}
}

/// A String expected to be a comma-separated list of values.
/// Use [`iter`] to iterate over the values.
///
/// [`iter`]: CsvString::iter
#[derive(Clone, Debug, PartialEq)]
pub struct CsvString(pub String);

impl CsvString {
	/// Collect the values into a [`Vec<&str>`](Vec).
	pub fn collect(&self) -> Vec<&str> {
		self.iter().collect()
	}

	/// Returns an iterator over each value in the string.
	pub fn iter(&self) -> impl Iterator<Item = &str> + DoubleEndedIterator {
		self.0.split(',')
	}

	/// Returns the wrapped [`String`].
	pub fn take(self) -> String {
		self.0
	}
}

impl AsRef<str> for CsvString {
	fn as_ref(&self) -> &str {
		&self.0
	}
}

impl From<CsvString> for String {
	fn from(CsvString(value): CsvString) -> Self {
		value
	}
}

/// > Unique identifier for a depot.
///
/// See [Depots] for an explanation.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#DepotId_t)
///
/// [Depots]: https://partner.steamgames.com/doc/store/application/depots
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct DepotId(pub u32);

impl DepotId {
	pub fn new(id: u32) -> Self {
		Self(id)
	}

	pub fn valid(self) -> bool {
		self.0 != 0u32
	}
}

impl Display for DepotId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<u32> for DepotId {
	fn from(value: u32) -> Self {
		Self(value)
	}
}

impl From<DepotId> for u32 {
	fn from(DepotId(value): DepotId) -> Self {
		value
	}
}

/// > The globally unique identifier for Steam Games.
///
/// This is an [`AppId`] with additional information.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#CGameID)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GameId(pub u64);

impl GameId {
	/// The contained [`AppId`].
	pub fn app_id(self) -> AppId {
		AppId(self.c_union().m_nAppID() as u32)
	}

	/// # Panics
	/// If the type byte is not in the range of `0..4`.  
	/// This only happens if the internal representation of the `GameId` is incorrectly tampered with.
	pub fn id_type(self) -> GameIdType {
		self.c_union().m_nType().try_into().ok().unwrap()
	}

	/// Checks if the `GameId` is of a valid representation.
	pub fn valid(self) -> bool {
		let Ok(id_type) = self.c_union().m_nType().try_into() else {
			return false;
		};

		/// `0b__1000_0000___0000_0000___0000_0000___0000_0000`
		const MOD_MASK: u32 = 0x80_00_00_00;

		match id_type {
			//normal steam app
			GameIdType::App => self.app_id().valid(),

			//mod, steam pipe maybe?
			GameIdType::GameMod => self.app_id().valid() && (self.mod_id() & MOD_MASK) != 0u32,

			GameIdType::Shortcut => {
				let mod_id = self.mod_id();

				!self.app_id().valid() && (mod_id & MOD_MASK) != 0 && mod_id >= (5000 | MOD_MASK)
				// 5000 k_unMaxExpectedLocalAppId - shortcuts are pushed beyond that range
			}

			GameIdType::P2p => !self.app_id().valid() && (self.mod_id() & MOD_MASK) != 0u32,
		}
	}

	/// Seems to be only used interally in Steam.
	pub fn mod_id(self) -> u32 {
		self.c_union().m_nModID() as u32
	}

	/// Changes the [`AppId`]
	pub fn set_app_id(&mut self, app_id: impl Into<AppId>) {
		self.c_union().set_m_nAppID(app_id.into().0 as _);
	}

	/// Same as [`set_app_id`] but returns a copy of `self`.
	///
	/// [`set_app_id`]: Self::set_app_id
	pub fn with_app_id(mut self, app_id: impl Into<AppId>) -> Self {
		self.set_app_id(app_id);

		self
	}

	fn c_union(self) -> sys::CGameID_GameID_t {
		unsafe { sys::CGameID__bindgen_ty_1 { m_ulGameID: self.0 }.m_gameID }
	}
}

impl Display for GameId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<u64> for GameId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl From<GameId> for u64 {
	fn from(GameId(value): GameId) -> Self {
		value
	}
}

impl From<sys::CGameID> for GameId {
	fn from(value: sys::CGameID) -> Self {
		Self(unsafe { value.__bindgen_anon_1.m_ulGameID } as u64)
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameIdType {
	App,
	GameMod,
	Shortcut,
	P2p,
}

impl From<sys::CGameID_EGameIDType> for GameIdType {
	fn from(value: sys::CGameID_EGameIDType) -> Self {
		use sys::CGameID_EGameIDType::*;

		match value {
			k_EGameIDTypeApp => Self::App,
			k_EGameIDTypeGameMod => Self::GameMod,
			k_EGameIDTypeShortcut => Self::Shortcut,
			k_EGameIDTypeP2P => Self::P2p,
		}
	}
}

impl TryFrom<u32> for GameIdType {
	type Error = ();

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		Ok(match value {
			0 => Self::App,
			1 => Self::GameMod,
			2 => Self::Shortcut,
			3 => Self::P2p,
			_ => return Err(()),
		})
	}
}

/// A handle to an image managed by the Steam API.
///
/// To get the image, enabled the `image` feature and call [`UtilsInterface::image`].
///
/// [`UtilsInterface::image`]: crate::interfaces::utils::UtilsInterface::image
#[derive(Clone, Copy, Debug)]
pub struct ImageHandle {
	/// The handle to the image cached/stored by Steam.
	pub(crate) handle: i32,

	/// The width and height packed into a `u64`.
	/// Width is packed in the 32 most-significant bits
	/// and height is in the 32 least-significant bits.
	///
	/// Image dimensions should never be zero,
	/// thus we can utilize Rust's null pointer optimization here.
	pub(crate) size: Option<NonZeroU64>,
}

impl ImageHandle {
	/// Gets the size of the image if it is cached.
	/// 
	/// The order is `[width, height]`.
	pub fn get_cached_size(&self) -> Option<[u32; 2]> {
		let packed = self.size?.get();

		Some([
			(packed >> 32) as u32,
			//we don't have to actually use the mask
			//we could just `packed as u32` - but I'm scared
			//worst case scenario: this just gets optimized out anyways
			(packed & 0b11111111_11111111_11111111_11111111) as u32,
		])
	}

	/// # Panics
	/// If size is `Some([0, 0])`.
	pub(crate) fn new(handle: i32, size: Option<[u32; 2]>) -> Self {
		Self {
			handle,
			size: size.map(|[width, height]| NonZeroU64::new(((width as u64) << 32) + height as u64).unwrap()),
		}
	}
}

impl Hash for ImageHandle {
	fn hash<H: Hasher>(&self, state: &mut H) {
		state.write_i32(self.handle);
	}
}

impl PartialEq for ImageHandle {
	fn eq(&self, other: &Self) -> bool {
		self.handle == other.handle
	}
}

impl Eq for ImageHandle {}

/// > A Steam ID is a unique identifier for a Steam accounts, Steam groups, Lobbies and Chat rooms,
/// and used to differentiate users in all parts of the Steamworks API.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#CSteamID)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct SteamId(pub u64);

impl SteamId {
	pub fn new(id: u64) -> Self {
		Self(id)
	}

	pub fn account_id(self) -> u32 {
		self.comp().m_unAccountID()
	}

	/// An ID that is unique to this `SteamId`'s [`universe`] only.
	///
	/// [`universe`]: Self::universe
	pub fn account_instance(self) -> u32 {
		self.comp().m_unAccountInstance()
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn account_type(self) -> AccountType {
		self.comp().m_EAccountType().try_into().ok().unwrap_or(AccountType::Invalid)
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn anonymous(self) -> bool {
		match self.account_type() {
			AccountType::AnonGameServer => true,
			AccountType::AnonUser => true,
			_ => false,
		}
	}

	/// Steam groups, usually.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn clan(self) -> bool {
		self.account_type() == AccountType::Clan
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn game_server(self) -> bool {
		match self.account_type() {
			AccountType::AnonGameServer => true,
			AccountType::GameServer => true,
			_ => false,
		}
	}

	/// Returns `true` if the Steam ID is for a lobby **OR** group chat.
	/// Steam does not differentiate between the two.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn lobby(self) -> bool {
		self.account_type() == AccountType::Chat
	}

	/// Returns `None` if the provided value was [`invalid`] when turned into a `SteamId`.
	///
	/// [`invalid`]: Self::valid
	pub fn valid_from(value: impl Into<SteamId>) -> Option<Self> {
		let steam_id = value.into();

		if steam_id.valid() {
			Some(steam_id)
		} else {
			None
		}
	}

	/// Is a user account that can play games.
	pub fn user(self) -> bool {
		match self.account_type() {
			AccountType::Individual => true,
			AccountType::Multiseat => true,
			AccountType::ConsoleUser => true,
			_ => false,
		}
	}

	/// Returns `None` if the unpacked enum was invalid.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	pub fn universe(self) -> Option<Universe> {
		use sys::EUniverse::*;

		match self.comp().m_EUniverse() {
			k_EUniverseInvalid => Some(Universe::Invalid),
			k_EUniversePublic => Some(Universe::Public),
			k_EUniverseBeta => Some(Universe::Beta),
			k_EUniverseInternal => Some(Universe::Internal),
			k_EUniverseDev => Some(Universe::Dev),
			k_EUniverseMax => None,
		}
	}

	/// Checks if the `SteamId` is of valid representation.
	/// This does not check if the `SteamId` is associated with an existing entity on Steam.
	pub fn valid(self) -> bool {
		self.0 != 0
	}

	#[doc(hidden)]
	fn comp(self) -> sys::CSteamID_SteamID_t_SteamIDComponent_t {
		unsafe { sys::CSteamID_SteamID_t { m_unAll64Bits: self.0 }.m_comp }
	}
}

impl Display for SteamId {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		Display::fmt(&self.0, f)
	}
}

impl From<u64> for SteamId {
	fn from(value: u64) -> Self {
		Self(value)
	}
}

impl From<sys::CSteamID> for SteamId {
	fn from(value: sys::CSteamID) -> Self {
		Self(unsafe { value.m_steamid.m_unAll64Bits })
	}
}

impl From<SteamId> for u64 {
	fn from(SteamId(value): SteamId) -> Self {
		value
	}
}

impl From<SteamId> for sys::CSteamID {
	fn from(SteamId(value): SteamId) -> Self {
		sys::CSteamID {
			m_steamid: sys::CSteamID_SteamID_t { m_unAll64Bits: value },
		}
	}
}

/// > Steam universes. Each universe is a self-contained Steam instance.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Universe {
	/// > Invalid.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	Invalid,

	/// > The standard public universe.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	Public,

	/// > Beta universe used inside Valve.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	Beta,

	/// > Internal universe used inside Valve.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	Internal,

	/// > Dev universe used inside Valve.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EUniverse)
	Dev,
}

/// For conversion into a positive `i32`,
/// which is the equivalent type for iteration in Steamworks.
#[cfg(feature = "steam")]
pub trait IntoCIndex: Sized {
	/// Always returns an `i32` that is positive.
	///
	/// # Panics
	/// If `self` cannot be converted into a positive `i32`.
	fn into_c_index(self) -> i32 {
		self.try_into_c_index().unwrap()
	}

	fn try_into_c_index(self) -> Result<i32, crate::error::IntoCIndexError>;
}

#[cfg(feature = "steam")]
impl IntoCIndex for i32 {
	fn try_into_c_index(self) -> Result<i32, crate::error::IntoCIndexError> {
		if self < 0 {
			Err(crate::error::IntoCIndexError::Negative)
		} else {
			Ok(self)
		}
	}
}

#[cfg(feature = "steam")]
impl IntoCIndex for usize {
	fn try_into_c_index(self) -> Result<i32, crate::error::IntoCIndexError> {
		Ok(self.try_into()?)
	}
}

#[cfg(test)]
mod test {
	use crate::sys;
	use static_assertions::assert_eq_size;

	#[test]
	fn assert_sizes() {
		assert_eq_size!(super::AppId, sys::AppId_t);
		assert_eq_size!(super::AuthTicket, sys::HAuthTicket);
		assert_eq_size!(super::DepotId, sys::DepotId_t);
		assert_eq_size!(super::GameId, sys::CGameID);
		assert_eq_size!(super::SteamId, sys::CSteamID);
	}
}
