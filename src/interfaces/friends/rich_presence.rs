use crate::dt::SteamId;
use crate::error::{SteamError, UnspecifiedError};
use crate::interfaces::friends::FriendsInterface;
use crate::iter::SteamApiIterator;
use crate::util::{success, FiniteBytes};
use crate::{sys, Private};
use std::ffi::{c_char, c_int, CStr, CString};
use std::ptr::null;

/// The maximum amount of key-value pairs the Steam API will sync.
pub const MAX_KEYS: usize = sys::k_cchMaxRichPresenceKeys as usize;

/// The maximum amount of bytes a key can be, excluding the nul terminator.
pub const KEY_MAX: usize = sys::k_cchMaxRichPresenceKeyLength as usize - 1;

/// The maximum amount of bytes a value can be, excluding the nul terminator.
pub const VALUE_MAX: usize = sys::k_cchMaxRichPresenceValueLength as usize - 1;

/// Accessor for the rich presence features of the Steam API's [`FriendsInterface`].
/// Allows the mutation of the current user's rich presence details.
#[derive(Debug)]
pub struct RichPresenceInterface<'a> {
	pub(super) ifc: &'a FriendsInterface,
}

impl<'a> RichPresenceInterface<'a> {
	/// > Clears all of the current user's Rich Presence key/values.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ClearRichPresence)
	pub fn clear(&mut self) {
		unsafe { sys::SteamAPI_ISteamFriends_ClearRichPresence(*self.ifc.fip) };
	}

	/// Sets the rich presence key-value pair.
	///
	/// # Errors
	/// - [`StrNulError`] if `key` or `value` contains a byte of `0`
	/// - [`Unspecified`] if the Steam API reported an error (always unspecified)
	///
	/// # Panics
	/// - If the `key` is not less than 64/[`KEY_MAX`] bytes
	/// - If the `value` is not less than 256/[`VALUE_MAX`] bytes
	///
	/// [`StrNulError`]: SteamError::StrNulError
	/// [`Unspecified`]: SteamError::Unspecified
	pub fn insert(&mut self, key: impl FiniteBytes<KEY_MAX>, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		let key = key.to_finite_cstring()?;
		let value = value.to_finite_cstring()?;

		Ok(unsafe { self.set_unchecked(key.as_ptr(), value.as_ptr()) }?)
	}

	/// # Errors
	/// - [`StrNulError`] if `key` or `value` contains a byte of `0`
	/// - [`Unspecified`] if the Steam API reported an error (always unspecified)
	///
	/// # Panics
	/// - If the `key` is not less than 64 bytes
	/// - If the `value` is not less than 256 bytes
	///
	/// [`StrNulError`]: SteamError::StrNulError
	/// [`Unspecified`]: SteamError::Unspecified
	pub fn remove(&mut self, key: impl FiniteBytes<KEY_MAX>) -> Result<(), SteamError> {
		let key = key.to_finite_cstring()?;

		unsafe { self.set_unchecked(key.as_ptr(), null()) }?;

		Ok(())
	}

	/// > A UTF-8 string that contains the command-line for how a friend can connect to a game.
	/// This enables the 'join game' button in the 'view game info' dialog,
	/// in the steam friends list right click menu,
	/// and on the players Steam community profile.
	/// Be sure your app implements [`AppsInterface::launch_command_line`] so you can disable the popup warning when launched via a command line.
	///
	/// Same as [`insert`] with `connect` as the key.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetRichPresence)
	///
	/// [`AppsInterface::launch_command_line`]: crate::interfaces::apps::AppsInterface::launch_command_line
	/// [`insert`]: Self::insert
	pub fn set_connect(&mut self, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		self.set(c"connect", value)
	}

	/// > Names a rich presence localization token that will be displayed in the viewing user's selected language in the Steam client UI.
	/// See [Rich Presence Localization] for more info,
	/// including a link to a page for testing this rich presence data.
	/// If steam_display is not set to a valid localization tag,
	/// then rich presence will not be displayed in the Steam client.
	///
	/// Same as [`insert`] with `steam_display` as the key.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetRichPresence)
	///
	/// [`insert`]: Self::insert
	/// [Rich Presence Localization]: https://partner.steamgames.com/doc/api/ISteamFriends#richpresencelocalization
	pub fn set_display(&mut self, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		self.set(c"steam_display", value)
	}

