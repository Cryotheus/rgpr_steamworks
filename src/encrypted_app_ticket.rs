//! Utilities for verifying a user's identity
//! See [`DecryptedAppTicket`].
//!
//! The app ticketing system is entirely independent from the rest of the Steam API.  
//! No IPCs are ever made in this module, and even Steam itself isn't even necessary.
//!
//! This module does however, require the associated `sdkencryptedappticket` dylib.

use crate::dt::{AppId, SteamId};
use crate::sys;
use std::ffi::c_uchar;
use std::time::{Duration, SystemTime};

/// Expect a lack of documentation, as Steamworks lacks documentation for half of this API.  
/// You can create a decrypted ticket with the bytes of an encrypted one using [`DecryptedAppTicket::new`].
///
/// [`DecryptedAppTicket::new`]: DecryptedAppTicket::new
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/features/auth#encryptedapptickets)
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct DecryptedAppTicket(Vec<u8>);

impl DecryptedAppTicket {
	/// Attempts to decrypt the bytes of an encrypted app ticket.  
	/// `capacity` is how many bytes to use _as a minimum_ for the decrypted ticket's buffer.
	/// If this is too small, the decryption will fail.
	/// The docs use `1024` for their example.
	///
	/// Clients should never have access to `key_bytes`,
	/// make sure the key
	///
	/// # Panics
	/// If `key_bytes` is longer than [`k_nSteamEncryptedAppTicketSymmetricKeyLen`](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#k_nSteamEncryptedAppTicketSymmetricKeyLen).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BDecryptTicket)
	pub fn new(encrypted_bytes: &[u8], key_bytes: &[u8], capacity: usize) -> Result<Self, DecryptionError> {
		assert!(
			key_bytes.len() <= sys::k_nSteamEncryptedAppTicketSymmetricKeyLen as usize,
			"EncryptedAppTicket key length must be <= {} bytes",
			sys::k_nSteamEncryptedAppTicketSymmetricKeyLen
		);

		let mut buffer: Vec<u8> = Vec::with_capacity(encrypted_bytes.len().max(capacity));
		let buffer_capacity = buffer.capacity();
		let mut buffer_used = buffer_capacity as u32;

		//zero the memory the vec points to
		//just in case the decryption doesn't write zeroes
		buffer.fill(0);

		unsafe {
			if sys::SteamEncryptedAppTicket_BDecryptTicket(
				encrypted_bytes.as_ptr(),
				encrypted_bytes.len() as u32,
				buffer.as_mut_ptr(),
				&mut buffer_used as _,
				key_bytes.as_ptr() as _,
				key_bytes.len() as _,
			) {
				//update the length of the vec to how much was written
				assert!(buffer_used <= buffer_capacity as u32, "DecryptedAppTicket buffer length was larger than the buffer itself");
				buffer.set_len(buffer_used as usize);

				if buffer_used == 0 {
					Err(DecryptionError::DataUnfulfilled)
				} else {
					Ok(Self(buffer))
				}
			} else {
				Err(DecryptionError::Unspecified)
			}
		}
	}

	/// No documentation for this function is currently available on the Steamworks reference.
	pub fn app_defined_value(&self) -> Option<u32> {
		let mut value = 0u32;

		if unsafe { sys::SteamEncryptedAppTicket_BGetAppDefinedValue(self.ptr(), self.cub(), &mut value as _) } {
			Some(value)
		} else {
			None
		}
	}

	/// > Gets the App ID associated with a ticket.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#GetTicketAppID)
	pub fn app_id(&self) -> AppId {
		unsafe { sys::SteamEncryptedAppTicket_GetTicketAppID(self.ptr(), self.cub()).into() }
	}

	/// The bytes decrypted.
	/// It is typically better to use the built-in methods instead of parsing the raw bytes.
	pub fn as_slice(&self) -> &[u8] {
		self.0.as_slice()
	}

	/// > Gets the time that a ticket was issued.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#GetTicketIssueTime)
	pub fn issue_time(&self) -> SystemTime {
		let unix_time = unsafe { sys::SteamEncryptedAppTicket_GetTicketIssueTime(self.ptr(), self.cub()) };

		SystemTime::UNIX_EPOCH + Duration::from_secs(unix_time as u64)
	}

	/// > Verifies that a decrypted app ticket is for the expected application.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BIsTicketForApp)
	pub fn is_for_app(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamEncryptedAppTicket_BIsTicketForApp(self.ptr(), self.cub(), app_id.into().0) }
	}

	/// No documentation for this function is currently available on the Steamworks reference.
	pub fn is_license_borrowed(&self) -> bool {
		unsafe { sys::SteamEncryptedAppTicket_BIsLicenseBorrowed(self.ptr(), self.cub()) }
	}

	/// No documentation for this function is currently available on the Steamworks reference.
	pub fn is_license_temporary(&self) -> bool {
		unsafe { sys::SteamEncryptedAppTicket_BIsLicenseTemporary(self.ptr(), self.cub()) }
	}

	/// No documentation for this function is currently available on the Steamworks reference.
	pub fn is_signed(&self, rsa_key: &[u8]) -> bool {
		//k_nSteamEncryptedAppTicketSymmetricKeyLen
		unsafe { sys::SteamEncryptedAppTicket_BIsTicketSigned(self.ptr(), self.cub(), rsa_key.as_ptr(), rsa_key.len() as u32) }
	}

	/// > Gets the Steam ID associated with a ticket.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#GetTicketSteamID)
	pub fn steam_id(&self) -> SteamId {
		unsafe {
			let mut c_steam_id: sys::CSteamID = std::mem::zeroed();

			sys::SteamEncryptedAppTicket_GetTicketSteamID(self.ptr(), self.cub(), &mut c_steam_id as _);

			c_steam_id.into()
		}
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BUserOwnsAppInTicket)
	pub fn user_owns_app_in_ticket(&self, app_id: impl Into<AppId>) -> bool {
		unsafe { sys::SteamEncryptedAppTicket_BUserOwnsAppInTicket(self.ptr(), self.cub(), app_id.into().0) }
	}

	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/SteamEncryptedAppTicket#BUserIsVacBanned)
	pub fn vac_banned(&self) -> bool {
		unsafe { sys::SteamEncryptedAppTicket_BUserIsVacBanned(self.ptr(), self.cub()) }
	}

	#[doc(hidden)]
	fn cub(&self) -> u32 {
		self.0.len() as u32
	}

	#[doc(hidden)]
	unsafe fn ptr(&self) -> *mut c_uchar {
		//SAFETY: the Vec is always allocated upon creation
		//thus the pointer is not dangling
		self.0.as_ptr() as *mut c_uchar
	}

	/// No documentation for this function is currently available on the Steamworks reference.
	#[cfg(feature = "sys")]
	pub unsafe fn get_user_variable_data(&self, ptr: *mut std::ffi::c_uint) -> *const c_uchar {
		//TODO: figure out a good way to make this rusty
		sys::SteamEncryptedAppTicket_GetUserVariableData(self.ptr(), self.cub(), ptr)
	}
}

impl AsRef<[u8]> for DecryptedAppTicket {
	fn as_ref(&self) -> &[u8] {
		self.as_slice()
	}
}

impl From<DecryptedAppTicket> for Vec<u8> {
	fn from(DecryptedAppTicket(value): DecryptedAppTicket) -> Self {
		value
	}
}

/// The error type used in the return of [`DecryptedAppTicket::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum DecryptionError {
	#[error("locations for data were not filled")]
	DataUnfulfilled,

	#[error("failed, no error message from the SteamEncryptedAppTicket API is available")]
	Unspecified,
}
