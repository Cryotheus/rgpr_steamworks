//! Rust data types and traits loosely modeling what is used by Steamworks.
//!
//! [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#typedefs)

use crate::error::IntoCIndexError;
use rgpr_steamworks_sys as sys;
use std::fmt::{Display, Formatter};
use crate::interfaces::Steam;

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

	/// Returns the string wrapped.
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
			k_EAccountTypeMax | _ => unreachable!(),
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

/// > A Steam ID is a unique identifier for a Steam accounts, Steam groups, Lobbies and Chat rooms,
/// and used to differentiate users in all parts of the Steamworks API.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#CSteamID)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SteamId(pub u64);

impl SteamId {
	pub fn account_id(self) -> u32 {
		self.comp().m_unAccountID()
	}

	pub fn account_instance(self) -> u32 {
		self.comp().m_unAccountInstance()
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn account_type(self) -> AccountType {
		self.comp().m_EAccountType().try_into().ok().unwrap_or(AccountType::Invalid)
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn is_anonymous(self) -> bool {
		match self.account_type() {
			AccountType::AnonGameServer => true,
			AccountType::AnonUser => true,
			_ => false,
		}
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn is_game_server(self) -> bool {
		match self.account_type() {
			AccountType::AnonGameServer => true,
			AccountType::GameServer => true,
			_ => false,
		}
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn is_group(self) -> bool {
		self.account_type() == AccountType::Clan
	}

	/// Returns `true` if the Steam ID is for a lobby **OR** group chat.
	/// Steam does not differentiate between the two.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EAccountType)
	pub fn is_lobby(self) -> bool {
		self.account_type() == AccountType::Chat
	}

	/// Is a user account that can play games.
	pub fn is_user(self) -> bool {
		match self.account_type() {
			AccountType::Individual => true,
			AccountType::Multiseat => true,
			AccountType::ConsoleUser => true,
			_ => false,
		}
	}

	/// Returns `None` if the `SteamId` is 0.
	pub fn non_zero(self) -> Option<Self> {
		if self.0 == 0 {
			None
		} else {
			Some(self)
		}
	}

	/// Returns `None` if the provided value was 0 when turned into a `SteamId`.
	pub fn non_zero_from(value: impl Into<SteamId>)  -> Option<Self> {
		value.into().non_zero()
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
			k_EUniverseMax | _ => None,
		}
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
pub trait IntoCIndex: Sized {
	/// Always returns an `i32` that is positive.
	///
	/// # Panics
	/// If `self` cannot be converted into a positive `i32`.
	fn into_c_index(self) -> i32 {
		self.try_into_c_index().unwrap()
	}

	fn try_into_c_index(self) -> Result<i32, IntoCIndexError>;
}

impl IntoCIndex for i32 {
	fn try_into_c_index(self) -> Result<i32, IntoCIndexError> {
		if self < 0 {
			Err(IntoCIndexError::Negative)
		} else {
			Ok(self)
		}
	}
}

impl IntoCIndex for usize {
	fn try_into_c_index(self) -> Result<i32, IntoCIndexError> {
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
		assert_eq_size!(super::SteamId, sys::CSteamID);
	}
}