	/// > When set, indicates to the Steam client that the player is a member of a particular group.
	/// Players in the same group may be organized together in various places in the Steam UI.
	/// This string could identify a party, a server, or whatever grouping is relevant for your game.
	/// The string itself is not displayed to users.
	///
	/// Same as [`insert`] with `steam_player_group` as the key.
	///
	/// Do not put data that is should not be networked outside of the users in the group.
	/// Even if the provided value is not displayed,
	/// it is still networked to Steam users.
	///
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetRichPresence)
	///
	/// [`insert`]: Self::insert
	pub fn set_group(&mut self, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		self.set(c"steam_player_group", value)
	}

	/// > When set, indicates the total number of players in the group set by [`set_group`].
	/// The Steam client may use this number to display additional information about a group when all of the members are not part of a user's friends list.
	/// (For example, "Bob, Pete, and 4 more".)
	///
	/// Same as [`insert`] with `steam_player_group_size` as the key.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetRichPresence)
	///
	/// [`insert`]: Self::insert
	/// [`set_group`]: Self::set_group
	pub fn set_group_size(&mut self, value: u32) -> Result<(), SteamError> {
		//parsing int to str never yields nul bytes
		let parsed = CString::new(value.to_string().into_bytes()).unwrap();

		Ok(unsafe { self.set_unchecked(c"steam_player_group_size".as_ptr(), parsed.as_ptr()) }?)
	}

	/// > A UTF-8 string that will show up in the 'view game info' dialog in the Steam friends list.
	///
	/// Same as [`insert`] with `status` as the key.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetRichPresence)
	///
	/// [`insert`]: Self::insert
	pub fn set_status(&mut self, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		self.set(c"status", value)
	}

	/// Only performs assertions on the value.
	/// Used for creating methods for "special keys" used by Steam.
	fn set(&mut self, key: &'static CStr, value: impl FiniteBytes<VALUE_MAX>) -> Result<(), SteamError> {
		let value = value.to_finite_cstring()?;

		Ok(unsafe { self.set_unchecked(key.as_ptr(), value.as_ptr()) }?)
	}

	/// # Errors
	/// Returns [`Unspecified`]
	///
	/// # Safety
	/// - Pointers must be null-terminated *const c_char pointers (C strings)
	/// - `key` must not be a null ptr
	///
	/// [`Unspecified`]: SteamError::Unspecified
	unsafe fn set_unchecked(&mut self, key: *const c_char, value: *const c_char) -> Result<(), UnspecifiedError> {
		debug_assert!(!key.is_null());

		success(unsafe { sys::SteamAPI_ISteamFriends_SetRichPresence(*self.ifc.fip, key, value) })
	}
}

#[derive(Debug)]
pub struct RichPresenceIter<'a> {
	pub(super) cursor: c_int,
	pub(super) ifc: &'a FriendsInterface,
	pub(super) steam_id: SteamId,
}

/// A key-value pair with rich presence details.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RichPresenceKeyValue {
	bytes: Vec<u8>,
	partition: u8,
}

impl RichPresenceKeyValue {
	/// Returns a reference to the bytes of the key.
	#[inline(always)]
	pub fn as_key(&self) -> &[u8] {
		&self.bytes[0..self.partition as usize]
	}

	#[inline(always)]
	pub fn as_key_value(&self) -> (&[u8], &[u8]) {
		(self.as_key(), self.as_value())
	}

	#[inline(always)]
	pub fn as_value(&self) -> &[u8] {
		&self.bytes[self.partition as usize..]
	}
}

unsafe impl<'a> SteamApiIterator for RichPresenceIter<'a> {
	type Item = RichPresenceKeyValue;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		let key_char_ptr = sys::SteamAPI_ISteamFriends_GetFriendRichPresenceKeyByIndex(*self.ifc.fip, self.steam_id.0 as _, index);

		//the ptr shouldnt be null, and the string should never be empty
		if key_char_ptr.is_null() || *key_char_ptr == 0 {
			return None;
		}

		let mut bytes = CStr::from_ptr(key_char_ptr).to_bytes().to_vec();
		let value_char_ptr = sys::SteamAPI_ISteamFriends_GetFriendRichPresence(*self.ifc.fip, self.steam_id.0 as _, key_char_ptr);

		//the ptr shouldnt be null, and the string should never be empty
		if value_char_ptr.is_null() || *value_char_ptr == 0 {
			return None;
		}

		//we should panic if this can't convert
		let partition: u8 = bytes.len().try_into().unwrap();

		bytes.extend_from_slice(CStr::from_ptr(value_char_ptr).to_bytes());

		Some(RichPresenceKeyValue { bytes, partition })
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetFriendRichPresenceKeyCount(*self.ifc.fip, self.steam_id.0 as _);
	}
}

#[derive(Debug)]
pub struct RichPresenceKeyIter<'a> {
	pub(super) cursor: c_int,
	pub(super) ifc: &'a FriendsInterface,
	pub(super) steam_id: SteamId,
}

unsafe impl<'a> SteamApiIterator for RichPresenceKeyIter<'a> {
	type Item = Vec<u8>;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		let char_ptr = sys::SteamAPI_ISteamFriends_GetFriendRichPresenceKeyByIndex(*self.ifc.fip, self.steam_id.0 as _, index);

		if char_ptr.is_null() {
			return None;
		}

		if *char_ptr == 0 {
			return None;
		}

		Some(CStr::from_ptr(char_ptr).to_bytes().to_vec())
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetFriendRichPresenceKeyCount(*self.ifc.fip, self.steam_id.0 as _);
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn assert_overflow() {
		assert!(super::KEY_MAX < u8::MAX as usize);
	}
}
