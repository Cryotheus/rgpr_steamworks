#![allow(unused_qualifications)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    fn extract_bit(byte: u8, index: usize) -> bool {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        Self::extract_bit(byte, index)
    }
    #[inline]
    pub unsafe fn raw_get_bit(this: *const Self, index: usize) -> bool {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = *(core::ptr::addr_of!((*this).storage) as *const u8)
            .offset(byte_index as isize);
        Self::extract_bit(byte, index)
    }
    #[inline]
    fn change_bit(byte: u8, index: usize, val: bool) -> u8 {
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val { byte | mask } else { byte & !mask }
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub unsafe fn raw_set_bit(this: *mut Self, index: usize, val: bool) {
        debug_assert!(index / 8 < core::mem::size_of::<Storage>());
        let byte_index = index / 8;
        let byte = (core::ptr::addr_of_mut!((*this).storage) as *mut u8)
            .offset(byte_index as isize);
        *byte = Self::change_bit(*byte, index, val);
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub unsafe fn raw_get(this: *const Self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if Self::raw_get_bit(this, i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
    #[inline]
    pub unsafe fn raw_set(this: *mut Self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < core::mem::size_of::<Storage>());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <= core::mem::size_of::<Storage>(),
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            Self::raw_set_bit(this, index + bit_offset, val_bit_is_set);
        }
    }
}
pub const _STRING_H: u32 = 1;
pub const _FEATURES_H: u32 = 1;
pub const _ISOC95_SOURCE: u32 = 1;
pub const _ISOC99_SOURCE: u32 = 1;
pub const _ISOC11_SOURCE: u32 = 1;
pub const _ISOC2X_SOURCE: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const _XOPEN_SOURCE: u32 = 700;
pub const _XOPEN_SOURCE_EXTENDED: u32 = 1;
pub const _LARGEFILE64_SOURCE: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const _DYNAMIC_STACK_SIZE_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 1;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_ISOCXX11: u32 = 1;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const __USE_XOPEN: u32 = 1;
pub const __USE_XOPEN_EXTENDED: u32 = 1;
pub const __USE_UNIX98: u32 = 1;
pub const _LARGEFILE_SOURCE: u32 = 1;
pub const __USE_XOPEN2K8XSI: u32 = 1;
pub const __USE_XOPEN2KXSI: u32 = 1;
pub const __USE_LARGEFILE: u32 = 1;
pub const __USE_LARGEFILE64: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_DYNAMIC_STACK_SIZE: u32 = 1;
pub const __USE_GNU: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 36;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 0;
pub const __GLIBC_USE_LIB_EXT2: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 1;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 1;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const STEAMCLIENT_INTERFACE_VERSION: &[u8; 15] = b"SteamClient021\0";
pub const STEAMUSER_INTERFACE_VERSION: &[u8; 13] = b"SteamUser023\0";
pub const STEAMFRIENDS_INTERFACE_VERSION: &[u8; 16] = b"SteamFriends018\0";
pub const STEAMUTILS_INTERFACE_VERSION: &[u8; 14] = b"SteamUtils010\0";
pub const _STDIO_H: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const __cookie_io_functions_t_defined: u32 = 1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_DATA: u32 = 3;
pub const SEEK_HOLE: u32 = 4;
pub const P_tmpdir: &[u8; 5] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const L_cuserid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const _PRINTF_NAN_LEN_MAX: u32 = 4;
pub const RENAME_NOREPLACE: u32 = 1;
pub const RENAME_EXCHANGE: u32 = 2;
pub const RENAME_WHITEOUT: u32 = 4;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const k_nMaxLobbyKeyLength: u32 = 255;
pub const STEAMMATCHMAKING_INTERFACE_VERSION: &[u8; 20] = b"SteamMatchMaking009\0";
pub const STEAMMATCHMAKINGSERVERS_INTERFACE_VERSION: &[u8; 27] = b"SteamMatchMakingServers002\0";
pub const STEAMGAMESEARCH_INTERFACE_VERSION: &[u8; 24] = b"SteamMatchGameSearch001\0";
pub const STEAMPARTIES_INTERFACE_VERSION: &[u8; 16] = b"SteamParties002\0";
pub const STEAMREMOTESTORAGE_INTERFACE_VERSION: &[u8; 40] = b"STEAMREMOTESTORAGE_INTERFACE_VERSION016\0";
pub const STEAMUSERSTATS_INTERFACE_VERSION: &[u8; 36] = b"STEAMUSERSTATS_INTERFACE_VERSION013\0";
pub const STEAMAPPS_INTERFACE_VERSION: &[u8; 31] = b"STEAMAPPS_INTERFACE_VERSION008\0";
pub const STEAMNETWORKING_INTERFACE_VERSION: &[u8; 19] = b"SteamNetworking006\0";
pub const INVALID_SCREENSHOT_HANDLE: u32 = 0;
pub const STEAMSCREENSHOTS_INTERFACE_VERSION: &[u8; 38] = b"STEAMSCREENSHOTS_INTERFACE_VERSION003\0";
pub const STEAMMUSIC_INTERFACE_VERSION: &[u8; 32] = b"STEAMMUSIC_INTERFACE_VERSION001\0";
pub const k_SteamMusicNameMaxLength: u32 = 255;
pub const k_SteamMusicPNGMaxLength: u32 = 65535;
pub const STEAMMUSICREMOTE_INTERFACE_VERSION: &[u8; 38] = b"STEAMMUSICREMOTE_INTERFACE_VERSION001\0";
pub const INVALID_HTTPREQUEST_HANDLE: u32 = 0;
pub const INVALID_HTTPCOOKIE_HANDLE: u32 = 0;
pub const STEAMHTTP_INTERFACE_VERSION: &[u8; 31] = b"STEAMHTTP_INTERFACE_VERSION003\0";
pub const STEAM_INPUT_MAX_COUNT: u32 = 16;
pub const STEAM_INPUT_MAX_ANALOG_ACTIONS: u32 = 24;
pub const STEAM_INPUT_MAX_DIGITAL_ACTIONS: u32 = 256;
pub const STEAM_INPUT_MAX_ORIGINS: u32 = 8;
pub const STEAM_INPUT_MAX_ACTIVE_LAYERS: u32 = 16;
pub const STEAM_INPUT_MIN_ANALOG_ACTION_DATA: f64 = -1.0;
pub const STEAM_INPUT_MAX_ANALOG_ACTION_DATA: f64 = 1.0;
pub const STEAMINPUT_INTERFACE_VERSION: &[u8; 14] = b"SteamInput006\0";
pub const STEAM_CONTROLLER_MAX_COUNT: u32 = 16;
pub const STEAM_CONTROLLER_MAX_ANALOG_ACTIONS: u32 = 24;
pub const STEAM_CONTROLLER_MAX_DIGITAL_ACTIONS: u32 = 256;
pub const STEAM_CONTROLLER_MAX_ORIGINS: u32 = 8;
pub const STEAM_CONTROLLER_MAX_ACTIVE_LAYERS: u32 = 16;
pub const STEAM_CONTROLLER_MIN_ANALOG_ACTION_DATA: f64 = -1.0;
pub const STEAM_CONTROLLER_MAX_ANALOG_ACTION_DATA: f64 = 1.0;
pub const STEAMCONTROLLER_INTERFACE_VERSION: &[u8; 19] = b"SteamController008\0";
pub const STEAMUGC_INTERFACE_VERSION: &[u8; 30] = b"STEAMUGC_INTERFACE_VERSION021\0";
pub const STEAMHTMLSURFACE_INTERFACE_VERSION: &[u8; 39] = b"STEAMHTMLSURFACE_INTERFACE_VERSION_005\0";
pub const STEAMINVENTORY_INTERFACE_VERSION: &[u8; 30] = b"STEAMINVENTORY_INTERFACE_V003\0";
pub const STEAMTIMELINE_INTERFACE_VERSION: &[u8; 29] = b"STEAMTIMELINE_INTERFACE_V004\0";
pub const STEAMVIDEO_INTERFACE_VERSION: &[u8; 26] = b"STEAMVIDEO_INTERFACE_V007\0";
pub const STEAMPARENTALSETTINGS_INTERFACE_VERSION: &[u8; 43] = b"STEAMPARENTALSETTINGS_INTERFACE_VERSION001\0";
pub const STEAMREMOTEPLAY_INTERFACE_VERSION: &[u8; 37] = b"STEAMREMOTEPLAY_INTERFACE_VERSION003\0";
pub const _STDINT_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const INT8_WIDTH: u32 = 8;
pub const UINT8_WIDTH: u32 = 8;
pub const INT16_WIDTH: u32 = 16;
pub const UINT16_WIDTH: u32 = 16;
pub const INT32_WIDTH: u32 = 32;
pub const UINT32_WIDTH: u32 = 32;
pub const INT64_WIDTH: u32 = 64;
pub const UINT64_WIDTH: u32 = 64;
pub const INT_LEAST8_WIDTH: u32 = 8;
pub const UINT_LEAST8_WIDTH: u32 = 8;
pub const INT_LEAST16_WIDTH: u32 = 16;
pub const UINT_LEAST16_WIDTH: u32 = 16;
pub const INT_LEAST32_WIDTH: u32 = 32;
pub const UINT_LEAST32_WIDTH: u32 = 32;
pub const INT_LEAST64_WIDTH: u32 = 64;
pub const UINT_LEAST64_WIDTH: u32 = 64;
pub const INT_FAST8_WIDTH: u32 = 8;
pub const UINT_FAST8_WIDTH: u32 = 8;
pub const INT_FAST16_WIDTH: u32 = 64;
pub const UINT_FAST16_WIDTH: u32 = 64;
pub const INT_FAST32_WIDTH: u32 = 64;
pub const UINT_FAST32_WIDTH: u32 = 64;
pub const INT_FAST64_WIDTH: u32 = 64;
pub const UINT_FAST64_WIDTH: u32 = 64;
pub const INTPTR_WIDTH: u32 = 64;
pub const UINTPTR_WIDTH: u32 = 64;
pub const INTMAX_WIDTH: u32 = 64;
pub const UINTMAX_WIDTH: u32 = 64;
pub const PTRDIFF_WIDTH: u32 = 64;
pub const SIG_ATOMIC_WIDTH: u32 = 32;
pub const SIZE_WIDTH: u32 = 64;
pub const WCHAR_WIDTH: u32 = 32;
pub const WINT_WIDTH: u32 = 32;
pub const STEAMNETWORKINGMESSAGES_INTERFACE_VERSION: &[u8; 27] = b"SteamNetworkingMessages002\0";
pub const STEAMNETWORKINGSOCKETS_INTERFACE_VERSION: &[u8; 26] = b"SteamNetworkingSockets012\0";
pub const STEAMNETWORKINGUTILS_INTERFACE_VERSION: &[u8; 24] = b"SteamNetworkingUtils004\0";
pub const STEAMGAMESERVER_INTERFACE_VERSION: &[u8; 19] = b"SteamGameServer015\0";
pub const STEAMGAMESERVERSTATS_INTERFACE_VERSION: &[u8; 24] = b"SteamGameServerStats001\0";
pub const STEAMAPPTICKET_INTERFACE_VERSION: &[u8; 36] = b"STEAMAPPTICKET_INTERFACE_VERSION001\0";
pub type uint8 = ::std::os::raw::c_uchar;
pub type int8 = ::std::os::raw::c_schar;
pub type int16 = ::std::os::raw::c_short;
pub type uint16 = ::std::os::raw::c_ushort;
pub type int32 = ::std::os::raw::c_int;
pub type uint32 = ::std::os::raw::c_uint;
pub type int64 = ::std::os::raw::c_longlong;
pub type uint64 = ::std::os::raw::c_ulonglong;
pub type lint64 = ::std::os::raw::c_long;
pub type ulint64 = ::std::os::raw::c_ulong;
pub type intp = ::std::os::raw::c_longlong;
pub type uintp = ::std::os::raw::c_ulonglong;
pub type AppId_t = uint32;
pub const k_uAppIdInvalid: AppId_t = 0;
pub type DepotId_t = uint32;
pub const k_uDepotIdInvalid: DepotId_t = 0;
pub type RTime32 = uint32;
pub type SteamAPICall_t = uint64;
pub const k_uAPICallInvalid: SteamAPICall_t = 0;
pub type AccountID_t = uint32;
pub const k_uAccountIdInvalid: AccountID_t = 0;
pub type PartyBeaconID_t = uint64;
pub const k_ulPartyBeaconIdInvalid: PartyBeaconID_t = 0;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamIPType {
    k_ESteamIPTypeIPv4 = 0,
    k_ESteamIPTypeIPv6 = 1,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SteamIPAddress_t {
    pub __bindgen_anon_1: SteamIPAddress_t__bindgen_ty_1,
    pub m_eType: ESteamIPType,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union SteamIPAddress_t__bindgen_ty_1 {
    pub m_unIPv4: uint32,
    pub m_rgubIPv6: [uint8; 16usize],
    pub m_ipv6Qword: [uint64; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamIPAddress_t__bindgen_ty_1",
    ][::std::mem::size_of::<SteamIPAddress_t__bindgen_ty_1>() - 16usize];
    [
        "Alignment of SteamIPAddress_t__bindgen_ty_1",
    ][::std::mem::align_of::<SteamIPAddress_t__bindgen_ty_1>() - 1usize];
    [
        "Offset of field: SteamIPAddress_t__bindgen_ty_1::m_unIPv4",
    ][::std::mem::offset_of!(SteamIPAddress_t__bindgen_ty_1, m_unIPv4) - 0usize];
    [
        "Offset of field: SteamIPAddress_t__bindgen_ty_1::m_rgubIPv6",
    ][::std::mem::offset_of!(SteamIPAddress_t__bindgen_ty_1, m_rgubIPv6) - 0usize];
    [
        "Offset of field: SteamIPAddress_t__bindgen_ty_1::m_ipv6Qword",
    ][::std::mem::offset_of!(SteamIPAddress_t__bindgen_ty_1, m_ipv6Qword) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SteamIPAddress_t"][::std::mem::size_of::<SteamIPAddress_t>() - 20usize];
    [
        "Alignment of SteamIPAddress_t",
    ][::std::mem::align_of::<SteamIPAddress_t>() - 1usize];
    [
        "Offset of field: SteamIPAddress_t::m_eType",
    ][::std::mem::offset_of!(SteamIPAddress_t, m_eType) - 16usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUniverse {
    k_EUniverseInvalid = 0,
    k_EUniversePublic = 1,
    k_EUniverseBeta = 2,
    k_EUniverseInternal = 3,
    k_EUniverseDev = 4,
    k_EUniverseMax = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EResult {
    k_EResultNone = 0,
    k_EResultOK = 1,
    k_EResultFail = 2,
    k_EResultNoConnection = 3,
    k_EResultInvalidPassword = 5,
    k_EResultLoggedInElsewhere = 6,
    k_EResultInvalidProtocolVer = 7,
    k_EResultInvalidParam = 8,
    k_EResultFileNotFound = 9,
    k_EResultBusy = 10,
    k_EResultInvalidState = 11,
    k_EResultInvalidName = 12,
    k_EResultInvalidEmail = 13,
    k_EResultDuplicateName = 14,
    k_EResultAccessDenied = 15,
    k_EResultTimeout = 16,
    k_EResultBanned = 17,
    k_EResultAccountNotFound = 18,
    k_EResultInvalidSteamID = 19,
    k_EResultServiceUnavailable = 20,
    k_EResultNotLoggedOn = 21,
    k_EResultPending = 22,
    k_EResultEncryptionFailure = 23,
    k_EResultInsufficientPrivilege = 24,
    k_EResultLimitExceeded = 25,
    k_EResultRevoked = 26,
    k_EResultExpired = 27,
    k_EResultAlreadyRedeemed = 28,
    k_EResultDuplicateRequest = 29,
    k_EResultAlreadyOwned = 30,
    k_EResultIPNotFound = 31,
    k_EResultPersistFailed = 32,
    k_EResultLockingFailed = 33,
    k_EResultLogonSessionReplaced = 34,
    k_EResultConnectFailed = 35,
    k_EResultHandshakeFailed = 36,
    k_EResultIOFailure = 37,
    k_EResultRemoteDisconnect = 38,
    k_EResultShoppingCartNotFound = 39,
    k_EResultBlocked = 40,
    k_EResultIgnored = 41,
    k_EResultNoMatch = 42,
    k_EResultAccountDisabled = 43,
    k_EResultServiceReadOnly = 44,
    k_EResultAccountNotFeatured = 45,
    k_EResultAdministratorOK = 46,
    k_EResultContentVersion = 47,
    k_EResultTryAnotherCM = 48,
    k_EResultPasswordRequiredToKickSession = 49,
    k_EResultAlreadyLoggedInElsewhere = 50,
    k_EResultSuspended = 51,
    k_EResultCancelled = 52,
    k_EResultDataCorruption = 53,
    k_EResultDiskFull = 54,
    k_EResultRemoteCallFailed = 55,
    k_EResultPasswordUnset = 56,
    k_EResultExternalAccountUnlinked = 57,
    k_EResultPSNTicketInvalid = 58,
    k_EResultExternalAccountAlreadyLinked = 59,
    k_EResultRemoteFileConflict = 60,
    k_EResultIllegalPassword = 61,
    k_EResultSameAsPreviousValue = 62,
    k_EResultAccountLogonDenied = 63,
    k_EResultCannotUseOldPassword = 64,
    k_EResultInvalidLoginAuthCode = 65,
    k_EResultAccountLogonDeniedNoMail = 66,
    k_EResultHardwareNotCapableOfIPT = 67,
    k_EResultIPTInitError = 68,
    k_EResultParentalControlRestricted = 69,
    k_EResultFacebookQueryError = 70,
    k_EResultExpiredLoginAuthCode = 71,
    k_EResultIPLoginRestrictionFailed = 72,
    k_EResultAccountLockedDown = 73,
    k_EResultAccountLogonDeniedVerifiedEmailRequired = 74,
    k_EResultNoMatchingURL = 75,
    k_EResultBadResponse = 76,
    k_EResultRequirePasswordReEntry = 77,
    k_EResultValueOutOfRange = 78,
    k_EResultUnexpectedError = 79,
    k_EResultDisabled = 80,
    k_EResultInvalidCEGSubmission = 81,
    k_EResultRestrictedDevice = 82,
    k_EResultRegionLocked = 83,
    k_EResultRateLimitExceeded = 84,
    k_EResultAccountLoginDeniedNeedTwoFactor = 85,
    k_EResultItemDeleted = 86,
    k_EResultAccountLoginDeniedThrottle = 87,
    k_EResultTwoFactorCodeMismatch = 88,
    k_EResultTwoFactorActivationCodeMismatch = 89,
    k_EResultAccountAssociatedToMultiplePartners = 90,
    k_EResultNotModified = 91,
    k_EResultNoMobileDevice = 92,
    k_EResultTimeNotSynced = 93,
    k_EResultSmsCodeFailed = 94,
    k_EResultAccountLimitExceeded = 95,
    k_EResultAccountActivityLimitExceeded = 96,
    k_EResultPhoneActivityLimitExceeded = 97,
    k_EResultRefundToWallet = 98,
    k_EResultEmailSendFailure = 99,
    k_EResultNotSettled = 100,
    k_EResultNeedCaptcha = 101,
    k_EResultGSLTDenied = 102,
    k_EResultGSOwnerDenied = 103,
    k_EResultInvalidItemType = 104,
    k_EResultIPBanned = 105,
    k_EResultGSLTExpired = 106,
    k_EResultInsufficientFunds = 107,
    k_EResultTooManyPending = 108,
    k_EResultNoSiteLicensesFound = 109,
    k_EResultWGNetworkSendExceeded = 110,
    k_EResultAccountNotFriends = 111,
    k_EResultLimitedUserAccount = 112,
    k_EResultCantRemoveItem = 113,
    k_EResultAccountDeleted = 114,
    k_EResultExistingUserCancelledLicense = 115,
    k_EResultCommunityCooldown = 116,
    k_EResultNoLauncherSpecified = 117,
    k_EResultMustAgreeToSSA = 118,
    k_EResultLauncherMigrated = 119,
    k_EResultSteamRealmMismatch = 120,
    k_EResultInvalidSignature = 121,
    k_EResultParseFailure = 122,
    k_EResultNoVerifiedPhone = 123,
    k_EResultInsufficientBattery = 124,
    k_EResultChargerRequired = 125,
    k_EResultCachedCredentialInvalid = 126,
    K_EResultPhoneNumberIsVOIP = 127,
    k_EResultNotSupported = 128,
    k_EResultFamilySizeLimitExceeded = 129,
    k_EResultOfflineAppCacheInvalid = 130,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EVoiceResult {
    k_EVoiceResultOK = 0,
    k_EVoiceResultNotInitialized = 1,
    k_EVoiceResultNotRecording = 2,
    k_EVoiceResultNoData = 3,
    k_EVoiceResultBufferTooSmall = 4,
    k_EVoiceResultDataCorrupted = 5,
    k_EVoiceResultRestricted = 6,
    k_EVoiceResultUnsupportedCodec = 7,
    k_EVoiceResultReceiverOutOfDate = 8,
    k_EVoiceResultReceiverDidNotAnswer = 9,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EDenyReason {
    k_EDenyInvalid = 0,
    k_EDenyInvalidVersion = 1,
    k_EDenyGeneric = 2,
    k_EDenyNotLoggedOn = 3,
    k_EDenyNoLicense = 4,
    k_EDenyCheater = 5,
    k_EDenyLoggedInElseWhere = 6,
    k_EDenyUnknownText = 7,
    k_EDenyIncompatibleAnticheat = 8,
    k_EDenyMemoryCorruption = 9,
    k_EDenyIncompatibleSoftware = 10,
    k_EDenySteamConnectionLost = 11,
    k_EDenySteamConnectionError = 12,
    k_EDenySteamResponseTimedOut = 13,
    k_EDenySteamValidationStalled = 14,
    k_EDenySteamOwnerLeftGuestUser = 15,
}
pub type HAuthTicket = uint32;
pub const k_HAuthTicketInvalid: HAuthTicket = 0;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EBeginAuthSessionResult {
    k_EBeginAuthSessionResultOK = 0,
    k_EBeginAuthSessionResultInvalidTicket = 1,
    k_EBeginAuthSessionResultDuplicateRequest = 2,
    k_EBeginAuthSessionResultInvalidVersion = 3,
    k_EBeginAuthSessionResultGameMismatch = 4,
    k_EBeginAuthSessionResultExpiredTicket = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EAuthSessionResponse {
    k_EAuthSessionResponseOK = 0,
    k_EAuthSessionResponseUserNotConnectedToSteam = 1,
    k_EAuthSessionResponseNoLicenseOrExpired = 2,
    k_EAuthSessionResponseVACBanned = 3,
    k_EAuthSessionResponseLoggedInElseWhere = 4,
    k_EAuthSessionResponseVACCheckTimedOut = 5,
    k_EAuthSessionResponseAuthTicketCanceled = 6,
    k_EAuthSessionResponseAuthTicketInvalidAlreadyUsed = 7,
    k_EAuthSessionResponseAuthTicketInvalid = 8,
    k_EAuthSessionResponsePublisherIssuedBan = 9,
    k_EAuthSessionResponseAuthTicketNetworkIdentityFailure = 10,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUserHasLicenseForAppResult {
    k_EUserHasLicenseResultHasLicense = 0,
    k_EUserHasLicenseResultDoesNotHaveLicense = 1,
    k_EUserHasLicenseResultNoAuth = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EAccountType {
    k_EAccountTypeInvalid = 0,
    k_EAccountTypeIndividual = 1,
    k_EAccountTypeMultiseat = 2,
    k_EAccountTypeGameServer = 3,
    k_EAccountTypeAnonGameServer = 4,
    k_EAccountTypePending = 5,
    k_EAccountTypeContentServer = 6,
    k_EAccountTypeClan = 7,
    k_EAccountTypeChat = 8,
    k_EAccountTypeConsoleUser = 9,
    k_EAccountTypeAnonUser = 10,
    k_EAccountTypeMax = 11,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EChatEntryType {
    k_EChatEntryTypeInvalid = 0,
    k_EChatEntryTypeChatMsg = 1,
    k_EChatEntryTypeTyping = 2,
    k_EChatEntryTypeInviteGame = 3,
    k_EChatEntryTypeEmote = 4,
    k_EChatEntryTypeLeftConversation = 6,
    k_EChatEntryTypeEntered = 7,
    k_EChatEntryTypeWasKicked = 8,
    k_EChatEntryTypeWasBanned = 9,
    k_EChatEntryTypeDisconnected = 10,
    k_EChatEntryTypeHistoricalChat = 11,
    k_EChatEntryTypeLinkBlocked = 14,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EChatRoomEnterResponse {
    k_EChatRoomEnterResponseSuccess = 1,
    k_EChatRoomEnterResponseDoesntExist = 2,
    k_EChatRoomEnterResponseNotAllowed = 3,
    k_EChatRoomEnterResponseFull = 4,
    k_EChatRoomEnterResponseError = 5,
    k_EChatRoomEnterResponseBanned = 6,
    k_EChatRoomEnterResponseLimited = 7,
    k_EChatRoomEnterResponseClanDisabled = 8,
    k_EChatRoomEnterResponseCommunityBan = 9,
    k_EChatRoomEnterResponseMemberBlockedYou = 10,
    k_EChatRoomEnterResponseYouBlockedMember = 11,
    k_EChatRoomEnterResponseRatelimitExceeded = 15,
}
pub const k_unSteamAccountIDMask: ::std::os::raw::c_uint = 4294967295;
pub const k_unSteamAccountInstanceMask: ::std::os::raw::c_uint = 1048575;
pub const k_unSteamUserDefaultInstance: ::std::os::raw::c_uint = 1;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EChatSteamIDInstanceFlags {
    k_EChatAccountInstanceMask = 4095,
    k_EChatInstanceFlagClan = 524288,
    k_EChatInstanceFlagLobby = 262144,
    k_EChatInstanceFlagMMSLobby = 131072,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ENotificationPosition {
    k_EPositionInvalid = -1,
    k_EPositionTopLeft = 0,
    k_EPositionTopRight = 1,
    k_EPositionBottomLeft = 2,
    k_EPositionBottomRight = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EBroadcastUploadResult {
    k_EBroadcastUploadResultNone = 0,
    k_EBroadcastUploadResultOK = 1,
    k_EBroadcastUploadResultInitFailed = 2,
    k_EBroadcastUploadResultFrameFailed = 3,
    k_EBroadcastUploadResultTimeout = 4,
    k_EBroadcastUploadResultBandwidthExceeded = 5,
    k_EBroadcastUploadResultLowFPS = 6,
    k_EBroadcastUploadResultMissingKeyFrames = 7,
    k_EBroadcastUploadResultNoConnection = 8,
    k_EBroadcastUploadResultRelayFailed = 9,
    k_EBroadcastUploadResultSettingsChanged = 10,
    k_EBroadcastUploadResultMissingAudio = 11,
    k_EBroadcastUploadResultTooFarBehind = 12,
    k_EBroadcastUploadResultTranscodeBehind = 13,
    k_EBroadcastUploadResultNotAllowedToPlay = 14,
    k_EBroadcastUploadResultBusy = 15,
    k_EBroadcastUploadResultBanned = 16,
    k_EBroadcastUploadResultAlreadyActive = 17,
    k_EBroadcastUploadResultForcedOff = 18,
    k_EBroadcastUploadResultAudioBehind = 19,
    k_EBroadcastUploadResultShutdown = 20,
    k_EBroadcastUploadResultDisconnect = 21,
    k_EBroadcastUploadResultVideoInitFailed = 22,
    k_EBroadcastUploadResultAudioInitFailed = 23,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EMarketNotAllowedReasonFlags {
    k_EMarketNotAllowedReason_None = 0,
    k_EMarketNotAllowedReason_TemporaryFailure = 1,
    k_EMarketNotAllowedReason_AccountDisabled = 2,
    k_EMarketNotAllowedReason_AccountLockedDown = 4,
    k_EMarketNotAllowedReason_AccountLimited = 8,
    k_EMarketNotAllowedReason_TradeBanned = 16,
    k_EMarketNotAllowedReason_AccountNotTrusted = 32,
    k_EMarketNotAllowedReason_SteamGuardNotEnabled = 64,
    k_EMarketNotAllowedReason_SteamGuardOnlyRecentlyEnabled = 128,
    k_EMarketNotAllowedReason_RecentPasswordReset = 256,
    k_EMarketNotAllowedReason_NewPaymentMethod = 512,
    k_EMarketNotAllowedReason_InvalidCookie = 1024,
    k_EMarketNotAllowedReason_UsingNewDevice = 2048,
    k_EMarketNotAllowedReason_RecentSelfRefund = 4096,
    k_EMarketNotAllowedReason_NewPaymentMethodCannotBeVerified = 8192,
    k_EMarketNotAllowedReason_NoRecentPurchases = 16384,
    k_EMarketNotAllowedReason_AcceptedWalletGift = 32768,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EDurationControlProgress {
    k_EDurationControlProgress_Full = 0,
    k_EDurationControlProgress_Half = 1,
    k_EDurationControlProgress_None = 2,
    k_EDurationControl_ExitSoon_3h = 3,
    k_EDurationControl_ExitSoon_5h = 4,
    k_EDurationControl_ExitSoon_Night = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EDurationControlNotification {
    k_EDurationControlNotification_None = 0,
    k_EDurationControlNotification_1Hour = 1,
    k_EDurationControlNotification_3Hours = 2,
    k_EDurationControlNotification_HalfProgress = 3,
    k_EDurationControlNotification_NoProgress = 4,
    k_EDurationControlNotification_ExitSoon_3h = 5,
    k_EDurationControlNotification_ExitSoon_5h = 6,
    k_EDurationControlNotification_ExitSoon_Night = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EDurationControlOnlineState {
    k_EDurationControlOnlineState_Invalid = 0,
    k_EDurationControlOnlineState_Offline = 1,
    k_EDurationControlOnlineState_Online = 2,
    k_EDurationControlOnlineState_OnlineHighPri = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EBetaBranchFlags {
    k_EBetaBranch_None = 0,
    k_EBetaBranch_Default = 1,
    k_EBetaBranch_Available = 2,
    k_EBetaBranch_Private = 4,
    k_EBetaBranch_Selected = 8,
    k_EBetaBranch_Installed = 16,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CSteamID {
    pub m_steamid: CSteamID_SteamID_t,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union CSteamID_SteamID_t {
    pub m_comp: CSteamID_SteamID_t_SteamIDComponent_t,
    pub m_unAll64Bits: uint64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CSteamID_SteamID_t_SteamIDComponent_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of CSteamID_SteamID_t_SteamIDComponent_t",
    ][::std::mem::size_of::<CSteamID_SteamID_t_SteamIDComponent_t>() - 8usize];
    [
        "Alignment of CSteamID_SteamID_t_SteamIDComponent_t",
    ][::std::mem::align_of::<CSteamID_SteamID_t_SteamIDComponent_t>() - 1usize];
};
impl CSteamID_SteamID_t_SteamIDComponent_t {
    #[inline]
    pub fn m_unAccountID(&self) -> uint32 {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_m_unAccountID(&mut self, val: uint32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_unAccountID_raw(this: *const Self) -> uint32 {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_unAccountID_raw(this: *mut Self, val: uint32) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn m_unAccountInstance(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 20u8) as u32) }
    }
    #[inline]
    pub fn set_m_unAccountInstance(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 20u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_unAccountInstance_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 32usize, 20u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_unAccountInstance_raw(
        this: *mut Self,
        val: ::std::os::raw::c_uint,
    ) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                20u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn m_EAccountType(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(52usize, 4u8) as u32) }
    }
    #[inline]
    pub fn set_m_EAccountType(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(52usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_EAccountType_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 52usize, 4u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_EAccountType_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                52usize,
                4u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn m_EUniverse(&self) -> EUniverse {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(56usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_m_EUniverse(&mut self, val: EUniverse) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(56usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_EUniverse_raw(this: *const Self) -> EUniverse {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 56usize, 8u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_EUniverse_raw(this: *mut Self, val: EUniverse) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                56usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        m_unAccountID: uint32,
        m_unAccountInstance: ::std::os::raw::c_uint,
        m_EAccountType: ::std::os::raw::c_uint,
        m_EUniverse: EUniverse,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                32u8,
                {
                    let m_unAccountID: u32 = unsafe {
                        ::std::mem::transmute(m_unAccountID)
                    };
                    m_unAccountID as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                32usize,
                20u8,
                {
                    let m_unAccountInstance: u32 = unsafe {
                        ::std::mem::transmute(m_unAccountInstance)
                    };
                    m_unAccountInstance as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                52usize,
                4u8,
                {
                    let m_EAccountType: u32 = unsafe {
                        ::std::mem::transmute(m_EAccountType)
                    };
                    m_EAccountType as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                56usize,
                8u8,
                {
                    let m_EUniverse: u32 = unsafe { ::std::mem::transmute(m_EUniverse) };
                    m_EUniverse as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CSteamID_SteamID_t"][::std::mem::size_of::<CSteamID_SteamID_t>() - 8usize];
    [
        "Alignment of CSteamID_SteamID_t",
    ][::std::mem::align_of::<CSteamID_SteamID_t>() - 1usize];
    [
        "Offset of field: CSteamID_SteamID_t::m_comp",
    ][::std::mem::offset_of!(CSteamID_SteamID_t, m_comp) - 0usize];
    [
        "Offset of field: CSteamID_SteamID_t::m_unAll64Bits",
    ][::std::mem::offset_of!(CSteamID_SteamID_t, m_unAll64Bits) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CSteamID"][::std::mem::size_of::<CSteamID>() - 8usize];
    ["Alignment of CSteamID"][::std::mem::align_of::<CSteamID>() - 1usize];
    [
        "Offset of field: CSteamID::m_steamid",
    ][::std::mem::offset_of!(CSteamID, m_steamid) - 0usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK8CSteamID6RenderEv"]
    pub fn CSteamID_Render(this: *const CSteamID) -> *const ::std::os::raw::c_char;
    #[link_name = "\u{1}_ZN8CSteamID6RenderEy"]
    pub fn CSteamID_Render1(ulSteamID: uint64) -> *const ::std::os::raw::c_char;
    #[link_name = "\u{1}_ZN8CSteamID13SetFromStringEPKc9EUniverse"]
    pub fn CSteamID_SetFromString(
        this: *mut CSteamID,
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    );
    #[link_name = "\u{1}_ZN8CSteamID19SetFromStringStrictEPKc9EUniverse"]
    pub fn CSteamID_SetFromStringStrict(
        this: *mut CSteamID,
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    ) -> bool;
    #[link_name = "\u{1}_ZNK8CSteamID21BValidExternalSteamIDEv"]
    pub fn CSteamID_BValidExternalSteamID(this: *const CSteamID) -> bool;
    #[link_name = "\u{1}_ZN8CSteamIDC1EPKc9EUniverse"]
    pub fn CSteamID_CSteamID(
        this: *mut CSteamID,
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    );
}
impl CSteamID {
    #[inline]
    pub unsafe fn Render(&self) -> *const ::std::os::raw::c_char {
        CSteamID_Render(self)
    }
    #[inline]
    pub unsafe fn Render1(ulSteamID: uint64) -> *const ::std::os::raw::c_char {
        CSteamID_Render1(ulSteamID)
    }
    #[inline]
    pub unsafe fn SetFromString(
        &mut self,
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    ) {
        CSteamID_SetFromString(self, pchSteamID, eDefaultUniverse)
    }
    #[inline]
    pub unsafe fn SetFromStringStrict(
        &mut self,
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    ) -> bool {
        CSteamID_SetFromStringStrict(self, pchSteamID, eDefaultUniverse)
    }
    #[inline]
    pub unsafe fn BValidExternalSteamID(&self) -> bool {
        CSteamID_BValidExternalSteamID(self)
    }
    #[inline]
    pub unsafe fn new(
        pchSteamID: *const ::std::os::raw::c_char,
        eDefaultUniverse: EUniverse,
    ) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CSteamID_CSteamID(__bindgen_tmp.as_mut_ptr(), pchSteamID, eDefaultUniverse);
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CGameID {
    pub __bindgen_anon_1: CGameID__bindgen_ty_1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CGameID_EGameIDType {
    k_EGameIDTypeApp = 0,
    k_EGameIDTypeGameMod = 1,
    k_EGameIDTypeShortcut = 2,
    k_EGameIDTypeP2P = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CGameID_GameID_t {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 8usize]>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CGameID_GameID_t"][::std::mem::size_of::<CGameID_GameID_t>() - 8usize];
    [
        "Alignment of CGameID_GameID_t",
    ][::std::mem::align_of::<CGameID_GameID_t>() - 1usize];
};
impl CGameID_GameID_t {
    #[inline]
    pub fn m_nAppID(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 24u8) as u32) }
    }
    #[inline]
    pub fn set_m_nAppID(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 24u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_nAppID_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 0usize, 24u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_nAppID_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                0usize,
                24u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn m_nType(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(24usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_m_nType(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(24usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_nType_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 24usize, 8u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_nType_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                24usize,
                8u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn m_nModID(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(32usize, 32u8) as u32) }
    }
    #[inline]
    pub fn set_m_nModID(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(32usize, 32u8, val as u64)
        }
    }
    #[inline]
    pub unsafe fn m_nModID_raw(this: *const Self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(
                <__BindgenBitfieldUnit<
                    [u8; 8usize],
                >>::raw_get(::std::ptr::addr_of!((*this)._bitfield_1), 32usize, 32u8)
                    as u32,
            )
        }
    }
    #[inline]
    pub unsafe fn set_m_nModID_raw(this: *mut Self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            <__BindgenBitfieldUnit<
                [u8; 8usize],
            >>::raw_set(
                ::std::ptr::addr_of_mut!((*this)._bitfield_1),
                32usize,
                32u8,
                val as u64,
            )
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        m_nAppID: ::std::os::raw::c_uint,
        m_nType: ::std::os::raw::c_uint,
        m_nModID: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 8usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 8usize]> = Default::default();
        __bindgen_bitfield_unit
            .set(
                0usize,
                24u8,
                {
                    let m_nAppID: u32 = unsafe { ::std::mem::transmute(m_nAppID) };
                    m_nAppID as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                24usize,
                8u8,
                {
                    let m_nType: u32 = unsafe { ::std::mem::transmute(m_nType) };
                    m_nType as u64
                },
            );
        __bindgen_bitfield_unit
            .set(
                32usize,
                32u8,
                {
                    let m_nModID: u32 = unsafe { ::std::mem::transmute(m_nModID) };
                    m_nModID as u64
                },
            );
        __bindgen_bitfield_unit
    }
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union CGameID__bindgen_ty_1 {
    pub m_ulGameID: uint64,
    pub m_gameID: CGameID_GameID_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of CGameID__bindgen_ty_1",
    ][::std::mem::size_of::<CGameID__bindgen_ty_1>() - 8usize];
    [
        "Alignment of CGameID__bindgen_ty_1",
    ][::std::mem::align_of::<CGameID__bindgen_ty_1>() - 1usize];
    [
        "Offset of field: CGameID__bindgen_ty_1::m_ulGameID",
    ][::std::mem::offset_of!(CGameID__bindgen_ty_1, m_ulGameID) - 0usize];
    [
        "Offset of field: CGameID__bindgen_ty_1::m_gameID",
    ][::std::mem::offset_of!(CGameID__bindgen_ty_1, m_gameID) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CGameID"][::std::mem::size_of::<CGameID>() - 8usize];
    ["Alignment of CGameID"][::std::mem::align_of::<CGameID>() - 1usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZNK7CGameID6RenderEv"]
    pub fn CGameID_Render(this: *const CGameID) -> *const ::std::os::raw::c_char;
    #[link_name = "\u{1}_ZN7CGameID6RenderEy"]
    pub fn CGameID_Render1(ulGameID: uint64) -> *const ::std::os::raw::c_char;
    #[link_name = "\u{1}_ZN7CGameIDC1EPKc"]
    pub fn CGameID_CGameID(this: *mut CGameID, pchGameID: *const ::std::os::raw::c_char);
}
impl CGameID {
    #[inline]
    pub unsafe fn Render(&self) -> *const ::std::os::raw::c_char {
        CGameID_Render(self)
    }
    #[inline]
    pub unsafe fn Render1(ulGameID: uint64) -> *const ::std::os::raw::c_char {
        CGameID_Render1(ulGameID)
    }
    #[inline]
    pub unsafe fn new(pchGameID: *const ::std::os::raw::c_char) -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        CGameID_CGameID(__bindgen_tmp.as_mut_ptr(), pchGameID);
        __bindgen_tmp.assume_init()
    }
}
pub const k_cchGameExtraInfoMax: ::std::os::raw::c_int = 64;
pub type PFNPreMinidumpCallback = ::std::option::Option<
    unsafe extern "C" fn(context: *mut ::std::os::raw::c_void),
>;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EGameSearchErrorCode_t {
    k_EGameSearchErrorCode_OK = 1,
    k_EGameSearchErrorCode_Failed_Search_Already_In_Progress = 2,
    k_EGameSearchErrorCode_Failed_No_Search_In_Progress = 3,
    k_EGameSearchErrorCode_Failed_Not_Lobby_Leader = 4,
    k_EGameSearchErrorCode_Failed_No_Host_Available = 5,
    k_EGameSearchErrorCode_Failed_Search_Params_Invalid = 6,
    k_EGameSearchErrorCode_Failed_Offline = 7,
    k_EGameSearchErrorCode_Failed_NotAuthorized = 8,
    k_EGameSearchErrorCode_Failed_Unknown_Error = 9,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EPlayerResult_t {
    k_EPlayerResultFailedToConnect = 1,
    k_EPlayerResultAbandoned = 2,
    k_EPlayerResultKicked = 3,
    k_EPlayerResultIncomplete = 4,
    k_EPlayerResultCompleted = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamIPv6ConnectivityProtocol {
    k_ESteamIPv6ConnectivityProtocol_Invalid = 0,
    k_ESteamIPv6ConnectivityProtocol_HTTP = 1,
    k_ESteamIPv6ConnectivityProtocol_UDP = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamIPv6ConnectivityState {
    k_ESteamIPv6ConnectivityState_Unknown = 0,
    k_ESteamIPv6ConnectivityState_Good = 1,
    k_ESteamIPv6ConnectivityState_Bad = 2,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct ValvePackingSentinel_t {
    pub m_u32: uint32,
    pub m_u64: uint64,
    pub m_u16: uint16,
    pub m_d: f64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ValvePackingSentinel_t",
    ][::std::mem::size_of::<ValvePackingSentinel_t>() - 24usize];
    [
        "Alignment of ValvePackingSentinel_t",
    ][::std::mem::align_of::<ValvePackingSentinel_t>() - 4usize];
    [
        "Offset of field: ValvePackingSentinel_t::m_u32",
    ][::std::mem::offset_of!(ValvePackingSentinel_t, m_u32) - 0usize];
    [
        "Offset of field: ValvePackingSentinel_t::m_u64",
    ][::std::mem::offset_of!(ValvePackingSentinel_t, m_u64) - 4usize];
    [
        "Offset of field: ValvePackingSentinel_t::m_u16",
    ][::std::mem::offset_of!(ValvePackingSentinel_t, m_u16) - 12usize];
    [
        "Offset of field: ValvePackingSentinel_t::m_d",
    ][::std::mem::offset_of!(ValvePackingSentinel_t, m_d) - 16usize];
};
pub type compile_time_assert_type = [::std::os::raw::c_char; 1usize];
pub type HSteamPipe = int32;
pub type HSteamUser = int32;
pub const k_cchMaxSteamErrMsg: ::std::os::raw::c_int = 1024;
pub type SteamErrMsg = [::std::os::raw::c_char; 1024usize];
pub type SteamAPIWarningMessageHook_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
    ),
>;
pub type SteamAPI_CheckCallbackRegistered_t = ::std::option::Option<
    unsafe extern "C" fn(iCallbackNum: ::std::os::raw::c_int) -> uint32,
>;
unsafe extern "C" {
    pub fn SteamAPI_RunCallbacks();
    pub fn SteamGameServer_RunCallbacks();
}
#[repr(C)]
pub struct CCallbackBase__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CCallbackBase {
    pub vtable_: *const CCallbackBase__bindgen_vtable,
    pub m_nCallbackFlags: uint8,
    pub m_iCallback: ::std::os::raw::c_int,
}
pub const CCallbackBase_k_ECallbackFlagsRegistered: CCallbackBase__bindgen_ty_1 = CCallbackBase__bindgen_ty_1::k_ECallbackFlagsRegistered;
pub const CCallbackBase_k_ECallbackFlagsGameServer: CCallbackBase__bindgen_ty_1 = CCallbackBase__bindgen_ty_1::k_ECallbackFlagsGameServer;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CCallbackBase__bindgen_ty_1 {
    k_ECallbackFlagsRegistered = 1,
    k_ECallbackFlagsGameServer = 2,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CCallbackBase"][::std::mem::size_of::<CCallbackBase>() - 16usize];
    ["Alignment of CCallbackBase"][::std::mem::align_of::<CCallbackBase>() - 8usize];
    [
        "Offset of field: CCallbackBase::m_nCallbackFlags",
    ][::std::mem::offset_of!(CCallbackBase, m_nCallbackFlags) - 8usize];
    [
        "Offset of field: CCallbackBase::m_iCallback",
    ][::std::mem::offset_of!(CCallbackBase, m_iCallback) - 12usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct CCallResult<T, P> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<T>>,
    pub _phantom_1: ::std::marker::PhantomData<::std::cell::UnsafeCell<P>>,
    pub _base: CCallbackBase,
    pub m_hAPICall: SteamAPICall_t,
    pub m_pObj: *mut T,
    pub m_Func: CCallResult_func_t<P>,
}
pub type CCallResult_func_t<P> = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut P, arg2: bool),
>;
pub type CCallback_func_t<P> = ::std::option::Option<unsafe extern "C" fn(arg1: *mut P)>;
unsafe extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn __memcmpeq(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn memchr(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn rawmemchr(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_void;
    pub fn memrchr(
        __s: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> usize;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __locale_struct"][::std::mem::size_of::<__locale_struct>() - 232usize];
    ["Alignment of __locale_struct"][::std::mem::align_of::<__locale_struct>() - 8usize];
    [
        "Offset of field: __locale_struct::__locales",
    ][::std::mem::offset_of!(__locale_struct, __locales) - 0usize];
    [
        "Offset of field: __locale_struct::__ctype_b",
    ][::std::mem::offset_of!(__locale_struct, __ctype_b) - 104usize];
    [
        "Offset of field: __locale_struct::__ctype_tolower",
    ][::std::mem::offset_of!(__locale_struct, __ctype_tolower) - 112usize];
    [
        "Offset of field: __locale_struct::__ctype_toupper",
    ][::std::mem::offset_of!(__locale_struct, __ctype_toupper) - 120usize];
    [
        "Offset of field: __locale_struct::__names",
    ][::std::mem::offset_of!(__locale_struct, __names) - 128usize];
};
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
unsafe extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strchr(
        __s: *mut ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strrchr(
        __s: *mut ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strchrnul(
        __s: *mut ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> usize;
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> usize;
    pub fn strpbrk(
        __s: *mut ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strstr(
        __haystack: *mut ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strcasestr(
        __haystack: *mut ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn memmem(
        __haystack: *const ::std::os::raw::c_void,
        __haystacklen: usize,
        __needle: *const ::std::os::raw::c_void,
        __needlelen: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn __mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn mempcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> usize;
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strerrordesc_np(
        __err: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn strerrorname_np(
        __err: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    pub fn sigabbrev_np(__sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn sigdescr_np(__sig: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
    pub fn strverscmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn strfry(__string: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn memfrob(
        __s: *mut ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    pub fn basename(
        __filename: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn SteamAPI_GetHSteamPipe() -> HSteamPipe;
    pub fn SteamAPI_GetHSteamUser() -> HSteamUser;
    pub fn SteamGameServer_GetHSteamPipe() -> HSteamPipe;
    pub fn SteamGameServer_GetHSteamUser() -> HSteamUser;
    pub fn SteamInternal_ContextInit(
        pContextInitData: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
    pub fn SteamInternal_CreateInterface(
        ver: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn SteamInternal_FindOrCreateUserInterface(
        hSteamUser: HSteamUser,
        pszVersion: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn SteamInternal_FindOrCreateGameServerInterface(
        hSteamUser: HSteamUser,
        pszVersion: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn SteamAPI_RegisterCallback(
        pCallback: *mut CCallbackBase,
        iCallback: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_UnregisterCallback(pCallback: *mut CCallbackBase);
    pub fn SteamAPI_RegisterCallResult(
        pCallback: *mut CCallbackBase,
        hAPICall: SteamAPICall_t,
    );
    pub fn SteamAPI_UnregisterCallResult(
        pCallback: *mut CCallbackBase,
        hAPICall: SteamAPICall_t,
    );
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CallbackMsg_t {
    pub m_hSteamUser: HSteamUser,
    pub m_iCallback: ::std::os::raw::c_int,
    pub m_pubParam: *mut uint8,
    pub m_cubParam: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of CallbackMsg_t"][::std::mem::size_of::<CallbackMsg_t>() - 20usize];
    ["Alignment of CallbackMsg_t"][::std::mem::align_of::<CallbackMsg_t>() - 4usize];
    [
        "Offset of field: CallbackMsg_t::m_hSteamUser",
    ][::std::mem::offset_of!(CallbackMsg_t, m_hSteamUser) - 0usize];
    [
        "Offset of field: CallbackMsg_t::m_iCallback",
    ][::std::mem::offset_of!(CallbackMsg_t, m_iCallback) - 4usize];
    [
        "Offset of field: CallbackMsg_t::m_pubParam",
    ][::std::mem::offset_of!(CallbackMsg_t, m_pubParam) - 8usize];
    [
        "Offset of field: CallbackMsg_t::m_cubParam",
    ][::std::mem::offset_of!(CallbackMsg_t, m_cubParam) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamContentServer {
    _unused: [u8; 0],
}
pub const k_iSteamUserCallbacks: _bindgen_ty_1 = _bindgen_ty_1::k_iSteamUserCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_1 {
    k_iSteamUserCallbacks = 100,
}
pub const k_iSteamGameServerCallbacks: _bindgen_ty_2 = _bindgen_ty_2::k_iSteamGameServerCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_2 {
    k_iSteamGameServerCallbacks = 200,
}
pub const k_iSteamFriendsCallbacks: _bindgen_ty_3 = _bindgen_ty_3::k_iSteamFriendsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_3 {
    k_iSteamFriendsCallbacks = 300,
}
pub const k_iSteamBillingCallbacks: _bindgen_ty_4 = _bindgen_ty_4::k_iSteamBillingCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_4 {
    k_iSteamBillingCallbacks = 400,
}
pub const k_iSteamMatchmakingCallbacks: _bindgen_ty_5 = _bindgen_ty_5::k_iSteamMatchmakingCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_5 {
    k_iSteamMatchmakingCallbacks = 500,
}
pub const k_iSteamContentServerCallbacks: _bindgen_ty_6 = _bindgen_ty_6::k_iSteamContentServerCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_6 {
    k_iSteamContentServerCallbacks = 600,
}
pub const k_iSteamUtilsCallbacks: _bindgen_ty_7 = _bindgen_ty_7::k_iSteamUtilsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_7 {
    k_iSteamUtilsCallbacks = 700,
}
pub const k_iSteamAppsCallbacks: _bindgen_ty_8 = _bindgen_ty_8::k_iSteamAppsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_8 {
    k_iSteamAppsCallbacks = 1000,
}
pub const k_iSteamUserStatsCallbacks: _bindgen_ty_9 = _bindgen_ty_9::k_iSteamUserStatsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_9 {
    k_iSteamUserStatsCallbacks = 1100,
}
pub const k_iSteamNetworkingCallbacks: _bindgen_ty_10 = _bindgen_ty_10::k_iSteamNetworkingCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_10 {
    k_iSteamNetworkingCallbacks = 1200,
}
pub const k_iSteamNetworkingSocketsCallbacks: _bindgen_ty_11 = _bindgen_ty_11::k_iSteamNetworkingSocketsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_11 {
    k_iSteamNetworkingSocketsCallbacks = 1220,
}
pub const k_iSteamNetworkingMessagesCallbacks: _bindgen_ty_12 = _bindgen_ty_12::k_iSteamNetworkingMessagesCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_12 {
    k_iSteamNetworkingMessagesCallbacks = 1250,
}
pub const k_iSteamNetworkingUtilsCallbacks: _bindgen_ty_13 = _bindgen_ty_13::k_iSteamNetworkingUtilsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_13 {
    k_iSteamNetworkingUtilsCallbacks = 1280,
}
pub const k_iSteamRemoteStorageCallbacks: _bindgen_ty_14 = _bindgen_ty_14::k_iSteamRemoteStorageCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_14 {
    k_iSteamRemoteStorageCallbacks = 1300,
}
pub const k_iSteamGameServerItemsCallbacks: _bindgen_ty_15 = _bindgen_ty_15::k_iSteamGameServerItemsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_15 {
    k_iSteamGameServerItemsCallbacks = 1500,
}
pub const k_iSteamGameCoordinatorCallbacks: _bindgen_ty_16 = _bindgen_ty_16::k_iSteamGameCoordinatorCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_16 {
    k_iSteamGameCoordinatorCallbacks = 1700,
}
pub const k_iSteamGameServerStatsCallbacks: _bindgen_ty_17 = _bindgen_ty_17::k_iSteamGameServerStatsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_17 {
    k_iSteamGameServerStatsCallbacks = 1800,
}
pub const k_iSteam2AsyncCallbacks: _bindgen_ty_18 = _bindgen_ty_18::k_iSteam2AsyncCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_18 {
    k_iSteam2AsyncCallbacks = 1900,
}
pub const k_iSteamGameStatsCallbacks: _bindgen_ty_19 = _bindgen_ty_19::k_iSteamGameStatsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_19 {
    k_iSteamGameStatsCallbacks = 2000,
}
pub const k_iSteamHTTPCallbacks: _bindgen_ty_20 = _bindgen_ty_20::k_iSteamHTTPCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_20 {
    k_iSteamHTTPCallbacks = 2100,
}
pub const k_iSteamScreenshotsCallbacks: _bindgen_ty_21 = _bindgen_ty_21::k_iSteamScreenshotsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_21 {
    k_iSteamScreenshotsCallbacks = 2300,
}
pub const k_iSteamStreamLauncherCallbacks: _bindgen_ty_22 = _bindgen_ty_22::k_iSteamStreamLauncherCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_22 {
    k_iSteamStreamLauncherCallbacks = 2600,
}
pub const k_iSteamControllerCallbacks: _bindgen_ty_23 = _bindgen_ty_23::k_iSteamControllerCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_23 {
    k_iSteamControllerCallbacks = 2800,
}
pub const k_iSteamUGCCallbacks: _bindgen_ty_24 = _bindgen_ty_24::k_iSteamUGCCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_24 {
    k_iSteamUGCCallbacks = 3400,
}
pub const k_iSteamStreamClientCallbacks: _bindgen_ty_25 = _bindgen_ty_25::k_iSteamStreamClientCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_25 {
    k_iSteamStreamClientCallbacks = 3500,
}
pub const k_iSteamMusicCallbacks: _bindgen_ty_26 = _bindgen_ty_26::k_iSteamMusicCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_26 {
    k_iSteamMusicCallbacks = 4000,
}
pub const k_iSteamMusicRemoteCallbacks: _bindgen_ty_27 = _bindgen_ty_27::k_iSteamMusicRemoteCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_27 {
    k_iSteamMusicRemoteCallbacks = 4100,
}
pub const k_iSteamGameNotificationCallbacks: _bindgen_ty_28 = _bindgen_ty_28::k_iSteamGameNotificationCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_28 {
    k_iSteamGameNotificationCallbacks = 4400,
}
pub const k_iSteamHTMLSurfaceCallbacks: _bindgen_ty_29 = _bindgen_ty_29::k_iSteamHTMLSurfaceCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_29 {
    k_iSteamHTMLSurfaceCallbacks = 4500,
}
pub const k_iSteamVideoCallbacks: _bindgen_ty_30 = _bindgen_ty_30::k_iSteamVideoCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_30 {
    k_iSteamVideoCallbacks = 4600,
}
pub const k_iSteamInventoryCallbacks: _bindgen_ty_31 = _bindgen_ty_31::k_iSteamInventoryCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_31 {
    k_iSteamInventoryCallbacks = 4700,
}
pub const k_ISteamParentalSettingsCallbacks: _bindgen_ty_32 = _bindgen_ty_32::k_ISteamParentalSettingsCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_32 {
    k_ISteamParentalSettingsCallbacks = 5000,
}
pub const k_iSteamGameSearchCallbacks: _bindgen_ty_33 = _bindgen_ty_33::k_iSteamGameSearchCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_33 {
    k_iSteamGameSearchCallbacks = 5200,
}
pub const k_iSteamPartiesCallbacks: _bindgen_ty_34 = _bindgen_ty_34::k_iSteamPartiesCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_34 {
    k_iSteamPartiesCallbacks = 5300,
}
pub const k_iSteamSTARCallbacks: _bindgen_ty_35 = _bindgen_ty_35::k_iSteamSTARCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_35 {
    k_iSteamSTARCallbacks = 5500,
}
pub const k_iSteamRemotePlayCallbacks: _bindgen_ty_36 = _bindgen_ty_36::k_iSteamRemotePlayCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_36 {
    k_iSteamRemotePlayCallbacks = 5700,
}
pub const k_iSteamChatCallbacks: _bindgen_ty_37 = _bindgen_ty_37::k_iSteamChatCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_37 {
    k_iSteamChatCallbacks = 5900,
}
pub const k_iSteamTimelineCallbacks: _bindgen_ty_38 = _bindgen_ty_38::k_iSteamTimelineCallbacks;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_38 {
    k_iSteamTimelineCallbacks = 6000,
}
#[repr(C)]
pub struct ISteamClient__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamClient {
    pub vtable_: *const ISteamClient__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamClient"][::std::mem::size_of::<ISteamClient>() - 8usize];
    ["Alignment of ISteamClient"][::std::mem::align_of::<ISteamClient>() - 8usize];
};
#[repr(C)]
pub struct ISteamUser__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamUser {
    pub vtable_: *const ISteamUser__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamUser"][::std::mem::size_of::<ISteamUser>() - 8usize];
    ["Alignment of ISteamUser"][::std::mem::align_of::<ISteamUser>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamServersConnected_t {
    pub _address: u8,
}
pub const SteamServersConnected_t_k_iCallback: SteamServersConnected_t__bindgen_ty_1 = SteamServersConnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamServersConnected_t__bindgen_ty_1 {
    k_iCallback = 101,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamServersConnected_t",
    ][::std::mem::size_of::<SteamServersConnected_t>() - 1usize];
    [
        "Alignment of SteamServersConnected_t",
    ][::std::mem::align_of::<SteamServersConnected_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamServerConnectFailure_t {
    pub m_eResult: EResult,
    pub m_bStillRetrying: bool,
}
pub const SteamServerConnectFailure_t_k_iCallback: SteamServerConnectFailure_t__bindgen_ty_1 = SteamServerConnectFailure_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamServerConnectFailure_t__bindgen_ty_1 {
    k_iCallback = 102,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamServerConnectFailure_t",
    ][::std::mem::size_of::<SteamServerConnectFailure_t>() - 8usize];
    [
        "Alignment of SteamServerConnectFailure_t",
    ][::std::mem::align_of::<SteamServerConnectFailure_t>() - 4usize];
    [
        "Offset of field: SteamServerConnectFailure_t::m_eResult",
    ][::std::mem::offset_of!(SteamServerConnectFailure_t, m_eResult) - 0usize];
    [
        "Offset of field: SteamServerConnectFailure_t::m_bStillRetrying",
    ][::std::mem::offset_of!(SteamServerConnectFailure_t, m_bStillRetrying) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamServersDisconnected_t {
    pub m_eResult: EResult,
}
pub const SteamServersDisconnected_t_k_iCallback: SteamServersDisconnected_t__bindgen_ty_1 = SteamServersDisconnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamServersDisconnected_t__bindgen_ty_1 {
    k_iCallback = 103,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamServersDisconnected_t",
    ][::std::mem::size_of::<SteamServersDisconnected_t>() - 4usize];
    [
        "Alignment of SteamServersDisconnected_t",
    ][::std::mem::align_of::<SteamServersDisconnected_t>() - 4usize];
    [
        "Offset of field: SteamServersDisconnected_t::m_eResult",
    ][::std::mem::offset_of!(SteamServersDisconnected_t, m_eResult) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ClientGameServerDeny_t {
    pub m_uAppID: uint32,
    pub m_unGameServerIP: uint32,
    pub m_usGameServerPort: uint16,
    pub m_bSecure: uint16,
    pub m_uReason: uint32,
}
pub const ClientGameServerDeny_t_k_iCallback: ClientGameServerDeny_t__bindgen_ty_1 = ClientGameServerDeny_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ClientGameServerDeny_t__bindgen_ty_1 {
    k_iCallback = 113,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ClientGameServerDeny_t",
    ][::std::mem::size_of::<ClientGameServerDeny_t>() - 16usize];
    [
        "Alignment of ClientGameServerDeny_t",
    ][::std::mem::align_of::<ClientGameServerDeny_t>() - 4usize];
    [
        "Offset of field: ClientGameServerDeny_t::m_uAppID",
    ][::std::mem::offset_of!(ClientGameServerDeny_t, m_uAppID) - 0usize];
    [
        "Offset of field: ClientGameServerDeny_t::m_unGameServerIP",
    ][::std::mem::offset_of!(ClientGameServerDeny_t, m_unGameServerIP) - 4usize];
    [
        "Offset of field: ClientGameServerDeny_t::m_usGameServerPort",
    ][::std::mem::offset_of!(ClientGameServerDeny_t, m_usGameServerPort) - 8usize];
    [
        "Offset of field: ClientGameServerDeny_t::m_bSecure",
    ][::std::mem::offset_of!(ClientGameServerDeny_t, m_bSecure) - 10usize];
    [
        "Offset of field: ClientGameServerDeny_t::m_uReason",
    ][::std::mem::offset_of!(ClientGameServerDeny_t, m_uReason) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPCFailure_t {
    pub m_eFailureType: uint8,
}
pub const IPCFailure_t_k_iCallback: IPCFailure_t__bindgen_ty_1 = IPCFailure_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IPCFailure_t__bindgen_ty_1 {
    k_iCallback = 117,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IPCFailure_t_EFailureType {
    k_EFailureFlushedCallbackQueue = 0,
    k_EFailurePipeFail = 1,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IPCFailure_t"][::std::mem::size_of::<IPCFailure_t>() - 1usize];
    ["Alignment of IPCFailure_t"][::std::mem::align_of::<IPCFailure_t>() - 1usize];
    [
        "Offset of field: IPCFailure_t::m_eFailureType",
    ][::std::mem::offset_of!(IPCFailure_t, m_eFailureType) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LicensesUpdated_t {
    pub _address: u8,
}
pub const LicensesUpdated_t_k_iCallback: LicensesUpdated_t__bindgen_ty_1 = LicensesUpdated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LicensesUpdated_t__bindgen_ty_1 {
    k_iCallback = 125,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LicensesUpdated_t"][::std::mem::size_of::<LicensesUpdated_t>() - 1usize];
    [
        "Alignment of LicensesUpdated_t",
    ][::std::mem::align_of::<LicensesUpdated_t>() - 1usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ValidateAuthTicketResponse_t {
    pub m_SteamID: CSteamID,
    pub m_eAuthSessionResponse: EAuthSessionResponse,
    pub m_OwnerSteamID: CSteamID,
}
pub const ValidateAuthTicketResponse_t_k_iCallback: ValidateAuthTicketResponse_t__bindgen_ty_1 = ValidateAuthTicketResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ValidateAuthTicketResponse_t__bindgen_ty_1 {
    k_iCallback = 143,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ValidateAuthTicketResponse_t",
    ][::std::mem::size_of::<ValidateAuthTicketResponse_t>() - 20usize];
    [
        "Alignment of ValidateAuthTicketResponse_t",
    ][::std::mem::align_of::<ValidateAuthTicketResponse_t>() - 4usize];
    [
        "Offset of field: ValidateAuthTicketResponse_t::m_SteamID",
    ][::std::mem::offset_of!(ValidateAuthTicketResponse_t, m_SteamID) - 0usize];
    [
        "Offset of field: ValidateAuthTicketResponse_t::m_eAuthSessionResponse",
    ][::std::mem::offset_of!(ValidateAuthTicketResponse_t, m_eAuthSessionResponse)
        - 8usize];
    [
        "Offset of field: ValidateAuthTicketResponse_t::m_OwnerSteamID",
    ][::std::mem::offset_of!(ValidateAuthTicketResponse_t, m_OwnerSteamID) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct MicroTxnAuthorizationResponse_t {
    pub m_unAppID: uint32,
    pub m_ulOrderID: uint64,
    pub m_bAuthorized: uint8,
}
pub const MicroTxnAuthorizationResponse_t_k_iCallback: MicroTxnAuthorizationResponse_t__bindgen_ty_1 = MicroTxnAuthorizationResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MicroTxnAuthorizationResponse_t__bindgen_ty_1 {
    k_iCallback = 152,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MicroTxnAuthorizationResponse_t",
    ][::std::mem::size_of::<MicroTxnAuthorizationResponse_t>() - 16usize];
    [
        "Alignment of MicroTxnAuthorizationResponse_t",
    ][::std::mem::align_of::<MicroTxnAuthorizationResponse_t>() - 4usize];
    [
        "Offset of field: MicroTxnAuthorizationResponse_t::m_unAppID",
    ][::std::mem::offset_of!(MicroTxnAuthorizationResponse_t, m_unAppID) - 0usize];
    [
        "Offset of field: MicroTxnAuthorizationResponse_t::m_ulOrderID",
    ][::std::mem::offset_of!(MicroTxnAuthorizationResponse_t, m_ulOrderID) - 4usize];
    [
        "Offset of field: MicroTxnAuthorizationResponse_t::m_bAuthorized",
    ][::std::mem::offset_of!(MicroTxnAuthorizationResponse_t, m_bAuthorized) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EncryptedAppTicketResponse_t {
    pub m_eResult: EResult,
}
pub const EncryptedAppTicketResponse_t_k_iCallback: EncryptedAppTicketResponse_t__bindgen_ty_1 = EncryptedAppTicketResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EncryptedAppTicketResponse_t__bindgen_ty_1 {
    k_iCallback = 154,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of EncryptedAppTicketResponse_t",
    ][::std::mem::size_of::<EncryptedAppTicketResponse_t>() - 4usize];
    [
        "Alignment of EncryptedAppTicketResponse_t",
    ][::std::mem::align_of::<EncryptedAppTicketResponse_t>() - 4usize];
    [
        "Offset of field: EncryptedAppTicketResponse_t::m_eResult",
    ][::std::mem::offset_of!(EncryptedAppTicketResponse_t, m_eResult) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GetAuthSessionTicketResponse_t {
    pub m_hAuthTicket: HAuthTicket,
    pub m_eResult: EResult,
}
pub const GetAuthSessionTicketResponse_t_k_iCallback: GetAuthSessionTicketResponse_t__bindgen_ty_1 = GetAuthSessionTicketResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetAuthSessionTicketResponse_t__bindgen_ty_1 {
    k_iCallback = 163,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetAuthSessionTicketResponse_t",
    ][::std::mem::size_of::<GetAuthSessionTicketResponse_t>() - 8usize];
    [
        "Alignment of GetAuthSessionTicketResponse_t",
    ][::std::mem::align_of::<GetAuthSessionTicketResponse_t>() - 4usize];
    [
        "Offset of field: GetAuthSessionTicketResponse_t::m_hAuthTicket",
    ][::std::mem::offset_of!(GetAuthSessionTicketResponse_t, m_hAuthTicket) - 0usize];
    [
        "Offset of field: GetAuthSessionTicketResponse_t::m_eResult",
    ][::std::mem::offset_of!(GetAuthSessionTicketResponse_t, m_eResult) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameWebCallback_t {
    pub m_szURL: [::std::os::raw::c_char; 256usize],
}
pub const GameWebCallback_t_k_iCallback: GameWebCallback_t__bindgen_ty_1 = GameWebCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameWebCallback_t__bindgen_ty_1 {
    k_iCallback = 164,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GameWebCallback_t"][::std::mem::size_of::<GameWebCallback_t>() - 256usize];
    [
        "Alignment of GameWebCallback_t",
    ][::std::mem::align_of::<GameWebCallback_t>() - 1usize];
    [
        "Offset of field: GameWebCallback_t::m_szURL",
    ][::std::mem::offset_of!(GameWebCallback_t, m_szURL) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StoreAuthURLResponse_t {
    pub m_szURL: [::std::os::raw::c_char; 512usize],
}
pub const StoreAuthURLResponse_t_k_iCallback: StoreAuthURLResponse_t__bindgen_ty_1 = StoreAuthURLResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StoreAuthURLResponse_t__bindgen_ty_1 {
    k_iCallback = 165,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StoreAuthURLResponse_t",
    ][::std::mem::size_of::<StoreAuthURLResponse_t>() - 512usize];
    [
        "Alignment of StoreAuthURLResponse_t",
    ][::std::mem::align_of::<StoreAuthURLResponse_t>() - 1usize];
    [
        "Offset of field: StoreAuthURLResponse_t::m_szURL",
    ][::std::mem::offset_of!(StoreAuthURLResponse_t, m_szURL) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MarketEligibilityResponse_t {
    pub m_bAllowed: bool,
    pub m_eNotAllowedReason: EMarketNotAllowedReasonFlags,
    pub m_rtAllowedAtTime: RTime32,
    pub m_cdaySteamGuardRequiredDays: ::std::os::raw::c_int,
    pub m_cdayNewDeviceCooldown: ::std::os::raw::c_int,
}
pub const MarketEligibilityResponse_t_k_iCallback: MarketEligibilityResponse_t__bindgen_ty_1 = MarketEligibilityResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MarketEligibilityResponse_t__bindgen_ty_1 {
    k_iCallback = 166,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MarketEligibilityResponse_t",
    ][::std::mem::size_of::<MarketEligibilityResponse_t>() - 20usize];
    [
        "Alignment of MarketEligibilityResponse_t",
    ][::std::mem::align_of::<MarketEligibilityResponse_t>() - 4usize];
    [
        "Offset of field: MarketEligibilityResponse_t::m_bAllowed",
    ][::std::mem::offset_of!(MarketEligibilityResponse_t, m_bAllowed) - 0usize];
    [
        "Offset of field: MarketEligibilityResponse_t::m_eNotAllowedReason",
    ][::std::mem::offset_of!(MarketEligibilityResponse_t, m_eNotAllowedReason) - 4usize];
    [
        "Offset of field: MarketEligibilityResponse_t::m_rtAllowedAtTime",
    ][::std::mem::offset_of!(MarketEligibilityResponse_t, m_rtAllowedAtTime) - 8usize];
    [
        "Offset of field: MarketEligibilityResponse_t::m_cdaySteamGuardRequiredDays",
    ][::std::mem::offset_of!(MarketEligibilityResponse_t, m_cdaySteamGuardRequiredDays)
        - 12usize];
    [
        "Offset of field: MarketEligibilityResponse_t::m_cdayNewDeviceCooldown",
    ][::std::mem::offset_of!(MarketEligibilityResponse_t, m_cdayNewDeviceCooldown)
        - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DurationControl_t {
    pub m_eResult: EResult,
    pub m_appid: AppId_t,
    pub m_bApplicable: bool,
    pub m_csecsLast5h: int32,
    pub m_progress: EDurationControlProgress,
    pub m_notification: EDurationControlNotification,
    pub m_csecsToday: int32,
    pub m_csecsRemaining: int32,
}
pub const DurationControl_t_k_iCallback: DurationControl_t__bindgen_ty_1 = DurationControl_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DurationControl_t__bindgen_ty_1 {
    k_iCallback = 167,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DurationControl_t"][::std::mem::size_of::<DurationControl_t>() - 32usize];
    [
        "Alignment of DurationControl_t",
    ][::std::mem::align_of::<DurationControl_t>() - 4usize];
    [
        "Offset of field: DurationControl_t::m_eResult",
    ][::std::mem::offset_of!(DurationControl_t, m_eResult) - 0usize];
    [
        "Offset of field: DurationControl_t::m_appid",
    ][::std::mem::offset_of!(DurationControl_t, m_appid) - 4usize];
    [
        "Offset of field: DurationControl_t::m_bApplicable",
    ][::std::mem::offset_of!(DurationControl_t, m_bApplicable) - 8usize];
    [
        "Offset of field: DurationControl_t::m_csecsLast5h",
    ][::std::mem::offset_of!(DurationControl_t, m_csecsLast5h) - 12usize];
    [
        "Offset of field: DurationControl_t::m_progress",
    ][::std::mem::offset_of!(DurationControl_t, m_progress) - 16usize];
    [
        "Offset of field: DurationControl_t::m_notification",
    ][::std::mem::offset_of!(DurationControl_t, m_notification) - 20usize];
    [
        "Offset of field: DurationControl_t::m_csecsToday",
    ][::std::mem::offset_of!(DurationControl_t, m_csecsToday) - 24usize];
    [
        "Offset of field: DurationControl_t::m_csecsRemaining",
    ][::std::mem::offset_of!(DurationControl_t, m_csecsRemaining) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GetTicketForWebApiResponse_t {
    pub m_hAuthTicket: HAuthTicket,
    pub m_eResult: EResult,
    pub m_cubTicket: ::std::os::raw::c_int,
    pub m_rgubTicket: [uint8; 2560usize],
}
pub const GetTicketForWebApiResponse_t_k_iCallback: GetTicketForWebApiResponse_t__bindgen_ty_1 = GetTicketForWebApiResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetTicketForWebApiResponse_t__bindgen_ty_1 {
    k_iCallback = 168,
}
pub const GetTicketForWebApiResponse_t_k_nCubTicketMaxLength: ::std::os::raw::c_int = 2560;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetTicketForWebApiResponse_t",
    ][::std::mem::size_of::<GetTicketForWebApiResponse_t>() - 2572usize];
    [
        "Alignment of GetTicketForWebApiResponse_t",
    ][::std::mem::align_of::<GetTicketForWebApiResponse_t>() - 4usize];
    [
        "Offset of field: GetTicketForWebApiResponse_t::m_hAuthTicket",
    ][::std::mem::offset_of!(GetTicketForWebApiResponse_t, m_hAuthTicket) - 0usize];
    [
        "Offset of field: GetTicketForWebApiResponse_t::m_eResult",
    ][::std::mem::offset_of!(GetTicketForWebApiResponse_t, m_eResult) - 4usize];
    [
        "Offset of field: GetTicketForWebApiResponse_t::m_cubTicket",
    ][::std::mem::offset_of!(GetTicketForWebApiResponse_t, m_cubTicket) - 8usize];
    [
        "Offset of field: GetTicketForWebApiResponse_t::m_rgubTicket",
    ][::std::mem::offset_of!(GetTicketForWebApiResponse_t, m_rgubTicket) - 12usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EFriendRelationship {
    k_EFriendRelationshipNone = 0,
    k_EFriendRelationshipBlocked = 1,
    k_EFriendRelationshipRequestRecipient = 2,
    k_EFriendRelationshipFriend = 3,
    k_EFriendRelationshipRequestInitiator = 4,
    k_EFriendRelationshipIgnored = 5,
    k_EFriendRelationshipIgnoredFriend = 6,
    k_EFriendRelationshipSuggested_DEPRECATED = 7,
    k_EFriendRelationshipMax = 8,
}
pub const k_cchMaxFriendsGroupName: ::std::os::raw::c_int = 64;
pub const k_cFriendsGroupLimit: ::std::os::raw::c_int = 100;
pub type FriendsGroupID_t = int16;
pub const k_FriendsGroupID_Invalid: FriendsGroupID_t = -1;
pub const k_cEnumerateFollowersMax: ::std::os::raw::c_int = 50;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EPersonaState {
    k_EPersonaStateOffline = 0,
    k_EPersonaStateOnline = 1,
    k_EPersonaStateBusy = 2,
    k_EPersonaStateAway = 3,
    k_EPersonaStateSnooze = 4,
    k_EPersonaStateLookingToTrade = 5,
    k_EPersonaStateLookingToPlay = 6,
    k_EPersonaStateInvisible = 7,
    k_EPersonaStateMax = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EFriendFlags {
    k_EFriendFlagNone = 0,
    k_EFriendFlagBlocked = 1,
    k_EFriendFlagFriendshipRequested = 2,
    k_EFriendFlagImmediate = 4,
    k_EFriendFlagClanMember = 8,
    k_EFriendFlagOnGameServer = 16,
    k_EFriendFlagRequestingFriendship = 128,
    k_EFriendFlagRequestingInfo = 256,
    k_EFriendFlagIgnored = 512,
    k_EFriendFlagIgnoredFriend = 1024,
    k_EFriendFlagChatMember = 4096,
    k_EFriendFlagAll = 65535,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FriendGameInfo_t {
    pub m_gameID: CGameID,
    pub m_unGameIP: uint32,
    pub m_usGamePort: uint16,
    pub m_usQueryPort: uint16,
    pub m_steamIDLobby: CSteamID,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of FriendGameInfo_t"][::std::mem::size_of::<FriendGameInfo_t>() - 24usize];
    [
        "Alignment of FriendGameInfo_t",
    ][::std::mem::align_of::<FriendGameInfo_t>() - 4usize];
    [
        "Offset of field: FriendGameInfo_t::m_gameID",
    ][::std::mem::offset_of!(FriendGameInfo_t, m_gameID) - 0usize];
    [
        "Offset of field: FriendGameInfo_t::m_unGameIP",
    ][::std::mem::offset_of!(FriendGameInfo_t, m_unGameIP) - 8usize];
    [
        "Offset of field: FriendGameInfo_t::m_usGamePort",
    ][::std::mem::offset_of!(FriendGameInfo_t, m_usGamePort) - 12usize];
    [
        "Offset of field: FriendGameInfo_t::m_usQueryPort",
    ][::std::mem::offset_of!(FriendGameInfo_t, m_usQueryPort) - 14usize];
    [
        "Offset of field: FriendGameInfo_t::m_steamIDLobby",
    ][::std::mem::offset_of!(FriendGameInfo_t, m_steamIDLobby) - 16usize];
};
pub const k_usFriendGameInfoQueryPort_NotInitialized: uint16 = 65535;
pub const k_usFriendGameInfoQueryPort_Error: uint16 = 65534;
pub const k_cchPersonaNameMax: _bindgen_ty_39 = _bindgen_ty_39::k_cchPersonaNameMax;
pub const k_cwchPersonaNameMax: _bindgen_ty_39 = _bindgen_ty_39::k_cwchPersonaNameMax;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_39 {
    k_cchPersonaNameMax = 128,
    k_cwchPersonaNameMax = 32,
}
pub const k_cubChatMetadataMax: uint32 = 8192;
pub const k_cchMaxRichPresenceKeys: _bindgen_ty_40 = _bindgen_ty_40::k_cchMaxRichPresenceKeys;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_40 {
    k_cchMaxRichPresenceKeys = 30,
}
pub const k_cchMaxRichPresenceKeyLength: _bindgen_ty_41 = _bindgen_ty_41::k_cchMaxRichPresenceKeyLength;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_41 {
    k_cchMaxRichPresenceKeyLength = 64,
}
pub const k_cchMaxRichPresenceValueLength: _bindgen_ty_42 = _bindgen_ty_42::k_cchMaxRichPresenceValueLength;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_42 {
    k_cchMaxRichPresenceValueLength = 256,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EOverlayToStoreFlag {
    k_EOverlayToStoreFlag_None = 0,
    k_EOverlayToStoreFlag_AddToCart = 1,
    k_EOverlayToStoreFlag_AddToCartAndShow = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EActivateGameOverlayToWebPageMode {
    k_EActivateGameOverlayToWebPageMode_Default = 0,
    k_EActivateGameOverlayToWebPageMode_Modal = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ECommunityProfileItemType {
    k_ECommunityProfileItemType_AnimatedAvatar = 0,
    k_ECommunityProfileItemType_AvatarFrame = 1,
    k_ECommunityProfileItemType_ProfileModifier = 2,
    k_ECommunityProfileItemType_ProfileBackground = 3,
    k_ECommunityProfileItemType_MiniProfileBackground = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ECommunityProfileItemProperty {
    k_ECommunityProfileItemProperty_ImageSmall = 0,
    k_ECommunityProfileItemProperty_ImageLarge = 1,
    k_ECommunityProfileItemProperty_InternalName = 2,
    k_ECommunityProfileItemProperty_Title = 3,
    k_ECommunityProfileItemProperty_Description = 4,
    k_ECommunityProfileItemProperty_AppID = 5,
    k_ECommunityProfileItemProperty_TypeID = 6,
    k_ECommunityProfileItemProperty_Class = 7,
    k_ECommunityProfileItemProperty_MovieWebM = 8,
    k_ECommunityProfileItemProperty_MovieMP4 = 9,
    k_ECommunityProfileItemProperty_MovieWebMSmall = 10,
    k_ECommunityProfileItemProperty_MovieMP4Small = 11,
}
#[repr(C)]
pub struct ISteamFriends__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamFriends {
    pub vtable_: *const ISteamFriends__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamFriends"][::std::mem::size_of::<ISteamFriends>() - 8usize];
    ["Alignment of ISteamFriends"][::std::mem::align_of::<ISteamFriends>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct PersonaStateChange_t {
    pub m_ulSteamID: uint64,
    pub m_nChangeFlags: ::std::os::raw::c_int,
}
pub const PersonaStateChange_t_k_iCallback: PersonaStateChange_t__bindgen_ty_1 = PersonaStateChange_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PersonaStateChange_t__bindgen_ty_1 {
    k_iCallback = 304,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of PersonaStateChange_t",
    ][::std::mem::size_of::<PersonaStateChange_t>() - 12usize];
    [
        "Alignment of PersonaStateChange_t",
    ][::std::mem::align_of::<PersonaStateChange_t>() - 4usize];
    [
        "Offset of field: PersonaStateChange_t::m_ulSteamID",
    ][::std::mem::offset_of!(PersonaStateChange_t, m_ulSteamID) - 0usize];
    [
        "Offset of field: PersonaStateChange_t::m_nChangeFlags",
    ][::std::mem::offset_of!(PersonaStateChange_t, m_nChangeFlags) - 8usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EPersonaChange {
    k_EPersonaChangeName = 1,
    k_EPersonaChangeStatus = 2,
    k_EPersonaChangeComeOnline = 4,
    k_EPersonaChangeGoneOffline = 8,
    k_EPersonaChangeGamePlayed = 16,
    k_EPersonaChangeGameServer = 32,
    k_EPersonaChangeAvatar = 64,
    k_EPersonaChangeJoinedSource = 128,
    k_EPersonaChangeLeftSource = 256,
    k_EPersonaChangeRelationshipChanged = 512,
    k_EPersonaChangeNameFirstSet = 1024,
    k_EPersonaChangeBroadcast = 2048,
    k_EPersonaChangeNickname = 4096,
    k_EPersonaChangeSteamLevel = 8192,
    k_EPersonaChangeRichPresence = 16384,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameOverlayActivated_t {
    pub m_bActive: uint8,
    pub m_bUserInitiated: bool,
    pub m_nAppID: AppId_t,
    pub m_dwOverlayPID: uint32,
}
pub const GameOverlayActivated_t_k_iCallback: GameOverlayActivated_t__bindgen_ty_1 = GameOverlayActivated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameOverlayActivated_t__bindgen_ty_1 {
    k_iCallback = 331,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameOverlayActivated_t",
    ][::std::mem::size_of::<GameOverlayActivated_t>() - 12usize];
    [
        "Alignment of GameOverlayActivated_t",
    ][::std::mem::align_of::<GameOverlayActivated_t>() - 4usize];
    [
        "Offset of field: GameOverlayActivated_t::m_bActive",
    ][::std::mem::offset_of!(GameOverlayActivated_t, m_bActive) - 0usize];
    [
        "Offset of field: GameOverlayActivated_t::m_bUserInitiated",
    ][::std::mem::offset_of!(GameOverlayActivated_t, m_bUserInitiated) - 1usize];
    [
        "Offset of field: GameOverlayActivated_t::m_nAppID",
    ][::std::mem::offset_of!(GameOverlayActivated_t, m_nAppID) - 4usize];
    [
        "Offset of field: GameOverlayActivated_t::m_dwOverlayPID",
    ][::std::mem::offset_of!(GameOverlayActivated_t, m_dwOverlayPID) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GameServerChangeRequested_t {
    pub m_rgchServer: [::std::os::raw::c_char; 64usize],
    pub m_rgchPassword: [::std::os::raw::c_char; 64usize],
}
pub const GameServerChangeRequested_t_k_iCallback: GameServerChangeRequested_t__bindgen_ty_1 = GameServerChangeRequested_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameServerChangeRequested_t__bindgen_ty_1 {
    k_iCallback = 332,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameServerChangeRequested_t",
    ][::std::mem::size_of::<GameServerChangeRequested_t>() - 128usize];
    [
        "Alignment of GameServerChangeRequested_t",
    ][::std::mem::align_of::<GameServerChangeRequested_t>() - 1usize];
    [
        "Offset of field: GameServerChangeRequested_t::m_rgchServer",
    ][::std::mem::offset_of!(GameServerChangeRequested_t, m_rgchServer) - 0usize];
    [
        "Offset of field: GameServerChangeRequested_t::m_rgchPassword",
    ][::std::mem::offset_of!(GameServerChangeRequested_t, m_rgchPassword) - 64usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameLobbyJoinRequested_t {
    pub m_steamIDLobby: CSteamID,
    pub m_steamIDFriend: CSteamID,
}
pub const GameLobbyJoinRequested_t_k_iCallback: GameLobbyJoinRequested_t__bindgen_ty_1 = GameLobbyJoinRequested_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameLobbyJoinRequested_t__bindgen_ty_1 {
    k_iCallback = 333,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameLobbyJoinRequested_t",
    ][::std::mem::size_of::<GameLobbyJoinRequested_t>() - 16usize];
    [
        "Alignment of GameLobbyJoinRequested_t",
    ][::std::mem::align_of::<GameLobbyJoinRequested_t>() - 1usize];
    [
        "Offset of field: GameLobbyJoinRequested_t::m_steamIDLobby",
    ][::std::mem::offset_of!(GameLobbyJoinRequested_t, m_steamIDLobby) - 0usize];
    [
        "Offset of field: GameLobbyJoinRequested_t::m_steamIDFriend",
    ][::std::mem::offset_of!(GameLobbyJoinRequested_t, m_steamIDFriend) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AvatarImageLoaded_t {
    pub m_steamID: CSteamID,
    pub m_iImage: ::std::os::raw::c_int,
    pub m_iWide: ::std::os::raw::c_int,
    pub m_iTall: ::std::os::raw::c_int,
}
pub const AvatarImageLoaded_t_k_iCallback: AvatarImageLoaded_t__bindgen_ty_1 = AvatarImageLoaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AvatarImageLoaded_t__bindgen_ty_1 {
    k_iCallback = 334,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AvatarImageLoaded_t",
    ][::std::mem::size_of::<AvatarImageLoaded_t>() - 20usize];
    [
        "Alignment of AvatarImageLoaded_t",
    ][::std::mem::align_of::<AvatarImageLoaded_t>() - 4usize];
    [
        "Offset of field: AvatarImageLoaded_t::m_steamID",
    ][::std::mem::offset_of!(AvatarImageLoaded_t, m_steamID) - 0usize];
    [
        "Offset of field: AvatarImageLoaded_t::m_iImage",
    ][::std::mem::offset_of!(AvatarImageLoaded_t, m_iImage) - 8usize];
    [
        "Offset of field: AvatarImageLoaded_t::m_iWide",
    ][::std::mem::offset_of!(AvatarImageLoaded_t, m_iWide) - 12usize];
    [
        "Offset of field: AvatarImageLoaded_t::m_iTall",
    ][::std::mem::offset_of!(AvatarImageLoaded_t, m_iTall) - 16usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ClanOfficerListResponse_t {
    pub m_steamIDClan: CSteamID,
    pub m_cOfficers: ::std::os::raw::c_int,
    pub m_bSuccess: uint8,
}
pub const ClanOfficerListResponse_t_k_iCallback: ClanOfficerListResponse_t__bindgen_ty_1 = ClanOfficerListResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ClanOfficerListResponse_t__bindgen_ty_1 {
    k_iCallback = 335,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ClanOfficerListResponse_t",
    ][::std::mem::size_of::<ClanOfficerListResponse_t>() - 16usize];
    [
        "Alignment of ClanOfficerListResponse_t",
    ][::std::mem::align_of::<ClanOfficerListResponse_t>() - 4usize];
    [
        "Offset of field: ClanOfficerListResponse_t::m_steamIDClan",
    ][::std::mem::offset_of!(ClanOfficerListResponse_t, m_steamIDClan) - 0usize];
    [
        "Offset of field: ClanOfficerListResponse_t::m_cOfficers",
    ][::std::mem::offset_of!(ClanOfficerListResponse_t, m_cOfficers) - 8usize];
    [
        "Offset of field: ClanOfficerListResponse_t::m_bSuccess",
    ][::std::mem::offset_of!(ClanOfficerListResponse_t, m_bSuccess) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FriendRichPresenceUpdate_t {
    pub m_steamIDFriend: CSteamID,
    pub m_nAppID: AppId_t,
}
pub const FriendRichPresenceUpdate_t_k_iCallback: FriendRichPresenceUpdate_t__bindgen_ty_1 = FriendRichPresenceUpdate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FriendRichPresenceUpdate_t__bindgen_ty_1 {
    k_iCallback = 336,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FriendRichPresenceUpdate_t",
    ][::std::mem::size_of::<FriendRichPresenceUpdate_t>() - 12usize];
    [
        "Alignment of FriendRichPresenceUpdate_t",
    ][::std::mem::align_of::<FriendRichPresenceUpdate_t>() - 4usize];
    [
        "Offset of field: FriendRichPresenceUpdate_t::m_steamIDFriend",
    ][::std::mem::offset_of!(FriendRichPresenceUpdate_t, m_steamIDFriend) - 0usize];
    [
        "Offset of field: FriendRichPresenceUpdate_t::m_nAppID",
    ][::std::mem::offset_of!(FriendRichPresenceUpdate_t, m_nAppID) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameRichPresenceJoinRequested_t {
    pub m_steamIDFriend: CSteamID,
    pub m_rgchConnect: [::std::os::raw::c_char; 256usize],
}
pub const GameRichPresenceJoinRequested_t_k_iCallback: GameRichPresenceJoinRequested_t__bindgen_ty_1 = GameRichPresenceJoinRequested_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameRichPresenceJoinRequested_t__bindgen_ty_1 {
    k_iCallback = 337,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameRichPresenceJoinRequested_t",
    ][::std::mem::size_of::<GameRichPresenceJoinRequested_t>() - 264usize];
    [
        "Alignment of GameRichPresenceJoinRequested_t",
    ][::std::mem::align_of::<GameRichPresenceJoinRequested_t>() - 1usize];
    [
        "Offset of field: GameRichPresenceJoinRequested_t::m_steamIDFriend",
    ][::std::mem::offset_of!(GameRichPresenceJoinRequested_t, m_steamIDFriend) - 0usize];
    [
        "Offset of field: GameRichPresenceJoinRequested_t::m_rgchConnect",
    ][::std::mem::offset_of!(GameRichPresenceJoinRequested_t, m_rgchConnect) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameConnectedClanChatMsg_t {
    pub m_steamIDClanChat: CSteamID,
    pub m_steamIDUser: CSteamID,
    pub m_iMessageID: ::std::os::raw::c_int,
}
pub const GameConnectedClanChatMsg_t_k_iCallback: GameConnectedClanChatMsg_t__bindgen_ty_1 = GameConnectedClanChatMsg_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameConnectedClanChatMsg_t__bindgen_ty_1 {
    k_iCallback = 338,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameConnectedClanChatMsg_t",
    ][::std::mem::size_of::<GameConnectedClanChatMsg_t>() - 20usize];
    [
        "Alignment of GameConnectedClanChatMsg_t",
    ][::std::mem::align_of::<GameConnectedClanChatMsg_t>() - 4usize];
    [
        "Offset of field: GameConnectedClanChatMsg_t::m_steamIDClanChat",
    ][::std::mem::offset_of!(GameConnectedClanChatMsg_t, m_steamIDClanChat) - 0usize];
    [
        "Offset of field: GameConnectedClanChatMsg_t::m_steamIDUser",
    ][::std::mem::offset_of!(GameConnectedClanChatMsg_t, m_steamIDUser) - 8usize];
    [
        "Offset of field: GameConnectedClanChatMsg_t::m_iMessageID",
    ][::std::mem::offset_of!(GameConnectedClanChatMsg_t, m_iMessageID) - 16usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameConnectedChatJoin_t {
    pub m_steamIDClanChat: CSteamID,
    pub m_steamIDUser: CSteamID,
}
pub const GameConnectedChatJoin_t_k_iCallback: GameConnectedChatJoin_t__bindgen_ty_1 = GameConnectedChatJoin_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameConnectedChatJoin_t__bindgen_ty_1 {
    k_iCallback = 339,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameConnectedChatJoin_t",
    ][::std::mem::size_of::<GameConnectedChatJoin_t>() - 16usize];
    [
        "Alignment of GameConnectedChatJoin_t",
    ][::std::mem::align_of::<GameConnectedChatJoin_t>() - 1usize];
    [
        "Offset of field: GameConnectedChatJoin_t::m_steamIDClanChat",
    ][::std::mem::offset_of!(GameConnectedChatJoin_t, m_steamIDClanChat) - 0usize];
    [
        "Offset of field: GameConnectedChatJoin_t::m_steamIDUser",
    ][::std::mem::offset_of!(GameConnectedChatJoin_t, m_steamIDUser) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameConnectedChatLeave_t {
    pub m_steamIDClanChat: CSteamID,
    pub m_steamIDUser: CSteamID,
    pub m_bKicked: bool,
    pub m_bDropped: bool,
}
pub const GameConnectedChatLeave_t_k_iCallback: GameConnectedChatLeave_t__bindgen_ty_1 = GameConnectedChatLeave_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameConnectedChatLeave_t__bindgen_ty_1 {
    k_iCallback = 340,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameConnectedChatLeave_t",
    ][::std::mem::size_of::<GameConnectedChatLeave_t>() - 18usize];
    [
        "Alignment of GameConnectedChatLeave_t",
    ][::std::mem::align_of::<GameConnectedChatLeave_t>() - 1usize];
    [
        "Offset of field: GameConnectedChatLeave_t::m_steamIDClanChat",
    ][::std::mem::offset_of!(GameConnectedChatLeave_t, m_steamIDClanChat) - 0usize];
    [
        "Offset of field: GameConnectedChatLeave_t::m_steamIDUser",
    ][::std::mem::offset_of!(GameConnectedChatLeave_t, m_steamIDUser) - 8usize];
    [
        "Offset of field: GameConnectedChatLeave_t::m_bKicked",
    ][::std::mem::offset_of!(GameConnectedChatLeave_t, m_bKicked) - 16usize];
    [
        "Offset of field: GameConnectedChatLeave_t::m_bDropped",
    ][::std::mem::offset_of!(GameConnectedChatLeave_t, m_bDropped) - 17usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DownloadClanActivityCountsResult_t {
    pub m_bSuccess: bool,
}
pub const DownloadClanActivityCountsResult_t_k_iCallback: DownloadClanActivityCountsResult_t__bindgen_ty_1 = DownloadClanActivityCountsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DownloadClanActivityCountsResult_t__bindgen_ty_1 {
    k_iCallback = 341,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of DownloadClanActivityCountsResult_t",
    ][::std::mem::size_of::<DownloadClanActivityCountsResult_t>() - 1usize];
    [
        "Alignment of DownloadClanActivityCountsResult_t",
    ][::std::mem::align_of::<DownloadClanActivityCountsResult_t>() - 1usize];
    [
        "Offset of field: DownloadClanActivityCountsResult_t::m_bSuccess",
    ][::std::mem::offset_of!(DownloadClanActivityCountsResult_t, m_bSuccess) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct JoinClanChatRoomCompletionResult_t {
    pub m_steamIDClanChat: CSteamID,
    pub m_eChatRoomEnterResponse: EChatRoomEnterResponse,
}
pub const JoinClanChatRoomCompletionResult_t_k_iCallback: JoinClanChatRoomCompletionResult_t__bindgen_ty_1 = JoinClanChatRoomCompletionResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JoinClanChatRoomCompletionResult_t__bindgen_ty_1 {
    k_iCallback = 342,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of JoinClanChatRoomCompletionResult_t",
    ][::std::mem::size_of::<JoinClanChatRoomCompletionResult_t>() - 12usize];
    [
        "Alignment of JoinClanChatRoomCompletionResult_t",
    ][::std::mem::align_of::<JoinClanChatRoomCompletionResult_t>() - 4usize];
    [
        "Offset of field: JoinClanChatRoomCompletionResult_t::m_steamIDClanChat",
    ][::std::mem::offset_of!(JoinClanChatRoomCompletionResult_t, m_steamIDClanChat)
        - 0usize];
    [
        "Offset of field: JoinClanChatRoomCompletionResult_t::m_eChatRoomEnterResponse",
    ][::std::mem::offset_of!(
        JoinClanChatRoomCompletionResult_t, m_eChatRoomEnterResponse
    ) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GameConnectedFriendChatMsg_t {
    pub m_steamIDUser: CSteamID,
    pub m_iMessageID: ::std::os::raw::c_int,
}
pub const GameConnectedFriendChatMsg_t_k_iCallback: GameConnectedFriendChatMsg_t__bindgen_ty_1 = GameConnectedFriendChatMsg_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GameConnectedFriendChatMsg_t__bindgen_ty_1 {
    k_iCallback = 343,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GameConnectedFriendChatMsg_t",
    ][::std::mem::size_of::<GameConnectedFriendChatMsg_t>() - 12usize];
    [
        "Alignment of GameConnectedFriendChatMsg_t",
    ][::std::mem::align_of::<GameConnectedFriendChatMsg_t>() - 4usize];
    [
        "Offset of field: GameConnectedFriendChatMsg_t::m_steamIDUser",
    ][::std::mem::offset_of!(GameConnectedFriendChatMsg_t, m_steamIDUser) - 0usize];
    [
        "Offset of field: GameConnectedFriendChatMsg_t::m_iMessageID",
    ][::std::mem::offset_of!(GameConnectedFriendChatMsg_t, m_iMessageID) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FriendsGetFollowerCount_t {
    pub m_eResult: EResult,
    pub m_steamID: CSteamID,
    pub m_nCount: ::std::os::raw::c_int,
}
pub const FriendsGetFollowerCount_t_k_iCallback: FriendsGetFollowerCount_t__bindgen_ty_1 = FriendsGetFollowerCount_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FriendsGetFollowerCount_t__bindgen_ty_1 {
    k_iCallback = 344,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FriendsGetFollowerCount_t",
    ][::std::mem::size_of::<FriendsGetFollowerCount_t>() - 16usize];
    [
        "Alignment of FriendsGetFollowerCount_t",
    ][::std::mem::align_of::<FriendsGetFollowerCount_t>() - 4usize];
    [
        "Offset of field: FriendsGetFollowerCount_t::m_eResult",
    ][::std::mem::offset_of!(FriendsGetFollowerCount_t, m_eResult) - 0usize];
    [
        "Offset of field: FriendsGetFollowerCount_t::m_steamID",
    ][::std::mem::offset_of!(FriendsGetFollowerCount_t, m_steamID) - 4usize];
    [
        "Offset of field: FriendsGetFollowerCount_t::m_nCount",
    ][::std::mem::offset_of!(FriendsGetFollowerCount_t, m_nCount) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FriendsIsFollowing_t {
    pub m_eResult: EResult,
    pub m_steamID: CSteamID,
    pub m_bIsFollowing: bool,
}
pub const FriendsIsFollowing_t_k_iCallback: FriendsIsFollowing_t__bindgen_ty_1 = FriendsIsFollowing_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FriendsIsFollowing_t__bindgen_ty_1 {
    k_iCallback = 345,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FriendsIsFollowing_t",
    ][::std::mem::size_of::<FriendsIsFollowing_t>() - 16usize];
    [
        "Alignment of FriendsIsFollowing_t",
    ][::std::mem::align_of::<FriendsIsFollowing_t>() - 4usize];
    [
        "Offset of field: FriendsIsFollowing_t::m_eResult",
    ][::std::mem::offset_of!(FriendsIsFollowing_t, m_eResult) - 0usize];
    [
        "Offset of field: FriendsIsFollowing_t::m_steamID",
    ][::std::mem::offset_of!(FriendsIsFollowing_t, m_steamID) - 4usize];
    [
        "Offset of field: FriendsIsFollowing_t::m_bIsFollowing",
    ][::std::mem::offset_of!(FriendsIsFollowing_t, m_bIsFollowing) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FriendsEnumerateFollowingList_t {
    pub m_eResult: EResult,
    pub m_rgSteamID: [CSteamID; 50usize],
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
}
pub const FriendsEnumerateFollowingList_t_k_iCallback: FriendsEnumerateFollowingList_t__bindgen_ty_1 = FriendsEnumerateFollowingList_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FriendsEnumerateFollowingList_t__bindgen_ty_1 {
    k_iCallback = 346,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FriendsEnumerateFollowingList_t",
    ][::std::mem::size_of::<FriendsEnumerateFollowingList_t>() - 412usize];
    [
        "Alignment of FriendsEnumerateFollowingList_t",
    ][::std::mem::align_of::<FriendsEnumerateFollowingList_t>() - 4usize];
    [
        "Offset of field: FriendsEnumerateFollowingList_t::m_eResult",
    ][::std::mem::offset_of!(FriendsEnumerateFollowingList_t, m_eResult) - 0usize];
    [
        "Offset of field: FriendsEnumerateFollowingList_t::m_rgSteamID",
    ][::std::mem::offset_of!(FriendsEnumerateFollowingList_t, m_rgSteamID) - 4usize];
    [
        "Offset of field: FriendsEnumerateFollowingList_t::m_nResultsReturned",
    ][::std::mem::offset_of!(FriendsEnumerateFollowingList_t, m_nResultsReturned)
        - 404usize];
    [
        "Offset of field: FriendsEnumerateFollowingList_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(FriendsEnumerateFollowingList_t, m_nTotalResultCount)
        - 408usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UnreadChatMessagesChanged_t {
    pub _address: u8,
}
pub const UnreadChatMessagesChanged_t_k_iCallback: UnreadChatMessagesChanged_t__bindgen_ty_1 = UnreadChatMessagesChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UnreadChatMessagesChanged_t__bindgen_ty_1 {
    k_iCallback = 348,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UnreadChatMessagesChanged_t",
    ][::std::mem::size_of::<UnreadChatMessagesChanged_t>() - 1usize];
    [
        "Alignment of UnreadChatMessagesChanged_t",
    ][::std::mem::align_of::<UnreadChatMessagesChanged_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OverlayBrowserProtocolNavigation_t {
    pub rgchURI: [::std::os::raw::c_char; 1024usize],
}
pub const OverlayBrowserProtocolNavigation_t_k_iCallback: OverlayBrowserProtocolNavigation_t__bindgen_ty_1 = OverlayBrowserProtocolNavigation_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OverlayBrowserProtocolNavigation_t__bindgen_ty_1 {
    k_iCallback = 349,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of OverlayBrowserProtocolNavigation_t",
    ][::std::mem::size_of::<OverlayBrowserProtocolNavigation_t>() - 1024usize];
    [
        "Alignment of OverlayBrowserProtocolNavigation_t",
    ][::std::mem::align_of::<OverlayBrowserProtocolNavigation_t>() - 1usize];
    [
        "Offset of field: OverlayBrowserProtocolNavigation_t::rgchURI",
    ][::std::mem::offset_of!(OverlayBrowserProtocolNavigation_t, rgchURI) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EquippedProfileItemsChanged_t {
    pub m_steamID: CSteamID,
}
pub const EquippedProfileItemsChanged_t_k_iCallback: EquippedProfileItemsChanged_t__bindgen_ty_1 = EquippedProfileItemsChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EquippedProfileItemsChanged_t__bindgen_ty_1 {
    k_iCallback = 350,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of EquippedProfileItemsChanged_t",
    ][::std::mem::size_of::<EquippedProfileItemsChanged_t>() - 8usize];
    [
        "Alignment of EquippedProfileItemsChanged_t",
    ][::std::mem::align_of::<EquippedProfileItemsChanged_t>() - 1usize];
    [
        "Offset of field: EquippedProfileItemsChanged_t::m_steamID",
    ][::std::mem::offset_of!(EquippedProfileItemsChanged_t, m_steamID) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct EquippedProfileItems_t {
    pub m_eResult: EResult,
    pub m_steamID: CSteamID,
    pub m_bHasAnimatedAvatar: bool,
    pub m_bHasAvatarFrame: bool,
    pub m_bHasProfileModifier: bool,
    pub m_bHasProfileBackground: bool,
    pub m_bHasMiniProfileBackground: bool,
    pub m_bFromCache: bool,
}
pub const EquippedProfileItems_t_k_iCallback: EquippedProfileItems_t__bindgen_ty_1 = EquippedProfileItems_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EquippedProfileItems_t__bindgen_ty_1 {
    k_iCallback = 351,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of EquippedProfileItems_t",
    ][::std::mem::size_of::<EquippedProfileItems_t>() - 20usize];
    [
        "Alignment of EquippedProfileItems_t",
    ][::std::mem::align_of::<EquippedProfileItems_t>() - 4usize];
    [
        "Offset of field: EquippedProfileItems_t::m_eResult",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_eResult) - 0usize];
    [
        "Offset of field: EquippedProfileItems_t::m_steamID",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_steamID) - 4usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bHasAnimatedAvatar",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bHasAnimatedAvatar) - 12usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bHasAvatarFrame",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bHasAvatarFrame) - 13usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bHasProfileModifier",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bHasProfileModifier) - 14usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bHasProfileBackground",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bHasProfileBackground) - 15usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bHasMiniProfileBackground",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bHasMiniProfileBackground)
        - 16usize];
    [
        "Offset of field: EquippedProfileItems_t::m_bFromCache",
    ][::std::mem::offset_of!(EquippedProfileItems_t, m_bFromCache) - 17usize];
};
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamAPICallFailure {
    k_ESteamAPICallFailureNone = -1,
    k_ESteamAPICallFailureSteamGone = 0,
    k_ESteamAPICallFailureNetworkFailure = 1,
    k_ESteamAPICallFailureInvalidHandle = 2,
    k_ESteamAPICallFailureMismatchedCallback = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EGamepadTextInputMode {
    k_EGamepadTextInputModeNormal = 0,
    k_EGamepadTextInputModePassword = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EGamepadTextInputLineMode {
    k_EGamepadTextInputLineModeSingleLine = 0,
    k_EGamepadTextInputLineModeMultipleLines = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EFloatingGamepadTextInputMode {
    k_EFloatingGamepadTextInputModeModeSingleLine = 0,
    k_EFloatingGamepadTextInputModeModeMultipleLines = 1,
    k_EFloatingGamepadTextInputModeModeEmail = 2,
    k_EFloatingGamepadTextInputModeModeNumeric = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ETextFilteringContext {
    k_ETextFilteringContextUnknown = 0,
    k_ETextFilteringContextGameContent = 1,
    k_ETextFilteringContextChat = 2,
    k_ETextFilteringContextName = 3,
}
#[repr(C)]
pub struct ISteamUtils__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamUtils {
    pub vtable_: *const ISteamUtils__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamUtils"][::std::mem::size_of::<ISteamUtils>() - 8usize];
    ["Alignment of ISteamUtils"][::std::mem::align_of::<ISteamUtils>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IPCountry_t {
    pub _address: u8,
}
pub const IPCountry_t_k_iCallback: IPCountry_t__bindgen_ty_1 = IPCountry_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IPCountry_t__bindgen_ty_1 {
    k_iCallback = 701,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IPCountry_t"][::std::mem::size_of::<IPCountry_t>() - 1usize];
    ["Alignment of IPCountry_t"][::std::mem::align_of::<IPCountry_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LowBatteryPower_t {
    pub m_nMinutesBatteryLeft: uint8,
}
pub const LowBatteryPower_t_k_iCallback: LowBatteryPower_t__bindgen_ty_1 = LowBatteryPower_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LowBatteryPower_t__bindgen_ty_1 {
    k_iCallback = 702,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LowBatteryPower_t"][::std::mem::size_of::<LowBatteryPower_t>() - 1usize];
    [
        "Alignment of LowBatteryPower_t",
    ][::std::mem::align_of::<LowBatteryPower_t>() - 1usize];
    [
        "Offset of field: LowBatteryPower_t::m_nMinutesBatteryLeft",
    ][::std::mem::offset_of!(LowBatteryPower_t, m_nMinutesBatteryLeft) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamAPICallCompleted_t {
    pub m_hAsyncCall: SteamAPICall_t,
    pub m_iCallback: ::std::os::raw::c_int,
    pub m_cubParam: uint32,
}
pub const SteamAPICallCompleted_t_k_iCallback: SteamAPICallCompleted_t__bindgen_ty_1 = SteamAPICallCompleted_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamAPICallCompleted_t__bindgen_ty_1 {
    k_iCallback = 703,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamAPICallCompleted_t",
    ][::std::mem::size_of::<SteamAPICallCompleted_t>() - 16usize];
    [
        "Alignment of SteamAPICallCompleted_t",
    ][::std::mem::align_of::<SteamAPICallCompleted_t>() - 4usize];
    [
        "Offset of field: SteamAPICallCompleted_t::m_hAsyncCall",
    ][::std::mem::offset_of!(SteamAPICallCompleted_t, m_hAsyncCall) - 0usize];
    [
        "Offset of field: SteamAPICallCompleted_t::m_iCallback",
    ][::std::mem::offset_of!(SteamAPICallCompleted_t, m_iCallback) - 8usize];
    [
        "Offset of field: SteamAPICallCompleted_t::m_cubParam",
    ][::std::mem::offset_of!(SteamAPICallCompleted_t, m_cubParam) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamShutdown_t {
    pub _address: u8,
}
pub const SteamShutdown_t_k_iCallback: SteamShutdown_t__bindgen_ty_1 = SteamShutdown_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamShutdown_t__bindgen_ty_1 {
    k_iCallback = 704,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of SteamShutdown_t"][::std::mem::size_of::<SteamShutdown_t>() - 1usize];
    ["Alignment of SteamShutdown_t"][::std::mem::align_of::<SteamShutdown_t>() - 1usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ECheckFileSignature {
    k_ECheckFileSignatureInvalidSignature = 0,
    k_ECheckFileSignatureValidSignature = 1,
    k_ECheckFileSignatureFileNotFound = 2,
    k_ECheckFileSignatureNoSignaturesFoundForThisApp = 3,
    k_ECheckFileSignatureNoSignaturesFoundForThisFile = 4,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CheckFileSignature_t {
    pub m_eCheckFileSignature: ECheckFileSignature,
}
pub const CheckFileSignature_t_k_iCallback: CheckFileSignature_t__bindgen_ty_1 = CheckFileSignature_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CheckFileSignature_t__bindgen_ty_1 {
    k_iCallback = 705,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of CheckFileSignature_t",
    ][::std::mem::size_of::<CheckFileSignature_t>() - 4usize];
    [
        "Alignment of CheckFileSignature_t",
    ][::std::mem::align_of::<CheckFileSignature_t>() - 4usize];
    [
        "Offset of field: CheckFileSignature_t::m_eCheckFileSignature",
    ][::std::mem::offset_of!(CheckFileSignature_t, m_eCheckFileSignature) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GamepadTextInputDismissed_t {
    pub m_bSubmitted: bool,
    pub m_unSubmittedText: uint32,
    pub m_unAppID: AppId_t,
}
pub const GamepadTextInputDismissed_t_k_iCallback: GamepadTextInputDismissed_t__bindgen_ty_1 = GamepadTextInputDismissed_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GamepadTextInputDismissed_t__bindgen_ty_1 {
    k_iCallback = 714,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GamepadTextInputDismissed_t",
    ][::std::mem::size_of::<GamepadTextInputDismissed_t>() - 12usize];
    [
        "Alignment of GamepadTextInputDismissed_t",
    ][::std::mem::align_of::<GamepadTextInputDismissed_t>() - 4usize];
    [
        "Offset of field: GamepadTextInputDismissed_t::m_bSubmitted",
    ][::std::mem::offset_of!(GamepadTextInputDismissed_t, m_bSubmitted) - 0usize];
    [
        "Offset of field: GamepadTextInputDismissed_t::m_unSubmittedText",
    ][::std::mem::offset_of!(GamepadTextInputDismissed_t, m_unSubmittedText) - 4usize];
    [
        "Offset of field: GamepadTextInputDismissed_t::m_unAppID",
    ][::std::mem::offset_of!(GamepadTextInputDismissed_t, m_unAppID) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppResumingFromSuspend_t {
    pub _address: u8,
}
pub const AppResumingFromSuspend_t_k_iCallback: AppResumingFromSuspend_t__bindgen_ty_1 = AppResumingFromSuspend_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AppResumingFromSuspend_t__bindgen_ty_1 {
    k_iCallback = 736,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AppResumingFromSuspend_t",
    ][::std::mem::size_of::<AppResumingFromSuspend_t>() - 1usize];
    [
        "Alignment of AppResumingFromSuspend_t",
    ][::std::mem::align_of::<AppResumingFromSuspend_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FloatingGamepadTextInputDismissed_t {
    pub _address: u8,
}
pub const FloatingGamepadTextInputDismissed_t_k_iCallback: FloatingGamepadTextInputDismissed_t__bindgen_ty_1 = FloatingGamepadTextInputDismissed_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FloatingGamepadTextInputDismissed_t__bindgen_ty_1 {
    k_iCallback = 738,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FloatingGamepadTextInputDismissed_t",
    ][::std::mem::size_of::<FloatingGamepadTextInputDismissed_t>() - 1usize];
    [
        "Alignment of FloatingGamepadTextInputDismissed_t",
    ][::std::mem::align_of::<FloatingGamepadTextInputDismissed_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FilterTextDictionaryChanged_t {
    pub m_eLanguage: ::std::os::raw::c_int,
}
pub const FilterTextDictionaryChanged_t_k_iCallback: FilterTextDictionaryChanged_t__bindgen_ty_1 = FilterTextDictionaryChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FilterTextDictionaryChanged_t__bindgen_ty_1 {
    k_iCallback = 739,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FilterTextDictionaryChanged_t",
    ][::std::mem::size_of::<FilterTextDictionaryChanged_t>() - 4usize];
    [
        "Alignment of FilterTextDictionaryChanged_t",
    ][::std::mem::align_of::<FilterTextDictionaryChanged_t>() - 4usize];
    [
        "Offset of field: FilterTextDictionaryChanged_t::m_eLanguage",
    ][::std::mem::offset_of!(FilterTextDictionaryChanged_t, m_eLanguage) - 0usize];
};
pub type __gnuc_va_list = __builtin_va_list;
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __fsid_t"][::std::mem::size_of::<__fsid_t>() - 8usize];
    ["Alignment of __fsid_t"][::std::mem::align_of::<__fsid_t>() - 4usize];
    [
        "Offset of field: __fsid_t::__val",
    ][::std::mem::offset_of!(__fsid_t, __val) - 0usize];
};
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of __mbstate_t__bindgen_ty_1",
    ][::std::mem::size_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    [
        "Alignment of __mbstate_t__bindgen_ty_1",
    ][::std::mem::align_of::<__mbstate_t__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: __mbstate_t__bindgen_ty_1::__wch",
    ][::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wch) - 0usize];
    [
        "Offset of field: __mbstate_t__bindgen_ty_1::__wchb",
    ][::std::mem::offset_of!(__mbstate_t__bindgen_ty_1, __wchb) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __mbstate_t"][::std::mem::size_of::<__mbstate_t>() - 8usize];
    ["Alignment of __mbstate_t"][::std::mem::align_of::<__mbstate_t>() - 4usize];
    [
        "Offset of field: __mbstate_t::__count",
    ][::std::mem::offset_of!(__mbstate_t, __count) - 0usize];
    [
        "Offset of field: __mbstate_t::__value",
    ][::std::mem::offset_of!(__mbstate_t, __value) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos_t"][::std::mem::size_of::<_G_fpos_t>() - 16usize];
    ["Alignment of _G_fpos_t"][::std::mem::align_of::<_G_fpos_t>() - 8usize];
    [
        "Offset of field: _G_fpos_t::__pos",
    ][::std::mem::offset_of!(_G_fpos_t, __pos) - 0usize];
    [
        "Offset of field: _G_fpos_t::__state",
    ][::std::mem::offset_of!(_G_fpos_t, __state) - 8usize];
};
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _G_fpos64_t"][::std::mem::size_of::<_G_fpos64_t>() - 16usize];
    ["Alignment of _G_fpos64_t"][::std::mem::align_of::<_G_fpos64_t>() - 8usize];
    [
        "Offset of field: _G_fpos64_t::__pos",
    ][::std::mem::offset_of!(_G_fpos64_t, __pos) - 0usize];
    [
        "Offset of field: _G_fpos64_t::__state",
    ][::std::mem::offset_of!(_G_fpos64_t, __state) - 8usize];
};
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of _IO_FILE"][::std::mem::size_of::<_IO_FILE>() - 216usize];
    ["Alignment of _IO_FILE"][::std::mem::align_of::<_IO_FILE>() - 8usize];
    [
        "Offset of field: _IO_FILE::_flags",
    ][::std::mem::offset_of!(_IO_FILE, _flags) - 0usize];
    [
        "Offset of field: _IO_FILE::_IO_read_ptr",
    ][::std::mem::offset_of!(_IO_FILE, _IO_read_ptr) - 8usize];
    [
        "Offset of field: _IO_FILE::_IO_read_end",
    ][::std::mem::offset_of!(_IO_FILE, _IO_read_end) - 16usize];
    [
        "Offset of field: _IO_FILE::_IO_read_base",
    ][::std::mem::offset_of!(_IO_FILE, _IO_read_base) - 24usize];
    [
        "Offset of field: _IO_FILE::_IO_write_base",
    ][::std::mem::offset_of!(_IO_FILE, _IO_write_base) - 32usize];
    [
        "Offset of field: _IO_FILE::_IO_write_ptr",
    ][::std::mem::offset_of!(_IO_FILE, _IO_write_ptr) - 40usize];
    [
        "Offset of field: _IO_FILE::_IO_write_end",
    ][::std::mem::offset_of!(_IO_FILE, _IO_write_end) - 48usize];
    [
        "Offset of field: _IO_FILE::_IO_buf_base",
    ][::std::mem::offset_of!(_IO_FILE, _IO_buf_base) - 56usize];
    [
        "Offset of field: _IO_FILE::_IO_buf_end",
    ][::std::mem::offset_of!(_IO_FILE, _IO_buf_end) - 64usize];
    [
        "Offset of field: _IO_FILE::_IO_save_base",
    ][::std::mem::offset_of!(_IO_FILE, _IO_save_base) - 72usize];
    [
        "Offset of field: _IO_FILE::_IO_backup_base",
    ][::std::mem::offset_of!(_IO_FILE, _IO_backup_base) - 80usize];
    [
        "Offset of field: _IO_FILE::_IO_save_end",
    ][::std::mem::offset_of!(_IO_FILE, _IO_save_end) - 88usize];
    [
        "Offset of field: _IO_FILE::_markers",
    ][::std::mem::offset_of!(_IO_FILE, _markers) - 96usize];
    [
        "Offset of field: _IO_FILE::_chain",
    ][::std::mem::offset_of!(_IO_FILE, _chain) - 104usize];
    [
        "Offset of field: _IO_FILE::_fileno",
    ][::std::mem::offset_of!(_IO_FILE, _fileno) - 112usize];
    [
        "Offset of field: _IO_FILE::_flags2",
    ][::std::mem::offset_of!(_IO_FILE, _flags2) - 116usize];
    [
        "Offset of field: _IO_FILE::_old_offset",
    ][::std::mem::offset_of!(_IO_FILE, _old_offset) - 120usize];
    [
        "Offset of field: _IO_FILE::_cur_column",
    ][::std::mem::offset_of!(_IO_FILE, _cur_column) - 128usize];
    [
        "Offset of field: _IO_FILE::_vtable_offset",
    ][::std::mem::offset_of!(_IO_FILE, _vtable_offset) - 130usize];
    [
        "Offset of field: _IO_FILE::_shortbuf",
    ][::std::mem::offset_of!(_IO_FILE, _shortbuf) - 131usize];
    [
        "Offset of field: _IO_FILE::_lock",
    ][::std::mem::offset_of!(_IO_FILE, _lock) - 136usize];
    [
        "Offset of field: _IO_FILE::_offset",
    ][::std::mem::offset_of!(_IO_FILE, _offset) - 144usize];
    [
        "Offset of field: _IO_FILE::_codecvt",
    ][::std::mem::offset_of!(_IO_FILE, _codecvt) - 152usize];
    [
        "Offset of field: _IO_FILE::_wide_data",
    ][::std::mem::offset_of!(_IO_FILE, _wide_data) - 160usize];
    [
        "Offset of field: _IO_FILE::_freeres_list",
    ][::std::mem::offset_of!(_IO_FILE, _freeres_list) - 168usize];
    [
        "Offset of field: _IO_FILE::_freeres_buf",
    ][::std::mem::offset_of!(_IO_FILE, _freeres_buf) - 176usize];
    [
        "Offset of field: _IO_FILE::__pad5",
    ][::std::mem::offset_of!(_IO_FILE, __pad5) - 184usize];
    [
        "Offset of field: _IO_FILE::_mode",
    ][::std::mem::offset_of!(_IO_FILE, _mode) - 192usize];
    [
        "Offset of field: _IO_FILE::_unused2",
    ][::std::mem::offset_of!(_IO_FILE, _unused2) - 196usize];
};
pub type cookie_read_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *mut ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_write_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __buf: *const ::std::os::raw::c_char,
        __nbytes: usize,
    ) -> __ssize_t,
>;
pub type cookie_seek_function_t = ::std::option::Option<
    unsafe extern "C" fn(
        __cookie: *mut ::std::os::raw::c_void,
        __pos: *mut __off64_t,
        __w: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,
>;
pub type cookie_close_function_t = ::std::option::Option<
    unsafe extern "C" fn(__cookie: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_cookie_io_functions_t {
    pub read: cookie_read_function_t,
    pub write: cookie_write_function_t,
    pub seek: cookie_seek_function_t,
    pub close: cookie_close_function_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of _IO_cookie_io_functions_t",
    ][::std::mem::size_of::<_IO_cookie_io_functions_t>() - 32usize];
    [
        "Alignment of _IO_cookie_io_functions_t",
    ][::std::mem::align_of::<_IO_cookie_io_functions_t>() - 8usize];
    [
        "Offset of field: _IO_cookie_io_functions_t::read",
    ][::std::mem::offset_of!(_IO_cookie_io_functions_t, read) - 0usize];
    [
        "Offset of field: _IO_cookie_io_functions_t::write",
    ][::std::mem::offset_of!(_IO_cookie_io_functions_t, write) - 8usize];
    [
        "Offset of field: _IO_cookie_io_functions_t::seek",
    ][::std::mem::offset_of!(_IO_cookie_io_functions_t, seek) - 16usize];
    [
        "Offset of field: _IO_cookie_io_functions_t::close",
    ][::std::mem::offset_of!(_IO_cookie_io_functions_t, close) - 24usize];
};
pub type cookie_io_functions_t = _IO_cookie_io_functions_t;
pub type va_list = __gnuc_va_list;
pub type off_t = __off_t;
pub type off64_t = __off64_t;
pub type fpos_t = __fpos_t;
pub type fpos64_t = __fpos64_t;
unsafe extern "C" {
    pub static mut stdin: *mut FILE;
    pub static mut stdout: *mut FILE;
    pub static mut stderr: *mut FILE;
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn renameat2(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
        __flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpfile64() -> *mut FILE;
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fcloseall() -> ::std::os::raw::c_int;
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    pub fn fopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn freopen64(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    pub fn fdopen(
        __fd: ::std::os::raw::c_int,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn fopencookie(
        __magic_cookie: *mut ::std::os::raw::c_void,
        __modes: *const ::std::os::raw::c_char,
        __io_funcs: cookie_io_functions_t,
    ) -> *mut FILE;
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
    pub fn setbuffer(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __size: usize,
    );
    pub fn setlinebuf(__stream: *mut FILE);
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: usize,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn vasprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __f: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn __asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn asprintf(
        __ptr: *mut *mut ::std::os::raw::c_char,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
unsafe extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar() -> ::std::os::raw::c_int;
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fputc(
        __c: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn putc(
        __c: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn fputc_unlocked(
        __c: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn putc_unlocked(
        __c: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn putw(
        __w: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
    pub fn fgets_unlocked(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
    pub fn fputs(
        __s: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    pub fn ungetc(
        __c: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __s: *mut FILE,
    ) -> usize;
    pub fn fputs_unlocked(
        __s: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
    pub fn rewind(__stream: *mut FILE);
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftello(__stream: *mut FILE) -> __off_t;
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
    pub fn fseeko64(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn ftello64(__stream: *mut FILE) -> __off64_t;
    pub fn fgetpos64(__stream: *mut FILE, __pos: *mut fpos64_t) -> ::std::os::raw::c_int;
    pub fn fsetpos64(
        __stream: *mut FILE,
        __pos: *const fpos64_t,
    ) -> ::std::os::raw::c_int;
    pub fn clearerr(__stream: *mut FILE);
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn clearerr_unlocked(__stream: *mut FILE);
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn perror(__s: *const ::std::os::raw::c_char);
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
    pub fn cuserid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct obstack {
    _unused: [u8; 0],
}
unsafe extern "C" {
    pub fn obstack_printf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
    pub fn obstack_vprintf(
        __obstack: *mut obstack,
        __format: *const ::std::os::raw::c_char,
        __args: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
    pub fn flockfile(__stream: *mut FILE);
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
    pub fn funlockfile(__stream: *mut FILE);
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
    pub fn __overflow(
        arg1: *mut FILE,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
pub const k_cbMaxGameServerGameDir: ::std::os::raw::c_int = 32;
pub const k_cbMaxGameServerMapName: ::std::os::raw::c_int = 32;
pub const k_cbMaxGameServerGameDescription: ::std::os::raw::c_int = 64;
pub const k_cbMaxGameServerName: ::std::os::raw::c_int = 64;
pub const k_cbMaxGameServerTags: ::std::os::raw::c_int = 128;
pub const k_cbMaxGameServerGameData: ::std::os::raw::c_int = 2048;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MatchMakingKeyValuePair_t {
    pub m_szKey: [::std::os::raw::c_char; 256usize],
    pub m_szValue: [::std::os::raw::c_char; 256usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MatchMakingKeyValuePair_t",
    ][::std::mem::size_of::<MatchMakingKeyValuePair_t>() - 512usize];
    [
        "Alignment of MatchMakingKeyValuePair_t",
    ][::std::mem::align_of::<MatchMakingKeyValuePair_t>() - 1usize];
    [
        "Offset of field: MatchMakingKeyValuePair_t::m_szKey",
    ][::std::mem::offset_of!(MatchMakingKeyValuePair_t, m_szKey) - 0usize];
    [
        "Offset of field: MatchMakingKeyValuePair_t::m_szValue",
    ][::std::mem::offset_of!(MatchMakingKeyValuePair_t, m_szValue) - 256usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EMatchMakingServerResponse {
    eServerResponded = 0,
    eServerFailedToRespond = 1,
    eNoServersListedOnMasterServer = 2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct servernetadr_t {
    pub m_usConnectionPort: uint16,
    pub m_usQueryPort: uint16,
    pub m_unIP: uint32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of servernetadr_t"][::std::mem::size_of::<servernetadr_t>() - 8usize];
    ["Alignment of servernetadr_t"][::std::mem::align_of::<servernetadr_t>() - 4usize];
    [
        "Offset of field: servernetadr_t::m_usConnectionPort",
    ][::std::mem::offset_of!(servernetadr_t, m_usConnectionPort) - 0usize];
    [
        "Offset of field: servernetadr_t::m_usQueryPort",
    ][::std::mem::offset_of!(servernetadr_t, m_usQueryPort) - 2usize];
    [
        "Offset of field: servernetadr_t::m_unIP",
    ][::std::mem::offset_of!(servernetadr_t, m_unIP) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gameserveritem_t {
    pub m_NetAdr: servernetadr_t,
    pub m_nPing: ::std::os::raw::c_int,
    pub m_bHadSuccessfulResponse: bool,
    pub m_bDoNotRefresh: bool,
    pub m_szGameDir: [::std::os::raw::c_char; 32usize],
    pub m_szMap: [::std::os::raw::c_char; 32usize],
    pub m_szGameDescription: [::std::os::raw::c_char; 64usize],
    pub m_nAppID: uint32,
    pub m_nPlayers: ::std::os::raw::c_int,
    pub m_nMaxPlayers: ::std::os::raw::c_int,
    pub m_nBotPlayers: ::std::os::raw::c_int,
    pub m_bPassword: bool,
    pub m_bSecure: bool,
    pub m_ulTimeLastPlayed: uint32,
    pub m_nServerVersion: ::std::os::raw::c_int,
    pub m_szServerName: [::std::os::raw::c_char; 64usize],
    pub m_szGameTags: [::std::os::raw::c_char; 128usize],
    pub m_steamID: CSteamID,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gameserveritem_t"][::std::mem::size_of::<gameserveritem_t>() - 372usize];
    [
        "Alignment of gameserveritem_t",
    ][::std::mem::align_of::<gameserveritem_t>() - 4usize];
    [
        "Offset of field: gameserveritem_t::m_NetAdr",
    ][::std::mem::offset_of!(gameserveritem_t, m_NetAdr) - 0usize];
    [
        "Offset of field: gameserveritem_t::m_nPing",
    ][::std::mem::offset_of!(gameserveritem_t, m_nPing) - 8usize];
    [
        "Offset of field: gameserveritem_t::m_bHadSuccessfulResponse",
    ][::std::mem::offset_of!(gameserveritem_t, m_bHadSuccessfulResponse) - 12usize];
    [
        "Offset of field: gameserveritem_t::m_bDoNotRefresh",
    ][::std::mem::offset_of!(gameserveritem_t, m_bDoNotRefresh) - 13usize];
    [
        "Offset of field: gameserveritem_t::m_szGameDir",
    ][::std::mem::offset_of!(gameserveritem_t, m_szGameDir) - 14usize];
    [
        "Offset of field: gameserveritem_t::m_szMap",
    ][::std::mem::offset_of!(gameserveritem_t, m_szMap) - 46usize];
    [
        "Offset of field: gameserveritem_t::m_szGameDescription",
    ][::std::mem::offset_of!(gameserveritem_t, m_szGameDescription) - 78usize];
    [
        "Offset of field: gameserveritem_t::m_nAppID",
    ][::std::mem::offset_of!(gameserveritem_t, m_nAppID) - 144usize];
    [
        "Offset of field: gameserveritem_t::m_nPlayers",
    ][::std::mem::offset_of!(gameserveritem_t, m_nPlayers) - 148usize];
    [
        "Offset of field: gameserveritem_t::m_nMaxPlayers",
    ][::std::mem::offset_of!(gameserveritem_t, m_nMaxPlayers) - 152usize];
    [
        "Offset of field: gameserveritem_t::m_nBotPlayers",
    ][::std::mem::offset_of!(gameserveritem_t, m_nBotPlayers) - 156usize];
    [
        "Offset of field: gameserveritem_t::m_bPassword",
    ][::std::mem::offset_of!(gameserveritem_t, m_bPassword) - 160usize];
    [
        "Offset of field: gameserveritem_t::m_bSecure",
    ][::std::mem::offset_of!(gameserveritem_t, m_bSecure) - 161usize];
    [
        "Offset of field: gameserveritem_t::m_ulTimeLastPlayed",
    ][::std::mem::offset_of!(gameserveritem_t, m_ulTimeLastPlayed) - 164usize];
    [
        "Offset of field: gameserveritem_t::m_nServerVersion",
    ][::std::mem::offset_of!(gameserveritem_t, m_nServerVersion) - 168usize];
    [
        "Offset of field: gameserveritem_t::m_szServerName",
    ][::std::mem::offset_of!(gameserveritem_t, m_szServerName) - 172usize];
    [
        "Offset of field: gameserveritem_t::m_szGameTags",
    ][::std::mem::offset_of!(gameserveritem_t, m_szGameTags) - 236usize];
    [
        "Offset of field: gameserveritem_t::m_steamID",
    ][::std::mem::offset_of!(gameserveritem_t, m_steamID) - 364usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELobbyType {
    k_ELobbyTypePrivate = 0,
    k_ELobbyTypeFriendsOnly = 1,
    k_ELobbyTypePublic = 2,
    k_ELobbyTypeInvisible = 3,
    k_ELobbyTypePrivateUnique = 4,
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELobbyComparison {
    k_ELobbyComparisonEqualToOrLessThan = -2,
    k_ELobbyComparisonLessThan = -1,
    k_ELobbyComparisonEqual = 0,
    k_ELobbyComparisonGreaterThan = 1,
    k_ELobbyComparisonEqualToOrGreaterThan = 2,
    k_ELobbyComparisonNotEqual = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELobbyDistanceFilter {
    k_ELobbyDistanceFilterClose = 0,
    k_ELobbyDistanceFilterDefault = 1,
    k_ELobbyDistanceFilterFar = 2,
    k_ELobbyDistanceFilterWorldwide = 3,
}
#[repr(C)]
pub struct ISteamMatchmaking__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmaking {
    pub vtable_: *const ISteamMatchmaking__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamMatchmaking"][::std::mem::size_of::<ISteamMatchmaking>() - 8usize];
    [
        "Alignment of ISteamMatchmaking",
    ][::std::mem::align_of::<ISteamMatchmaking>() - 8usize];
};
pub type HServerListRequest = *mut ::std::os::raw::c_void;
#[repr(C)]
pub struct ISteamMatchmakingServerListResponse__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmakingServerListResponse {
    pub vtable_: *const ISteamMatchmakingServerListResponse__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamMatchmakingServerListResponse",
    ][::std::mem::size_of::<ISteamMatchmakingServerListResponse>() - 8usize];
    [
        "Alignment of ISteamMatchmakingServerListResponse",
    ][::std::mem::align_of::<ISteamMatchmakingServerListResponse>() - 8usize];
};
#[repr(C)]
pub struct ISteamMatchmakingPingResponse__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmakingPingResponse {
    pub vtable_: *const ISteamMatchmakingPingResponse__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamMatchmakingPingResponse",
    ][::std::mem::size_of::<ISteamMatchmakingPingResponse>() - 8usize];
    [
        "Alignment of ISteamMatchmakingPingResponse",
    ][::std::mem::align_of::<ISteamMatchmakingPingResponse>() - 8usize];
};
#[repr(C)]
pub struct ISteamMatchmakingPlayersResponse__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmakingPlayersResponse {
    pub vtable_: *const ISteamMatchmakingPlayersResponse__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamMatchmakingPlayersResponse",
    ][::std::mem::size_of::<ISteamMatchmakingPlayersResponse>() - 8usize];
    [
        "Alignment of ISteamMatchmakingPlayersResponse",
    ][::std::mem::align_of::<ISteamMatchmakingPlayersResponse>() - 8usize];
};
#[repr(C)]
pub struct ISteamMatchmakingRulesResponse__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmakingRulesResponse {
    pub vtable_: *const ISteamMatchmakingRulesResponse__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamMatchmakingRulesResponse",
    ][::std::mem::size_of::<ISteamMatchmakingRulesResponse>() - 8usize];
    [
        "Alignment of ISteamMatchmakingRulesResponse",
    ][::std::mem::align_of::<ISteamMatchmakingRulesResponse>() - 8usize];
};
pub type HServerQuery = ::std::os::raw::c_int;
pub const HSERVERQUERY_INVALID: ::std::os::raw::c_int = -1;
#[repr(C)]
pub struct ISteamMatchmakingServers__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMatchmakingServers {
    pub vtable_: *const ISteamMatchmakingServers__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamMatchmakingServers",
    ][::std::mem::size_of::<ISteamMatchmakingServers>() - 8usize];
    [
        "Alignment of ISteamMatchmakingServers",
    ][::std::mem::align_of::<ISteamMatchmakingServers>() - 8usize];
};
pub const k_unFavoriteFlagNone: uint32 = 0;
pub const k_unFavoriteFlagFavorite: uint32 = 1;
pub const k_unFavoriteFlagHistory: uint32 = 2;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EChatMemberStateChange {
    k_EChatMemberStateChangeEntered = 1,
    k_EChatMemberStateChangeLeft = 2,
    k_EChatMemberStateChangeDisconnected = 4,
    k_EChatMemberStateChangeKicked = 8,
    k_EChatMemberStateChangeBanned = 16,
}
#[repr(C)]
pub struct ISteamGameSearch__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamGameSearch {
    pub vtable_: *const ISteamGameSearch__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamGameSearch"][::std::mem::size_of::<ISteamGameSearch>() - 8usize];
    [
        "Alignment of ISteamGameSearch",
    ][::std::mem::align_of::<ISteamGameSearch>() - 8usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamPartyBeaconLocationType {
    k_ESteamPartyBeaconLocationType_Invalid = 0,
    k_ESteamPartyBeaconLocationType_ChatGroup = 1,
    k_ESteamPartyBeaconLocationType_Max = 2,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamPartyBeaconLocation_t {
    pub m_eType: ESteamPartyBeaconLocationType,
    pub m_ulLocationID: uint64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamPartyBeaconLocation_t",
    ][::std::mem::size_of::<SteamPartyBeaconLocation_t>() - 12usize];
    [
        "Alignment of SteamPartyBeaconLocation_t",
    ][::std::mem::align_of::<SteamPartyBeaconLocation_t>() - 4usize];
    [
        "Offset of field: SteamPartyBeaconLocation_t::m_eType",
    ][::std::mem::offset_of!(SteamPartyBeaconLocation_t, m_eType) - 0usize];
    [
        "Offset of field: SteamPartyBeaconLocation_t::m_ulLocationID",
    ][::std::mem::offset_of!(SteamPartyBeaconLocation_t, m_ulLocationID) - 4usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamPartyBeaconLocationData {
    k_ESteamPartyBeaconLocationDataInvalid = 0,
    k_ESteamPartyBeaconLocationDataName = 1,
    k_ESteamPartyBeaconLocationDataIconURLSmall = 2,
    k_ESteamPartyBeaconLocationDataIconURLMedium = 3,
    k_ESteamPartyBeaconLocationDataIconURLLarge = 4,
}
#[repr(C)]
pub struct ISteamParties__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamParties {
    pub vtable_: *const ISteamParties__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamParties"][::std::mem::size_of::<ISteamParties>() - 8usize];
    ["Alignment of ISteamParties"][::std::mem::align_of::<ISteamParties>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FavoritesListChanged_t {
    pub m_nIP: uint32,
    pub m_nQueryPort: uint32,
    pub m_nConnPort: uint32,
    pub m_nAppID: uint32,
    pub m_nFlags: uint32,
    pub m_bAdd: bool,
    pub m_unAccountId: AccountID_t,
}
pub const FavoritesListChanged_t_k_iCallback: FavoritesListChanged_t__bindgen_ty_1 = FavoritesListChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FavoritesListChanged_t__bindgen_ty_1 {
    k_iCallback = 502,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FavoritesListChanged_t",
    ][::std::mem::size_of::<FavoritesListChanged_t>() - 28usize];
    [
        "Alignment of FavoritesListChanged_t",
    ][::std::mem::align_of::<FavoritesListChanged_t>() - 4usize];
    [
        "Offset of field: FavoritesListChanged_t::m_nIP",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_nIP) - 0usize];
    [
        "Offset of field: FavoritesListChanged_t::m_nQueryPort",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_nQueryPort) - 4usize];
    [
        "Offset of field: FavoritesListChanged_t::m_nConnPort",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_nConnPort) - 8usize];
    [
        "Offset of field: FavoritesListChanged_t::m_nAppID",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_nAppID) - 12usize];
    [
        "Offset of field: FavoritesListChanged_t::m_nFlags",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_nFlags) - 16usize];
    [
        "Offset of field: FavoritesListChanged_t::m_bAdd",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_bAdd) - 20usize];
    [
        "Offset of field: FavoritesListChanged_t::m_unAccountId",
    ][::std::mem::offset_of!(FavoritesListChanged_t, m_unAccountId) - 24usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyInvite_t {
    pub m_ulSteamIDUser: uint64,
    pub m_ulSteamIDLobby: uint64,
    pub m_ulGameID: uint64,
}
pub const LobbyInvite_t_k_iCallback: LobbyInvite_t__bindgen_ty_1 = LobbyInvite_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyInvite_t__bindgen_ty_1 {
    k_iCallback = 503,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyInvite_t"][::std::mem::size_of::<LobbyInvite_t>() - 24usize];
    ["Alignment of LobbyInvite_t"][::std::mem::align_of::<LobbyInvite_t>() - 4usize];
    [
        "Offset of field: LobbyInvite_t::m_ulSteamIDUser",
    ][::std::mem::offset_of!(LobbyInvite_t, m_ulSteamIDUser) - 0usize];
    [
        "Offset of field: LobbyInvite_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyInvite_t, m_ulSteamIDLobby) - 8usize];
    [
        "Offset of field: LobbyInvite_t::m_ulGameID",
    ][::std::mem::offset_of!(LobbyInvite_t, m_ulGameID) - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyEnter_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_rgfChatPermissions: uint32,
    pub m_bLocked: bool,
    pub m_EChatRoomEnterResponse: uint32,
}
pub const LobbyEnter_t_k_iCallback: LobbyEnter_t__bindgen_ty_1 = LobbyEnter_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyEnter_t__bindgen_ty_1 {
    k_iCallback = 504,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyEnter_t"][::std::mem::size_of::<LobbyEnter_t>() - 20usize];
    ["Alignment of LobbyEnter_t"][::std::mem::align_of::<LobbyEnter_t>() - 4usize];
    [
        "Offset of field: LobbyEnter_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyEnter_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyEnter_t::m_rgfChatPermissions",
    ][::std::mem::offset_of!(LobbyEnter_t, m_rgfChatPermissions) - 8usize];
    [
        "Offset of field: LobbyEnter_t::m_bLocked",
    ][::std::mem::offset_of!(LobbyEnter_t, m_bLocked) - 12usize];
    [
        "Offset of field: LobbyEnter_t::m_EChatRoomEnterResponse",
    ][::std::mem::offset_of!(LobbyEnter_t, m_EChatRoomEnterResponse) - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyDataUpdate_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_ulSteamIDMember: uint64,
    pub m_bSuccess: uint8,
}
pub const LobbyDataUpdate_t_k_iCallback: LobbyDataUpdate_t__bindgen_ty_1 = LobbyDataUpdate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyDataUpdate_t__bindgen_ty_1 {
    k_iCallback = 505,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyDataUpdate_t"][::std::mem::size_of::<LobbyDataUpdate_t>() - 20usize];
    [
        "Alignment of LobbyDataUpdate_t",
    ][::std::mem::align_of::<LobbyDataUpdate_t>() - 4usize];
    [
        "Offset of field: LobbyDataUpdate_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyDataUpdate_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyDataUpdate_t::m_ulSteamIDMember",
    ][::std::mem::offset_of!(LobbyDataUpdate_t, m_ulSteamIDMember) - 8usize];
    [
        "Offset of field: LobbyDataUpdate_t::m_bSuccess",
    ][::std::mem::offset_of!(LobbyDataUpdate_t, m_bSuccess) - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyChatUpdate_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_ulSteamIDUserChanged: uint64,
    pub m_ulSteamIDMakingChange: uint64,
    pub m_rgfChatMemberStateChange: uint32,
}
pub const LobbyChatUpdate_t_k_iCallback: LobbyChatUpdate_t__bindgen_ty_1 = LobbyChatUpdate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyChatUpdate_t__bindgen_ty_1 {
    k_iCallback = 506,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyChatUpdate_t"][::std::mem::size_of::<LobbyChatUpdate_t>() - 28usize];
    [
        "Alignment of LobbyChatUpdate_t",
    ][::std::mem::align_of::<LobbyChatUpdate_t>() - 4usize];
    [
        "Offset of field: LobbyChatUpdate_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyChatUpdate_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyChatUpdate_t::m_ulSteamIDUserChanged",
    ][::std::mem::offset_of!(LobbyChatUpdate_t, m_ulSteamIDUserChanged) - 8usize];
    [
        "Offset of field: LobbyChatUpdate_t::m_ulSteamIDMakingChange",
    ][::std::mem::offset_of!(LobbyChatUpdate_t, m_ulSteamIDMakingChange) - 16usize];
    [
        "Offset of field: LobbyChatUpdate_t::m_rgfChatMemberStateChange",
    ][::std::mem::offset_of!(LobbyChatUpdate_t, m_rgfChatMemberStateChange) - 24usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyChatMsg_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_ulSteamIDUser: uint64,
    pub m_eChatEntryType: uint8,
    pub m_iChatID: uint32,
}
pub const LobbyChatMsg_t_k_iCallback: LobbyChatMsg_t__bindgen_ty_1 = LobbyChatMsg_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyChatMsg_t__bindgen_ty_1 {
    k_iCallback = 507,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyChatMsg_t"][::std::mem::size_of::<LobbyChatMsg_t>() - 24usize];
    ["Alignment of LobbyChatMsg_t"][::std::mem::align_of::<LobbyChatMsg_t>() - 4usize];
    [
        "Offset of field: LobbyChatMsg_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyChatMsg_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyChatMsg_t::m_ulSteamIDUser",
    ][::std::mem::offset_of!(LobbyChatMsg_t, m_ulSteamIDUser) - 8usize];
    [
        "Offset of field: LobbyChatMsg_t::m_eChatEntryType",
    ][::std::mem::offset_of!(LobbyChatMsg_t, m_eChatEntryType) - 16usize];
    [
        "Offset of field: LobbyChatMsg_t::m_iChatID",
    ][::std::mem::offset_of!(LobbyChatMsg_t, m_iChatID) - 20usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyGameCreated_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_ulSteamIDGameServer: uint64,
    pub m_unIP: uint32,
    pub m_usPort: uint16,
}
pub const LobbyGameCreated_t_k_iCallback: LobbyGameCreated_t__bindgen_ty_1 = LobbyGameCreated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyGameCreated_t__bindgen_ty_1 {
    k_iCallback = 509,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LobbyGameCreated_t",
    ][::std::mem::size_of::<LobbyGameCreated_t>() - 24usize];
    [
        "Alignment of LobbyGameCreated_t",
    ][::std::mem::align_of::<LobbyGameCreated_t>() - 4usize];
    [
        "Offset of field: LobbyGameCreated_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyGameCreated_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyGameCreated_t::m_ulSteamIDGameServer",
    ][::std::mem::offset_of!(LobbyGameCreated_t, m_ulSteamIDGameServer) - 8usize];
    [
        "Offset of field: LobbyGameCreated_t::m_unIP",
    ][::std::mem::offset_of!(LobbyGameCreated_t, m_unIP) - 16usize];
    [
        "Offset of field: LobbyGameCreated_t::m_usPort",
    ][::std::mem::offset_of!(LobbyGameCreated_t, m_usPort) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LobbyMatchList_t {
    pub m_nLobbiesMatching: uint32,
}
pub const LobbyMatchList_t_k_iCallback: LobbyMatchList_t__bindgen_ty_1 = LobbyMatchList_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyMatchList_t__bindgen_ty_1 {
    k_iCallback = 510,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyMatchList_t"][::std::mem::size_of::<LobbyMatchList_t>() - 4usize];
    [
        "Alignment of LobbyMatchList_t",
    ][::std::mem::align_of::<LobbyMatchList_t>() - 4usize];
    [
        "Offset of field: LobbyMatchList_t::m_nLobbiesMatching",
    ][::std::mem::offset_of!(LobbyMatchList_t, m_nLobbiesMatching) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyKicked_t {
    pub m_ulSteamIDLobby: uint64,
    pub m_ulSteamIDAdmin: uint64,
    pub m_bKickedDueToDisconnect: uint8,
}
pub const LobbyKicked_t_k_iCallback: LobbyKicked_t__bindgen_ty_1 = LobbyKicked_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyKicked_t__bindgen_ty_1 {
    k_iCallback = 512,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyKicked_t"][::std::mem::size_of::<LobbyKicked_t>() - 20usize];
    ["Alignment of LobbyKicked_t"][::std::mem::align_of::<LobbyKicked_t>() - 4usize];
    [
        "Offset of field: LobbyKicked_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyKicked_t, m_ulSteamIDLobby) - 0usize];
    [
        "Offset of field: LobbyKicked_t::m_ulSteamIDAdmin",
    ][::std::mem::offset_of!(LobbyKicked_t, m_ulSteamIDAdmin) - 8usize];
    [
        "Offset of field: LobbyKicked_t::m_bKickedDueToDisconnect",
    ][::std::mem::offset_of!(LobbyKicked_t, m_bKickedDueToDisconnect) - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LobbyCreated_t {
    pub m_eResult: EResult,
    pub m_ulSteamIDLobby: uint64,
}
pub const LobbyCreated_t_k_iCallback: LobbyCreated_t__bindgen_ty_1 = LobbyCreated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LobbyCreated_t__bindgen_ty_1 {
    k_iCallback = 513,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of LobbyCreated_t"][::std::mem::size_of::<LobbyCreated_t>() - 12usize];
    ["Alignment of LobbyCreated_t"][::std::mem::align_of::<LobbyCreated_t>() - 4usize];
    [
        "Offset of field: LobbyCreated_t::m_eResult",
    ][::std::mem::offset_of!(LobbyCreated_t, m_eResult) - 0usize];
    [
        "Offset of field: LobbyCreated_t::m_ulSteamIDLobby",
    ][::std::mem::offset_of!(LobbyCreated_t, m_ulSteamIDLobby) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FavoritesListAccountsUpdated_t {
    pub m_eResult: EResult,
}
pub const FavoritesListAccountsUpdated_t_k_iCallback: FavoritesListAccountsUpdated_t__bindgen_ty_1 = FavoritesListAccountsUpdated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FavoritesListAccountsUpdated_t__bindgen_ty_1 {
    k_iCallback = 516,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FavoritesListAccountsUpdated_t",
    ][::std::mem::size_of::<FavoritesListAccountsUpdated_t>() - 4usize];
    [
        "Alignment of FavoritesListAccountsUpdated_t",
    ][::std::mem::align_of::<FavoritesListAccountsUpdated_t>() - 4usize];
    [
        "Offset of field: FavoritesListAccountsUpdated_t::m_eResult",
    ][::std::mem::offset_of!(FavoritesListAccountsUpdated_t, m_eResult) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct SearchForGameProgressCallback_t {
    pub m_ullSearchID: uint64,
    pub m_eResult: EResult,
    pub m_lobbyID: CSteamID,
    pub m_steamIDEndedSearch: CSteamID,
    pub m_nSecondsRemainingEstimate: int32,
    pub m_cPlayersSearching: int32,
}
pub const SearchForGameProgressCallback_t_k_iCallback: SearchForGameProgressCallback_t__bindgen_ty_1 = SearchForGameProgressCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SearchForGameProgressCallback_t__bindgen_ty_1 {
    k_iCallback = 5201,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SearchForGameProgressCallback_t",
    ][::std::mem::size_of::<SearchForGameProgressCallback_t>() - 36usize];
    [
        "Alignment of SearchForGameProgressCallback_t",
    ][::std::mem::align_of::<SearchForGameProgressCallback_t>() - 4usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_ullSearchID",
    ][::std::mem::offset_of!(SearchForGameProgressCallback_t, m_ullSearchID) - 0usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_eResult",
    ][::std::mem::offset_of!(SearchForGameProgressCallback_t, m_eResult) - 8usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_lobbyID",
    ][::std::mem::offset_of!(SearchForGameProgressCallback_t, m_lobbyID) - 12usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_steamIDEndedSearch",
    ][::std::mem::offset_of!(SearchForGameProgressCallback_t, m_steamIDEndedSearch)
        - 20usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_nSecondsRemainingEstimate",
    ][::std::mem::offset_of!(
        SearchForGameProgressCallback_t, m_nSecondsRemainingEstimate
    ) - 28usize];
    [
        "Offset of field: SearchForGameProgressCallback_t::m_cPlayersSearching",
    ][::std::mem::offset_of!(SearchForGameProgressCallback_t, m_cPlayersSearching)
        - 32usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct SearchForGameResultCallback_t {
    pub m_ullSearchID: uint64,
    pub m_eResult: EResult,
    pub m_nCountPlayersInGame: int32,
    pub m_nCountAcceptedGame: int32,
    pub m_steamIDHost: CSteamID,
    pub m_bFinalCallback: bool,
}
pub const SearchForGameResultCallback_t_k_iCallback: SearchForGameResultCallback_t__bindgen_ty_1 = SearchForGameResultCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SearchForGameResultCallback_t__bindgen_ty_1 {
    k_iCallback = 5202,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SearchForGameResultCallback_t",
    ][::std::mem::size_of::<SearchForGameResultCallback_t>() - 32usize];
    [
        "Alignment of SearchForGameResultCallback_t",
    ][::std::mem::align_of::<SearchForGameResultCallback_t>() - 4usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_ullSearchID",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_ullSearchID) - 0usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_eResult",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_eResult) - 8usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_nCountPlayersInGame",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_nCountPlayersInGame)
        - 12usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_nCountAcceptedGame",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_nCountAcceptedGame)
        - 16usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_steamIDHost",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_steamIDHost) - 20usize];
    [
        "Offset of field: SearchForGameResultCallback_t::m_bFinalCallback",
    ][::std::mem::offset_of!(SearchForGameResultCallback_t, m_bFinalCallback) - 28usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RequestPlayersForGameProgressCallback_t {
    pub m_eResult: EResult,
    pub m_ullSearchID: uint64,
}
pub const RequestPlayersForGameProgressCallback_t_k_iCallback: RequestPlayersForGameProgressCallback_t__bindgen_ty_1 = RequestPlayersForGameProgressCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RequestPlayersForGameProgressCallback_t__bindgen_ty_1 {
    k_iCallback = 5211,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RequestPlayersForGameProgressCallback_t",
    ][::std::mem::size_of::<RequestPlayersForGameProgressCallback_t>() - 12usize];
    [
        "Alignment of RequestPlayersForGameProgressCallback_t",
    ][::std::mem::align_of::<RequestPlayersForGameProgressCallback_t>() - 4usize];
    [
        "Offset of field: RequestPlayersForGameProgressCallback_t::m_eResult",
    ][::std::mem::offset_of!(RequestPlayersForGameProgressCallback_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RequestPlayersForGameProgressCallback_t::m_ullSearchID",
    ][::std::mem::offset_of!(RequestPlayersForGameProgressCallback_t, m_ullSearchID)
        - 4usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct RequestPlayersForGameResultCallback_t {
    pub m_eResult: EResult,
    pub m_ullSearchID: uint64,
    pub m_SteamIDPlayerFound: CSteamID,
    pub m_SteamIDLobby: CSteamID,
    pub m_ePlayerAcceptState: RequestPlayersForGameResultCallback_t_PlayerAcceptState_t,
    pub m_nPlayerIndex: int32,
    pub m_nTotalPlayersFound: int32,
    pub m_nTotalPlayersAcceptedGame: int32,
    pub m_nSuggestedTeamIndex: int32,
    pub m_ullUniqueGameID: uint64,
}
pub const RequestPlayersForGameResultCallback_t_k_iCallback: RequestPlayersForGameResultCallback_t__bindgen_ty_1 = RequestPlayersForGameResultCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RequestPlayersForGameResultCallback_t__bindgen_ty_1 {
    k_iCallback = 5212,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RequestPlayersForGameResultCallback_t_PlayerAcceptState_t {
    k_EStateUnknown = 0,
    k_EStatePlayerAccepted = 1,
    k_EStatePlayerDeclined = 2,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RequestPlayersForGameResultCallback_t",
    ][::std::mem::size_of::<RequestPlayersForGameResultCallback_t>() - 56usize];
    [
        "Alignment of RequestPlayersForGameResultCallback_t",
    ][::std::mem::align_of::<RequestPlayersForGameResultCallback_t>() - 4usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_eResult",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_eResult) - 0usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_ullSearchID",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_ullSearchID)
        - 4usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_SteamIDPlayerFound",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_SteamIDPlayerFound)
        - 12usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_SteamIDLobby",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_SteamIDLobby)
        - 20usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_ePlayerAcceptState",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_ePlayerAcceptState)
        - 28usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_nPlayerIndex",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_nPlayerIndex)
        - 32usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_nTotalPlayersFound",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_nTotalPlayersFound)
        - 36usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_nTotalPlayersAcceptedGame",
    ][::std::mem::offset_of!(
        RequestPlayersForGameResultCallback_t, m_nTotalPlayersAcceptedGame
    ) - 40usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_nSuggestedTeamIndex",
    ][::std::mem::offset_of!(
        RequestPlayersForGameResultCallback_t, m_nSuggestedTeamIndex
    ) - 44usize];
    [
        "Offset of field: RequestPlayersForGameResultCallback_t::m_ullUniqueGameID",
    ][::std::mem::offset_of!(RequestPlayersForGameResultCallback_t, m_ullUniqueGameID)
        - 48usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RequestPlayersForGameFinalResultCallback_t {
    pub m_eResult: EResult,
    pub m_ullSearchID: uint64,
    pub m_ullUniqueGameID: uint64,
}
pub const RequestPlayersForGameFinalResultCallback_t_k_iCallback: RequestPlayersForGameFinalResultCallback_t__bindgen_ty_1 = RequestPlayersForGameFinalResultCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RequestPlayersForGameFinalResultCallback_t__bindgen_ty_1 {
    k_iCallback = 5213,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RequestPlayersForGameFinalResultCallback_t",
    ][::std::mem::size_of::<RequestPlayersForGameFinalResultCallback_t>() - 20usize];
    [
        "Alignment of RequestPlayersForGameFinalResultCallback_t",
    ][::std::mem::align_of::<RequestPlayersForGameFinalResultCallback_t>() - 4usize];
    [
        "Offset of field: RequestPlayersForGameFinalResultCallback_t::m_eResult",
    ][::std::mem::offset_of!(RequestPlayersForGameFinalResultCallback_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RequestPlayersForGameFinalResultCallback_t::m_ullSearchID",
    ][::std::mem::offset_of!(RequestPlayersForGameFinalResultCallback_t, m_ullSearchID)
        - 4usize];
    [
        "Offset of field: RequestPlayersForGameFinalResultCallback_t::m_ullUniqueGameID",
    ][::std::mem::offset_of!(
        RequestPlayersForGameFinalResultCallback_t, m_ullUniqueGameID
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct SubmitPlayerResultResultCallback_t {
    pub m_eResult: EResult,
    pub ullUniqueGameID: uint64,
    pub steamIDPlayer: CSteamID,
}
pub const SubmitPlayerResultResultCallback_t_k_iCallback: SubmitPlayerResultResultCallback_t__bindgen_ty_1 = SubmitPlayerResultResultCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SubmitPlayerResultResultCallback_t__bindgen_ty_1 {
    k_iCallback = 5214,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SubmitPlayerResultResultCallback_t",
    ][::std::mem::size_of::<SubmitPlayerResultResultCallback_t>() - 20usize];
    [
        "Alignment of SubmitPlayerResultResultCallback_t",
    ][::std::mem::align_of::<SubmitPlayerResultResultCallback_t>() - 4usize];
    [
        "Offset of field: SubmitPlayerResultResultCallback_t::m_eResult",
    ][::std::mem::offset_of!(SubmitPlayerResultResultCallback_t, m_eResult) - 0usize];
    [
        "Offset of field: SubmitPlayerResultResultCallback_t::ullUniqueGameID",
    ][::std::mem::offset_of!(SubmitPlayerResultResultCallback_t, ullUniqueGameID)
        - 4usize];
    [
        "Offset of field: SubmitPlayerResultResultCallback_t::steamIDPlayer",
    ][::std::mem::offset_of!(SubmitPlayerResultResultCallback_t, steamIDPlayer)
        - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct EndGameResultCallback_t {
    pub m_eResult: EResult,
    pub ullUniqueGameID: uint64,
}
pub const EndGameResultCallback_t_k_iCallback: EndGameResultCallback_t__bindgen_ty_1 = EndGameResultCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EndGameResultCallback_t__bindgen_ty_1 {
    k_iCallback = 5215,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of EndGameResultCallback_t",
    ][::std::mem::size_of::<EndGameResultCallback_t>() - 12usize];
    [
        "Alignment of EndGameResultCallback_t",
    ][::std::mem::align_of::<EndGameResultCallback_t>() - 4usize];
    [
        "Offset of field: EndGameResultCallback_t::m_eResult",
    ][::std::mem::offset_of!(EndGameResultCallback_t, m_eResult) - 0usize];
    [
        "Offset of field: EndGameResultCallback_t::ullUniqueGameID",
    ][::std::mem::offset_of!(EndGameResultCallback_t, ullUniqueGameID) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct JoinPartyCallback_t {
    pub m_eResult: EResult,
    pub m_ulBeaconID: PartyBeaconID_t,
    pub m_SteamIDBeaconOwner: CSteamID,
    pub m_rgchConnectString: [::std::os::raw::c_char; 256usize],
}
pub const JoinPartyCallback_t_k_iCallback: JoinPartyCallback_t__bindgen_ty_1 = JoinPartyCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum JoinPartyCallback_t__bindgen_ty_1 {
    k_iCallback = 5301,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of JoinPartyCallback_t",
    ][::std::mem::size_of::<JoinPartyCallback_t>() - 276usize];
    [
        "Alignment of JoinPartyCallback_t",
    ][::std::mem::align_of::<JoinPartyCallback_t>() - 4usize];
    [
        "Offset of field: JoinPartyCallback_t::m_eResult",
    ][::std::mem::offset_of!(JoinPartyCallback_t, m_eResult) - 0usize];
    [
        "Offset of field: JoinPartyCallback_t::m_ulBeaconID",
    ][::std::mem::offset_of!(JoinPartyCallback_t, m_ulBeaconID) - 4usize];
    [
        "Offset of field: JoinPartyCallback_t::m_SteamIDBeaconOwner",
    ][::std::mem::offset_of!(JoinPartyCallback_t, m_SteamIDBeaconOwner) - 12usize];
    [
        "Offset of field: JoinPartyCallback_t::m_rgchConnectString",
    ][::std::mem::offset_of!(JoinPartyCallback_t, m_rgchConnectString) - 20usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CreateBeaconCallback_t {
    pub m_eResult: EResult,
    pub m_ulBeaconID: PartyBeaconID_t,
}
pub const CreateBeaconCallback_t_k_iCallback: CreateBeaconCallback_t__bindgen_ty_1 = CreateBeaconCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CreateBeaconCallback_t__bindgen_ty_1 {
    k_iCallback = 5302,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of CreateBeaconCallback_t",
    ][::std::mem::size_of::<CreateBeaconCallback_t>() - 12usize];
    [
        "Alignment of CreateBeaconCallback_t",
    ][::std::mem::align_of::<CreateBeaconCallback_t>() - 4usize];
    [
        "Offset of field: CreateBeaconCallback_t::m_eResult",
    ][::std::mem::offset_of!(CreateBeaconCallback_t, m_eResult) - 0usize];
    [
        "Offset of field: CreateBeaconCallback_t::m_ulBeaconID",
    ][::std::mem::offset_of!(CreateBeaconCallback_t, m_ulBeaconID) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct ReservationNotificationCallback_t {
    pub m_ulBeaconID: PartyBeaconID_t,
    pub m_steamIDJoiner: CSteamID,
}
pub const ReservationNotificationCallback_t_k_iCallback: ReservationNotificationCallback_t__bindgen_ty_1 = ReservationNotificationCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ReservationNotificationCallback_t__bindgen_ty_1 {
    k_iCallback = 5303,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ReservationNotificationCallback_t",
    ][::std::mem::size_of::<ReservationNotificationCallback_t>() - 16usize];
    [
        "Alignment of ReservationNotificationCallback_t",
    ][::std::mem::align_of::<ReservationNotificationCallback_t>() - 4usize];
    [
        "Offset of field: ReservationNotificationCallback_t::m_ulBeaconID",
    ][::std::mem::offset_of!(ReservationNotificationCallback_t, m_ulBeaconID) - 0usize];
    [
        "Offset of field: ReservationNotificationCallback_t::m_steamIDJoiner",
    ][::std::mem::offset_of!(ReservationNotificationCallback_t, m_steamIDJoiner)
        - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ChangeNumOpenSlotsCallback_t {
    pub m_eResult: EResult,
}
pub const ChangeNumOpenSlotsCallback_t_k_iCallback: ChangeNumOpenSlotsCallback_t__bindgen_ty_1 = ChangeNumOpenSlotsCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ChangeNumOpenSlotsCallback_t__bindgen_ty_1 {
    k_iCallback = 5304,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ChangeNumOpenSlotsCallback_t",
    ][::std::mem::size_of::<ChangeNumOpenSlotsCallback_t>() - 4usize];
    [
        "Alignment of ChangeNumOpenSlotsCallback_t",
    ][::std::mem::align_of::<ChangeNumOpenSlotsCallback_t>() - 4usize];
    [
        "Offset of field: ChangeNumOpenSlotsCallback_t::m_eResult",
    ][::std::mem::offset_of!(ChangeNumOpenSlotsCallback_t, m_eResult) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AvailableBeaconLocationsUpdated_t {
    pub _address: u8,
}
pub const AvailableBeaconLocationsUpdated_t_k_iCallback: AvailableBeaconLocationsUpdated_t__bindgen_ty_1 = AvailableBeaconLocationsUpdated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AvailableBeaconLocationsUpdated_t__bindgen_ty_1 {
    k_iCallback = 5305,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AvailableBeaconLocationsUpdated_t",
    ][::std::mem::size_of::<AvailableBeaconLocationsUpdated_t>() - 1usize];
    [
        "Alignment of AvailableBeaconLocationsUpdated_t",
    ][::std::mem::align_of::<AvailableBeaconLocationsUpdated_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ActiveBeaconsUpdated_t {
    pub _address: u8,
}
pub const ActiveBeaconsUpdated_t_k_iCallback: ActiveBeaconsUpdated_t__bindgen_ty_1 = ActiveBeaconsUpdated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ActiveBeaconsUpdated_t__bindgen_ty_1 {
    k_iCallback = 5306,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ActiveBeaconsUpdated_t",
    ][::std::mem::size_of::<ActiveBeaconsUpdated_t>() - 1usize];
    [
        "Alignment of ActiveBeaconsUpdated_t",
    ][::std::mem::align_of::<ActiveBeaconsUpdated_t>() - 1usize];
};
pub const k_unMaxCloudFileChunkSize: uint32 = 104857600;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamParamStringArray_t {
    pub m_ppStrings: *mut *const ::std::os::raw::c_char,
    pub m_nNumStrings: int32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamParamStringArray_t",
    ][::std::mem::size_of::<SteamParamStringArray_t>() - 12usize];
    [
        "Alignment of SteamParamStringArray_t",
    ][::std::mem::align_of::<SteamParamStringArray_t>() - 4usize];
    [
        "Offset of field: SteamParamStringArray_t::m_ppStrings",
    ][::std::mem::offset_of!(SteamParamStringArray_t, m_ppStrings) - 0usize];
    [
        "Offset of field: SteamParamStringArray_t::m_nNumStrings",
    ][::std::mem::offset_of!(SteamParamStringArray_t, m_nNumStrings) - 8usize];
};
pub type UGCHandle_t = uint64;
pub type PublishedFileUpdateHandle_t = uint64;
pub type PublishedFileId_t = uint64;
pub const k_PublishedFileIdInvalid: PublishedFileId_t = 0;
pub const k_UGCHandleInvalid: UGCHandle_t = 18446744073709551615;
pub const k_PublishedFileUpdateHandleInvalid: PublishedFileUpdateHandle_t = 18446744073709551615;
pub type UGCFileWriteStreamHandle_t = uint64;
pub const k_UGCFileStreamHandleInvalid: UGCFileWriteStreamHandle_t = 18446744073709551615;
pub const k_cchPublishedDocumentTitleMax: uint32 = 129;
pub const k_cchPublishedDocumentDescriptionMax: uint32 = 8000;
pub const k_cchPublishedDocumentChangeDescriptionMax: uint32 = 8000;
pub const k_unEnumeratePublishedFilesMaxResults: uint32 = 50;
pub const k_cchTagListMax: uint32 = 1025;
pub const k_cchFilenameMax: uint32 = 260;
pub const k_cchPublishedFileURLMax: uint32 = 256;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemoteStoragePlatform {
    k_ERemoteStoragePlatformNone = 0,
    k_ERemoteStoragePlatformWindows = 1,
    k_ERemoteStoragePlatformOSX = 2,
    k_ERemoteStoragePlatformPS3 = 4,
    k_ERemoteStoragePlatformLinux = 8,
    k_ERemoteStoragePlatformSwitch = 16,
    k_ERemoteStoragePlatformAndroid = 32,
    k_ERemoteStoragePlatformIOS = 64,
    k_ERemoteStoragePlatformAll = 4294967295,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemoteStoragePublishedFileVisibility {
    k_ERemoteStoragePublishedFileVisibilityPublic = 0,
    k_ERemoteStoragePublishedFileVisibilityFriendsOnly = 1,
    k_ERemoteStoragePublishedFileVisibilityPrivate = 2,
    k_ERemoteStoragePublishedFileVisibilityUnlisted = 3,
}
impl EWorkshopFileType {
    pub const k_EWorkshopFileTypeCommunity: EWorkshopFileType = EWorkshopFileType::k_EWorkshopFileTypeFirst;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EWorkshopFileType {
    k_EWorkshopFileTypeFirst = 0,
    k_EWorkshopFileTypeMicrotransaction = 1,
    k_EWorkshopFileTypeCollection = 2,
    k_EWorkshopFileTypeArt = 3,
    k_EWorkshopFileTypeVideo = 4,
    k_EWorkshopFileTypeScreenshot = 5,
    k_EWorkshopFileTypeGame = 6,
    k_EWorkshopFileTypeSoftware = 7,
    k_EWorkshopFileTypeConcept = 8,
    k_EWorkshopFileTypeWebGuide = 9,
    k_EWorkshopFileTypeIntegratedGuide = 10,
    k_EWorkshopFileTypeMerch = 11,
    k_EWorkshopFileTypeControllerBinding = 12,
    k_EWorkshopFileTypeSteamworksAccessInvite = 13,
    k_EWorkshopFileTypeSteamVideo = 14,
    k_EWorkshopFileTypeGameManagedItem = 15,
    k_EWorkshopFileTypeClip = 16,
    k_EWorkshopFileTypeMax = 17,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EWorkshopVote {
    k_EWorkshopVoteUnvoted = 0,
    k_EWorkshopVoteFor = 1,
    k_EWorkshopVoteAgainst = 2,
    k_EWorkshopVoteLater = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EWorkshopFileAction {
    k_EWorkshopFileActionPlayed = 0,
    k_EWorkshopFileActionCompleted = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EWorkshopEnumerationType {
    k_EWorkshopEnumerationTypeRankedByVote = 0,
    k_EWorkshopEnumerationTypeRecent = 1,
    k_EWorkshopEnumerationTypeTrending = 2,
    k_EWorkshopEnumerationTypeFavoritesOfFriends = 3,
    k_EWorkshopEnumerationTypeVotedByFriends = 4,
    k_EWorkshopEnumerationTypeContentByFriends = 5,
    k_EWorkshopEnumerationTypeRecentFromFollowedUsers = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EWorkshopVideoProvider {
    k_EWorkshopVideoProviderNone = 0,
    k_EWorkshopVideoProviderYoutube = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUGCReadAction {
    k_EUGCRead_ContinueReadingUntilFinished = 0,
    k_EUGCRead_ContinueReading = 1,
    k_EUGCRead_Close = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemoteStorageLocalFileChange {
    k_ERemoteStorageLocalFileChange_Invalid = 0,
    k_ERemoteStorageLocalFileChange_FileUpdated = 1,
    k_ERemoteStorageLocalFileChange_FileDeleted = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemoteStorageFilePathType {
    k_ERemoteStorageFilePathType_Invalid = 0,
    k_ERemoteStorageFilePathType_Absolute = 1,
    k_ERemoteStorageFilePathType_APIFilename = 2,
}
#[repr(C)]
pub struct ISteamRemoteStorage__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamRemoteStorage {
    pub vtable_: *const ISteamRemoteStorage__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamRemoteStorage",
    ][::std::mem::size_of::<ISteamRemoteStorage>() - 8usize];
    [
        "Alignment of ISteamRemoteStorage",
    ][::std::mem::align_of::<ISteamRemoteStorage>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageFileShareResult_t {
    pub m_eResult: EResult,
    pub m_hFile: UGCHandle_t,
    pub m_rgchFilename: [::std::os::raw::c_char; 260usize],
}
pub const RemoteStorageFileShareResult_t_k_iCallback: RemoteStorageFileShareResult_t__bindgen_ty_1 = RemoteStorageFileShareResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageFileShareResult_t__bindgen_ty_1 {
    k_iCallback = 1307,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageFileShareResult_t",
    ][::std::mem::size_of::<RemoteStorageFileShareResult_t>() - 272usize];
    [
        "Alignment of RemoteStorageFileShareResult_t",
    ][::std::mem::align_of::<RemoteStorageFileShareResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageFileShareResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageFileShareResult_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoteStorageFileShareResult_t::m_hFile",
    ][::std::mem::offset_of!(RemoteStorageFileShareResult_t, m_hFile) - 4usize];
    [
        "Offset of field: RemoteStorageFileShareResult_t::m_rgchFilename",
    ][::std::mem::offset_of!(RemoteStorageFileShareResult_t, m_rgchFilename) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishFileResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_bUserNeedsToAcceptWorkshopLegalAgreement: bool,
}
pub const RemoteStoragePublishFileResult_t_k_iCallback: RemoteStoragePublishFileResult_t__bindgen_ty_1 = RemoteStoragePublishFileResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishFileResult_t__bindgen_ty_1 {
    k_iCallback = 1309,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishFileResult_t",
    ][::std::mem::size_of::<RemoteStoragePublishFileResult_t>() - 16usize];
    [
        "Alignment of RemoteStoragePublishFileResult_t",
    ][::std::mem::align_of::<RemoteStoragePublishFileResult_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishFileResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStoragePublishFileResult_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoteStoragePublishFileResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoteStoragePublishFileResult_t, m_nPublishedFileId)
        - 4usize];
    [
        "Offset of field: RemoteStoragePublishFileResult_t::m_bUserNeedsToAcceptWorkshopLegalAgreement",
    ][::std::mem::offset_of!(
        RemoteStoragePublishFileResult_t, m_bUserNeedsToAcceptWorkshopLegalAgreement
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageDeletePublishedFileResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const RemoteStorageDeletePublishedFileResult_t_k_iCallback: RemoteStorageDeletePublishedFileResult_t__bindgen_ty_1 = RemoteStorageDeletePublishedFileResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageDeletePublishedFileResult_t__bindgen_ty_1 {
    k_iCallback = 1311,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageDeletePublishedFileResult_t",
    ][::std::mem::size_of::<RemoteStorageDeletePublishedFileResult_t>() - 12usize];
    [
        "Alignment of RemoteStorageDeletePublishedFileResult_t",
    ][::std::mem::align_of::<RemoteStorageDeletePublishedFileResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageDeletePublishedFileResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageDeletePublishedFileResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageDeletePublishedFileResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageDeletePublishedFileResult_t, m_nPublishedFileId
    ) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageEnumerateUserPublishedFilesResult_t {
    pub m_eResult: EResult,
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
    pub m_rgPublishedFileId: [PublishedFileId_t; 50usize],
}
pub const RemoteStorageEnumerateUserPublishedFilesResult_t_k_iCallback: RemoteStorageEnumerateUserPublishedFilesResult_t__bindgen_ty_1 = RemoteStorageEnumerateUserPublishedFilesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageEnumerateUserPublishedFilesResult_t__bindgen_ty_1 {
    k_iCallback = 1312,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageEnumerateUserPublishedFilesResult_t",
    ][::std::mem::size_of::<RemoteStorageEnumerateUserPublishedFilesResult_t>()
        - 412usize];
    [
        "Alignment of RemoteStorageEnumerateUserPublishedFilesResult_t",
    ][::std::mem::align_of::<RemoteStorageEnumerateUserPublishedFilesResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserPublishedFilesResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageEnumerateUserPublishedFilesResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageEnumerateUserPublishedFilesResult_t::m_nResultsReturned",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserPublishedFilesResult_t, m_nResultsReturned
    ) - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserPublishedFilesResult_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserPublishedFilesResult_t, m_nTotalResultCount
    ) - 8usize];
    [
        "Offset of field: RemoteStorageEnumerateUserPublishedFilesResult_t::m_rgPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserPublishedFilesResult_t, m_rgPublishedFileId
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageSubscribePublishedFileResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const RemoteStorageSubscribePublishedFileResult_t_k_iCallback: RemoteStorageSubscribePublishedFileResult_t__bindgen_ty_1 = RemoteStorageSubscribePublishedFileResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageSubscribePublishedFileResult_t__bindgen_ty_1 {
    k_iCallback = 1313,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageSubscribePublishedFileResult_t",
    ][::std::mem::size_of::<RemoteStorageSubscribePublishedFileResult_t>() - 12usize];
    [
        "Alignment of RemoteStorageSubscribePublishedFileResult_t",
    ][::std::mem::align_of::<RemoteStorageSubscribePublishedFileResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageSubscribePublishedFileResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageSubscribePublishedFileResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageSubscribePublishedFileResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageSubscribePublishedFileResult_t, m_nPublishedFileId
    ) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageEnumerateUserSubscribedFilesResult_t {
    pub m_eResult: EResult,
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
    pub m_rgPublishedFileId: [PublishedFileId_t; 50usize],
    pub m_rgRTimeSubscribed: [uint32; 50usize],
}
pub const RemoteStorageEnumerateUserSubscribedFilesResult_t_k_iCallback: RemoteStorageEnumerateUserSubscribedFilesResult_t__bindgen_ty_1 = RemoteStorageEnumerateUserSubscribedFilesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageEnumerateUserSubscribedFilesResult_t__bindgen_ty_1 {
    k_iCallback = 1314,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageEnumerateUserSubscribedFilesResult_t",
    ][::std::mem::size_of::<RemoteStorageEnumerateUserSubscribedFilesResult_t>()
        - 612usize];
    [
        "Alignment of RemoteStorageEnumerateUserSubscribedFilesResult_t",
    ][::std::mem::align_of::<RemoteStorageEnumerateUserSubscribedFilesResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSubscribedFilesResult_t::m_eResult",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSubscribedFilesResult_t, m_eResult
    ) - 0usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSubscribedFilesResult_t::m_nResultsReturned",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSubscribedFilesResult_t, m_nResultsReturned
    ) - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSubscribedFilesResult_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSubscribedFilesResult_t, m_nTotalResultCount
    ) - 8usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSubscribedFilesResult_t::m_rgPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSubscribedFilesResult_t, m_rgPublishedFileId
    ) - 12usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSubscribedFilesResult_t::m_rgRTimeSubscribed",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSubscribedFilesResult_t, m_rgRTimeSubscribed
    ) - 412usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageUnsubscribePublishedFileResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const RemoteStorageUnsubscribePublishedFileResult_t_k_iCallback: RemoteStorageUnsubscribePublishedFileResult_t__bindgen_ty_1 = RemoteStorageUnsubscribePublishedFileResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageUnsubscribePublishedFileResult_t__bindgen_ty_1 {
    k_iCallback = 1315,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageUnsubscribePublishedFileResult_t",
    ][::std::mem::size_of::<RemoteStorageUnsubscribePublishedFileResult_t>() - 12usize];
    [
        "Alignment of RemoteStorageUnsubscribePublishedFileResult_t",
    ][::std::mem::align_of::<RemoteStorageUnsubscribePublishedFileResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageUnsubscribePublishedFileResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageUnsubscribePublishedFileResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageUnsubscribePublishedFileResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageUnsubscribePublishedFileResult_t, m_nPublishedFileId
    ) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageUpdatePublishedFileResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_bUserNeedsToAcceptWorkshopLegalAgreement: bool,
}
pub const RemoteStorageUpdatePublishedFileResult_t_k_iCallback: RemoteStorageUpdatePublishedFileResult_t__bindgen_ty_1 = RemoteStorageUpdatePublishedFileResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageUpdatePublishedFileResult_t__bindgen_ty_1 {
    k_iCallback = 1316,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageUpdatePublishedFileResult_t",
    ][::std::mem::size_of::<RemoteStorageUpdatePublishedFileResult_t>() - 16usize];
    [
        "Alignment of RemoteStorageUpdatePublishedFileResult_t",
    ][::std::mem::align_of::<RemoteStorageUpdatePublishedFileResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageUpdatePublishedFileResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageUpdatePublishedFileResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageUpdatePublishedFileResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageUpdatePublishedFileResult_t, m_nPublishedFileId
    ) - 4usize];
    [
        "Offset of field: RemoteStorageUpdatePublishedFileResult_t::m_bUserNeedsToAcceptWorkshopLegalAgreement",
    ][::std::mem::offset_of!(
        RemoteStorageUpdatePublishedFileResult_t,
        m_bUserNeedsToAcceptWorkshopLegalAgreement
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageDownloadUGCResult_t {
    pub m_eResult: EResult,
    pub m_hFile: UGCHandle_t,
    pub m_nAppID: AppId_t,
    pub m_nSizeInBytes: int32,
    pub m_pchFileName: [::std::os::raw::c_char; 260usize],
    pub m_ulSteamIDOwner: uint64,
}
pub const RemoteStorageDownloadUGCResult_t_k_iCallback: RemoteStorageDownloadUGCResult_t__bindgen_ty_1 = RemoteStorageDownloadUGCResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageDownloadUGCResult_t__bindgen_ty_1 {
    k_iCallback = 1317,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageDownloadUGCResult_t",
    ][::std::mem::size_of::<RemoteStorageDownloadUGCResult_t>() - 288usize];
    [
        "Alignment of RemoteStorageDownloadUGCResult_t",
    ][::std::mem::align_of::<RemoteStorageDownloadUGCResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_hFile",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_hFile) - 4usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_nAppID",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_nAppID) - 12usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_nSizeInBytes",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_nSizeInBytes)
        - 16usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_pchFileName",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_pchFileName) - 20usize];
    [
        "Offset of field: RemoteStorageDownloadUGCResult_t::m_ulSteamIDOwner",
    ][::std::mem::offset_of!(RemoteStorageDownloadUGCResult_t, m_ulSteamIDOwner)
        - 280usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageGetPublishedFileDetailsResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nCreatorAppID: AppId_t,
    pub m_nConsumerAppID: AppId_t,
    pub m_rgchTitle: [::std::os::raw::c_char; 129usize],
    pub m_rgchDescription: [::std::os::raw::c_char; 8000usize],
    pub m_hFile: UGCHandle_t,
    pub m_hPreviewFile: UGCHandle_t,
    pub m_ulSteamIDOwner: uint64,
    pub m_rtimeCreated: uint32,
    pub m_rtimeUpdated: uint32,
    pub m_eVisibility: ERemoteStoragePublishedFileVisibility,
    pub m_bBanned: bool,
    pub m_rgchTags: [::std::os::raw::c_char; 1025usize],
    pub m_bTagsTruncated: bool,
    pub m_pchFileName: [::std::os::raw::c_char; 260usize],
    pub m_nFileSize: int32,
    pub m_nPreviewFileSize: int32,
    pub m_rgchURL: [::std::os::raw::c_char; 256usize],
    pub m_eFileType: EWorkshopFileType,
    pub m_bAcceptedForUse: bool,
}
pub const RemoteStorageGetPublishedFileDetailsResult_t_k_iCallback: RemoteStorageGetPublishedFileDetailsResult_t__bindgen_ty_1 = RemoteStorageGetPublishedFileDetailsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageGetPublishedFileDetailsResult_t__bindgen_ty_1 {
    k_iCallback = 1318,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageGetPublishedFileDetailsResult_t",
    ][::std::mem::size_of::<RemoteStorageGetPublishedFileDetailsResult_t>() - 9748usize];
    [
        "Alignment of RemoteStorageGetPublishedFileDetailsResult_t",
    ][::std::mem::align_of::<RemoteStorageGetPublishedFileDetailsResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_nPublishedFileId
    ) - 4usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_nCreatorAppID",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_nCreatorAppID
    ) - 12usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_nConsumerAppID",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_nConsumerAppID
    ) - 16usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rgchTitle",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_rgchTitle)
        - 20usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rgchDescription",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_rgchDescription
    ) - 149usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_hFile",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_hFile)
        - 8152usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_hPreviewFile",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_hPreviewFile
    ) - 8160usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_ulSteamIDOwner",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_ulSteamIDOwner
    ) - 8168usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rtimeCreated",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_rtimeCreated
    ) - 8176usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rtimeUpdated",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_rtimeUpdated
    ) - 8180usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_eVisibility",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_eVisibility)
        - 8184usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_bBanned",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_bBanned)
        - 8188usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rgchTags",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_rgchTags)
        - 8189usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_bTagsTruncated",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_bTagsTruncated
    ) - 9214usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_pchFileName",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_pchFileName)
        - 9215usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_nFileSize",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_nFileSize)
        - 9476usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_nPreviewFileSize",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_nPreviewFileSize
    ) - 9480usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_rgchURL",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_rgchURL)
        - 9484usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_eFileType",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedFileDetailsResult_t, m_eFileType)
        - 9740usize];
    [
        "Offset of field: RemoteStorageGetPublishedFileDetailsResult_t::m_bAcceptedForUse",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedFileDetailsResult_t, m_bAcceptedForUse
    ) - 9744usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageEnumerateWorkshopFilesResult_t {
    pub m_eResult: EResult,
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
    pub m_rgPublishedFileId: [PublishedFileId_t; 50usize],
    pub m_rgScore: [f32; 50usize],
    pub m_nAppId: AppId_t,
    pub m_unStartIndex: uint32,
}
pub const RemoteStorageEnumerateWorkshopFilesResult_t_k_iCallback: RemoteStorageEnumerateWorkshopFilesResult_t__bindgen_ty_1 = RemoteStorageEnumerateWorkshopFilesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageEnumerateWorkshopFilesResult_t__bindgen_ty_1 {
    k_iCallback = 1319,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageEnumerateWorkshopFilesResult_t",
    ][::std::mem::size_of::<RemoteStorageEnumerateWorkshopFilesResult_t>() - 620usize];
    [
        "Alignment of RemoteStorageEnumerateWorkshopFilesResult_t",
    ][::std::mem::align_of::<RemoteStorageEnumerateWorkshopFilesResult_t>() - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageEnumerateWorkshopFilesResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_nResultsReturned",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateWorkshopFilesResult_t, m_nResultsReturned
    ) - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateWorkshopFilesResult_t, m_nTotalResultCount
    ) - 8usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_rgPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateWorkshopFilesResult_t, m_rgPublishedFileId
    ) - 12usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_rgScore",
    ][::std::mem::offset_of!(RemoteStorageEnumerateWorkshopFilesResult_t, m_rgScore)
        - 412usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_nAppId",
    ][::std::mem::offset_of!(RemoteStorageEnumerateWorkshopFilesResult_t, m_nAppId)
        - 612usize];
    [
        "Offset of field: RemoteStorageEnumerateWorkshopFilesResult_t::m_unStartIndex",
    ][::std::mem::offset_of!(RemoteStorageEnumerateWorkshopFilesResult_t, m_unStartIndex)
        - 616usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageGetPublishedItemVoteDetailsResult_t {
    pub m_eResult: EResult,
    pub m_unPublishedFileId: PublishedFileId_t,
    pub m_nVotesFor: int32,
    pub m_nVotesAgainst: int32,
    pub m_nReports: int32,
    pub m_fScore: f32,
}
pub const RemoteStorageGetPublishedItemVoteDetailsResult_t_k_iCallback: RemoteStorageGetPublishedItemVoteDetailsResult_t__bindgen_ty_1 = RemoteStorageGetPublishedItemVoteDetailsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageGetPublishedItemVoteDetailsResult_t__bindgen_ty_1 {
    k_iCallback = 1320,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageGetPublishedItemVoteDetailsResult_t",
    ][::std::mem::size_of::<RemoteStorageGetPublishedItemVoteDetailsResult_t>()
        - 28usize];
    [
        "Alignment of RemoteStorageGetPublishedItemVoteDetailsResult_t",
    ][::std::mem::align_of::<RemoteStorageGetPublishedItemVoteDetailsResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedItemVoteDetailsResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_unPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedItemVoteDetailsResult_t, m_unPublishedFileId
    ) - 4usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_nVotesFor",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedItemVoteDetailsResult_t, m_nVotesFor
    ) - 12usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_nVotesAgainst",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedItemVoteDetailsResult_t, m_nVotesAgainst
    ) - 16usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_nReports",
    ][::std::mem::offset_of!(
        RemoteStorageGetPublishedItemVoteDetailsResult_t, m_nReports
    ) - 20usize];
    [
        "Offset of field: RemoteStorageGetPublishedItemVoteDetailsResult_t::m_fScore",
    ][::std::mem::offset_of!(RemoteStorageGetPublishedItemVoteDetailsResult_t, m_fScore)
        - 24usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishedFileSubscribed_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
}
pub const RemoteStoragePublishedFileSubscribed_t_k_iCallback: RemoteStoragePublishedFileSubscribed_t__bindgen_ty_1 = RemoteStoragePublishedFileSubscribed_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishedFileSubscribed_t__bindgen_ty_1 {
    k_iCallback = 1321,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishedFileSubscribed_t",
    ][::std::mem::size_of::<RemoteStoragePublishedFileSubscribed_t>() - 12usize];
    [
        "Alignment of RemoteStoragePublishedFileSubscribed_t",
    ][::std::mem::align_of::<RemoteStoragePublishedFileSubscribed_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishedFileSubscribed_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileSubscribed_t, m_nPublishedFileId)
        - 0usize];
    [
        "Offset of field: RemoteStoragePublishedFileSubscribed_t::m_nAppID",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileSubscribed_t, m_nAppID) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishedFileUnsubscribed_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
}
pub const RemoteStoragePublishedFileUnsubscribed_t_k_iCallback: RemoteStoragePublishedFileUnsubscribed_t__bindgen_ty_1 = RemoteStoragePublishedFileUnsubscribed_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishedFileUnsubscribed_t__bindgen_ty_1 {
    k_iCallback = 1322,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishedFileUnsubscribed_t",
    ][::std::mem::size_of::<RemoteStoragePublishedFileUnsubscribed_t>() - 12usize];
    [
        "Alignment of RemoteStoragePublishedFileUnsubscribed_t",
    ][::std::mem::align_of::<RemoteStoragePublishedFileUnsubscribed_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishedFileUnsubscribed_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStoragePublishedFileUnsubscribed_t, m_nPublishedFileId
    ) - 0usize];
    [
        "Offset of field: RemoteStoragePublishedFileUnsubscribed_t::m_nAppID",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileUnsubscribed_t, m_nAppID)
        - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishedFileDeleted_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
}
pub const RemoteStoragePublishedFileDeleted_t_k_iCallback: RemoteStoragePublishedFileDeleted_t__bindgen_ty_1 = RemoteStoragePublishedFileDeleted_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishedFileDeleted_t__bindgen_ty_1 {
    k_iCallback = 1323,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishedFileDeleted_t",
    ][::std::mem::size_of::<RemoteStoragePublishedFileDeleted_t>() - 12usize];
    [
        "Alignment of RemoteStoragePublishedFileDeleted_t",
    ][::std::mem::align_of::<RemoteStoragePublishedFileDeleted_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishedFileDeleted_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileDeleted_t, m_nPublishedFileId)
        - 0usize];
    [
        "Offset of field: RemoteStoragePublishedFileDeleted_t::m_nAppID",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileDeleted_t, m_nAppID) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageUpdateUserPublishedItemVoteResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const RemoteStorageUpdateUserPublishedItemVoteResult_t_k_iCallback: RemoteStorageUpdateUserPublishedItemVoteResult_t__bindgen_ty_1 = RemoteStorageUpdateUserPublishedItemVoteResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageUpdateUserPublishedItemVoteResult_t__bindgen_ty_1 {
    k_iCallback = 1324,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageUpdateUserPublishedItemVoteResult_t",
    ][::std::mem::size_of::<RemoteStorageUpdateUserPublishedItemVoteResult_t>()
        - 12usize];
    [
        "Alignment of RemoteStorageUpdateUserPublishedItemVoteResult_t",
    ][::std::mem::align_of::<RemoteStorageUpdateUserPublishedItemVoteResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageUpdateUserPublishedItemVoteResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageUpdateUserPublishedItemVoteResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageUpdateUserPublishedItemVoteResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageUpdateUserPublishedItemVoteResult_t, m_nPublishedFileId
    ) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageUserVoteDetails_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eVote: EWorkshopVote,
}
pub const RemoteStorageUserVoteDetails_t_k_iCallback: RemoteStorageUserVoteDetails_t__bindgen_ty_1 = RemoteStorageUserVoteDetails_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageUserVoteDetails_t__bindgen_ty_1 {
    k_iCallback = 1325,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageUserVoteDetails_t",
    ][::std::mem::size_of::<RemoteStorageUserVoteDetails_t>() - 16usize];
    [
        "Alignment of RemoteStorageUserVoteDetails_t",
    ][::std::mem::align_of::<RemoteStorageUserVoteDetails_t>() - 4usize];
    [
        "Offset of field: RemoteStorageUserVoteDetails_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageUserVoteDetails_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoteStorageUserVoteDetails_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoteStorageUserVoteDetails_t, m_nPublishedFileId)
        - 4usize];
    [
        "Offset of field: RemoteStorageUserVoteDetails_t::m_eVote",
    ][::std::mem::offset_of!(RemoteStorageUserVoteDetails_t, m_eVote) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageEnumerateUserSharedWorkshopFilesResult_t {
    pub m_eResult: EResult,
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
    pub m_rgPublishedFileId: [PublishedFileId_t; 50usize],
}
pub const RemoteStorageEnumerateUserSharedWorkshopFilesResult_t_k_iCallback: RemoteStorageEnumerateUserSharedWorkshopFilesResult_t__bindgen_ty_1 = RemoteStorageEnumerateUserSharedWorkshopFilesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageEnumerateUserSharedWorkshopFilesResult_t__bindgen_ty_1 {
    k_iCallback = 1326,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageEnumerateUserSharedWorkshopFilesResult_t",
    ][::std::mem::size_of::<RemoteStorageEnumerateUserSharedWorkshopFilesResult_t>()
        - 412usize];
    [
        "Alignment of RemoteStorageEnumerateUserSharedWorkshopFilesResult_t",
    ][::std::mem::align_of::<RemoteStorageEnumerateUserSharedWorkshopFilesResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSharedWorkshopFilesResult_t::m_eResult",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSharedWorkshopFilesResult_t, m_eResult
    ) - 0usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSharedWorkshopFilesResult_t::m_nResultsReturned",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSharedWorkshopFilesResult_t, m_nResultsReturned
    ) - 4usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSharedWorkshopFilesResult_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSharedWorkshopFilesResult_t, m_nTotalResultCount
    ) - 8usize];
    [
        "Offset of field: RemoteStorageEnumerateUserSharedWorkshopFilesResult_t::m_rgPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageEnumerateUserSharedWorkshopFilesResult_t, m_rgPublishedFileId
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageSetUserPublishedFileActionResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eAction: EWorkshopFileAction,
}
pub const RemoteStorageSetUserPublishedFileActionResult_t_k_iCallback: RemoteStorageSetUserPublishedFileActionResult_t__bindgen_ty_1 = RemoteStorageSetUserPublishedFileActionResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageSetUserPublishedFileActionResult_t__bindgen_ty_1 {
    k_iCallback = 1327,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageSetUserPublishedFileActionResult_t",
    ][::std::mem::size_of::<RemoteStorageSetUserPublishedFileActionResult_t>()
        - 16usize];
    [
        "Alignment of RemoteStorageSetUserPublishedFileActionResult_t",
    ][::std::mem::align_of::<RemoteStorageSetUserPublishedFileActionResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageSetUserPublishedFileActionResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageSetUserPublishedFileActionResult_t, m_eResult)
        - 0usize];
    [
        "Offset of field: RemoteStorageSetUserPublishedFileActionResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageSetUserPublishedFileActionResult_t, m_nPublishedFileId
    ) - 4usize];
    [
        "Offset of field: RemoteStorageSetUserPublishedFileActionResult_t::m_eAction",
    ][::std::mem::offset_of!(RemoteStorageSetUserPublishedFileActionResult_t, m_eAction)
        - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageEnumeratePublishedFilesByUserActionResult_t {
    pub m_eResult: EResult,
    pub m_eAction: EWorkshopFileAction,
    pub m_nResultsReturned: int32,
    pub m_nTotalResultCount: int32,
    pub m_rgPublishedFileId: [PublishedFileId_t; 50usize],
    pub m_rgRTimeUpdated: [uint32; 50usize],
}
pub const RemoteStorageEnumeratePublishedFilesByUserActionResult_t_k_iCallback: RemoteStorageEnumeratePublishedFilesByUserActionResult_t__bindgen_ty_1 = RemoteStorageEnumeratePublishedFilesByUserActionResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageEnumeratePublishedFilesByUserActionResult_t__bindgen_ty_1 {
    k_iCallback = 1328,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageEnumeratePublishedFilesByUserActionResult_t",
    ][::std::mem::size_of::<RemoteStorageEnumeratePublishedFilesByUserActionResult_t>()
        - 616usize];
    [
        "Alignment of RemoteStorageEnumeratePublishedFilesByUserActionResult_t",
    ][::std::mem::align_of::<RemoteStorageEnumeratePublishedFilesByUserActionResult_t>()
        - 4usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_eResult",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_eResult
    ) - 0usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_eAction",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_eAction
    ) - 4usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_nResultsReturned",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_nResultsReturned
    ) - 8usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_nTotalResultCount",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_nTotalResultCount
    ) - 12usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_rgPublishedFileId",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_rgPublishedFileId
    ) - 16usize];
    [
        "Offset of field: RemoteStorageEnumeratePublishedFilesByUserActionResult_t::m_rgRTimeUpdated",
    ][::std::mem::offset_of!(
        RemoteStorageEnumeratePublishedFilesByUserActionResult_t, m_rgRTimeUpdated
    ) - 416usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishFileProgress_t {
    pub m_dPercentFile: f64,
    pub m_bPreview: bool,
}
pub const RemoteStoragePublishFileProgress_t_k_iCallback: RemoteStoragePublishFileProgress_t__bindgen_ty_1 = RemoteStoragePublishFileProgress_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishFileProgress_t__bindgen_ty_1 {
    k_iCallback = 1329,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishFileProgress_t",
    ][::std::mem::size_of::<RemoteStoragePublishFileProgress_t>() - 12usize];
    [
        "Alignment of RemoteStoragePublishFileProgress_t",
    ][::std::mem::align_of::<RemoteStoragePublishFileProgress_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishFileProgress_t::m_dPercentFile",
    ][::std::mem::offset_of!(RemoteStoragePublishFileProgress_t, m_dPercentFile)
        - 0usize];
    [
        "Offset of field: RemoteStoragePublishFileProgress_t::m_bPreview",
    ][::std::mem::offset_of!(RemoteStoragePublishFileProgress_t, m_bPreview) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStoragePublishedFileUpdated_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
    pub m_ulUnused: uint64,
}
pub const RemoteStoragePublishedFileUpdated_t_k_iCallback: RemoteStoragePublishedFileUpdated_t__bindgen_ty_1 = RemoteStoragePublishedFileUpdated_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStoragePublishedFileUpdated_t__bindgen_ty_1 {
    k_iCallback = 1330,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStoragePublishedFileUpdated_t",
    ][::std::mem::size_of::<RemoteStoragePublishedFileUpdated_t>() - 20usize];
    [
        "Alignment of RemoteStoragePublishedFileUpdated_t",
    ][::std::mem::align_of::<RemoteStoragePublishedFileUpdated_t>() - 4usize];
    [
        "Offset of field: RemoteStoragePublishedFileUpdated_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileUpdated_t, m_nPublishedFileId)
        - 0usize];
    [
        "Offset of field: RemoteStoragePublishedFileUpdated_t::m_nAppID",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileUpdated_t, m_nAppID) - 8usize];
    [
        "Offset of field: RemoteStoragePublishedFileUpdated_t::m_ulUnused",
    ][::std::mem::offset_of!(RemoteStoragePublishedFileUpdated_t, m_ulUnused) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageFileWriteAsyncComplete_t {
    pub m_eResult: EResult,
}
pub const RemoteStorageFileWriteAsyncComplete_t_k_iCallback: RemoteStorageFileWriteAsyncComplete_t__bindgen_ty_1 = RemoteStorageFileWriteAsyncComplete_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageFileWriteAsyncComplete_t__bindgen_ty_1 {
    k_iCallback = 1331,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageFileWriteAsyncComplete_t",
    ][::std::mem::size_of::<RemoteStorageFileWriteAsyncComplete_t>() - 4usize];
    [
        "Alignment of RemoteStorageFileWriteAsyncComplete_t",
    ][::std::mem::align_of::<RemoteStorageFileWriteAsyncComplete_t>() - 4usize];
    [
        "Offset of field: RemoteStorageFileWriteAsyncComplete_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageFileWriteAsyncComplete_t, m_eResult) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageFileReadAsyncComplete_t {
    pub m_hFileReadAsync: SteamAPICall_t,
    pub m_eResult: EResult,
    pub m_nOffset: uint32,
    pub m_cubRead: uint32,
}
pub const RemoteStorageFileReadAsyncComplete_t_k_iCallback: RemoteStorageFileReadAsyncComplete_t__bindgen_ty_1 = RemoteStorageFileReadAsyncComplete_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageFileReadAsyncComplete_t__bindgen_ty_1 {
    k_iCallback = 1332,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageFileReadAsyncComplete_t",
    ][::std::mem::size_of::<RemoteStorageFileReadAsyncComplete_t>() - 20usize];
    [
        "Alignment of RemoteStorageFileReadAsyncComplete_t",
    ][::std::mem::align_of::<RemoteStorageFileReadAsyncComplete_t>() - 4usize];
    [
        "Offset of field: RemoteStorageFileReadAsyncComplete_t::m_hFileReadAsync",
    ][::std::mem::offset_of!(RemoteStorageFileReadAsyncComplete_t, m_hFileReadAsync)
        - 0usize];
    [
        "Offset of field: RemoteStorageFileReadAsyncComplete_t::m_eResult",
    ][::std::mem::offset_of!(RemoteStorageFileReadAsyncComplete_t, m_eResult) - 8usize];
    [
        "Offset of field: RemoteStorageFileReadAsyncComplete_t::m_nOffset",
    ][::std::mem::offset_of!(RemoteStorageFileReadAsyncComplete_t, m_nOffset) - 12usize];
    [
        "Offset of field: RemoteStorageFileReadAsyncComplete_t::m_cubRead",
    ][::std::mem::offset_of!(RemoteStorageFileReadAsyncComplete_t, m_cubRead) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RemoteStorageLocalFileChange_t {
    pub _address: u8,
}
pub const RemoteStorageLocalFileChange_t_k_iCallback: RemoteStorageLocalFileChange_t__bindgen_ty_1 = RemoteStorageLocalFileChange_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoteStorageLocalFileChange_t__bindgen_ty_1 {
    k_iCallback = 1333,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoteStorageLocalFileChange_t",
    ][::std::mem::size_of::<RemoteStorageLocalFileChange_t>() - 1usize];
    [
        "Alignment of RemoteStorageLocalFileChange_t",
    ][::std::mem::align_of::<RemoteStorageLocalFileChange_t>() - 1usize];
};
pub const k_cchStatNameMax: _bindgen_ty_43 = _bindgen_ty_43::k_cchStatNameMax;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_43 {
    k_cchStatNameMax = 128,
}
pub const k_cchLeaderboardNameMax: _bindgen_ty_44 = _bindgen_ty_44::k_cchLeaderboardNameMax;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_44 {
    k_cchLeaderboardNameMax = 128,
}
pub const k_cLeaderboardDetailsMax: _bindgen_ty_45 = _bindgen_ty_45::k_cLeaderboardDetailsMax;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum _bindgen_ty_45 {
    k_cLeaderboardDetailsMax = 64,
}
pub type SteamLeaderboard_t = uint64;
pub type SteamLeaderboardEntries_t = uint64;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELeaderboardDataRequest {
    k_ELeaderboardDataRequestGlobal = 0,
    k_ELeaderboardDataRequestGlobalAroundUser = 1,
    k_ELeaderboardDataRequestFriends = 2,
    k_ELeaderboardDataRequestUsers = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELeaderboardSortMethod {
    k_ELeaderboardSortMethodNone = 0,
    k_ELeaderboardSortMethodAscending = 1,
    k_ELeaderboardSortMethodDescending = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELeaderboardDisplayType {
    k_ELeaderboardDisplayTypeNone = 0,
    k_ELeaderboardDisplayTypeNumeric = 1,
    k_ELeaderboardDisplayTypeTimeSeconds = 2,
    k_ELeaderboardDisplayTypeTimeMilliSeconds = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ELeaderboardUploadScoreMethod {
    k_ELeaderboardUploadScoreMethodNone = 0,
    k_ELeaderboardUploadScoreMethodKeepBest = 1,
    k_ELeaderboardUploadScoreMethodForceUpdate = 2,
}
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct LeaderboardEntry_t {
    pub m_steamIDUser: CSteamID,
    pub m_nGlobalRank: int32,
    pub m_nScore: int32,
    pub m_cDetails: int32,
    pub m_hUGC: UGCHandle_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LeaderboardEntry_t",
    ][::std::mem::size_of::<LeaderboardEntry_t>() - 28usize];
    [
        "Alignment of LeaderboardEntry_t",
    ][::std::mem::align_of::<LeaderboardEntry_t>() - 4usize];
    [
        "Offset of field: LeaderboardEntry_t::m_steamIDUser",
    ][::std::mem::offset_of!(LeaderboardEntry_t, m_steamIDUser) - 0usize];
    [
        "Offset of field: LeaderboardEntry_t::m_nGlobalRank",
    ][::std::mem::offset_of!(LeaderboardEntry_t, m_nGlobalRank) - 8usize];
    [
        "Offset of field: LeaderboardEntry_t::m_nScore",
    ][::std::mem::offset_of!(LeaderboardEntry_t, m_nScore) - 12usize];
    [
        "Offset of field: LeaderboardEntry_t::m_cDetails",
    ][::std::mem::offset_of!(LeaderboardEntry_t, m_cDetails) - 16usize];
    [
        "Offset of field: LeaderboardEntry_t::m_hUGC",
    ][::std::mem::offset_of!(LeaderboardEntry_t, m_hUGC) - 20usize];
};
#[repr(C)]
pub struct ISteamUserStats__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamUserStats {
    pub vtable_: *const ISteamUserStats__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamUserStats"][::std::mem::size_of::<ISteamUserStats>() - 8usize];
    ["Alignment of ISteamUserStats"][::std::mem::align_of::<ISteamUserStats>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct UserStatsReceived_t {
    pub m_nGameID: uint64,
    pub m_eResult: EResult,
    pub m_steamIDUser: CSteamID,
}
pub const UserStatsReceived_t_k_iCallback: UserStatsReceived_t__bindgen_ty_1 = UserStatsReceived_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserStatsReceived_t__bindgen_ty_1 {
    k_iCallback = 1101,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserStatsReceived_t",
    ][::std::mem::size_of::<UserStatsReceived_t>() - 20usize];
    [
        "Alignment of UserStatsReceived_t",
    ][::std::mem::align_of::<UserStatsReceived_t>() - 4usize];
    [
        "Offset of field: UserStatsReceived_t::m_nGameID",
    ][::std::mem::offset_of!(UserStatsReceived_t, m_nGameID) - 0usize];
    [
        "Offset of field: UserStatsReceived_t::m_eResult",
    ][::std::mem::offset_of!(UserStatsReceived_t, m_eResult) - 8usize];
    [
        "Offset of field: UserStatsReceived_t::m_steamIDUser",
    ][::std::mem::offset_of!(UserStatsReceived_t, m_steamIDUser) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct UserStatsStored_t {
    pub m_nGameID: uint64,
    pub m_eResult: EResult,
}
pub const UserStatsStored_t_k_iCallback: UserStatsStored_t__bindgen_ty_1 = UserStatsStored_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserStatsStored_t__bindgen_ty_1 {
    k_iCallback = 1102,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of UserStatsStored_t"][::std::mem::size_of::<UserStatsStored_t>() - 12usize];
    [
        "Alignment of UserStatsStored_t",
    ][::std::mem::align_of::<UserStatsStored_t>() - 4usize];
    [
        "Offset of field: UserStatsStored_t::m_nGameID",
    ][::std::mem::offset_of!(UserStatsStored_t, m_nGameID) - 0usize];
    [
        "Offset of field: UserStatsStored_t::m_eResult",
    ][::std::mem::offset_of!(UserStatsStored_t, m_eResult) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct UserAchievementStored_t {
    pub m_nGameID: uint64,
    pub m_bGroupAchievement: bool,
    pub m_rgchAchievementName: [::std::os::raw::c_char; 128usize],
    pub m_nCurProgress: uint32,
    pub m_nMaxProgress: uint32,
}
pub const UserAchievementStored_t_k_iCallback: UserAchievementStored_t__bindgen_ty_1 = UserAchievementStored_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserAchievementStored_t__bindgen_ty_1 {
    k_iCallback = 1103,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserAchievementStored_t",
    ][::std::mem::size_of::<UserAchievementStored_t>() - 148usize];
    [
        "Alignment of UserAchievementStored_t",
    ][::std::mem::align_of::<UserAchievementStored_t>() - 4usize];
    [
        "Offset of field: UserAchievementStored_t::m_nGameID",
    ][::std::mem::offset_of!(UserAchievementStored_t, m_nGameID) - 0usize];
    [
        "Offset of field: UserAchievementStored_t::m_bGroupAchievement",
    ][::std::mem::offset_of!(UserAchievementStored_t, m_bGroupAchievement) - 8usize];
    [
        "Offset of field: UserAchievementStored_t::m_rgchAchievementName",
    ][::std::mem::offset_of!(UserAchievementStored_t, m_rgchAchievementName) - 9usize];
    [
        "Offset of field: UserAchievementStored_t::m_nCurProgress",
    ][::std::mem::offset_of!(UserAchievementStored_t, m_nCurProgress) - 140usize];
    [
        "Offset of field: UserAchievementStored_t::m_nMaxProgress",
    ][::std::mem::offset_of!(UserAchievementStored_t, m_nMaxProgress) - 144usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardFindResult_t {
    pub m_hSteamLeaderboard: SteamLeaderboard_t,
    pub m_bLeaderboardFound: uint8,
}
pub const LeaderboardFindResult_t_k_iCallback: LeaderboardFindResult_t__bindgen_ty_1 = LeaderboardFindResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LeaderboardFindResult_t__bindgen_ty_1 {
    k_iCallback = 1104,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LeaderboardFindResult_t",
    ][::std::mem::size_of::<LeaderboardFindResult_t>() - 12usize];
    [
        "Alignment of LeaderboardFindResult_t",
    ][::std::mem::align_of::<LeaderboardFindResult_t>() - 4usize];
    [
        "Offset of field: LeaderboardFindResult_t::m_hSteamLeaderboard",
    ][::std::mem::offset_of!(LeaderboardFindResult_t, m_hSteamLeaderboard) - 0usize];
    [
        "Offset of field: LeaderboardFindResult_t::m_bLeaderboardFound",
    ][::std::mem::offset_of!(LeaderboardFindResult_t, m_bLeaderboardFound) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardScoresDownloaded_t {
    pub m_hSteamLeaderboard: SteamLeaderboard_t,
    pub m_hSteamLeaderboardEntries: SteamLeaderboardEntries_t,
    pub m_cEntryCount: ::std::os::raw::c_int,
}
pub const LeaderboardScoresDownloaded_t_k_iCallback: LeaderboardScoresDownloaded_t__bindgen_ty_1 = LeaderboardScoresDownloaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LeaderboardScoresDownloaded_t__bindgen_ty_1 {
    k_iCallback = 1105,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LeaderboardScoresDownloaded_t",
    ][::std::mem::size_of::<LeaderboardScoresDownloaded_t>() - 20usize];
    [
        "Alignment of LeaderboardScoresDownloaded_t",
    ][::std::mem::align_of::<LeaderboardScoresDownloaded_t>() - 4usize];
    [
        "Offset of field: LeaderboardScoresDownloaded_t::m_hSteamLeaderboard",
    ][::std::mem::offset_of!(LeaderboardScoresDownloaded_t, m_hSteamLeaderboard)
        - 0usize];
    [
        "Offset of field: LeaderboardScoresDownloaded_t::m_hSteamLeaderboardEntries",
    ][::std::mem::offset_of!(LeaderboardScoresDownloaded_t, m_hSteamLeaderboardEntries)
        - 8usize];
    [
        "Offset of field: LeaderboardScoresDownloaded_t::m_cEntryCount",
    ][::std::mem::offset_of!(LeaderboardScoresDownloaded_t, m_cEntryCount) - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardScoreUploaded_t {
    pub m_bSuccess: uint8,
    pub m_hSteamLeaderboard: SteamLeaderboard_t,
    pub m_nScore: int32,
    pub m_bScoreChanged: uint8,
    pub m_nGlobalRankNew: ::std::os::raw::c_int,
    pub m_nGlobalRankPrevious: ::std::os::raw::c_int,
}
pub const LeaderboardScoreUploaded_t_k_iCallback: LeaderboardScoreUploaded_t__bindgen_ty_1 = LeaderboardScoreUploaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LeaderboardScoreUploaded_t__bindgen_ty_1 {
    k_iCallback = 1106,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LeaderboardScoreUploaded_t",
    ][::std::mem::size_of::<LeaderboardScoreUploaded_t>() - 28usize];
    [
        "Alignment of LeaderboardScoreUploaded_t",
    ][::std::mem::align_of::<LeaderboardScoreUploaded_t>() - 4usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_bSuccess",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_bSuccess) - 0usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_hSteamLeaderboard",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_hSteamLeaderboard) - 4usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_nScore",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_nScore) - 12usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_bScoreChanged",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_bScoreChanged) - 16usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_nGlobalRankNew",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_nGlobalRankNew) - 20usize];
    [
        "Offset of field: LeaderboardScoreUploaded_t::m_nGlobalRankPrevious",
    ][::std::mem::offset_of!(LeaderboardScoreUploaded_t, m_nGlobalRankPrevious)
        - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NumberOfCurrentPlayers_t {
    pub m_bSuccess: uint8,
    pub m_cPlayers: int32,
}
pub const NumberOfCurrentPlayers_t_k_iCallback: NumberOfCurrentPlayers_t__bindgen_ty_1 = NumberOfCurrentPlayers_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NumberOfCurrentPlayers_t__bindgen_ty_1 {
    k_iCallback = 1107,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of NumberOfCurrentPlayers_t",
    ][::std::mem::size_of::<NumberOfCurrentPlayers_t>() - 8usize];
    [
        "Alignment of NumberOfCurrentPlayers_t",
    ][::std::mem::align_of::<NumberOfCurrentPlayers_t>() - 4usize];
    [
        "Offset of field: NumberOfCurrentPlayers_t::m_bSuccess",
    ][::std::mem::offset_of!(NumberOfCurrentPlayers_t, m_bSuccess) - 0usize];
    [
        "Offset of field: NumberOfCurrentPlayers_t::m_cPlayers",
    ][::std::mem::offset_of!(NumberOfCurrentPlayers_t, m_cPlayers) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UserStatsUnloaded_t {
    pub m_steamIDUser: CSteamID,
}
pub const UserStatsUnloaded_t_k_iCallback: UserStatsUnloaded_t__bindgen_ty_1 = UserStatsUnloaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserStatsUnloaded_t__bindgen_ty_1 {
    k_iCallback = 1108,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserStatsUnloaded_t",
    ][::std::mem::size_of::<UserStatsUnloaded_t>() - 8usize];
    [
        "Alignment of UserStatsUnloaded_t",
    ][::std::mem::align_of::<UserStatsUnloaded_t>() - 1usize];
    [
        "Offset of field: UserStatsUnloaded_t::m_steamIDUser",
    ][::std::mem::offset_of!(UserStatsUnloaded_t, m_steamIDUser) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct UserAchievementIconFetched_t {
    pub m_nGameID: CGameID,
    pub m_rgchAchievementName: [::std::os::raw::c_char; 128usize],
    pub m_bAchieved: bool,
    pub m_nIconHandle: ::std::os::raw::c_int,
}
pub const UserAchievementIconFetched_t_k_iCallback: UserAchievementIconFetched_t__bindgen_ty_1 = UserAchievementIconFetched_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserAchievementIconFetched_t__bindgen_ty_1 {
    k_iCallback = 1109,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserAchievementIconFetched_t",
    ][::std::mem::size_of::<UserAchievementIconFetched_t>() - 144usize];
    [
        "Alignment of UserAchievementIconFetched_t",
    ][::std::mem::align_of::<UserAchievementIconFetched_t>() - 4usize];
    [
        "Offset of field: UserAchievementIconFetched_t::m_nGameID",
    ][::std::mem::offset_of!(UserAchievementIconFetched_t, m_nGameID) - 0usize];
    [
        "Offset of field: UserAchievementIconFetched_t::m_rgchAchievementName",
    ][::std::mem::offset_of!(UserAchievementIconFetched_t, m_rgchAchievementName)
        - 8usize];
    [
        "Offset of field: UserAchievementIconFetched_t::m_bAchieved",
    ][::std::mem::offset_of!(UserAchievementIconFetched_t, m_bAchieved) - 136usize];
    [
        "Offset of field: UserAchievementIconFetched_t::m_nIconHandle",
    ][::std::mem::offset_of!(UserAchievementIconFetched_t, m_nIconHandle) - 140usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GlobalAchievementPercentagesReady_t {
    pub m_nGameID: uint64,
    pub m_eResult: EResult,
}
pub const GlobalAchievementPercentagesReady_t_k_iCallback: GlobalAchievementPercentagesReady_t__bindgen_ty_1 = GlobalAchievementPercentagesReady_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GlobalAchievementPercentagesReady_t__bindgen_ty_1 {
    k_iCallback = 1110,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GlobalAchievementPercentagesReady_t",
    ][::std::mem::size_of::<GlobalAchievementPercentagesReady_t>() - 12usize];
    [
        "Alignment of GlobalAchievementPercentagesReady_t",
    ][::std::mem::align_of::<GlobalAchievementPercentagesReady_t>() - 4usize];
    [
        "Offset of field: GlobalAchievementPercentagesReady_t::m_nGameID",
    ][::std::mem::offset_of!(GlobalAchievementPercentagesReady_t, m_nGameID) - 0usize];
    [
        "Offset of field: GlobalAchievementPercentagesReady_t::m_eResult",
    ][::std::mem::offset_of!(GlobalAchievementPercentagesReady_t, m_eResult) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct LeaderboardUGCSet_t {
    pub m_eResult: EResult,
    pub m_hSteamLeaderboard: SteamLeaderboard_t,
}
pub const LeaderboardUGCSet_t_k_iCallback: LeaderboardUGCSet_t__bindgen_ty_1 = LeaderboardUGCSet_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum LeaderboardUGCSet_t__bindgen_ty_1 {
    k_iCallback = 1111,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of LeaderboardUGCSet_t",
    ][::std::mem::size_of::<LeaderboardUGCSet_t>() - 12usize];
    [
        "Alignment of LeaderboardUGCSet_t",
    ][::std::mem::align_of::<LeaderboardUGCSet_t>() - 4usize];
    [
        "Offset of field: LeaderboardUGCSet_t::m_eResult",
    ][::std::mem::offset_of!(LeaderboardUGCSet_t, m_eResult) - 0usize];
    [
        "Offset of field: LeaderboardUGCSet_t::m_hSteamLeaderboard",
    ][::std::mem::offset_of!(LeaderboardUGCSet_t, m_hSteamLeaderboard) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GlobalStatsReceived_t {
    pub m_nGameID: uint64,
    pub m_eResult: EResult,
}
pub const GlobalStatsReceived_t_k_iCallback: GlobalStatsReceived_t__bindgen_ty_1 = GlobalStatsReceived_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GlobalStatsReceived_t__bindgen_ty_1 {
    k_iCallback = 1112,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GlobalStatsReceived_t",
    ][::std::mem::size_of::<GlobalStatsReceived_t>() - 12usize];
    [
        "Alignment of GlobalStatsReceived_t",
    ][::std::mem::align_of::<GlobalStatsReceived_t>() - 4usize];
    [
        "Offset of field: GlobalStatsReceived_t::m_nGameID",
    ][::std::mem::offset_of!(GlobalStatsReceived_t, m_nGameID) - 0usize];
    [
        "Offset of field: GlobalStatsReceived_t::m_eResult",
    ][::std::mem::offset_of!(GlobalStatsReceived_t, m_eResult) - 8usize];
};
pub const k_cubAppProofOfPurchaseKeyMax: ::std::os::raw::c_int = 240;
#[repr(C)]
pub struct ISteamApps__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamApps {
    pub vtable_: *const ISteamApps__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamApps"][::std::mem::size_of::<ISteamApps>() - 8usize];
    ["Alignment of ISteamApps"][::std::mem::align_of::<ISteamApps>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DlcInstalled_t {
    pub m_nAppID: AppId_t,
}
pub const DlcInstalled_t_k_iCallback: DlcInstalled_t__bindgen_ty_1 = DlcInstalled_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DlcInstalled_t__bindgen_ty_1 {
    k_iCallback = 1005,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of DlcInstalled_t"][::std::mem::size_of::<DlcInstalled_t>() - 4usize];
    ["Alignment of DlcInstalled_t"][::std::mem::align_of::<DlcInstalled_t>() - 4usize];
    [
        "Offset of field: DlcInstalled_t::m_nAppID",
    ][::std::mem::offset_of!(DlcInstalled_t, m_nAppID) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NewUrlLaunchParameters_t {
    pub _address: u8,
}
pub const NewUrlLaunchParameters_t_k_iCallback: NewUrlLaunchParameters_t__bindgen_ty_1 = NewUrlLaunchParameters_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum NewUrlLaunchParameters_t__bindgen_ty_1 {
    k_iCallback = 1014,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of NewUrlLaunchParameters_t",
    ][::std::mem::size_of::<NewUrlLaunchParameters_t>() - 1usize];
    [
        "Alignment of NewUrlLaunchParameters_t",
    ][::std::mem::align_of::<NewUrlLaunchParameters_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AppProofOfPurchaseKeyResponse_t {
    pub m_eResult: EResult,
    pub m_nAppID: uint32,
    pub m_cchKeyLength: uint32,
    pub m_rgchKey: [::std::os::raw::c_char; 240usize],
}
pub const AppProofOfPurchaseKeyResponse_t_k_iCallback: AppProofOfPurchaseKeyResponse_t__bindgen_ty_1 = AppProofOfPurchaseKeyResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AppProofOfPurchaseKeyResponse_t__bindgen_ty_1 {
    k_iCallback = 1021,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AppProofOfPurchaseKeyResponse_t",
    ][::std::mem::size_of::<AppProofOfPurchaseKeyResponse_t>() - 252usize];
    [
        "Alignment of AppProofOfPurchaseKeyResponse_t",
    ][::std::mem::align_of::<AppProofOfPurchaseKeyResponse_t>() - 4usize];
    [
        "Offset of field: AppProofOfPurchaseKeyResponse_t::m_eResult",
    ][::std::mem::offset_of!(AppProofOfPurchaseKeyResponse_t, m_eResult) - 0usize];
    [
        "Offset of field: AppProofOfPurchaseKeyResponse_t::m_nAppID",
    ][::std::mem::offset_of!(AppProofOfPurchaseKeyResponse_t, m_nAppID) - 4usize];
    [
        "Offset of field: AppProofOfPurchaseKeyResponse_t::m_cchKeyLength",
    ][::std::mem::offset_of!(AppProofOfPurchaseKeyResponse_t, m_cchKeyLength) - 8usize];
    [
        "Offset of field: AppProofOfPurchaseKeyResponse_t::m_rgchKey",
    ][::std::mem::offset_of!(AppProofOfPurchaseKeyResponse_t, m_rgchKey) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct FileDetailsResult_t {
    pub m_eResult: EResult,
    pub m_ulFileSize: uint64,
    pub m_FileSHA: [uint8; 20usize],
    pub m_unFlags: uint32,
}
pub const FileDetailsResult_t_k_iCallback: FileDetailsResult_t__bindgen_ty_1 = FileDetailsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FileDetailsResult_t__bindgen_ty_1 {
    k_iCallback = 1023,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of FileDetailsResult_t",
    ][::std::mem::size_of::<FileDetailsResult_t>() - 36usize];
    [
        "Alignment of FileDetailsResult_t",
    ][::std::mem::align_of::<FileDetailsResult_t>() - 4usize];
    [
        "Offset of field: FileDetailsResult_t::m_eResult",
    ][::std::mem::offset_of!(FileDetailsResult_t, m_eResult) - 0usize];
    [
        "Offset of field: FileDetailsResult_t::m_ulFileSize",
    ][::std::mem::offset_of!(FileDetailsResult_t, m_ulFileSize) - 4usize];
    [
        "Offset of field: FileDetailsResult_t::m_FileSHA",
    ][::std::mem::offset_of!(FileDetailsResult_t, m_FileSHA) - 12usize];
    [
        "Offset of field: FileDetailsResult_t::m_unFlags",
    ][::std::mem::offset_of!(FileDetailsResult_t, m_unFlags) - 32usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimedTrialStatus_t {
    pub m_unAppID: AppId_t,
    pub m_bIsOffline: bool,
    pub m_unSecondsAllowed: uint32,
    pub m_unSecondsPlayed: uint32,
}
pub const TimedTrialStatus_t_k_iCallback: TimedTrialStatus_t__bindgen_ty_1 = TimedTrialStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum TimedTrialStatus_t__bindgen_ty_1 {
    k_iCallback = 1030,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of TimedTrialStatus_t",
    ][::std::mem::size_of::<TimedTrialStatus_t>() - 16usize];
    [
        "Alignment of TimedTrialStatus_t",
    ][::std::mem::align_of::<TimedTrialStatus_t>() - 4usize];
    [
        "Offset of field: TimedTrialStatus_t::m_unAppID",
    ][::std::mem::offset_of!(TimedTrialStatus_t, m_unAppID) - 0usize];
    [
        "Offset of field: TimedTrialStatus_t::m_bIsOffline",
    ][::std::mem::offset_of!(TimedTrialStatus_t, m_bIsOffline) - 4usize];
    [
        "Offset of field: TimedTrialStatus_t::m_unSecondsAllowed",
    ][::std::mem::offset_of!(TimedTrialStatus_t, m_unSecondsAllowed) - 8usize];
    [
        "Offset of field: TimedTrialStatus_t::m_unSecondsPlayed",
    ][::std::mem::offset_of!(TimedTrialStatus_t, m_unSecondsPlayed) - 12usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EP2PSessionError {
    k_EP2PSessionErrorNone = 0,
    k_EP2PSessionErrorNoRightsToApp = 2,
    k_EP2PSessionErrorTimeout = 4,
    k_EP2PSessionErrorNotRunningApp_DELETED = 1,
    k_EP2PSessionErrorDestinationNotLoggedIn_DELETED = 3,
    k_EP2PSessionErrorMax = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EP2PSend {
    k_EP2PSendUnreliable = 0,
    k_EP2PSendUnreliableNoDelay = 1,
    k_EP2PSendReliable = 2,
    k_EP2PSendReliableWithBuffering = 3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct P2PSessionState_t {
    pub m_bConnectionActive: uint8,
    pub m_bConnecting: uint8,
    pub m_eP2PSessionError: uint8,
    pub m_bUsingRelay: uint8,
    pub m_nBytesQueuedForSend: int32,
    pub m_nPacketsQueuedForSend: int32,
    pub m_nRemoteIP: uint32,
    pub m_nRemotePort: uint16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of P2PSessionState_t"][::std::mem::size_of::<P2PSessionState_t>() - 20usize];
    [
        "Alignment of P2PSessionState_t",
    ][::std::mem::align_of::<P2PSessionState_t>() - 4usize];
    [
        "Offset of field: P2PSessionState_t::m_bConnectionActive",
    ][::std::mem::offset_of!(P2PSessionState_t, m_bConnectionActive) - 0usize];
    [
        "Offset of field: P2PSessionState_t::m_bConnecting",
    ][::std::mem::offset_of!(P2PSessionState_t, m_bConnecting) - 1usize];
    [
        "Offset of field: P2PSessionState_t::m_eP2PSessionError",
    ][::std::mem::offset_of!(P2PSessionState_t, m_eP2PSessionError) - 2usize];
    [
        "Offset of field: P2PSessionState_t::m_bUsingRelay",
    ][::std::mem::offset_of!(P2PSessionState_t, m_bUsingRelay) - 3usize];
    [
        "Offset of field: P2PSessionState_t::m_nBytesQueuedForSend",
    ][::std::mem::offset_of!(P2PSessionState_t, m_nBytesQueuedForSend) - 4usize];
    [
        "Offset of field: P2PSessionState_t::m_nPacketsQueuedForSend",
    ][::std::mem::offset_of!(P2PSessionState_t, m_nPacketsQueuedForSend) - 8usize];
    [
        "Offset of field: P2PSessionState_t::m_nRemoteIP",
    ][::std::mem::offset_of!(P2PSessionState_t, m_nRemoteIP) - 12usize];
    [
        "Offset of field: P2PSessionState_t::m_nRemotePort",
    ][::std::mem::offset_of!(P2PSessionState_t, m_nRemotePort) - 16usize];
};
pub type SNetSocket_t = uint32;
pub type SNetListenSocket_t = uint32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESNetSocketState {
    k_ESNetSocketStateInvalid = 0,
    k_ESNetSocketStateConnected = 1,
    k_ESNetSocketStateInitiated = 10,
    k_ESNetSocketStateLocalCandidatesFound = 11,
    k_ESNetSocketStateReceivedRemoteCandidates = 12,
    k_ESNetSocketStateChallengeHandshake = 15,
    k_ESNetSocketStateDisconnecting = 21,
    k_ESNetSocketStateLocalDisconnect = 22,
    k_ESNetSocketStateTimeoutDuringConnect = 23,
    k_ESNetSocketStateRemoteEndDisconnected = 24,
    k_ESNetSocketStateConnectionBroken = 25,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESNetSocketConnectionType {
    k_ESNetSocketConnectionTypeNotConnected = 0,
    k_ESNetSocketConnectionTypeUDP = 1,
    k_ESNetSocketConnectionTypeUDPRelay = 2,
}
#[repr(C)]
pub struct ISteamNetworking__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamNetworking {
    pub vtable_: *const ISteamNetworking__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamNetworking"][::std::mem::size_of::<ISteamNetworking>() - 8usize];
    [
        "Alignment of ISteamNetworking",
    ][::std::mem::align_of::<ISteamNetworking>() - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct P2PSessionRequest_t {
    pub m_steamIDRemote: CSteamID,
}
pub const P2PSessionRequest_t_k_iCallback: P2PSessionRequest_t__bindgen_ty_1 = P2PSessionRequest_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum P2PSessionRequest_t__bindgen_ty_1 {
    k_iCallback = 1202,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of P2PSessionRequest_t",
    ][::std::mem::size_of::<P2PSessionRequest_t>() - 8usize];
    [
        "Alignment of P2PSessionRequest_t",
    ][::std::mem::align_of::<P2PSessionRequest_t>() - 1usize];
    [
        "Offset of field: P2PSessionRequest_t::m_steamIDRemote",
    ][::std::mem::offset_of!(P2PSessionRequest_t, m_steamIDRemote) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct P2PSessionConnectFail_t {
    pub m_steamIDRemote: CSteamID,
    pub m_eP2PSessionError: uint8,
}
pub const P2PSessionConnectFail_t_k_iCallback: P2PSessionConnectFail_t__bindgen_ty_1 = P2PSessionConnectFail_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum P2PSessionConnectFail_t__bindgen_ty_1 {
    k_iCallback = 1203,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of P2PSessionConnectFail_t",
    ][::std::mem::size_of::<P2PSessionConnectFail_t>() - 9usize];
    [
        "Alignment of P2PSessionConnectFail_t",
    ][::std::mem::align_of::<P2PSessionConnectFail_t>() - 1usize];
    [
        "Offset of field: P2PSessionConnectFail_t::m_steamIDRemote",
    ][::std::mem::offset_of!(P2PSessionConnectFail_t, m_steamIDRemote) - 0usize];
    [
        "Offset of field: P2PSessionConnectFail_t::m_eP2PSessionError",
    ][::std::mem::offset_of!(P2PSessionConnectFail_t, m_eP2PSessionError) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SocketStatusCallback_t {
    pub m_hSocket: SNetSocket_t,
    pub m_hListenSocket: SNetListenSocket_t,
    pub m_steamIDRemote: CSteamID,
    pub m_eSNetSocketState: ::std::os::raw::c_int,
}
pub const SocketStatusCallback_t_k_iCallback: SocketStatusCallback_t__bindgen_ty_1 = SocketStatusCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SocketStatusCallback_t__bindgen_ty_1 {
    k_iCallback = 1201,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SocketStatusCallback_t",
    ][::std::mem::size_of::<SocketStatusCallback_t>() - 20usize];
    [
        "Alignment of SocketStatusCallback_t",
    ][::std::mem::align_of::<SocketStatusCallback_t>() - 4usize];
    [
        "Offset of field: SocketStatusCallback_t::m_hSocket",
    ][::std::mem::offset_of!(SocketStatusCallback_t, m_hSocket) - 0usize];
    [
        "Offset of field: SocketStatusCallback_t::m_hListenSocket",
    ][::std::mem::offset_of!(SocketStatusCallback_t, m_hListenSocket) - 4usize];
    [
        "Offset of field: SocketStatusCallback_t::m_steamIDRemote",
    ][::std::mem::offset_of!(SocketStatusCallback_t, m_steamIDRemote) - 8usize];
    [
        "Offset of field: SocketStatusCallback_t::m_eSNetSocketState",
    ][::std::mem::offset_of!(SocketStatusCallback_t, m_eSNetSocketState) - 16usize];
};
pub const k_nScreenshotMaxTaggedUsers: uint32 = 32;
pub const k_nScreenshotMaxTaggedPublishedFiles: uint32 = 32;
pub const k_cubUFSTagTypeMax: ::std::os::raw::c_int = 255;
pub const k_cubUFSTagValueMax: ::std::os::raw::c_int = 255;
pub const k_ScreenshotThumbWidth: ::std::os::raw::c_int = 200;
pub type ScreenshotHandle = uint32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EVRScreenshotType {
    k_EVRScreenshotType_None = 0,
    k_EVRScreenshotType_Mono = 1,
    k_EVRScreenshotType_Stereo = 2,
    k_EVRScreenshotType_MonoCubemap = 3,
    k_EVRScreenshotType_MonoPanorama = 4,
    k_EVRScreenshotType_StereoPanorama = 5,
}
#[repr(C)]
pub struct ISteamScreenshots__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamScreenshots {
    pub vtable_: *const ISteamScreenshots__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamScreenshots"][::std::mem::size_of::<ISteamScreenshots>() - 8usize];
    [
        "Alignment of ISteamScreenshots",
    ][::std::mem::align_of::<ISteamScreenshots>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenshotReady_t {
    pub m_hLocal: ScreenshotHandle,
    pub m_eResult: EResult,
}
pub const ScreenshotReady_t_k_iCallback: ScreenshotReady_t__bindgen_ty_1 = ScreenshotReady_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ScreenshotReady_t__bindgen_ty_1 {
    k_iCallback = 2301,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ScreenshotReady_t"][::std::mem::size_of::<ScreenshotReady_t>() - 8usize];
    [
        "Alignment of ScreenshotReady_t",
    ][::std::mem::align_of::<ScreenshotReady_t>() - 4usize];
    [
        "Offset of field: ScreenshotReady_t::m_hLocal",
    ][::std::mem::offset_of!(ScreenshotReady_t, m_hLocal) - 0usize];
    [
        "Offset of field: ScreenshotReady_t::m_eResult",
    ][::std::mem::offset_of!(ScreenshotReady_t, m_eResult) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScreenshotRequested_t {
    pub _address: u8,
}
pub const ScreenshotRequested_t_k_iCallback: ScreenshotRequested_t__bindgen_ty_1 = ScreenshotRequested_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ScreenshotRequested_t__bindgen_ty_1 {
    k_iCallback = 2302,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ScreenshotRequested_t",
    ][::std::mem::size_of::<ScreenshotRequested_t>() - 1usize];
    [
        "Alignment of ScreenshotRequested_t",
    ][::std::mem::align_of::<ScreenshotRequested_t>() - 1usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AudioPlayback_Status {
    AudioPlayback_Undefined = 0,
    AudioPlayback_Playing = 1,
    AudioPlayback_Paused = 2,
    AudioPlayback_Idle = 3,
}
#[repr(C)]
pub struct ISteamMusic__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMusic {
    pub vtable_: *const ISteamMusic__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamMusic"][::std::mem::size_of::<ISteamMusic>() - 8usize];
    ["Alignment of ISteamMusic"][::std::mem::align_of::<ISteamMusic>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlaybackStatusHasChanged_t {
    pub _address: u8,
}
pub const PlaybackStatusHasChanged_t_k_iCallback: PlaybackStatusHasChanged_t__bindgen_ty_1 = PlaybackStatusHasChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum PlaybackStatusHasChanged_t__bindgen_ty_1 {
    k_iCallback = 4001,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of PlaybackStatusHasChanged_t",
    ][::std::mem::size_of::<PlaybackStatusHasChanged_t>() - 1usize];
    [
        "Alignment of PlaybackStatusHasChanged_t",
    ][::std::mem::align_of::<PlaybackStatusHasChanged_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VolumeHasChanged_t {
    pub m_flNewVolume: f32,
}
pub const VolumeHasChanged_t_k_iCallback: VolumeHasChanged_t__bindgen_ty_1 = VolumeHasChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum VolumeHasChanged_t__bindgen_ty_1 {
    k_iCallback = 4002,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of VolumeHasChanged_t"][::std::mem::size_of::<VolumeHasChanged_t>() - 4usize];
    [
        "Alignment of VolumeHasChanged_t",
    ][::std::mem::align_of::<VolumeHasChanged_t>() - 4usize];
    [
        "Offset of field: VolumeHasChanged_t::m_flNewVolume",
    ][::std::mem::offset_of!(VolumeHasChanged_t, m_flNewVolume) - 0usize];
};
#[repr(C)]
pub struct ISteamMusicRemote__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamMusicRemote {
    pub vtable_: *const ISteamMusicRemote__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamMusicRemote"][::std::mem::size_of::<ISteamMusicRemote>() - 8usize];
    [
        "Alignment of ISteamMusicRemote",
    ][::std::mem::align_of::<ISteamMusicRemote>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerRemoteWillActivate_t {
    pub _address: u8,
}
pub const MusicPlayerRemoteWillActivate_t_k_iCallback: MusicPlayerRemoteWillActivate_t__bindgen_ty_1 = MusicPlayerRemoteWillActivate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerRemoteWillActivate_t__bindgen_ty_1 {
    k_iCallback = 4101,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerRemoteWillActivate_t",
    ][::std::mem::size_of::<MusicPlayerRemoteWillActivate_t>() - 1usize];
    [
        "Alignment of MusicPlayerRemoteWillActivate_t",
    ][::std::mem::align_of::<MusicPlayerRemoteWillActivate_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerRemoteWillDeactivate_t {
    pub _address: u8,
}
pub const MusicPlayerRemoteWillDeactivate_t_k_iCallback: MusicPlayerRemoteWillDeactivate_t__bindgen_ty_1 = MusicPlayerRemoteWillDeactivate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerRemoteWillDeactivate_t__bindgen_ty_1 {
    k_iCallback = 4102,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerRemoteWillDeactivate_t",
    ][::std::mem::size_of::<MusicPlayerRemoteWillDeactivate_t>() - 1usize];
    [
        "Alignment of MusicPlayerRemoteWillDeactivate_t",
    ][::std::mem::align_of::<MusicPlayerRemoteWillDeactivate_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerRemoteToFront_t {
    pub _address: u8,
}
pub const MusicPlayerRemoteToFront_t_k_iCallback: MusicPlayerRemoteToFront_t__bindgen_ty_1 = MusicPlayerRemoteToFront_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerRemoteToFront_t__bindgen_ty_1 {
    k_iCallback = 4103,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerRemoteToFront_t",
    ][::std::mem::size_of::<MusicPlayerRemoteToFront_t>() - 1usize];
    [
        "Alignment of MusicPlayerRemoteToFront_t",
    ][::std::mem::align_of::<MusicPlayerRemoteToFront_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWillQuit_t {
    pub _address: u8,
}
pub const MusicPlayerWillQuit_t_k_iCallback: MusicPlayerWillQuit_t__bindgen_ty_1 = MusicPlayerWillQuit_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWillQuit_t__bindgen_ty_1 {
    k_iCallback = 4104,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWillQuit_t",
    ][::std::mem::size_of::<MusicPlayerWillQuit_t>() - 1usize];
    [
        "Alignment of MusicPlayerWillQuit_t",
    ][::std::mem::align_of::<MusicPlayerWillQuit_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsPlay_t {
    pub _address: u8,
}
pub const MusicPlayerWantsPlay_t_k_iCallback: MusicPlayerWantsPlay_t__bindgen_ty_1 = MusicPlayerWantsPlay_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsPlay_t__bindgen_ty_1 {
    k_iCallback = 4105,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsPlay_t",
    ][::std::mem::size_of::<MusicPlayerWantsPlay_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsPlay_t",
    ][::std::mem::align_of::<MusicPlayerWantsPlay_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsPause_t {
    pub _address: u8,
}
pub const MusicPlayerWantsPause_t_k_iCallback: MusicPlayerWantsPause_t__bindgen_ty_1 = MusicPlayerWantsPause_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsPause_t__bindgen_ty_1 {
    k_iCallback = 4106,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsPause_t",
    ][::std::mem::size_of::<MusicPlayerWantsPause_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsPause_t",
    ][::std::mem::align_of::<MusicPlayerWantsPause_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsPlayPrevious_t {
    pub _address: u8,
}
pub const MusicPlayerWantsPlayPrevious_t_k_iCallback: MusicPlayerWantsPlayPrevious_t__bindgen_ty_1 = MusicPlayerWantsPlayPrevious_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsPlayPrevious_t__bindgen_ty_1 {
    k_iCallback = 4107,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsPlayPrevious_t",
    ][::std::mem::size_of::<MusicPlayerWantsPlayPrevious_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsPlayPrevious_t",
    ][::std::mem::align_of::<MusicPlayerWantsPlayPrevious_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsPlayNext_t {
    pub _address: u8,
}
pub const MusicPlayerWantsPlayNext_t_k_iCallback: MusicPlayerWantsPlayNext_t__bindgen_ty_1 = MusicPlayerWantsPlayNext_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsPlayNext_t__bindgen_ty_1 {
    k_iCallback = 4108,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsPlayNext_t",
    ][::std::mem::size_of::<MusicPlayerWantsPlayNext_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsPlayNext_t",
    ][::std::mem::align_of::<MusicPlayerWantsPlayNext_t>() - 1usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsShuffled_t {
    pub m_bShuffled: bool,
}
pub const MusicPlayerWantsShuffled_t_k_iCallback: MusicPlayerWantsShuffled_t__bindgen_ty_1 = MusicPlayerWantsShuffled_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsShuffled_t__bindgen_ty_1 {
    k_iCallback = 4109,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsShuffled_t",
    ][::std::mem::size_of::<MusicPlayerWantsShuffled_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsShuffled_t",
    ][::std::mem::align_of::<MusicPlayerWantsShuffled_t>() - 1usize];
    [
        "Offset of field: MusicPlayerWantsShuffled_t::m_bShuffled",
    ][::std::mem::offset_of!(MusicPlayerWantsShuffled_t, m_bShuffled) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsLooped_t {
    pub m_bLooped: bool,
}
pub const MusicPlayerWantsLooped_t_k_iCallback: MusicPlayerWantsLooped_t__bindgen_ty_1 = MusicPlayerWantsLooped_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsLooped_t__bindgen_ty_1 {
    k_iCallback = 4110,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsLooped_t",
    ][::std::mem::size_of::<MusicPlayerWantsLooped_t>() - 1usize];
    [
        "Alignment of MusicPlayerWantsLooped_t",
    ][::std::mem::align_of::<MusicPlayerWantsLooped_t>() - 1usize];
    [
        "Offset of field: MusicPlayerWantsLooped_t::m_bLooped",
    ][::std::mem::offset_of!(MusicPlayerWantsLooped_t, m_bLooped) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsVolume_t {
    pub m_flNewVolume: f32,
}
pub const MusicPlayerWantsVolume_t_k_iCallback: MusicPlayerWantsVolume_t__bindgen_ty_1 = MusicPlayerWantsVolume_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsVolume_t__bindgen_ty_1 {
    k_iCallback = 4011,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsVolume_t",
    ][::std::mem::size_of::<MusicPlayerWantsVolume_t>() - 4usize];
    [
        "Alignment of MusicPlayerWantsVolume_t",
    ][::std::mem::align_of::<MusicPlayerWantsVolume_t>() - 4usize];
    [
        "Offset of field: MusicPlayerWantsVolume_t::m_flNewVolume",
    ][::std::mem::offset_of!(MusicPlayerWantsVolume_t, m_flNewVolume) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerSelectsQueueEntry_t {
    pub nID: ::std::os::raw::c_int,
}
pub const MusicPlayerSelectsQueueEntry_t_k_iCallback: MusicPlayerSelectsQueueEntry_t__bindgen_ty_1 = MusicPlayerSelectsQueueEntry_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerSelectsQueueEntry_t__bindgen_ty_1 {
    k_iCallback = 4012,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerSelectsQueueEntry_t",
    ][::std::mem::size_of::<MusicPlayerSelectsQueueEntry_t>() - 4usize];
    [
        "Alignment of MusicPlayerSelectsQueueEntry_t",
    ][::std::mem::align_of::<MusicPlayerSelectsQueueEntry_t>() - 4usize];
    [
        "Offset of field: MusicPlayerSelectsQueueEntry_t::nID",
    ][::std::mem::offset_of!(MusicPlayerSelectsQueueEntry_t, nID) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerSelectsPlaylistEntry_t {
    pub nID: ::std::os::raw::c_int,
}
pub const MusicPlayerSelectsPlaylistEntry_t_k_iCallback: MusicPlayerSelectsPlaylistEntry_t__bindgen_ty_1 = MusicPlayerSelectsPlaylistEntry_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerSelectsPlaylistEntry_t__bindgen_ty_1 {
    k_iCallback = 4013,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerSelectsPlaylistEntry_t",
    ][::std::mem::size_of::<MusicPlayerSelectsPlaylistEntry_t>() - 4usize];
    [
        "Alignment of MusicPlayerSelectsPlaylistEntry_t",
    ][::std::mem::align_of::<MusicPlayerSelectsPlaylistEntry_t>() - 4usize];
    [
        "Offset of field: MusicPlayerSelectsPlaylistEntry_t::nID",
    ][::std::mem::offset_of!(MusicPlayerSelectsPlaylistEntry_t, nID) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MusicPlayerWantsPlayingRepeatStatus_t {
    pub m_nPlayingRepeatStatus: ::std::os::raw::c_int,
}
pub const MusicPlayerWantsPlayingRepeatStatus_t_k_iCallback: MusicPlayerWantsPlayingRepeatStatus_t__bindgen_ty_1 = MusicPlayerWantsPlayingRepeatStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum MusicPlayerWantsPlayingRepeatStatus_t__bindgen_ty_1 {
    k_iCallback = 4114,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of MusicPlayerWantsPlayingRepeatStatus_t",
    ][::std::mem::size_of::<MusicPlayerWantsPlayingRepeatStatus_t>() - 4usize];
    [
        "Alignment of MusicPlayerWantsPlayingRepeatStatus_t",
    ][::std::mem::align_of::<MusicPlayerWantsPlayingRepeatStatus_t>() - 4usize];
    [
        "Offset of field: MusicPlayerWantsPlayingRepeatStatus_t::m_nPlayingRepeatStatus",
    ][::std::mem::offset_of!(
        MusicPlayerWantsPlayingRepeatStatus_t, m_nPlayingRepeatStatus
    ) - 0usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EHTTPMethod {
    k_EHTTPMethodInvalid = 0,
    k_EHTTPMethodGET = 1,
    k_EHTTPMethodHEAD = 2,
    k_EHTTPMethodPOST = 3,
    k_EHTTPMethodPUT = 4,
    k_EHTTPMethodDELETE = 5,
    k_EHTTPMethodOPTIONS = 6,
    k_EHTTPMethodPATCH = 7,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EHTTPStatusCode {
    k_EHTTPStatusCodeInvalid = 0,
    k_EHTTPStatusCode100Continue = 100,
    k_EHTTPStatusCode101SwitchingProtocols = 101,
    k_EHTTPStatusCode200OK = 200,
    k_EHTTPStatusCode201Created = 201,
    k_EHTTPStatusCode202Accepted = 202,
    k_EHTTPStatusCode203NonAuthoritative = 203,
    k_EHTTPStatusCode204NoContent = 204,
    k_EHTTPStatusCode205ResetContent = 205,
    k_EHTTPStatusCode206PartialContent = 206,
    k_EHTTPStatusCode300MultipleChoices = 300,
    k_EHTTPStatusCode301MovedPermanently = 301,
    k_EHTTPStatusCode302Found = 302,
    k_EHTTPStatusCode303SeeOther = 303,
    k_EHTTPStatusCode304NotModified = 304,
    k_EHTTPStatusCode305UseProxy = 305,
    k_EHTTPStatusCode307TemporaryRedirect = 307,
    k_EHTTPStatusCode308PermanentRedirect = 308,
    k_EHTTPStatusCode400BadRequest = 400,
    k_EHTTPStatusCode401Unauthorized = 401,
    k_EHTTPStatusCode402PaymentRequired = 402,
    k_EHTTPStatusCode403Forbidden = 403,
    k_EHTTPStatusCode404NotFound = 404,
    k_EHTTPStatusCode405MethodNotAllowed = 405,
    k_EHTTPStatusCode406NotAcceptable = 406,
    k_EHTTPStatusCode407ProxyAuthRequired = 407,
    k_EHTTPStatusCode408RequestTimeout = 408,
    k_EHTTPStatusCode409Conflict = 409,
    k_EHTTPStatusCode410Gone = 410,
    k_EHTTPStatusCode411LengthRequired = 411,
    k_EHTTPStatusCode412PreconditionFailed = 412,
    k_EHTTPStatusCode413RequestEntityTooLarge = 413,
    k_EHTTPStatusCode414RequestURITooLong = 414,
    k_EHTTPStatusCode415UnsupportedMediaType = 415,
    k_EHTTPStatusCode416RequestedRangeNotSatisfiable = 416,
    k_EHTTPStatusCode417ExpectationFailed = 417,
    k_EHTTPStatusCode4xxUnknown = 418,
    k_EHTTPStatusCode429TooManyRequests = 429,
    k_EHTTPStatusCode444ConnectionClosed = 444,
    k_EHTTPStatusCode500InternalServerError = 500,
    k_EHTTPStatusCode501NotImplemented = 501,
    k_EHTTPStatusCode502BadGateway = 502,
    k_EHTTPStatusCode503ServiceUnavailable = 503,
    k_EHTTPStatusCode504GatewayTimeout = 504,
    k_EHTTPStatusCode505HTTPVersionNotSupported = 505,
    k_EHTTPStatusCode5xxUnknown = 599,
}
pub type HTTPRequestHandle = uint32;
pub type HTTPCookieContainerHandle = uint32;
#[repr(C)]
pub struct ISteamHTTP__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamHTTP {
    pub vtable_: *const ISteamHTTP__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamHTTP"][::std::mem::size_of::<ISteamHTTP>() - 8usize];
    ["Alignment of ISteamHTTP"][::std::mem::align_of::<ISteamHTTP>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTTPRequestCompleted_t {
    pub m_hRequest: HTTPRequestHandle,
    pub m_ulContextValue: uint64,
    pub m_bRequestSuccessful: bool,
    pub m_eStatusCode: EHTTPStatusCode,
    pub m_unBodySize: uint32,
}
pub const HTTPRequestCompleted_t_k_iCallback: HTTPRequestCompleted_t__bindgen_ty_1 = HTTPRequestCompleted_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTTPRequestCompleted_t__bindgen_ty_1 {
    k_iCallback = 2101,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTTPRequestCompleted_t",
    ][::std::mem::size_of::<HTTPRequestCompleted_t>() - 24usize];
    [
        "Alignment of HTTPRequestCompleted_t",
    ][::std::mem::align_of::<HTTPRequestCompleted_t>() - 4usize];
    [
        "Offset of field: HTTPRequestCompleted_t::m_hRequest",
    ][::std::mem::offset_of!(HTTPRequestCompleted_t, m_hRequest) - 0usize];
    [
        "Offset of field: HTTPRequestCompleted_t::m_ulContextValue",
    ][::std::mem::offset_of!(HTTPRequestCompleted_t, m_ulContextValue) - 4usize];
    [
        "Offset of field: HTTPRequestCompleted_t::m_bRequestSuccessful",
    ][::std::mem::offset_of!(HTTPRequestCompleted_t, m_bRequestSuccessful) - 12usize];
    [
        "Offset of field: HTTPRequestCompleted_t::m_eStatusCode",
    ][::std::mem::offset_of!(HTTPRequestCompleted_t, m_eStatusCode) - 16usize];
    [
        "Offset of field: HTTPRequestCompleted_t::m_unBodySize",
    ][::std::mem::offset_of!(HTTPRequestCompleted_t, m_unBodySize) - 20usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTTPRequestHeadersReceived_t {
    pub m_hRequest: HTTPRequestHandle,
    pub m_ulContextValue: uint64,
}
pub const HTTPRequestHeadersReceived_t_k_iCallback: HTTPRequestHeadersReceived_t__bindgen_ty_1 = HTTPRequestHeadersReceived_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTTPRequestHeadersReceived_t__bindgen_ty_1 {
    k_iCallback = 2102,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTTPRequestHeadersReceived_t",
    ][::std::mem::size_of::<HTTPRequestHeadersReceived_t>() - 12usize];
    [
        "Alignment of HTTPRequestHeadersReceived_t",
    ][::std::mem::align_of::<HTTPRequestHeadersReceived_t>() - 4usize];
    [
        "Offset of field: HTTPRequestHeadersReceived_t::m_hRequest",
    ][::std::mem::offset_of!(HTTPRequestHeadersReceived_t, m_hRequest) - 0usize];
    [
        "Offset of field: HTTPRequestHeadersReceived_t::m_ulContextValue",
    ][::std::mem::offset_of!(HTTPRequestHeadersReceived_t, m_ulContextValue) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTTPRequestDataReceived_t {
    pub m_hRequest: HTTPRequestHandle,
    pub m_ulContextValue: uint64,
    pub m_cOffset: uint32,
    pub m_cBytesReceived: uint32,
}
pub const HTTPRequestDataReceived_t_k_iCallback: HTTPRequestDataReceived_t__bindgen_ty_1 = HTTPRequestDataReceived_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTTPRequestDataReceived_t__bindgen_ty_1 {
    k_iCallback = 2103,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTTPRequestDataReceived_t",
    ][::std::mem::size_of::<HTTPRequestDataReceived_t>() - 20usize];
    [
        "Alignment of HTTPRequestDataReceived_t",
    ][::std::mem::align_of::<HTTPRequestDataReceived_t>() - 4usize];
    [
        "Offset of field: HTTPRequestDataReceived_t::m_hRequest",
    ][::std::mem::offset_of!(HTTPRequestDataReceived_t, m_hRequest) - 0usize];
    [
        "Offset of field: HTTPRequestDataReceived_t::m_ulContextValue",
    ][::std::mem::offset_of!(HTTPRequestDataReceived_t, m_ulContextValue) - 4usize];
    [
        "Offset of field: HTTPRequestDataReceived_t::m_cOffset",
    ][::std::mem::offset_of!(HTTPRequestDataReceived_t, m_cOffset) - 12usize];
    [
        "Offset of field: HTTPRequestDataReceived_t::m_cBytesReceived",
    ][::std::mem::offset_of!(HTTPRequestDataReceived_t, m_cBytesReceived) - 16usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EInputSourceMode {
    k_EInputSourceMode_None = 0,
    k_EInputSourceMode_Dpad = 1,
    k_EInputSourceMode_Buttons = 2,
    k_EInputSourceMode_FourButtons = 3,
    k_EInputSourceMode_AbsoluteMouse = 4,
    k_EInputSourceMode_RelativeMouse = 5,
    k_EInputSourceMode_JoystickMove = 6,
    k_EInputSourceMode_JoystickMouse = 7,
    k_EInputSourceMode_JoystickCamera = 8,
    k_EInputSourceMode_ScrollWheel = 9,
    k_EInputSourceMode_Trigger = 10,
    k_EInputSourceMode_TouchMenu = 11,
    k_EInputSourceMode_MouseJoystick = 12,
    k_EInputSourceMode_MouseRegion = 13,
    k_EInputSourceMode_RadialMenu = 14,
    k_EInputSourceMode_SingleButton = 15,
    k_EInputSourceMode_Switches = 16,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EInputActionOrigin {
    k_EInputActionOrigin_None = 0,
    k_EInputActionOrigin_SteamController_A = 1,
    k_EInputActionOrigin_SteamController_B = 2,
    k_EInputActionOrigin_SteamController_X = 3,
    k_EInputActionOrigin_SteamController_Y = 4,
    k_EInputActionOrigin_SteamController_LeftBumper = 5,
    k_EInputActionOrigin_SteamController_RightBumper = 6,
    k_EInputActionOrigin_SteamController_LeftGrip = 7,
    k_EInputActionOrigin_SteamController_RightGrip = 8,
    k_EInputActionOrigin_SteamController_Start = 9,
    k_EInputActionOrigin_SteamController_Back = 10,
    k_EInputActionOrigin_SteamController_LeftPad_Touch = 11,
    k_EInputActionOrigin_SteamController_LeftPad_Swipe = 12,
    k_EInputActionOrigin_SteamController_LeftPad_Click = 13,
    k_EInputActionOrigin_SteamController_LeftPad_DPadNorth = 14,
    k_EInputActionOrigin_SteamController_LeftPad_DPadSouth = 15,
    k_EInputActionOrigin_SteamController_LeftPad_DPadWest = 16,
    k_EInputActionOrigin_SteamController_LeftPad_DPadEast = 17,
    k_EInputActionOrigin_SteamController_RightPad_Touch = 18,
    k_EInputActionOrigin_SteamController_RightPad_Swipe = 19,
    k_EInputActionOrigin_SteamController_RightPad_Click = 20,
    k_EInputActionOrigin_SteamController_RightPad_DPadNorth = 21,
    k_EInputActionOrigin_SteamController_RightPad_DPadSouth = 22,
    k_EInputActionOrigin_SteamController_RightPad_DPadWest = 23,
    k_EInputActionOrigin_SteamController_RightPad_DPadEast = 24,
    k_EInputActionOrigin_SteamController_LeftTrigger_Pull = 25,
    k_EInputActionOrigin_SteamController_LeftTrigger_Click = 26,
    k_EInputActionOrigin_SteamController_RightTrigger_Pull = 27,
    k_EInputActionOrigin_SteamController_RightTrigger_Click = 28,
    k_EInputActionOrigin_SteamController_LeftStick_Move = 29,
    k_EInputActionOrigin_SteamController_LeftStick_Click = 30,
    k_EInputActionOrigin_SteamController_LeftStick_DPadNorth = 31,
    k_EInputActionOrigin_SteamController_LeftStick_DPadSouth = 32,
    k_EInputActionOrigin_SteamController_LeftStick_DPadWest = 33,
    k_EInputActionOrigin_SteamController_LeftStick_DPadEast = 34,
    k_EInputActionOrigin_SteamController_Gyro_Move = 35,
    k_EInputActionOrigin_SteamController_Gyro_Pitch = 36,
    k_EInputActionOrigin_SteamController_Gyro_Yaw = 37,
    k_EInputActionOrigin_SteamController_Gyro_Roll = 38,
    k_EInputActionOrigin_SteamController_Reserved0 = 39,
    k_EInputActionOrigin_SteamController_Reserved1 = 40,
    k_EInputActionOrigin_SteamController_Reserved2 = 41,
    k_EInputActionOrigin_SteamController_Reserved3 = 42,
    k_EInputActionOrigin_SteamController_Reserved4 = 43,
    k_EInputActionOrigin_SteamController_Reserved5 = 44,
    k_EInputActionOrigin_SteamController_Reserved6 = 45,
    k_EInputActionOrigin_SteamController_Reserved7 = 46,
    k_EInputActionOrigin_SteamController_Reserved8 = 47,
    k_EInputActionOrigin_SteamController_Reserved9 = 48,
    k_EInputActionOrigin_SteamController_Reserved10 = 49,
    k_EInputActionOrigin_PS4_X = 50,
    k_EInputActionOrigin_PS4_Circle = 51,
    k_EInputActionOrigin_PS4_Triangle = 52,
    k_EInputActionOrigin_PS4_Square = 53,
    k_EInputActionOrigin_PS4_LeftBumper = 54,
    k_EInputActionOrigin_PS4_RightBumper = 55,
    k_EInputActionOrigin_PS4_Options = 56,
    k_EInputActionOrigin_PS4_Share = 57,
    k_EInputActionOrigin_PS4_LeftPad_Touch = 58,
    k_EInputActionOrigin_PS4_LeftPad_Swipe = 59,
    k_EInputActionOrigin_PS4_LeftPad_Click = 60,
    k_EInputActionOrigin_PS4_LeftPad_DPadNorth = 61,
    k_EInputActionOrigin_PS4_LeftPad_DPadSouth = 62,
    k_EInputActionOrigin_PS4_LeftPad_DPadWest = 63,
    k_EInputActionOrigin_PS4_LeftPad_DPadEast = 64,
    k_EInputActionOrigin_PS4_RightPad_Touch = 65,
    k_EInputActionOrigin_PS4_RightPad_Swipe = 66,
    k_EInputActionOrigin_PS4_RightPad_Click = 67,
    k_EInputActionOrigin_PS4_RightPad_DPadNorth = 68,
    k_EInputActionOrigin_PS4_RightPad_DPadSouth = 69,
    k_EInputActionOrigin_PS4_RightPad_DPadWest = 70,
    k_EInputActionOrigin_PS4_RightPad_DPadEast = 71,
    k_EInputActionOrigin_PS4_CenterPad_Touch = 72,
    k_EInputActionOrigin_PS4_CenterPad_Swipe = 73,
    k_EInputActionOrigin_PS4_CenterPad_Click = 74,
    k_EInputActionOrigin_PS4_CenterPad_DPadNorth = 75,
    k_EInputActionOrigin_PS4_CenterPad_DPadSouth = 76,
    k_EInputActionOrigin_PS4_CenterPad_DPadWest = 77,
    k_EInputActionOrigin_PS4_CenterPad_DPadEast = 78,
    k_EInputActionOrigin_PS4_LeftTrigger_Pull = 79,
    k_EInputActionOrigin_PS4_LeftTrigger_Click = 80,
    k_EInputActionOrigin_PS4_RightTrigger_Pull = 81,
    k_EInputActionOrigin_PS4_RightTrigger_Click = 82,
    k_EInputActionOrigin_PS4_LeftStick_Move = 83,
    k_EInputActionOrigin_PS4_LeftStick_Click = 84,
    k_EInputActionOrigin_PS4_LeftStick_DPadNorth = 85,
    k_EInputActionOrigin_PS4_LeftStick_DPadSouth = 86,
    k_EInputActionOrigin_PS4_LeftStick_DPadWest = 87,
    k_EInputActionOrigin_PS4_LeftStick_DPadEast = 88,
    k_EInputActionOrigin_PS4_RightStick_Move = 89,
    k_EInputActionOrigin_PS4_RightStick_Click = 90,
    k_EInputActionOrigin_PS4_RightStick_DPadNorth = 91,
    k_EInputActionOrigin_PS4_RightStick_DPadSouth = 92,
    k_EInputActionOrigin_PS4_RightStick_DPadWest = 93,
    k_EInputActionOrigin_PS4_RightStick_DPadEast = 94,
    k_EInputActionOrigin_PS4_DPad_North = 95,
    k_EInputActionOrigin_PS4_DPad_South = 96,
    k_EInputActionOrigin_PS4_DPad_West = 97,
    k_EInputActionOrigin_PS4_DPad_East = 98,
    k_EInputActionOrigin_PS4_Gyro_Move = 99,
    k_EInputActionOrigin_PS4_Gyro_Pitch = 100,
    k_EInputActionOrigin_PS4_Gyro_Yaw = 101,
    k_EInputActionOrigin_PS4_Gyro_Roll = 102,
    k_EInputActionOrigin_PS4_DPad_Move = 103,
    k_EInputActionOrigin_PS4_Reserved1 = 104,
    k_EInputActionOrigin_PS4_Reserved2 = 105,
    k_EInputActionOrigin_PS4_Reserved3 = 106,
    k_EInputActionOrigin_PS4_Reserved4 = 107,
    k_EInputActionOrigin_PS4_Reserved5 = 108,
    k_EInputActionOrigin_PS4_Reserved6 = 109,
    k_EInputActionOrigin_PS4_Reserved7 = 110,
    k_EInputActionOrigin_PS4_Reserved8 = 111,
    k_EInputActionOrigin_PS4_Reserved9 = 112,
    k_EInputActionOrigin_PS4_Reserved10 = 113,
    k_EInputActionOrigin_XBoxOne_A = 114,
    k_EInputActionOrigin_XBoxOne_B = 115,
    k_EInputActionOrigin_XBoxOne_X = 116,
    k_EInputActionOrigin_XBoxOne_Y = 117,
    k_EInputActionOrigin_XBoxOne_LeftBumper = 118,
    k_EInputActionOrigin_XBoxOne_RightBumper = 119,
    k_EInputActionOrigin_XBoxOne_Menu = 120,
    k_EInputActionOrigin_XBoxOne_View = 121,
    k_EInputActionOrigin_XBoxOne_LeftTrigger_Pull = 122,
    k_EInputActionOrigin_XBoxOne_LeftTrigger_Click = 123,
    k_EInputActionOrigin_XBoxOne_RightTrigger_Pull = 124,
    k_EInputActionOrigin_XBoxOne_RightTrigger_Click = 125,
    k_EInputActionOrigin_XBoxOne_LeftStick_Move = 126,
    k_EInputActionOrigin_XBoxOne_LeftStick_Click = 127,
    k_EInputActionOrigin_XBoxOne_LeftStick_DPadNorth = 128,
    k_EInputActionOrigin_XBoxOne_LeftStick_DPadSouth = 129,
    k_EInputActionOrigin_XBoxOne_LeftStick_DPadWest = 130,
    k_EInputActionOrigin_XBoxOne_LeftStick_DPadEast = 131,
    k_EInputActionOrigin_XBoxOne_RightStick_Move = 132,
    k_EInputActionOrigin_XBoxOne_RightStick_Click = 133,
    k_EInputActionOrigin_XBoxOne_RightStick_DPadNorth = 134,
    k_EInputActionOrigin_XBoxOne_RightStick_DPadSouth = 135,
    k_EInputActionOrigin_XBoxOne_RightStick_DPadWest = 136,
    k_EInputActionOrigin_XBoxOne_RightStick_DPadEast = 137,
    k_EInputActionOrigin_XBoxOne_DPad_North = 138,
    k_EInputActionOrigin_XBoxOne_DPad_South = 139,
    k_EInputActionOrigin_XBoxOne_DPad_West = 140,
    k_EInputActionOrigin_XBoxOne_DPad_East = 141,
    k_EInputActionOrigin_XBoxOne_DPad_Move = 142,
    k_EInputActionOrigin_XBoxOne_LeftGrip_Lower = 143,
    k_EInputActionOrigin_XBoxOne_LeftGrip_Upper = 144,
    k_EInputActionOrigin_XBoxOne_RightGrip_Lower = 145,
    k_EInputActionOrigin_XBoxOne_RightGrip_Upper = 146,
    k_EInputActionOrigin_XBoxOne_Share = 147,
    k_EInputActionOrigin_XBoxOne_Reserved6 = 148,
    k_EInputActionOrigin_XBoxOne_Reserved7 = 149,
    k_EInputActionOrigin_XBoxOne_Reserved8 = 150,
    k_EInputActionOrigin_XBoxOne_Reserved9 = 151,
    k_EInputActionOrigin_XBoxOne_Reserved10 = 152,
    k_EInputActionOrigin_XBox360_A = 153,
    k_EInputActionOrigin_XBox360_B = 154,
    k_EInputActionOrigin_XBox360_X = 155,
    k_EInputActionOrigin_XBox360_Y = 156,
    k_EInputActionOrigin_XBox360_LeftBumper = 157,
    k_EInputActionOrigin_XBox360_RightBumper = 158,
    k_EInputActionOrigin_XBox360_Start = 159,
    k_EInputActionOrigin_XBox360_Back = 160,
    k_EInputActionOrigin_XBox360_LeftTrigger_Pull = 161,
    k_EInputActionOrigin_XBox360_LeftTrigger_Click = 162,
    k_EInputActionOrigin_XBox360_RightTrigger_Pull = 163,
    k_EInputActionOrigin_XBox360_RightTrigger_Click = 164,
    k_EInputActionOrigin_XBox360_LeftStick_Move = 165,
    k_EInputActionOrigin_XBox360_LeftStick_Click = 166,
    k_EInputActionOrigin_XBox360_LeftStick_DPadNorth = 167,
    k_EInputActionOrigin_XBox360_LeftStick_DPadSouth = 168,
    k_EInputActionOrigin_XBox360_LeftStick_DPadWest = 169,
    k_EInputActionOrigin_XBox360_LeftStick_DPadEast = 170,
    k_EInputActionOrigin_XBox360_RightStick_Move = 171,
    k_EInputActionOrigin_XBox360_RightStick_Click = 172,
    k_EInputActionOrigin_XBox360_RightStick_DPadNorth = 173,
    k_EInputActionOrigin_XBox360_RightStick_DPadSouth = 174,
    k_EInputActionOrigin_XBox360_RightStick_DPadWest = 175,
    k_EInputActionOrigin_XBox360_RightStick_DPadEast = 176,
    k_EInputActionOrigin_XBox360_DPad_North = 177,
    k_EInputActionOrigin_XBox360_DPad_South = 178,
    k_EInputActionOrigin_XBox360_DPad_West = 179,
    k_EInputActionOrigin_XBox360_DPad_East = 180,
    k_EInputActionOrigin_XBox360_DPad_Move = 181,
    k_EInputActionOrigin_XBox360_Reserved1 = 182,
    k_EInputActionOrigin_XBox360_Reserved2 = 183,
    k_EInputActionOrigin_XBox360_Reserved3 = 184,
    k_EInputActionOrigin_XBox360_Reserved4 = 185,
    k_EInputActionOrigin_XBox360_Reserved5 = 186,
    k_EInputActionOrigin_XBox360_Reserved6 = 187,
    k_EInputActionOrigin_XBox360_Reserved7 = 188,
    k_EInputActionOrigin_XBox360_Reserved8 = 189,
    k_EInputActionOrigin_XBox360_Reserved9 = 190,
    k_EInputActionOrigin_XBox360_Reserved10 = 191,
    k_EInputActionOrigin_Switch_A = 192,
    k_EInputActionOrigin_Switch_B = 193,
    k_EInputActionOrigin_Switch_X = 194,
    k_EInputActionOrigin_Switch_Y = 195,
    k_EInputActionOrigin_Switch_LeftBumper = 196,
    k_EInputActionOrigin_Switch_RightBumper = 197,
    k_EInputActionOrigin_Switch_Plus = 198,
    k_EInputActionOrigin_Switch_Minus = 199,
    k_EInputActionOrigin_Switch_Capture = 200,
    k_EInputActionOrigin_Switch_LeftTrigger_Pull = 201,
    k_EInputActionOrigin_Switch_LeftTrigger_Click = 202,
    k_EInputActionOrigin_Switch_RightTrigger_Pull = 203,
    k_EInputActionOrigin_Switch_RightTrigger_Click = 204,
    k_EInputActionOrigin_Switch_LeftStick_Move = 205,
    k_EInputActionOrigin_Switch_LeftStick_Click = 206,
    k_EInputActionOrigin_Switch_LeftStick_DPadNorth = 207,
    k_EInputActionOrigin_Switch_LeftStick_DPadSouth = 208,
    k_EInputActionOrigin_Switch_LeftStick_DPadWest = 209,
    k_EInputActionOrigin_Switch_LeftStick_DPadEast = 210,
    k_EInputActionOrigin_Switch_RightStick_Move = 211,
    k_EInputActionOrigin_Switch_RightStick_Click = 212,
    k_EInputActionOrigin_Switch_RightStick_DPadNorth = 213,
    k_EInputActionOrigin_Switch_RightStick_DPadSouth = 214,
    k_EInputActionOrigin_Switch_RightStick_DPadWest = 215,
    k_EInputActionOrigin_Switch_RightStick_DPadEast = 216,
    k_EInputActionOrigin_Switch_DPad_North = 217,
    k_EInputActionOrigin_Switch_DPad_South = 218,
    k_EInputActionOrigin_Switch_DPad_West = 219,
    k_EInputActionOrigin_Switch_DPad_East = 220,
    k_EInputActionOrigin_Switch_ProGyro_Move = 221,
    k_EInputActionOrigin_Switch_ProGyro_Pitch = 222,
    k_EInputActionOrigin_Switch_ProGyro_Yaw = 223,
    k_EInputActionOrigin_Switch_ProGyro_Roll = 224,
    k_EInputActionOrigin_Switch_DPad_Move = 225,
    k_EInputActionOrigin_Switch_Reserved1 = 226,
    k_EInputActionOrigin_Switch_Reserved2 = 227,
    k_EInputActionOrigin_Switch_Reserved3 = 228,
    k_EInputActionOrigin_Switch_Reserved4 = 229,
    k_EInputActionOrigin_Switch_Reserved5 = 230,
    k_EInputActionOrigin_Switch_Reserved6 = 231,
    k_EInputActionOrigin_Switch_Reserved7 = 232,
    k_EInputActionOrigin_Switch_Reserved8 = 233,
    k_EInputActionOrigin_Switch_Reserved9 = 234,
    k_EInputActionOrigin_Switch_Reserved10 = 235,
    k_EInputActionOrigin_Switch_RightGyro_Move = 236,
    k_EInputActionOrigin_Switch_RightGyro_Pitch = 237,
    k_EInputActionOrigin_Switch_RightGyro_Yaw = 238,
    k_EInputActionOrigin_Switch_RightGyro_Roll = 239,
    k_EInputActionOrigin_Switch_LeftGyro_Move = 240,
    k_EInputActionOrigin_Switch_LeftGyro_Pitch = 241,
    k_EInputActionOrigin_Switch_LeftGyro_Yaw = 242,
    k_EInputActionOrigin_Switch_LeftGyro_Roll = 243,
    k_EInputActionOrigin_Switch_LeftGrip_Lower = 244,
    k_EInputActionOrigin_Switch_LeftGrip_Upper = 245,
    k_EInputActionOrigin_Switch_RightGrip_Lower = 246,
    k_EInputActionOrigin_Switch_RightGrip_Upper = 247,
    k_EInputActionOrigin_Switch_JoyConButton_N = 248,
    k_EInputActionOrigin_Switch_JoyConButton_E = 249,
    k_EInputActionOrigin_Switch_JoyConButton_S = 250,
    k_EInputActionOrigin_Switch_JoyConButton_W = 251,
    k_EInputActionOrigin_Switch_Reserved15 = 252,
    k_EInputActionOrigin_Switch_Reserved16 = 253,
    k_EInputActionOrigin_Switch_Reserved17 = 254,
    k_EInputActionOrigin_Switch_Reserved18 = 255,
    k_EInputActionOrigin_Switch_Reserved19 = 256,
    k_EInputActionOrigin_Switch_Reserved20 = 257,
    k_EInputActionOrigin_PS5_X = 258,
    k_EInputActionOrigin_PS5_Circle = 259,
    k_EInputActionOrigin_PS5_Triangle = 260,
    k_EInputActionOrigin_PS5_Square = 261,
    k_EInputActionOrigin_PS5_LeftBumper = 262,
    k_EInputActionOrigin_PS5_RightBumper = 263,
    k_EInputActionOrigin_PS5_Option = 264,
    k_EInputActionOrigin_PS5_Create = 265,
    k_EInputActionOrigin_PS5_Mute = 266,
    k_EInputActionOrigin_PS5_LeftPad_Touch = 267,
    k_EInputActionOrigin_PS5_LeftPad_Swipe = 268,
    k_EInputActionOrigin_PS5_LeftPad_Click = 269,
    k_EInputActionOrigin_PS5_LeftPad_DPadNorth = 270,
    k_EInputActionOrigin_PS5_LeftPad_DPadSouth = 271,
    k_EInputActionOrigin_PS5_LeftPad_DPadWest = 272,
    k_EInputActionOrigin_PS5_LeftPad_DPadEast = 273,
    k_EInputActionOrigin_PS5_RightPad_Touch = 274,
    k_EInputActionOrigin_PS5_RightPad_Swipe = 275,
    k_EInputActionOrigin_PS5_RightPad_Click = 276,
    k_EInputActionOrigin_PS5_RightPad_DPadNorth = 277,
    k_EInputActionOrigin_PS5_RightPad_DPadSouth = 278,
    k_EInputActionOrigin_PS5_RightPad_DPadWest = 279,
    k_EInputActionOrigin_PS5_RightPad_DPadEast = 280,
    k_EInputActionOrigin_PS5_CenterPad_Touch = 281,
    k_EInputActionOrigin_PS5_CenterPad_Swipe = 282,
    k_EInputActionOrigin_PS5_CenterPad_Click = 283,
    k_EInputActionOrigin_PS5_CenterPad_DPadNorth = 284,
    k_EInputActionOrigin_PS5_CenterPad_DPadSouth = 285,
    k_EInputActionOrigin_PS5_CenterPad_DPadWest = 286,
    k_EInputActionOrigin_PS5_CenterPad_DPadEast = 287,
    k_EInputActionOrigin_PS5_LeftTrigger_Pull = 288,
    k_EInputActionOrigin_PS5_LeftTrigger_Click = 289,
    k_EInputActionOrigin_PS5_RightTrigger_Pull = 290,
    k_EInputActionOrigin_PS5_RightTrigger_Click = 291,
    k_EInputActionOrigin_PS5_LeftStick_Move = 292,
    k_EInputActionOrigin_PS5_LeftStick_Click = 293,
    k_EInputActionOrigin_PS5_LeftStick_DPadNorth = 294,
    k_EInputActionOrigin_PS5_LeftStick_DPadSouth = 295,
    k_EInputActionOrigin_PS5_LeftStick_DPadWest = 296,
    k_EInputActionOrigin_PS5_LeftStick_DPadEast = 297,
    k_EInputActionOrigin_PS5_RightStick_Move = 298,
    k_EInputActionOrigin_PS5_RightStick_Click = 299,
    k_EInputActionOrigin_PS5_RightStick_DPadNorth = 300,
    k_EInputActionOrigin_PS5_RightStick_DPadSouth = 301,
    k_EInputActionOrigin_PS5_RightStick_DPadWest = 302,
    k_EInputActionOrigin_PS5_RightStick_DPadEast = 303,
    k_EInputActionOrigin_PS5_DPad_North = 304,
    k_EInputActionOrigin_PS5_DPad_South = 305,
    k_EInputActionOrigin_PS5_DPad_West = 306,
    k_EInputActionOrigin_PS5_DPad_East = 307,
    k_EInputActionOrigin_PS5_Gyro_Move = 308,
    k_EInputActionOrigin_PS5_Gyro_Pitch = 309,
    k_EInputActionOrigin_PS5_Gyro_Yaw = 310,
    k_EInputActionOrigin_PS5_Gyro_Roll = 311,
    k_EInputActionOrigin_PS5_DPad_Move = 312,
    k_EInputActionOrigin_PS5_LeftGrip = 313,
    k_EInputActionOrigin_PS5_RightGrip = 314,
    k_EInputActionOrigin_PS5_LeftFn = 315,
    k_EInputActionOrigin_PS5_RightFn = 316,
    k_EInputActionOrigin_PS5_Reserved5 = 317,
    k_EInputActionOrigin_PS5_Reserved6 = 318,
    k_EInputActionOrigin_PS5_Reserved7 = 319,
    k_EInputActionOrigin_PS5_Reserved8 = 320,
    k_EInputActionOrigin_PS5_Reserved9 = 321,
    k_EInputActionOrigin_PS5_Reserved10 = 322,
    k_EInputActionOrigin_PS5_Reserved11 = 323,
    k_EInputActionOrigin_PS5_Reserved12 = 324,
    k_EInputActionOrigin_PS5_Reserved13 = 325,
    k_EInputActionOrigin_PS5_Reserved14 = 326,
    k_EInputActionOrigin_PS5_Reserved15 = 327,
    k_EInputActionOrigin_PS5_Reserved16 = 328,
    k_EInputActionOrigin_PS5_Reserved17 = 329,
    k_EInputActionOrigin_PS5_Reserved18 = 330,
    k_EInputActionOrigin_PS5_Reserved19 = 331,
    k_EInputActionOrigin_PS5_Reserved20 = 332,
    k_EInputActionOrigin_SteamDeck_A = 333,
    k_EInputActionOrigin_SteamDeck_B = 334,
    k_EInputActionOrigin_SteamDeck_X = 335,
    k_EInputActionOrigin_SteamDeck_Y = 336,
    k_EInputActionOrigin_SteamDeck_L1 = 337,
    k_EInputActionOrigin_SteamDeck_R1 = 338,
    k_EInputActionOrigin_SteamDeck_Menu = 339,
    k_EInputActionOrigin_SteamDeck_View = 340,
    k_EInputActionOrigin_SteamDeck_LeftPad_Touch = 341,
    k_EInputActionOrigin_SteamDeck_LeftPad_Swipe = 342,
    k_EInputActionOrigin_SteamDeck_LeftPad_Click = 343,
    k_EInputActionOrigin_SteamDeck_LeftPad_DPadNorth = 344,
    k_EInputActionOrigin_SteamDeck_LeftPad_DPadSouth = 345,
    k_EInputActionOrigin_SteamDeck_LeftPad_DPadWest = 346,
    k_EInputActionOrigin_SteamDeck_LeftPad_DPadEast = 347,
    k_EInputActionOrigin_SteamDeck_RightPad_Touch = 348,
    k_EInputActionOrigin_SteamDeck_RightPad_Swipe = 349,
    k_EInputActionOrigin_SteamDeck_RightPad_Click = 350,
    k_EInputActionOrigin_SteamDeck_RightPad_DPadNorth = 351,
    k_EInputActionOrigin_SteamDeck_RightPad_DPadSouth = 352,
    k_EInputActionOrigin_SteamDeck_RightPad_DPadWest = 353,
    k_EInputActionOrigin_SteamDeck_RightPad_DPadEast = 354,
    k_EInputActionOrigin_SteamDeck_L2_SoftPull = 355,
    k_EInputActionOrigin_SteamDeck_L2 = 356,
    k_EInputActionOrigin_SteamDeck_R2_SoftPull = 357,
    k_EInputActionOrigin_SteamDeck_R2 = 358,
    k_EInputActionOrigin_SteamDeck_LeftStick_Move = 359,
    k_EInputActionOrigin_SteamDeck_L3 = 360,
    k_EInputActionOrigin_SteamDeck_LeftStick_DPadNorth = 361,
    k_EInputActionOrigin_SteamDeck_LeftStick_DPadSouth = 362,
    k_EInputActionOrigin_SteamDeck_LeftStick_DPadWest = 363,
    k_EInputActionOrigin_SteamDeck_LeftStick_DPadEast = 364,
    k_EInputActionOrigin_SteamDeck_LeftStick_Touch = 365,
    k_EInputActionOrigin_SteamDeck_RightStick_Move = 366,
    k_EInputActionOrigin_SteamDeck_R3 = 367,
    k_EInputActionOrigin_SteamDeck_RightStick_DPadNorth = 368,
    k_EInputActionOrigin_SteamDeck_RightStick_DPadSouth = 369,
    k_EInputActionOrigin_SteamDeck_RightStick_DPadWest = 370,
    k_EInputActionOrigin_SteamDeck_RightStick_DPadEast = 371,
    k_EInputActionOrigin_SteamDeck_RightStick_Touch = 372,
    k_EInputActionOrigin_SteamDeck_L4 = 373,
    k_EInputActionOrigin_SteamDeck_R4 = 374,
    k_EInputActionOrigin_SteamDeck_L5 = 375,
    k_EInputActionOrigin_SteamDeck_R5 = 376,
    k_EInputActionOrigin_SteamDeck_DPad_Move = 377,
    k_EInputActionOrigin_SteamDeck_DPad_North = 378,
    k_EInputActionOrigin_SteamDeck_DPad_South = 379,
    k_EInputActionOrigin_SteamDeck_DPad_West = 380,
    k_EInputActionOrigin_SteamDeck_DPad_East = 381,
    k_EInputActionOrigin_SteamDeck_Gyro_Move = 382,
    k_EInputActionOrigin_SteamDeck_Gyro_Pitch = 383,
    k_EInputActionOrigin_SteamDeck_Gyro_Yaw = 384,
    k_EInputActionOrigin_SteamDeck_Gyro_Roll = 385,
    k_EInputActionOrigin_SteamDeck_Reserved1 = 386,
    k_EInputActionOrigin_SteamDeck_Reserved2 = 387,
    k_EInputActionOrigin_SteamDeck_Reserved3 = 388,
    k_EInputActionOrigin_SteamDeck_Reserved4 = 389,
    k_EInputActionOrigin_SteamDeck_Reserved5 = 390,
    k_EInputActionOrigin_SteamDeck_Reserved6 = 391,
    k_EInputActionOrigin_SteamDeck_Reserved7 = 392,
    k_EInputActionOrigin_SteamDeck_Reserved8 = 393,
    k_EInputActionOrigin_SteamDeck_Reserved9 = 394,
    k_EInputActionOrigin_SteamDeck_Reserved10 = 395,
    k_EInputActionOrigin_SteamDeck_Reserved11 = 396,
    k_EInputActionOrigin_SteamDeck_Reserved12 = 397,
    k_EInputActionOrigin_SteamDeck_Reserved13 = 398,
    k_EInputActionOrigin_SteamDeck_Reserved14 = 399,
    k_EInputActionOrigin_SteamDeck_Reserved15 = 400,
    k_EInputActionOrigin_SteamDeck_Reserved16 = 401,
    k_EInputActionOrigin_SteamDeck_Reserved17 = 402,
    k_EInputActionOrigin_SteamDeck_Reserved18 = 403,
    k_EInputActionOrigin_SteamDeck_Reserved19 = 404,
    k_EInputActionOrigin_SteamDeck_Reserved20 = 405,
    k_EInputActionOrigin_Horipad_M1 = 406,
    k_EInputActionOrigin_Horipad_M2 = 407,
    k_EInputActionOrigin_Horipad_L4 = 408,
    k_EInputActionOrigin_Horipad_R4 = 409,
    k_EInputActionOrigin_Count = 410,
    k_EInputActionOrigin_MaximumPossibleValue = 32767,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EXboxOrigin {
    k_EXboxOrigin_A = 0,
    k_EXboxOrigin_B = 1,
    k_EXboxOrigin_X = 2,
    k_EXboxOrigin_Y = 3,
    k_EXboxOrigin_LeftBumper = 4,
    k_EXboxOrigin_RightBumper = 5,
    k_EXboxOrigin_Menu = 6,
    k_EXboxOrigin_View = 7,
    k_EXboxOrigin_LeftTrigger_Pull = 8,
    k_EXboxOrigin_LeftTrigger_Click = 9,
    k_EXboxOrigin_RightTrigger_Pull = 10,
    k_EXboxOrigin_RightTrigger_Click = 11,
    k_EXboxOrigin_LeftStick_Move = 12,
    k_EXboxOrigin_LeftStick_Click = 13,
    k_EXboxOrigin_LeftStick_DPadNorth = 14,
    k_EXboxOrigin_LeftStick_DPadSouth = 15,
    k_EXboxOrigin_LeftStick_DPadWest = 16,
    k_EXboxOrigin_LeftStick_DPadEast = 17,
    k_EXboxOrigin_RightStick_Move = 18,
    k_EXboxOrigin_RightStick_Click = 19,
    k_EXboxOrigin_RightStick_DPadNorth = 20,
    k_EXboxOrigin_RightStick_DPadSouth = 21,
    k_EXboxOrigin_RightStick_DPadWest = 22,
    k_EXboxOrigin_RightStick_DPadEast = 23,
    k_EXboxOrigin_DPad_North = 24,
    k_EXboxOrigin_DPad_South = 25,
    k_EXboxOrigin_DPad_West = 26,
    k_EXboxOrigin_DPad_East = 27,
    k_EXboxOrigin_Count = 28,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamControllerPad {
    k_ESteamControllerPad_Left = 0,
    k_ESteamControllerPad_Right = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EControllerHapticLocation {
    k_EControllerHapticLocation_Left = 1,
    k_EControllerHapticLocation_Right = 2,
    k_EControllerHapticLocation_Both = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EControllerHapticType {
    k_EControllerHapticType_Off = 0,
    k_EControllerHapticType_Tick = 1,
    k_EControllerHapticType_Click = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputType {
    k_ESteamInputType_Unknown = 0,
    k_ESteamInputType_SteamController = 1,
    k_ESteamInputType_XBox360Controller = 2,
    k_ESteamInputType_XBoxOneController = 3,
    k_ESteamInputType_GenericGamepad = 4,
    k_ESteamInputType_PS4Controller = 5,
    k_ESteamInputType_AppleMFiController = 6,
    k_ESteamInputType_AndroidController = 7,
    k_ESteamInputType_SwitchJoyConPair = 8,
    k_ESteamInputType_SwitchJoyConSingle = 9,
    k_ESteamInputType_SwitchProController = 10,
    k_ESteamInputType_MobileTouch = 11,
    k_ESteamInputType_PS3Controller = 12,
    k_ESteamInputType_PS5Controller = 13,
    k_ESteamInputType_SteamDeckController = 14,
    k_ESteamInputType_Count = 15,
    k_ESteamInputType_MaximumPossibleValue = 255,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputConfigurationEnableType {
    k_ESteamInputConfigurationEnableType_None = 0,
    k_ESteamInputConfigurationEnableType_Playstation = 1,
    k_ESteamInputConfigurationEnableType_Xbox = 2,
    k_ESteamInputConfigurationEnableType_Generic = 4,
    k_ESteamInputConfigurationEnableType_Switch = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputLEDFlag {
    k_ESteamInputLEDFlag_SetColor = 0,
    k_ESteamInputLEDFlag_RestoreUserDefault = 1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputGlyphSize {
    k_ESteamInputGlyphSize_Small = 0,
    k_ESteamInputGlyphSize_Medium = 1,
    k_ESteamInputGlyphSize_Large = 2,
    k_ESteamInputGlyphSize_Count = 3,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputGlyphStyle {
    ESteamInputGlyphStyle_Knockout = 0,
    ESteamInputGlyphStyle_Light = 1,
    ESteamInputGlyphStyle_Dark = 2,
    ESteamInputGlyphStyle_NeutralColorABXY = 16,
    ESteamInputGlyphStyle_SolidABXY = 32,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamInputActionEventType {
    ESteamInputActionEventType_DigitalAction = 0,
    ESteamInputActionEventType_AnalogAction = 1,
}
pub type InputHandle_t = uint64;
pub type InputActionSetHandle_t = uint64;
pub type InputDigitalActionHandle_t = uint64;
pub type InputAnalogActionHandle_t = uint64;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InputAnalogActionData_t {
    pub eMode: EInputSourceMode,
    pub x: f32,
    pub y: f32,
    pub bActive: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of InputAnalogActionData_t",
    ][::std::mem::size_of::<InputAnalogActionData_t>() - 13usize];
    [
        "Alignment of InputAnalogActionData_t",
    ][::std::mem::align_of::<InputAnalogActionData_t>() - 1usize];
    [
        "Offset of field: InputAnalogActionData_t::eMode",
    ][::std::mem::offset_of!(InputAnalogActionData_t, eMode) - 0usize];
    [
        "Offset of field: InputAnalogActionData_t::x",
    ][::std::mem::offset_of!(InputAnalogActionData_t, x) - 4usize];
    [
        "Offset of field: InputAnalogActionData_t::y",
    ][::std::mem::offset_of!(InputAnalogActionData_t, y) - 8usize];
    [
        "Offset of field: InputAnalogActionData_t::bActive",
    ][::std::mem::offset_of!(InputAnalogActionData_t, bActive) - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputDigitalActionData_t {
    pub bState: bool,
    pub bActive: bool,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of InputDigitalActionData_t",
    ][::std::mem::size_of::<InputDigitalActionData_t>() - 2usize];
    [
        "Alignment of InputDigitalActionData_t",
    ][::std::mem::align_of::<InputDigitalActionData_t>() - 1usize];
    [
        "Offset of field: InputDigitalActionData_t::bState",
    ][::std::mem::offset_of!(InputDigitalActionData_t, bState) - 0usize];
    [
        "Offset of field: InputDigitalActionData_t::bActive",
    ][::std::mem::offset_of!(InputDigitalActionData_t, bActive) - 1usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct InputMotionData_t {
    pub rotQuatX: f32,
    pub rotQuatY: f32,
    pub rotQuatZ: f32,
    pub rotQuatW: f32,
    pub posAccelX: f32,
    pub posAccelY: f32,
    pub posAccelZ: f32,
    pub rotVelX: f32,
    pub rotVelY: f32,
    pub rotVelZ: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of InputMotionData_t"][::std::mem::size_of::<InputMotionData_t>() - 40usize];
    [
        "Alignment of InputMotionData_t",
    ][::std::mem::align_of::<InputMotionData_t>() - 1usize];
    [
        "Offset of field: InputMotionData_t::rotQuatX",
    ][::std::mem::offset_of!(InputMotionData_t, rotQuatX) - 0usize];
    [
        "Offset of field: InputMotionData_t::rotQuatY",
    ][::std::mem::offset_of!(InputMotionData_t, rotQuatY) - 4usize];
    [
        "Offset of field: InputMotionData_t::rotQuatZ",
    ][::std::mem::offset_of!(InputMotionData_t, rotQuatZ) - 8usize];
    [
        "Offset of field: InputMotionData_t::rotQuatW",
    ][::std::mem::offset_of!(InputMotionData_t, rotQuatW) - 12usize];
    [
        "Offset of field: InputMotionData_t::posAccelX",
    ][::std::mem::offset_of!(InputMotionData_t, posAccelX) - 16usize];
    [
        "Offset of field: InputMotionData_t::posAccelY",
    ][::std::mem::offset_of!(InputMotionData_t, posAccelY) - 20usize];
    [
        "Offset of field: InputMotionData_t::posAccelZ",
    ][::std::mem::offset_of!(InputMotionData_t, posAccelZ) - 24usize];
    [
        "Offset of field: InputMotionData_t::rotVelX",
    ][::std::mem::offset_of!(InputMotionData_t, rotVelX) - 28usize];
    [
        "Offset of field: InputMotionData_t::rotVelY",
    ][::std::mem::offset_of!(InputMotionData_t, rotVelY) - 32usize];
    [
        "Offset of field: InputMotionData_t::rotVelZ",
    ][::std::mem::offset_of!(InputMotionData_t, rotVelZ) - 36usize];
};
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SteamInputActionEvent_t {
    pub controllerHandle: InputHandle_t,
    pub eEventType: ESteamInputActionEventType,
    pub __bindgen_anon_1: SteamInputActionEvent_t__bindgen_ty_1,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInputActionEvent_t_AnalogAction_t {
    pub actionHandle: InputAnalogActionHandle_t,
    pub analogActionData: InputAnalogActionData_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputActionEvent_t_AnalogAction_t",
    ][::std::mem::size_of::<SteamInputActionEvent_t_AnalogAction_t>() - 21usize];
    [
        "Alignment of SteamInputActionEvent_t_AnalogAction_t",
    ][::std::mem::align_of::<SteamInputActionEvent_t_AnalogAction_t>() - 1usize];
    [
        "Offset of field: SteamInputActionEvent_t_AnalogAction_t::actionHandle",
    ][::std::mem::offset_of!(SteamInputActionEvent_t_AnalogAction_t, actionHandle)
        - 0usize];
    [
        "Offset of field: SteamInputActionEvent_t_AnalogAction_t::analogActionData",
    ][::std::mem::offset_of!(SteamInputActionEvent_t_AnalogAction_t, analogActionData)
        - 8usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInputActionEvent_t_DigitalAction_t {
    pub actionHandle: InputDigitalActionHandle_t,
    pub digitalActionData: InputDigitalActionData_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputActionEvent_t_DigitalAction_t",
    ][::std::mem::size_of::<SteamInputActionEvent_t_DigitalAction_t>() - 10usize];
    [
        "Alignment of SteamInputActionEvent_t_DigitalAction_t",
    ][::std::mem::align_of::<SteamInputActionEvent_t_DigitalAction_t>() - 1usize];
    [
        "Offset of field: SteamInputActionEvent_t_DigitalAction_t::actionHandle",
    ][::std::mem::offset_of!(SteamInputActionEvent_t_DigitalAction_t, actionHandle)
        - 0usize];
    [
        "Offset of field: SteamInputActionEvent_t_DigitalAction_t::digitalActionData",
    ][::std::mem::offset_of!(SteamInputActionEvent_t_DigitalAction_t, digitalActionData)
        - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union SteamInputActionEvent_t__bindgen_ty_1 {
    pub analogAction: SteamInputActionEvent_t_AnalogAction_t,
    pub digitalAction: SteamInputActionEvent_t_DigitalAction_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputActionEvent_t__bindgen_ty_1",
    ][::std::mem::size_of::<SteamInputActionEvent_t__bindgen_ty_1>() - 21usize];
    [
        "Alignment of SteamInputActionEvent_t__bindgen_ty_1",
    ][::std::mem::align_of::<SteamInputActionEvent_t__bindgen_ty_1>() - 1usize];
    [
        "Offset of field: SteamInputActionEvent_t__bindgen_ty_1::analogAction",
    ][::std::mem::offset_of!(SteamInputActionEvent_t__bindgen_ty_1, analogAction)
        - 0usize];
    [
        "Offset of field: SteamInputActionEvent_t__bindgen_ty_1::digitalAction",
    ][::std::mem::offset_of!(SteamInputActionEvent_t__bindgen_ty_1, digitalAction)
        - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputActionEvent_t",
    ][::std::mem::size_of::<SteamInputActionEvent_t>() - 33usize];
    [
        "Alignment of SteamInputActionEvent_t",
    ][::std::mem::align_of::<SteamInputActionEvent_t>() - 1usize];
    [
        "Offset of field: SteamInputActionEvent_t::controllerHandle",
    ][::std::mem::offset_of!(SteamInputActionEvent_t, controllerHandle) - 0usize];
    [
        "Offset of field: SteamInputActionEvent_t::eEventType",
    ][::std::mem::offset_of!(SteamInputActionEvent_t, eEventType) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ScePadTriggerEffectParam {
    _unused: [u8; 0],
}
pub type SteamInputActionEventCallbackPointer = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamInputActionEvent_t),
>;
#[repr(C)]
pub struct ISteamInput__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamInput {
    pub vtable_: *const ISteamInput__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamInput"][::std::mem::size_of::<ISteamInput>() - 8usize];
    ["Alignment of ISteamInput"][::std::mem::align_of::<ISteamInput>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamInputDeviceConnected_t {
    pub m_ulConnectedDeviceHandle: InputHandle_t,
}
pub const SteamInputDeviceConnected_t_k_iCallback: SteamInputDeviceConnected_t__bindgen_ty_1 = SteamInputDeviceConnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInputDeviceConnected_t__bindgen_ty_1 {
    k_iCallback = 2801,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputDeviceConnected_t",
    ][::std::mem::size_of::<SteamInputDeviceConnected_t>() - 8usize];
    [
        "Alignment of SteamInputDeviceConnected_t",
    ][::std::mem::align_of::<SteamInputDeviceConnected_t>() - 4usize];
    [
        "Offset of field: SteamInputDeviceConnected_t::m_ulConnectedDeviceHandle",
    ][::std::mem::offset_of!(SteamInputDeviceConnected_t, m_ulConnectedDeviceHandle)
        - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamInputDeviceDisconnected_t {
    pub m_ulDisconnectedDeviceHandle: InputHandle_t,
}
pub const SteamInputDeviceDisconnected_t_k_iCallback: SteamInputDeviceDisconnected_t__bindgen_ty_1 = SteamInputDeviceDisconnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInputDeviceDisconnected_t__bindgen_ty_1 {
    k_iCallback = 2802,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputDeviceDisconnected_t",
    ][::std::mem::size_of::<SteamInputDeviceDisconnected_t>() - 8usize];
    [
        "Alignment of SteamInputDeviceDisconnected_t",
    ][::std::mem::align_of::<SteamInputDeviceDisconnected_t>() - 4usize];
    [
        "Offset of field: SteamInputDeviceDisconnected_t::m_ulDisconnectedDeviceHandle",
    ][::std::mem::offset_of!(
        SteamInputDeviceDisconnected_t, m_ulDisconnectedDeviceHandle
    ) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct SteamInputConfigurationLoaded_t {
    pub m_unAppID: AppId_t,
    pub m_ulDeviceHandle: InputHandle_t,
    pub m_ulMappingCreator: CSteamID,
    pub m_unMajorRevision: uint32,
    pub m_unMinorRevision: uint32,
    pub m_bUsesSteamInputAPI: bool,
    pub m_bUsesGamepadAPI: bool,
}
pub const SteamInputConfigurationLoaded_t_k_iCallback: SteamInputConfigurationLoaded_t__bindgen_ty_1 = SteamInputConfigurationLoaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInputConfigurationLoaded_t__bindgen_ty_1 {
    k_iCallback = 2803,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputConfigurationLoaded_t",
    ][::std::mem::size_of::<SteamInputConfigurationLoaded_t>() - 32usize];
    [
        "Alignment of SteamInputConfigurationLoaded_t",
    ][::std::mem::align_of::<SteamInputConfigurationLoaded_t>() - 4usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_unAppID",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_unAppID) - 0usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_ulDeviceHandle",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_ulDeviceHandle)
        - 4usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_ulMappingCreator",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_ulMappingCreator)
        - 12usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_unMajorRevision",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_unMajorRevision)
        - 20usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_unMinorRevision",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_unMinorRevision)
        - 24usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_bUsesSteamInputAPI",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_bUsesSteamInputAPI)
        - 28usize];
    [
        "Offset of field: SteamInputConfigurationLoaded_t::m_bUsesGamepadAPI",
    ][::std::mem::offset_of!(SteamInputConfigurationLoaded_t, m_bUsesGamepadAPI)
        - 29usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamInputGamepadSlotChange_t {
    pub m_unAppID: AppId_t,
    pub m_ulDeviceHandle: InputHandle_t,
    pub m_eDeviceType: ESteamInputType,
    pub m_nOldGamepadSlot: ::std::os::raw::c_int,
    pub m_nNewGamepadSlot: ::std::os::raw::c_int,
}
pub const SteamInputGamepadSlotChange_t_k_iCallback: SteamInputGamepadSlotChange_t__bindgen_ty_1 = SteamInputGamepadSlotChange_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInputGamepadSlotChange_t__bindgen_ty_1 {
    k_iCallback = 2804,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInputGamepadSlotChange_t",
    ][::std::mem::size_of::<SteamInputGamepadSlotChange_t>() - 24usize];
    [
        "Alignment of SteamInputGamepadSlotChange_t",
    ][::std::mem::align_of::<SteamInputGamepadSlotChange_t>() - 4usize];
    [
        "Offset of field: SteamInputGamepadSlotChange_t::m_unAppID",
    ][::std::mem::offset_of!(SteamInputGamepadSlotChange_t, m_unAppID) - 0usize];
    [
        "Offset of field: SteamInputGamepadSlotChange_t::m_ulDeviceHandle",
    ][::std::mem::offset_of!(SteamInputGamepadSlotChange_t, m_ulDeviceHandle) - 4usize];
    [
        "Offset of field: SteamInputGamepadSlotChange_t::m_eDeviceType",
    ][::std::mem::offset_of!(SteamInputGamepadSlotChange_t, m_eDeviceType) - 12usize];
    [
        "Offset of field: SteamInputGamepadSlotChange_t::m_nOldGamepadSlot",
    ][::std::mem::offset_of!(SteamInputGamepadSlotChange_t, m_nOldGamepadSlot)
        - 16usize];
    [
        "Offset of field: SteamInputGamepadSlotChange_t::m_nNewGamepadSlot",
    ][::std::mem::offset_of!(SteamInputGamepadSlotChange_t, m_nNewGamepadSlot)
        - 20usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EControllerActionOrigin {
    k_EControllerActionOrigin_None = 0,
    k_EControllerActionOrigin_A = 1,
    k_EControllerActionOrigin_B = 2,
    k_EControllerActionOrigin_X = 3,
    k_EControllerActionOrigin_Y = 4,
    k_EControllerActionOrigin_LeftBumper = 5,
    k_EControllerActionOrigin_RightBumper = 6,
    k_EControllerActionOrigin_LeftGrip = 7,
    k_EControllerActionOrigin_RightGrip = 8,
    k_EControllerActionOrigin_Start = 9,
    k_EControllerActionOrigin_Back = 10,
    k_EControllerActionOrigin_LeftPad_Touch = 11,
    k_EControllerActionOrigin_LeftPad_Swipe = 12,
    k_EControllerActionOrigin_LeftPad_Click = 13,
    k_EControllerActionOrigin_LeftPad_DPadNorth = 14,
    k_EControllerActionOrigin_LeftPad_DPadSouth = 15,
    k_EControllerActionOrigin_LeftPad_DPadWest = 16,
    k_EControllerActionOrigin_LeftPad_DPadEast = 17,
    k_EControllerActionOrigin_RightPad_Touch = 18,
    k_EControllerActionOrigin_RightPad_Swipe = 19,
    k_EControllerActionOrigin_RightPad_Click = 20,
    k_EControllerActionOrigin_RightPad_DPadNorth = 21,
    k_EControllerActionOrigin_RightPad_DPadSouth = 22,
    k_EControllerActionOrigin_RightPad_DPadWest = 23,
    k_EControllerActionOrigin_RightPad_DPadEast = 24,
    k_EControllerActionOrigin_LeftTrigger_Pull = 25,
    k_EControllerActionOrigin_LeftTrigger_Click = 26,
    k_EControllerActionOrigin_RightTrigger_Pull = 27,
    k_EControllerActionOrigin_RightTrigger_Click = 28,
    k_EControllerActionOrigin_LeftStick_Move = 29,
    k_EControllerActionOrigin_LeftStick_Click = 30,
    k_EControllerActionOrigin_LeftStick_DPadNorth = 31,
    k_EControllerActionOrigin_LeftStick_DPadSouth = 32,
    k_EControllerActionOrigin_LeftStick_DPadWest = 33,
    k_EControllerActionOrigin_LeftStick_DPadEast = 34,
    k_EControllerActionOrigin_Gyro_Move = 35,
    k_EControllerActionOrigin_Gyro_Pitch = 36,
    k_EControllerActionOrigin_Gyro_Yaw = 37,
    k_EControllerActionOrigin_Gyro_Roll = 38,
    k_EControllerActionOrigin_PS4_X = 39,
    k_EControllerActionOrigin_PS4_Circle = 40,
    k_EControllerActionOrigin_PS4_Triangle = 41,
    k_EControllerActionOrigin_PS4_Square = 42,
    k_EControllerActionOrigin_PS4_LeftBumper = 43,
    k_EControllerActionOrigin_PS4_RightBumper = 44,
    k_EControllerActionOrigin_PS4_Options = 45,
    k_EControllerActionOrigin_PS4_Share = 46,
    k_EControllerActionOrigin_PS4_LeftPad_Touch = 47,
    k_EControllerActionOrigin_PS4_LeftPad_Swipe = 48,
    k_EControllerActionOrigin_PS4_LeftPad_Click = 49,
    k_EControllerActionOrigin_PS4_LeftPad_DPadNorth = 50,
    k_EControllerActionOrigin_PS4_LeftPad_DPadSouth = 51,
    k_EControllerActionOrigin_PS4_LeftPad_DPadWest = 52,
    k_EControllerActionOrigin_PS4_LeftPad_DPadEast = 53,
    k_EControllerActionOrigin_PS4_RightPad_Touch = 54,
    k_EControllerActionOrigin_PS4_RightPad_Swipe = 55,
    k_EControllerActionOrigin_PS4_RightPad_Click = 56,
    k_EControllerActionOrigin_PS4_RightPad_DPadNorth = 57,
    k_EControllerActionOrigin_PS4_RightPad_DPadSouth = 58,
    k_EControllerActionOrigin_PS4_RightPad_DPadWest = 59,
    k_EControllerActionOrigin_PS4_RightPad_DPadEast = 60,
    k_EControllerActionOrigin_PS4_CenterPad_Touch = 61,
    k_EControllerActionOrigin_PS4_CenterPad_Swipe = 62,
    k_EControllerActionOrigin_PS4_CenterPad_Click = 63,
    k_EControllerActionOrigin_PS4_CenterPad_DPadNorth = 64,
    k_EControllerActionOrigin_PS4_CenterPad_DPadSouth = 65,
    k_EControllerActionOrigin_PS4_CenterPad_DPadWest = 66,
    k_EControllerActionOrigin_PS4_CenterPad_DPadEast = 67,
    k_EControllerActionOrigin_PS4_LeftTrigger_Pull = 68,
    k_EControllerActionOrigin_PS4_LeftTrigger_Click = 69,
    k_EControllerActionOrigin_PS4_RightTrigger_Pull = 70,
    k_EControllerActionOrigin_PS4_RightTrigger_Click = 71,
    k_EControllerActionOrigin_PS4_LeftStick_Move = 72,
    k_EControllerActionOrigin_PS4_LeftStick_Click = 73,
    k_EControllerActionOrigin_PS4_LeftStick_DPadNorth = 74,
    k_EControllerActionOrigin_PS4_LeftStick_DPadSouth = 75,
    k_EControllerActionOrigin_PS4_LeftStick_DPadWest = 76,
    k_EControllerActionOrigin_PS4_LeftStick_DPadEast = 77,
    k_EControllerActionOrigin_PS4_RightStick_Move = 78,
    k_EControllerActionOrigin_PS4_RightStick_Click = 79,
    k_EControllerActionOrigin_PS4_RightStick_DPadNorth = 80,
    k_EControllerActionOrigin_PS4_RightStick_DPadSouth = 81,
    k_EControllerActionOrigin_PS4_RightStick_DPadWest = 82,
    k_EControllerActionOrigin_PS4_RightStick_DPadEast = 83,
    k_EControllerActionOrigin_PS4_DPad_North = 84,
    k_EControllerActionOrigin_PS4_DPad_South = 85,
    k_EControllerActionOrigin_PS4_DPad_West = 86,
    k_EControllerActionOrigin_PS4_DPad_East = 87,
    k_EControllerActionOrigin_PS4_Gyro_Move = 88,
    k_EControllerActionOrigin_PS4_Gyro_Pitch = 89,
    k_EControllerActionOrigin_PS4_Gyro_Yaw = 90,
    k_EControllerActionOrigin_PS4_Gyro_Roll = 91,
    k_EControllerActionOrigin_XBoxOne_A = 92,
    k_EControllerActionOrigin_XBoxOne_B = 93,
    k_EControllerActionOrigin_XBoxOne_X = 94,
    k_EControllerActionOrigin_XBoxOne_Y = 95,
    k_EControllerActionOrigin_XBoxOne_LeftBumper = 96,
    k_EControllerActionOrigin_XBoxOne_RightBumper = 97,
    k_EControllerActionOrigin_XBoxOne_Menu = 98,
    k_EControllerActionOrigin_XBoxOne_View = 99,
    k_EControllerActionOrigin_XBoxOne_LeftTrigger_Pull = 100,
    k_EControllerActionOrigin_XBoxOne_LeftTrigger_Click = 101,
    k_EControllerActionOrigin_XBoxOne_RightTrigger_Pull = 102,
    k_EControllerActionOrigin_XBoxOne_RightTrigger_Click = 103,
    k_EControllerActionOrigin_XBoxOne_LeftStick_Move = 104,
    k_EControllerActionOrigin_XBoxOne_LeftStick_Click = 105,
    k_EControllerActionOrigin_XBoxOne_LeftStick_DPadNorth = 106,
    k_EControllerActionOrigin_XBoxOne_LeftStick_DPadSouth = 107,
    k_EControllerActionOrigin_XBoxOne_LeftStick_DPadWest = 108,
    k_EControllerActionOrigin_XBoxOne_LeftStick_DPadEast = 109,
    k_EControllerActionOrigin_XBoxOne_RightStick_Move = 110,
    k_EControllerActionOrigin_XBoxOne_RightStick_Click = 111,
    k_EControllerActionOrigin_XBoxOne_RightStick_DPadNorth = 112,
    k_EControllerActionOrigin_XBoxOne_RightStick_DPadSouth = 113,
    k_EControllerActionOrigin_XBoxOne_RightStick_DPadWest = 114,
    k_EControllerActionOrigin_XBoxOne_RightStick_DPadEast = 115,
    k_EControllerActionOrigin_XBoxOne_DPad_North = 116,
    k_EControllerActionOrigin_XBoxOne_DPad_South = 117,
    k_EControllerActionOrigin_XBoxOne_DPad_West = 118,
    k_EControllerActionOrigin_XBoxOne_DPad_East = 119,
    k_EControllerActionOrigin_XBox360_A = 120,
    k_EControllerActionOrigin_XBox360_B = 121,
    k_EControllerActionOrigin_XBox360_X = 122,
    k_EControllerActionOrigin_XBox360_Y = 123,
    k_EControllerActionOrigin_XBox360_LeftBumper = 124,
    k_EControllerActionOrigin_XBox360_RightBumper = 125,
    k_EControllerActionOrigin_XBox360_Start = 126,
    k_EControllerActionOrigin_XBox360_Back = 127,
    k_EControllerActionOrigin_XBox360_LeftTrigger_Pull = 128,
    k_EControllerActionOrigin_XBox360_LeftTrigger_Click = 129,
    k_EControllerActionOrigin_XBox360_RightTrigger_Pull = 130,
    k_EControllerActionOrigin_XBox360_RightTrigger_Click = 131,
    k_EControllerActionOrigin_XBox360_LeftStick_Move = 132,
    k_EControllerActionOrigin_XBox360_LeftStick_Click = 133,
    k_EControllerActionOrigin_XBox360_LeftStick_DPadNorth = 134,
    k_EControllerActionOrigin_XBox360_LeftStick_DPadSouth = 135,
    k_EControllerActionOrigin_XBox360_LeftStick_DPadWest = 136,
    k_EControllerActionOrigin_XBox360_LeftStick_DPadEast = 137,
    k_EControllerActionOrigin_XBox360_RightStick_Move = 138,
    k_EControllerActionOrigin_XBox360_RightStick_Click = 139,
    k_EControllerActionOrigin_XBox360_RightStick_DPadNorth = 140,
    k_EControllerActionOrigin_XBox360_RightStick_DPadSouth = 141,
    k_EControllerActionOrigin_XBox360_RightStick_DPadWest = 142,
    k_EControllerActionOrigin_XBox360_RightStick_DPadEast = 143,
    k_EControllerActionOrigin_XBox360_DPad_North = 144,
    k_EControllerActionOrigin_XBox360_DPad_South = 145,
    k_EControllerActionOrigin_XBox360_DPad_West = 146,
    k_EControllerActionOrigin_XBox360_DPad_East = 147,
    k_EControllerActionOrigin_SteamV2_A = 148,
    k_EControllerActionOrigin_SteamV2_B = 149,
    k_EControllerActionOrigin_SteamV2_X = 150,
    k_EControllerActionOrigin_SteamV2_Y = 151,
    k_EControllerActionOrigin_SteamV2_LeftBumper = 152,
    k_EControllerActionOrigin_SteamV2_RightBumper = 153,
    k_EControllerActionOrigin_SteamV2_LeftGrip_Lower = 154,
    k_EControllerActionOrigin_SteamV2_LeftGrip_Upper = 155,
    k_EControllerActionOrigin_SteamV2_RightGrip_Lower = 156,
    k_EControllerActionOrigin_SteamV2_RightGrip_Upper = 157,
    k_EControllerActionOrigin_SteamV2_LeftBumper_Pressure = 158,
    k_EControllerActionOrigin_SteamV2_RightBumper_Pressure = 159,
    k_EControllerActionOrigin_SteamV2_LeftGrip_Pressure = 160,
    k_EControllerActionOrigin_SteamV2_RightGrip_Pressure = 161,
    k_EControllerActionOrigin_SteamV2_LeftGrip_Upper_Pressure = 162,
    k_EControllerActionOrigin_SteamV2_RightGrip_Upper_Pressure = 163,
    k_EControllerActionOrigin_SteamV2_Start = 164,
    k_EControllerActionOrigin_SteamV2_Back = 165,
    k_EControllerActionOrigin_SteamV2_LeftPad_Touch = 166,
    k_EControllerActionOrigin_SteamV2_LeftPad_Swipe = 167,
    k_EControllerActionOrigin_SteamV2_LeftPad_Click = 168,
    k_EControllerActionOrigin_SteamV2_LeftPad_Pressure = 169,
    k_EControllerActionOrigin_SteamV2_LeftPad_DPadNorth = 170,
    k_EControllerActionOrigin_SteamV2_LeftPad_DPadSouth = 171,
    k_EControllerActionOrigin_SteamV2_LeftPad_DPadWest = 172,
    k_EControllerActionOrigin_SteamV2_LeftPad_DPadEast = 173,
    k_EControllerActionOrigin_SteamV2_RightPad_Touch = 174,
    k_EControllerActionOrigin_SteamV2_RightPad_Swipe = 175,
    k_EControllerActionOrigin_SteamV2_RightPad_Click = 176,
    k_EControllerActionOrigin_SteamV2_RightPad_Pressure = 177,
    k_EControllerActionOrigin_SteamV2_RightPad_DPadNorth = 178,
    k_EControllerActionOrigin_SteamV2_RightPad_DPadSouth = 179,
    k_EControllerActionOrigin_SteamV2_RightPad_DPadWest = 180,
    k_EControllerActionOrigin_SteamV2_RightPad_DPadEast = 181,
    k_EControllerActionOrigin_SteamV2_LeftTrigger_Pull = 182,
    k_EControllerActionOrigin_SteamV2_LeftTrigger_Click = 183,
    k_EControllerActionOrigin_SteamV2_RightTrigger_Pull = 184,
    k_EControllerActionOrigin_SteamV2_RightTrigger_Click = 185,
    k_EControllerActionOrigin_SteamV2_LeftStick_Move = 186,
    k_EControllerActionOrigin_SteamV2_LeftStick_Click = 187,
    k_EControllerActionOrigin_SteamV2_LeftStick_DPadNorth = 188,
    k_EControllerActionOrigin_SteamV2_LeftStick_DPadSouth = 189,
    k_EControllerActionOrigin_SteamV2_LeftStick_DPadWest = 190,
    k_EControllerActionOrigin_SteamV2_LeftStick_DPadEast = 191,
    k_EControllerActionOrigin_SteamV2_Gyro_Move = 192,
    k_EControllerActionOrigin_SteamV2_Gyro_Pitch = 193,
    k_EControllerActionOrigin_SteamV2_Gyro_Yaw = 194,
    k_EControllerActionOrigin_SteamV2_Gyro_Roll = 195,
    k_EControllerActionOrigin_Switch_A = 196,
    k_EControllerActionOrigin_Switch_B = 197,
    k_EControllerActionOrigin_Switch_X = 198,
    k_EControllerActionOrigin_Switch_Y = 199,
    k_EControllerActionOrigin_Switch_LeftBumper = 200,
    k_EControllerActionOrigin_Switch_RightBumper = 201,
    k_EControllerActionOrigin_Switch_Plus = 202,
    k_EControllerActionOrigin_Switch_Minus = 203,
    k_EControllerActionOrigin_Switch_Capture = 204,
    k_EControllerActionOrigin_Switch_LeftTrigger_Pull = 205,
    k_EControllerActionOrigin_Switch_LeftTrigger_Click = 206,
    k_EControllerActionOrigin_Switch_RightTrigger_Pull = 207,
    k_EControllerActionOrigin_Switch_RightTrigger_Click = 208,
    k_EControllerActionOrigin_Switch_LeftStick_Move = 209,
    k_EControllerActionOrigin_Switch_LeftStick_Click = 210,
    k_EControllerActionOrigin_Switch_LeftStick_DPadNorth = 211,
    k_EControllerActionOrigin_Switch_LeftStick_DPadSouth = 212,
    k_EControllerActionOrigin_Switch_LeftStick_DPadWest = 213,
    k_EControllerActionOrigin_Switch_LeftStick_DPadEast = 214,
    k_EControllerActionOrigin_Switch_RightStick_Move = 215,
    k_EControllerActionOrigin_Switch_RightStick_Click = 216,
    k_EControllerActionOrigin_Switch_RightStick_DPadNorth = 217,
    k_EControllerActionOrigin_Switch_RightStick_DPadSouth = 218,
    k_EControllerActionOrigin_Switch_RightStick_DPadWest = 219,
    k_EControllerActionOrigin_Switch_RightStick_DPadEast = 220,
    k_EControllerActionOrigin_Switch_DPad_North = 221,
    k_EControllerActionOrigin_Switch_DPad_South = 222,
    k_EControllerActionOrigin_Switch_DPad_West = 223,
    k_EControllerActionOrigin_Switch_DPad_East = 224,
    k_EControllerActionOrigin_Switch_ProGyro_Move = 225,
    k_EControllerActionOrigin_Switch_ProGyro_Pitch = 226,
    k_EControllerActionOrigin_Switch_ProGyro_Yaw = 227,
    k_EControllerActionOrigin_Switch_ProGyro_Roll = 228,
    k_EControllerActionOrigin_Switch_RightGyro_Move = 229,
    k_EControllerActionOrigin_Switch_RightGyro_Pitch = 230,
    k_EControllerActionOrigin_Switch_RightGyro_Yaw = 231,
    k_EControllerActionOrigin_Switch_RightGyro_Roll = 232,
    k_EControllerActionOrigin_Switch_LeftGyro_Move = 233,
    k_EControllerActionOrigin_Switch_LeftGyro_Pitch = 234,
    k_EControllerActionOrigin_Switch_LeftGyro_Yaw = 235,
    k_EControllerActionOrigin_Switch_LeftGyro_Roll = 236,
    k_EControllerActionOrigin_Switch_LeftGrip_Lower = 237,
    k_EControllerActionOrigin_Switch_LeftGrip_Upper = 238,
    k_EControllerActionOrigin_Switch_RightGrip_Lower = 239,
    k_EControllerActionOrigin_Switch_RightGrip_Upper = 240,
    k_EControllerActionOrigin_PS4_DPad_Move = 241,
    k_EControllerActionOrigin_XBoxOne_DPad_Move = 242,
    k_EControllerActionOrigin_XBox360_DPad_Move = 243,
    k_EControllerActionOrigin_Switch_DPad_Move = 244,
    k_EControllerActionOrigin_PS5_X = 245,
    k_EControllerActionOrigin_PS5_Circle = 246,
    k_EControllerActionOrigin_PS5_Triangle = 247,
    k_EControllerActionOrigin_PS5_Square = 248,
    k_EControllerActionOrigin_PS5_LeftBumper = 249,
    k_EControllerActionOrigin_PS5_RightBumper = 250,
    k_EControllerActionOrigin_PS5_Option = 251,
    k_EControllerActionOrigin_PS5_Create = 252,
    k_EControllerActionOrigin_PS5_Mute = 253,
    k_EControllerActionOrigin_PS5_LeftPad_Touch = 254,
    k_EControllerActionOrigin_PS5_LeftPad_Swipe = 255,
    k_EControllerActionOrigin_PS5_LeftPad_Click = 256,
    k_EControllerActionOrigin_PS5_LeftPad_DPadNorth = 257,
    k_EControllerActionOrigin_PS5_LeftPad_DPadSouth = 258,
    k_EControllerActionOrigin_PS5_LeftPad_DPadWest = 259,
    k_EControllerActionOrigin_PS5_LeftPad_DPadEast = 260,
    k_EControllerActionOrigin_PS5_RightPad_Touch = 261,
    k_EControllerActionOrigin_PS5_RightPad_Swipe = 262,
    k_EControllerActionOrigin_PS5_RightPad_Click = 263,
    k_EControllerActionOrigin_PS5_RightPad_DPadNorth = 264,
    k_EControllerActionOrigin_PS5_RightPad_DPadSouth = 265,
    k_EControllerActionOrigin_PS5_RightPad_DPadWest = 266,
    k_EControllerActionOrigin_PS5_RightPad_DPadEast = 267,
    k_EControllerActionOrigin_PS5_CenterPad_Touch = 268,
    k_EControllerActionOrigin_PS5_CenterPad_Swipe = 269,
    k_EControllerActionOrigin_PS5_CenterPad_Click = 270,
    k_EControllerActionOrigin_PS5_CenterPad_DPadNorth = 271,
    k_EControllerActionOrigin_PS5_CenterPad_DPadSouth = 272,
    k_EControllerActionOrigin_PS5_CenterPad_DPadWest = 273,
    k_EControllerActionOrigin_PS5_CenterPad_DPadEast = 274,
    k_EControllerActionOrigin_PS5_LeftTrigger_Pull = 275,
    k_EControllerActionOrigin_PS5_LeftTrigger_Click = 276,
    k_EControllerActionOrigin_PS5_RightTrigger_Pull = 277,
    k_EControllerActionOrigin_PS5_RightTrigger_Click = 278,
    k_EControllerActionOrigin_PS5_LeftStick_Move = 279,
    k_EControllerActionOrigin_PS5_LeftStick_Click = 280,
    k_EControllerActionOrigin_PS5_LeftStick_DPadNorth = 281,
    k_EControllerActionOrigin_PS5_LeftStick_DPadSouth = 282,
    k_EControllerActionOrigin_PS5_LeftStick_DPadWest = 283,
    k_EControllerActionOrigin_PS5_LeftStick_DPadEast = 284,
    k_EControllerActionOrigin_PS5_RightStick_Move = 285,
    k_EControllerActionOrigin_PS5_RightStick_Click = 286,
    k_EControllerActionOrigin_PS5_RightStick_DPadNorth = 287,
    k_EControllerActionOrigin_PS5_RightStick_DPadSouth = 288,
    k_EControllerActionOrigin_PS5_RightStick_DPadWest = 289,
    k_EControllerActionOrigin_PS5_RightStick_DPadEast = 290,
    k_EControllerActionOrigin_PS5_DPad_Move = 291,
    k_EControllerActionOrigin_PS5_DPad_North = 292,
    k_EControllerActionOrigin_PS5_DPad_South = 293,
    k_EControllerActionOrigin_PS5_DPad_West = 294,
    k_EControllerActionOrigin_PS5_DPad_East = 295,
    k_EControllerActionOrigin_PS5_Gyro_Move = 296,
    k_EControllerActionOrigin_PS5_Gyro_Pitch = 297,
    k_EControllerActionOrigin_PS5_Gyro_Yaw = 298,
    k_EControllerActionOrigin_PS5_Gyro_Roll = 299,
    k_EControllerActionOrigin_XBoxOne_LeftGrip_Lower = 300,
    k_EControllerActionOrigin_XBoxOne_LeftGrip_Upper = 301,
    k_EControllerActionOrigin_XBoxOne_RightGrip_Lower = 302,
    k_EControllerActionOrigin_XBoxOne_RightGrip_Upper = 303,
    k_EControllerActionOrigin_XBoxOne_Share = 304,
    k_EControllerActionOrigin_SteamDeck_A = 305,
    k_EControllerActionOrigin_SteamDeck_B = 306,
    k_EControllerActionOrigin_SteamDeck_X = 307,
    k_EControllerActionOrigin_SteamDeck_Y = 308,
    k_EControllerActionOrigin_SteamDeck_L1 = 309,
    k_EControllerActionOrigin_SteamDeck_R1 = 310,
    k_EControllerActionOrigin_SteamDeck_Menu = 311,
    k_EControllerActionOrigin_SteamDeck_View = 312,
    k_EControllerActionOrigin_SteamDeck_LeftPad_Touch = 313,
    k_EControllerActionOrigin_SteamDeck_LeftPad_Swipe = 314,
    k_EControllerActionOrigin_SteamDeck_LeftPad_Click = 315,
    k_EControllerActionOrigin_SteamDeck_LeftPad_DPadNorth = 316,
    k_EControllerActionOrigin_SteamDeck_LeftPad_DPadSouth = 317,
    k_EControllerActionOrigin_SteamDeck_LeftPad_DPadWest = 318,
    k_EControllerActionOrigin_SteamDeck_LeftPad_DPadEast = 319,
    k_EControllerActionOrigin_SteamDeck_RightPad_Touch = 320,
    k_EControllerActionOrigin_SteamDeck_RightPad_Swipe = 321,
    k_EControllerActionOrigin_SteamDeck_RightPad_Click = 322,
    k_EControllerActionOrigin_SteamDeck_RightPad_DPadNorth = 323,
    k_EControllerActionOrigin_SteamDeck_RightPad_DPadSouth = 324,
    k_EControllerActionOrigin_SteamDeck_RightPad_DPadWest = 325,
    k_EControllerActionOrigin_SteamDeck_RightPad_DPadEast = 326,
    k_EControllerActionOrigin_SteamDeck_L2_SoftPull = 327,
    k_EControllerActionOrigin_SteamDeck_L2 = 328,
    k_EControllerActionOrigin_SteamDeck_R2_SoftPull = 329,
    k_EControllerActionOrigin_SteamDeck_R2 = 330,
    k_EControllerActionOrigin_SteamDeck_LeftStick_Move = 331,
    k_EControllerActionOrigin_SteamDeck_L3 = 332,
    k_EControllerActionOrigin_SteamDeck_LeftStick_DPadNorth = 333,
    k_EControllerActionOrigin_SteamDeck_LeftStick_DPadSouth = 334,
    k_EControllerActionOrigin_SteamDeck_LeftStick_DPadWest = 335,
    k_EControllerActionOrigin_SteamDeck_LeftStick_DPadEast = 336,
    k_EControllerActionOrigin_SteamDeck_LeftStick_Touch = 337,
    k_EControllerActionOrigin_SteamDeck_RightStick_Move = 338,
    k_EControllerActionOrigin_SteamDeck_R3 = 339,
    k_EControllerActionOrigin_SteamDeck_RightStick_DPadNorth = 340,
    k_EControllerActionOrigin_SteamDeck_RightStick_DPadSouth = 341,
    k_EControllerActionOrigin_SteamDeck_RightStick_DPadWest = 342,
    k_EControllerActionOrigin_SteamDeck_RightStick_DPadEast = 343,
    k_EControllerActionOrigin_SteamDeck_RightStick_Touch = 344,
    k_EControllerActionOrigin_SteamDeck_L4 = 345,
    k_EControllerActionOrigin_SteamDeck_R4 = 346,
    k_EControllerActionOrigin_SteamDeck_L5 = 347,
    k_EControllerActionOrigin_SteamDeck_R5 = 348,
    k_EControllerActionOrigin_SteamDeck_DPad_Move = 349,
    k_EControllerActionOrigin_SteamDeck_DPad_North = 350,
    k_EControllerActionOrigin_SteamDeck_DPad_South = 351,
    k_EControllerActionOrigin_SteamDeck_DPad_West = 352,
    k_EControllerActionOrigin_SteamDeck_DPad_East = 353,
    k_EControllerActionOrigin_SteamDeck_Gyro_Move = 354,
    k_EControllerActionOrigin_SteamDeck_Gyro_Pitch = 355,
    k_EControllerActionOrigin_SteamDeck_Gyro_Yaw = 356,
    k_EControllerActionOrigin_SteamDeck_Gyro_Roll = 357,
    k_EControllerActionOrigin_SteamDeck_Reserved1 = 358,
    k_EControllerActionOrigin_SteamDeck_Reserved2 = 359,
    k_EControllerActionOrigin_SteamDeck_Reserved3 = 360,
    k_EControllerActionOrigin_SteamDeck_Reserved4 = 361,
    k_EControllerActionOrigin_SteamDeck_Reserved5 = 362,
    k_EControllerActionOrigin_SteamDeck_Reserved6 = 363,
    k_EControllerActionOrigin_SteamDeck_Reserved7 = 364,
    k_EControllerActionOrigin_SteamDeck_Reserved8 = 365,
    k_EControllerActionOrigin_SteamDeck_Reserved9 = 366,
    k_EControllerActionOrigin_SteamDeck_Reserved10 = 367,
    k_EControllerActionOrigin_SteamDeck_Reserved11 = 368,
    k_EControllerActionOrigin_SteamDeck_Reserved12 = 369,
    k_EControllerActionOrigin_SteamDeck_Reserved13 = 370,
    k_EControllerActionOrigin_SteamDeck_Reserved14 = 371,
    k_EControllerActionOrigin_SteamDeck_Reserved15 = 372,
    k_EControllerActionOrigin_SteamDeck_Reserved16 = 373,
    k_EControllerActionOrigin_SteamDeck_Reserved17 = 374,
    k_EControllerActionOrigin_SteamDeck_Reserved18 = 375,
    k_EControllerActionOrigin_SteamDeck_Reserved19 = 376,
    k_EControllerActionOrigin_SteamDeck_Reserved20 = 377,
    k_EControllerActionOrigin_Switch_JoyConButton_N = 378,
    k_EControllerActionOrigin_Switch_JoyConButton_E = 379,
    k_EControllerActionOrigin_Switch_JoyConButton_S = 380,
    k_EControllerActionOrigin_Switch_JoyConButton_W = 381,
    k_EControllerActionOrigin_PS5_LeftGrip = 382,
    k_EControllerActionOrigin_PS5_RightGrip = 383,
    k_EControllerActionOrigin_PS5_LeftFn = 384,
    k_EControllerActionOrigin_PS5_RightFn = 385,
    k_EControllerActionOrigin_Horipad_M1 = 386,
    k_EControllerActionOrigin_Horipad_M2 = 387,
    k_EControllerActionOrigin_Horipad_L4 = 388,
    k_EControllerActionOrigin_Horipad_R4 = 389,
    k_EControllerActionOrigin_Count = 390,
    k_EControllerActionOrigin_MaximumPossibleValue = 32767,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamControllerLEDFlag {
    k_ESteamControllerLEDFlag_SetColor = 0,
    k_ESteamControllerLEDFlag_RestoreUserDefault = 1,
}
pub type ControllerHandle_t = uint64;
pub type ControllerActionSetHandle_t = uint64;
pub type ControllerDigitalActionHandle_t = uint64;
pub type ControllerAnalogActionHandle_t = uint64;
#[repr(C)]
pub struct ISteamController__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamController {
    pub vtable_: *const ISteamController__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamController"][::std::mem::size_of::<ISteamController>() - 8usize];
    [
        "Alignment of ISteamController",
    ][::std::mem::align_of::<ISteamController>() - 8usize];
};
pub type UGCQueryHandle_t = uint64;
pub type UGCUpdateHandle_t = uint64;
pub const k_UGCQueryHandleInvalid: UGCQueryHandle_t = 18446744073709551615;
pub const k_UGCUpdateHandleInvalid: UGCUpdateHandle_t = 18446744073709551615;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUGCMatchingUGCType {
    k_EUGCMatchingUGCType_Items = 0,
    k_EUGCMatchingUGCType_Items_Mtx = 1,
    k_EUGCMatchingUGCType_Items_ReadyToUse = 2,
    k_EUGCMatchingUGCType_Collections = 3,
    k_EUGCMatchingUGCType_Artwork = 4,
    k_EUGCMatchingUGCType_Videos = 5,
    k_EUGCMatchingUGCType_Screenshots = 6,
    k_EUGCMatchingUGCType_AllGuides = 7,
    k_EUGCMatchingUGCType_WebGuides = 8,
    k_EUGCMatchingUGCType_IntegratedGuides = 9,
    k_EUGCMatchingUGCType_UsableInGame = 10,
    k_EUGCMatchingUGCType_ControllerBindings = 11,
    k_EUGCMatchingUGCType_GameManagedItems = 12,
    k_EUGCMatchingUGCType_All = -1,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUserUGCList {
    k_EUserUGCList_Published = 0,
    k_EUserUGCList_VotedOn = 1,
    k_EUserUGCList_VotedUp = 2,
    k_EUserUGCList_VotedDown = 3,
    k_EUserUGCList_WillVoteLater = 4,
    k_EUserUGCList_Favorited = 5,
    k_EUserUGCList_Subscribed = 6,
    k_EUserUGCList_UsedOrPlayed = 7,
    k_EUserUGCList_Followed = 8,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUserUGCListSortOrder {
    k_EUserUGCListSortOrder_CreationOrderDesc = 0,
    k_EUserUGCListSortOrder_CreationOrderAsc = 1,
    k_EUserUGCListSortOrder_TitleAsc = 2,
    k_EUserUGCListSortOrder_LastUpdatedDesc = 3,
    k_EUserUGCListSortOrder_SubscriptionDateDesc = 4,
    k_EUserUGCListSortOrder_VoteScoreDesc = 5,
    k_EUserUGCListSortOrder_ForModeration = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUGCQuery {
    k_EUGCQuery_RankedByVote = 0,
    k_EUGCQuery_RankedByPublicationDate = 1,
    k_EUGCQuery_AcceptedForGameRankedByAcceptanceDate = 2,
    k_EUGCQuery_RankedByTrend = 3,
    k_EUGCQuery_FavoritedByFriendsRankedByPublicationDate = 4,
    k_EUGCQuery_CreatedByFriendsRankedByPublicationDate = 5,
    k_EUGCQuery_RankedByNumTimesReported = 6,
    k_EUGCQuery_CreatedByFollowedUsersRankedByPublicationDate = 7,
    k_EUGCQuery_NotYetRated = 8,
    k_EUGCQuery_RankedByTotalVotesAsc = 9,
    k_EUGCQuery_RankedByVotesUp = 10,
    k_EUGCQuery_RankedByTextSearch = 11,
    k_EUGCQuery_RankedByTotalUniqueSubscriptions = 12,
    k_EUGCQuery_RankedByPlaytimeTrend = 13,
    k_EUGCQuery_RankedByTotalPlaytime = 14,
    k_EUGCQuery_RankedByAveragePlaytimeTrend = 15,
    k_EUGCQuery_RankedByLifetimeAveragePlaytime = 16,
    k_EUGCQuery_RankedByPlaytimeSessionsTrend = 17,
    k_EUGCQuery_RankedByLifetimePlaytimeSessions = 18,
    k_EUGCQuery_RankedByLastUpdatedDate = 19,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EItemUpdateStatus {
    k_EItemUpdateStatusInvalid = 0,
    k_EItemUpdateStatusPreparingConfig = 1,
    k_EItemUpdateStatusPreparingContent = 2,
    k_EItemUpdateStatusUploadingContent = 3,
    k_EItemUpdateStatusUploadingPreviewFile = 4,
    k_EItemUpdateStatusCommittingChanges = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EItemState {
    k_EItemStateNone = 0,
    k_EItemStateSubscribed = 1,
    k_EItemStateLegacyItem = 2,
    k_EItemStateInstalled = 4,
    k_EItemStateNeedsUpdate = 8,
    k_EItemStateDownloading = 16,
    k_EItemStateDownloadPending = 32,
    k_EItemStateDisabledLocally = 64,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EItemStatistic {
    k_EItemStatistic_NumSubscriptions = 0,
    k_EItemStatistic_NumFavorites = 1,
    k_EItemStatistic_NumFollowers = 2,
    k_EItemStatistic_NumUniqueSubscriptions = 3,
    k_EItemStatistic_NumUniqueFavorites = 4,
    k_EItemStatistic_NumUniqueFollowers = 5,
    k_EItemStatistic_NumUniqueWebsiteViews = 6,
    k_EItemStatistic_ReportScore = 7,
    k_EItemStatistic_NumSecondsPlayed = 8,
    k_EItemStatistic_NumPlaytimeSessions = 9,
    k_EItemStatistic_NumComments = 10,
    k_EItemStatistic_NumSecondsPlayedDuringTimePeriod = 11,
    k_EItemStatistic_NumPlaytimeSessionsDuringTimePeriod = 12,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EItemPreviewType {
    k_EItemPreviewType_Image = 0,
    k_EItemPreviewType_YouTubeVideo = 1,
    k_EItemPreviewType_Sketchfab = 2,
    k_EItemPreviewType_EnvironmentMap_HorizontalCross = 3,
    k_EItemPreviewType_EnvironmentMap_LatLong = 4,
    k_EItemPreviewType_Clip = 5,
    k_EItemPreviewType_ReservedMax = 255,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EUGCContentDescriptorID {
    k_EUGCContentDescriptor_NudityOrSexualContent = 1,
    k_EUGCContentDescriptor_FrequentViolenceOrGore = 2,
    k_EUGCContentDescriptor_AdultOnlySexualContent = 3,
    k_EUGCContentDescriptor_GratuitousSexualContent = 4,
    k_EUGCContentDescriptor_AnyMatureContent = 5,
}
pub const kNumUGCResultsPerPage: uint32 = 50;
pub const k_cchDeveloperMetadataMax: uint32 = 5000;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamUGCDetails_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eResult: EResult,
    pub m_eFileType: EWorkshopFileType,
    pub m_nCreatorAppID: AppId_t,
    pub m_nConsumerAppID: AppId_t,
    pub m_rgchTitle: [::std::os::raw::c_char; 129usize],
    pub m_rgchDescription: [::std::os::raw::c_char; 8000usize],
    pub m_ulSteamIDOwner: uint64,
    pub m_rtimeCreated: uint32,
    pub m_rtimeUpdated: uint32,
    pub m_rtimeAddedToUserList: uint32,
    pub m_eVisibility: ERemoteStoragePublishedFileVisibility,
    pub m_bBanned: bool,
    pub m_bAcceptedForUse: bool,
    pub m_bTagsTruncated: bool,
    pub m_rgchTags: [::std::os::raw::c_char; 1025usize],
    pub m_hFile: UGCHandle_t,
    pub m_hPreviewFile: UGCHandle_t,
    pub m_pchFileName: [::std::os::raw::c_char; 260usize],
    pub m_nFileSize: int32,
    pub m_nPreviewFileSize: int32,
    pub m_rgchURL: [::std::os::raw::c_char; 256usize],
    pub m_unVotesUp: uint32,
    pub m_unVotesDown: uint32,
    pub m_flScore: f32,
    pub m_unNumChildren: uint32,
    pub m_ulTotalFilesSize: uint64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamUGCDetails_t",
    ][::std::mem::size_of::<SteamUGCDetails_t>() - 9772usize];
    [
        "Alignment of SteamUGCDetails_t",
    ][::std::mem::align_of::<SteamUGCDetails_t>() - 4usize];
    [
        "Offset of field: SteamUGCDetails_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_nPublishedFileId) - 0usize];
    [
        "Offset of field: SteamUGCDetails_t::m_eResult",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_eResult) - 8usize];
    [
        "Offset of field: SteamUGCDetails_t::m_eFileType",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_eFileType) - 12usize];
    [
        "Offset of field: SteamUGCDetails_t::m_nCreatorAppID",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_nCreatorAppID) - 16usize];
    [
        "Offset of field: SteamUGCDetails_t::m_nConsumerAppID",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_nConsumerAppID) - 20usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rgchTitle",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rgchTitle) - 24usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rgchDescription",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rgchDescription) - 153usize];
    [
        "Offset of field: SteamUGCDetails_t::m_ulSteamIDOwner",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_ulSteamIDOwner) - 8156usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rtimeCreated",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rtimeCreated) - 8164usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rtimeUpdated",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rtimeUpdated) - 8168usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rtimeAddedToUserList",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rtimeAddedToUserList) - 8172usize];
    [
        "Offset of field: SteamUGCDetails_t::m_eVisibility",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_eVisibility) - 8176usize];
    [
        "Offset of field: SteamUGCDetails_t::m_bBanned",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_bBanned) - 8180usize];
    [
        "Offset of field: SteamUGCDetails_t::m_bAcceptedForUse",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_bAcceptedForUse) - 8181usize];
    [
        "Offset of field: SteamUGCDetails_t::m_bTagsTruncated",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_bTagsTruncated) - 8182usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rgchTags",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rgchTags) - 8183usize];
    [
        "Offset of field: SteamUGCDetails_t::m_hFile",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_hFile) - 9208usize];
    [
        "Offset of field: SteamUGCDetails_t::m_hPreviewFile",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_hPreviewFile) - 9216usize];
    [
        "Offset of field: SteamUGCDetails_t::m_pchFileName",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_pchFileName) - 9224usize];
    [
        "Offset of field: SteamUGCDetails_t::m_nFileSize",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_nFileSize) - 9484usize];
    [
        "Offset of field: SteamUGCDetails_t::m_nPreviewFileSize",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_nPreviewFileSize) - 9488usize];
    [
        "Offset of field: SteamUGCDetails_t::m_rgchURL",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_rgchURL) - 9492usize];
    [
        "Offset of field: SteamUGCDetails_t::m_unVotesUp",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_unVotesUp) - 9748usize];
    [
        "Offset of field: SteamUGCDetails_t::m_unVotesDown",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_unVotesDown) - 9752usize];
    [
        "Offset of field: SteamUGCDetails_t::m_flScore",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_flScore) - 9756usize];
    [
        "Offset of field: SteamUGCDetails_t::m_unNumChildren",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_unNumChildren) - 9760usize];
    [
        "Offset of field: SteamUGCDetails_t::m_ulTotalFilesSize",
    ][::std::mem::offset_of!(SteamUGCDetails_t, m_ulTotalFilesSize) - 9764usize];
};
#[repr(C)]
pub struct ISteamUGC__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamUGC {
    pub vtable_: *const ISteamUGC__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamUGC"][::std::mem::size_of::<ISteamUGC>() - 8usize];
    ["Alignment of ISteamUGC"][::std::mem::align_of::<ISteamUGC>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamUGCQueryCompleted_t {
    pub m_handle: UGCQueryHandle_t,
    pub m_eResult: EResult,
    pub m_unNumResultsReturned: uint32,
    pub m_unTotalMatchingResults: uint32,
    pub m_bCachedData: bool,
    pub m_rgchNextCursor: [::std::os::raw::c_char; 256usize],
}
pub const SteamUGCQueryCompleted_t_k_iCallback: SteamUGCQueryCompleted_t__bindgen_ty_1 = SteamUGCQueryCompleted_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamUGCQueryCompleted_t__bindgen_ty_1 {
    k_iCallback = 3401,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamUGCQueryCompleted_t",
    ][::std::mem::size_of::<SteamUGCQueryCompleted_t>() - 280usize];
    [
        "Alignment of SteamUGCQueryCompleted_t",
    ][::std::mem::align_of::<SteamUGCQueryCompleted_t>() - 4usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_handle",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_handle) - 0usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_eResult",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_eResult) - 8usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_unNumResultsReturned",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_unNumResultsReturned)
        - 12usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_unTotalMatchingResults",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_unTotalMatchingResults)
        - 16usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_bCachedData",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_bCachedData) - 20usize];
    [
        "Offset of field: SteamUGCQueryCompleted_t::m_rgchNextCursor",
    ][::std::mem::offset_of!(SteamUGCQueryCompleted_t, m_rgchNextCursor) - 21usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamUGCRequestUGCDetailsResult_t {
    pub m_details: SteamUGCDetails_t,
    pub m_bCachedData: bool,
}
pub const SteamUGCRequestUGCDetailsResult_t_k_iCallback: SteamUGCRequestUGCDetailsResult_t__bindgen_ty_1 = SteamUGCRequestUGCDetailsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamUGCRequestUGCDetailsResult_t__bindgen_ty_1 {
    k_iCallback = 3402,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamUGCRequestUGCDetailsResult_t",
    ][::std::mem::size_of::<SteamUGCRequestUGCDetailsResult_t>() - 9776usize];
    [
        "Alignment of SteamUGCRequestUGCDetailsResult_t",
    ][::std::mem::align_of::<SteamUGCRequestUGCDetailsResult_t>() - 4usize];
    [
        "Offset of field: SteamUGCRequestUGCDetailsResult_t::m_details",
    ][::std::mem::offset_of!(SteamUGCRequestUGCDetailsResult_t, m_details) - 0usize];
    [
        "Offset of field: SteamUGCRequestUGCDetailsResult_t::m_bCachedData",
    ][::std::mem::offset_of!(SteamUGCRequestUGCDetailsResult_t, m_bCachedData)
        - 9772usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct CreateItemResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_bUserNeedsToAcceptWorkshopLegalAgreement: bool,
}
pub const CreateItemResult_t_k_iCallback: CreateItemResult_t__bindgen_ty_1 = CreateItemResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum CreateItemResult_t__bindgen_ty_1 {
    k_iCallback = 3403,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of CreateItemResult_t",
    ][::std::mem::size_of::<CreateItemResult_t>() - 16usize];
    [
        "Alignment of CreateItemResult_t",
    ][::std::mem::align_of::<CreateItemResult_t>() - 4usize];
    [
        "Offset of field: CreateItemResult_t::m_eResult",
    ][::std::mem::offset_of!(CreateItemResult_t, m_eResult) - 0usize];
    [
        "Offset of field: CreateItemResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(CreateItemResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: CreateItemResult_t::m_bUserNeedsToAcceptWorkshopLegalAgreement",
    ][::std::mem::offset_of!(
        CreateItemResult_t, m_bUserNeedsToAcceptWorkshopLegalAgreement
    ) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SubmitItemUpdateResult_t {
    pub m_eResult: EResult,
    pub m_bUserNeedsToAcceptWorkshopLegalAgreement: bool,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const SubmitItemUpdateResult_t_k_iCallback: SubmitItemUpdateResult_t__bindgen_ty_1 = SubmitItemUpdateResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SubmitItemUpdateResult_t__bindgen_ty_1 {
    k_iCallback = 3404,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SubmitItemUpdateResult_t",
    ][::std::mem::size_of::<SubmitItemUpdateResult_t>() - 16usize];
    [
        "Alignment of SubmitItemUpdateResult_t",
    ][::std::mem::align_of::<SubmitItemUpdateResult_t>() - 4usize];
    [
        "Offset of field: SubmitItemUpdateResult_t::m_eResult",
    ][::std::mem::offset_of!(SubmitItemUpdateResult_t, m_eResult) - 0usize];
    [
        "Offset of field: SubmitItemUpdateResult_t::m_bUserNeedsToAcceptWorkshopLegalAgreement",
    ][::std::mem::offset_of!(
        SubmitItemUpdateResult_t, m_bUserNeedsToAcceptWorkshopLegalAgreement
    ) - 4usize];
    [
        "Offset of field: SubmitItemUpdateResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(SubmitItemUpdateResult_t, m_nPublishedFileId) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct ItemInstalled_t {
    pub m_unAppID: AppId_t,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_hLegacyContent: UGCHandle_t,
    pub m_unManifestID: uint64,
}
pub const ItemInstalled_t_k_iCallback: ItemInstalled_t__bindgen_ty_1 = ItemInstalled_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ItemInstalled_t__bindgen_ty_1 {
    k_iCallback = 3405,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ItemInstalled_t"][::std::mem::size_of::<ItemInstalled_t>() - 28usize];
    ["Alignment of ItemInstalled_t"][::std::mem::align_of::<ItemInstalled_t>() - 4usize];
    [
        "Offset of field: ItemInstalled_t::m_unAppID",
    ][::std::mem::offset_of!(ItemInstalled_t, m_unAppID) - 0usize];
    [
        "Offset of field: ItemInstalled_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(ItemInstalled_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: ItemInstalled_t::m_hLegacyContent",
    ][::std::mem::offset_of!(ItemInstalled_t, m_hLegacyContent) - 12usize];
    [
        "Offset of field: ItemInstalled_t::m_unManifestID",
    ][::std::mem::offset_of!(ItemInstalled_t, m_unManifestID) - 20usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct DownloadItemResult_t {
    pub m_unAppID: AppId_t,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eResult: EResult,
}
pub const DownloadItemResult_t_k_iCallback: DownloadItemResult_t__bindgen_ty_1 = DownloadItemResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DownloadItemResult_t__bindgen_ty_1 {
    k_iCallback = 3406,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of DownloadItemResult_t",
    ][::std::mem::size_of::<DownloadItemResult_t>() - 16usize];
    [
        "Alignment of DownloadItemResult_t",
    ][::std::mem::align_of::<DownloadItemResult_t>() - 4usize];
    [
        "Offset of field: DownloadItemResult_t::m_unAppID",
    ][::std::mem::offset_of!(DownloadItemResult_t, m_unAppID) - 0usize];
    [
        "Offset of field: DownloadItemResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(DownloadItemResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: DownloadItemResult_t::m_eResult",
    ][::std::mem::offset_of!(DownloadItemResult_t, m_eResult) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct UserFavoriteItemsListChanged_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eResult: EResult,
    pub m_bWasAddRequest: bool,
}
pub const UserFavoriteItemsListChanged_t_k_iCallback: UserFavoriteItemsListChanged_t__bindgen_ty_1 = UserFavoriteItemsListChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserFavoriteItemsListChanged_t__bindgen_ty_1 {
    k_iCallback = 3407,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserFavoriteItemsListChanged_t",
    ][::std::mem::size_of::<UserFavoriteItemsListChanged_t>() - 16usize];
    [
        "Alignment of UserFavoriteItemsListChanged_t",
    ][::std::mem::align_of::<UserFavoriteItemsListChanged_t>() - 4usize];
    [
        "Offset of field: UserFavoriteItemsListChanged_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(UserFavoriteItemsListChanged_t, m_nPublishedFileId)
        - 0usize];
    [
        "Offset of field: UserFavoriteItemsListChanged_t::m_eResult",
    ][::std::mem::offset_of!(UserFavoriteItemsListChanged_t, m_eResult) - 8usize];
    [
        "Offset of field: UserFavoriteItemsListChanged_t::m_bWasAddRequest",
    ][::std::mem::offset_of!(UserFavoriteItemsListChanged_t, m_bWasAddRequest)
        - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SetUserItemVoteResult_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eResult: EResult,
    pub m_bVoteUp: bool,
}
pub const SetUserItemVoteResult_t_k_iCallback: SetUserItemVoteResult_t__bindgen_ty_1 = SetUserItemVoteResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SetUserItemVoteResult_t__bindgen_ty_1 {
    k_iCallback = 3408,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SetUserItemVoteResult_t",
    ][::std::mem::size_of::<SetUserItemVoteResult_t>() - 16usize];
    [
        "Alignment of SetUserItemVoteResult_t",
    ][::std::mem::align_of::<SetUserItemVoteResult_t>() - 4usize];
    [
        "Offset of field: SetUserItemVoteResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(SetUserItemVoteResult_t, m_nPublishedFileId) - 0usize];
    [
        "Offset of field: SetUserItemVoteResult_t::m_eResult",
    ][::std::mem::offset_of!(SetUserItemVoteResult_t, m_eResult) - 8usize];
    [
        "Offset of field: SetUserItemVoteResult_t::m_bVoteUp",
    ][::std::mem::offset_of!(SetUserItemVoteResult_t, m_bVoteUp) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GetUserItemVoteResult_t {
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_eResult: EResult,
    pub m_bVotedUp: bool,
    pub m_bVotedDown: bool,
    pub m_bVoteSkipped: bool,
}
pub const GetUserItemVoteResult_t_k_iCallback: GetUserItemVoteResult_t__bindgen_ty_1 = GetUserItemVoteResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetUserItemVoteResult_t__bindgen_ty_1 {
    k_iCallback = 3409,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetUserItemVoteResult_t",
    ][::std::mem::size_of::<GetUserItemVoteResult_t>() - 16usize];
    [
        "Alignment of GetUserItemVoteResult_t",
    ][::std::mem::align_of::<GetUserItemVoteResult_t>() - 4usize];
    [
        "Offset of field: GetUserItemVoteResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(GetUserItemVoteResult_t, m_nPublishedFileId) - 0usize];
    [
        "Offset of field: GetUserItemVoteResult_t::m_eResult",
    ][::std::mem::offset_of!(GetUserItemVoteResult_t, m_eResult) - 8usize];
    [
        "Offset of field: GetUserItemVoteResult_t::m_bVotedUp",
    ][::std::mem::offset_of!(GetUserItemVoteResult_t, m_bVotedUp) - 12usize];
    [
        "Offset of field: GetUserItemVoteResult_t::m_bVotedDown",
    ][::std::mem::offset_of!(GetUserItemVoteResult_t, m_bVotedDown) - 13usize];
    [
        "Offset of field: GetUserItemVoteResult_t::m_bVoteSkipped",
    ][::std::mem::offset_of!(GetUserItemVoteResult_t, m_bVoteSkipped) - 14usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StartPlaytimeTrackingResult_t {
    pub m_eResult: EResult,
}
pub const StartPlaytimeTrackingResult_t_k_iCallback: StartPlaytimeTrackingResult_t__bindgen_ty_1 = StartPlaytimeTrackingResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StartPlaytimeTrackingResult_t__bindgen_ty_1 {
    k_iCallback = 3410,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StartPlaytimeTrackingResult_t",
    ][::std::mem::size_of::<StartPlaytimeTrackingResult_t>() - 4usize];
    [
        "Alignment of StartPlaytimeTrackingResult_t",
    ][::std::mem::align_of::<StartPlaytimeTrackingResult_t>() - 4usize];
    [
        "Offset of field: StartPlaytimeTrackingResult_t::m_eResult",
    ][::std::mem::offset_of!(StartPlaytimeTrackingResult_t, m_eResult) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct StopPlaytimeTrackingResult_t {
    pub m_eResult: EResult,
}
pub const StopPlaytimeTrackingResult_t_k_iCallback: StopPlaytimeTrackingResult_t__bindgen_ty_1 = StopPlaytimeTrackingResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum StopPlaytimeTrackingResult_t__bindgen_ty_1 {
    k_iCallback = 3411,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of StopPlaytimeTrackingResult_t",
    ][::std::mem::size_of::<StopPlaytimeTrackingResult_t>() - 4usize];
    [
        "Alignment of StopPlaytimeTrackingResult_t",
    ][::std::mem::align_of::<StopPlaytimeTrackingResult_t>() - 4usize];
    [
        "Offset of field: StopPlaytimeTrackingResult_t::m_eResult",
    ][::std::mem::offset_of!(StopPlaytimeTrackingResult_t, m_eResult) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AddUGCDependencyResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nChildPublishedFileId: PublishedFileId_t,
}
pub const AddUGCDependencyResult_t_k_iCallback: AddUGCDependencyResult_t__bindgen_ty_1 = AddUGCDependencyResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AddUGCDependencyResult_t__bindgen_ty_1 {
    k_iCallback = 3412,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AddUGCDependencyResult_t",
    ][::std::mem::size_of::<AddUGCDependencyResult_t>() - 20usize];
    [
        "Alignment of AddUGCDependencyResult_t",
    ][::std::mem::align_of::<AddUGCDependencyResult_t>() - 4usize];
    [
        "Offset of field: AddUGCDependencyResult_t::m_eResult",
    ][::std::mem::offset_of!(AddUGCDependencyResult_t, m_eResult) - 0usize];
    [
        "Offset of field: AddUGCDependencyResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(AddUGCDependencyResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: AddUGCDependencyResult_t::m_nChildPublishedFileId",
    ][::std::mem::offset_of!(AddUGCDependencyResult_t, m_nChildPublishedFileId)
        - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoveUGCDependencyResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nChildPublishedFileId: PublishedFileId_t,
}
pub const RemoveUGCDependencyResult_t_k_iCallback: RemoveUGCDependencyResult_t__bindgen_ty_1 = RemoveUGCDependencyResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoveUGCDependencyResult_t__bindgen_ty_1 {
    k_iCallback = 3413,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoveUGCDependencyResult_t",
    ][::std::mem::size_of::<RemoveUGCDependencyResult_t>() - 20usize];
    [
        "Alignment of RemoveUGCDependencyResult_t",
    ][::std::mem::align_of::<RemoveUGCDependencyResult_t>() - 4usize];
    [
        "Offset of field: RemoveUGCDependencyResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoveUGCDependencyResult_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoveUGCDependencyResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoveUGCDependencyResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: RemoveUGCDependencyResult_t::m_nChildPublishedFileId",
    ][::std::mem::offset_of!(RemoveUGCDependencyResult_t, m_nChildPublishedFileId)
        - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct AddAppDependencyResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
}
pub const AddAppDependencyResult_t_k_iCallback: AddAppDependencyResult_t__bindgen_ty_1 = AddAppDependencyResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AddAppDependencyResult_t__bindgen_ty_1 {
    k_iCallback = 3414,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AddAppDependencyResult_t",
    ][::std::mem::size_of::<AddAppDependencyResult_t>() - 16usize];
    [
        "Alignment of AddAppDependencyResult_t",
    ][::std::mem::align_of::<AddAppDependencyResult_t>() - 4usize];
    [
        "Offset of field: AddAppDependencyResult_t::m_eResult",
    ][::std::mem::offset_of!(AddAppDependencyResult_t, m_eResult) - 0usize];
    [
        "Offset of field: AddAppDependencyResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(AddAppDependencyResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: AddAppDependencyResult_t::m_nAppID",
    ][::std::mem::offset_of!(AddAppDependencyResult_t, m_nAppID) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct RemoveAppDependencyResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_nAppID: AppId_t,
}
pub const RemoveAppDependencyResult_t_k_iCallback: RemoveAppDependencyResult_t__bindgen_ty_1 = RemoveAppDependencyResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum RemoveAppDependencyResult_t__bindgen_ty_1 {
    k_iCallback = 3415,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemoveAppDependencyResult_t",
    ][::std::mem::size_of::<RemoveAppDependencyResult_t>() - 16usize];
    [
        "Alignment of RemoveAppDependencyResult_t",
    ][::std::mem::align_of::<RemoveAppDependencyResult_t>() - 4usize];
    [
        "Offset of field: RemoveAppDependencyResult_t::m_eResult",
    ][::std::mem::offset_of!(RemoveAppDependencyResult_t, m_eResult) - 0usize];
    [
        "Offset of field: RemoveAppDependencyResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(RemoveAppDependencyResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: RemoveAppDependencyResult_t::m_nAppID",
    ][::std::mem::offset_of!(RemoveAppDependencyResult_t, m_nAppID) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GetAppDependenciesResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
    pub m_rgAppIDs: [AppId_t; 32usize],
    pub m_nNumAppDependencies: uint32,
    pub m_nTotalNumAppDependencies: uint32,
}
pub const GetAppDependenciesResult_t_k_iCallback: GetAppDependenciesResult_t__bindgen_ty_1 = GetAppDependenciesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetAppDependenciesResult_t__bindgen_ty_1 {
    k_iCallback = 3416,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetAppDependenciesResult_t",
    ][::std::mem::size_of::<GetAppDependenciesResult_t>() - 148usize];
    [
        "Alignment of GetAppDependenciesResult_t",
    ][::std::mem::align_of::<GetAppDependenciesResult_t>() - 4usize];
    [
        "Offset of field: GetAppDependenciesResult_t::m_eResult",
    ][::std::mem::offset_of!(GetAppDependenciesResult_t, m_eResult) - 0usize];
    [
        "Offset of field: GetAppDependenciesResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(GetAppDependenciesResult_t, m_nPublishedFileId) - 4usize];
    [
        "Offset of field: GetAppDependenciesResult_t::m_rgAppIDs",
    ][::std::mem::offset_of!(GetAppDependenciesResult_t, m_rgAppIDs) - 12usize];
    [
        "Offset of field: GetAppDependenciesResult_t::m_nNumAppDependencies",
    ][::std::mem::offset_of!(GetAppDependenciesResult_t, m_nNumAppDependencies)
        - 140usize];
    [
        "Offset of field: GetAppDependenciesResult_t::m_nTotalNumAppDependencies",
    ][::std::mem::offset_of!(GetAppDependenciesResult_t, m_nTotalNumAppDependencies)
        - 144usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct DeleteItemResult_t {
    pub m_eResult: EResult,
    pub m_nPublishedFileId: PublishedFileId_t,
}
pub const DeleteItemResult_t_k_iCallback: DeleteItemResult_t__bindgen_ty_1 = DeleteItemResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DeleteItemResult_t__bindgen_ty_1 {
    k_iCallback = 3417,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of DeleteItemResult_t",
    ][::std::mem::size_of::<DeleteItemResult_t>() - 12usize];
    [
        "Alignment of DeleteItemResult_t",
    ][::std::mem::align_of::<DeleteItemResult_t>() - 4usize];
    [
        "Offset of field: DeleteItemResult_t::m_eResult",
    ][::std::mem::offset_of!(DeleteItemResult_t, m_eResult) - 0usize];
    [
        "Offset of field: DeleteItemResult_t::m_nPublishedFileId",
    ][::std::mem::offset_of!(DeleteItemResult_t, m_nPublishedFileId) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UserSubscribedItemsListChanged_t {
    pub m_nAppID: AppId_t,
}
pub const UserSubscribedItemsListChanged_t_k_iCallback: UserSubscribedItemsListChanged_t__bindgen_ty_1 = UserSubscribedItemsListChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum UserSubscribedItemsListChanged_t__bindgen_ty_1 {
    k_iCallback = 3418,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of UserSubscribedItemsListChanged_t",
    ][::std::mem::size_of::<UserSubscribedItemsListChanged_t>() - 4usize];
    [
        "Alignment of UserSubscribedItemsListChanged_t",
    ][::std::mem::align_of::<UserSubscribedItemsListChanged_t>() - 4usize];
    [
        "Offset of field: UserSubscribedItemsListChanged_t::m_nAppID",
    ][::std::mem::offset_of!(UserSubscribedItemsListChanged_t, m_nAppID) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WorkshopEULAStatus_t {
    pub m_eResult: EResult,
    pub m_nAppID: AppId_t,
    pub m_unVersion: uint32,
    pub m_rtAction: RTime32,
    pub m_bAccepted: bool,
    pub m_bNeedsAction: bool,
}
pub const WorkshopEULAStatus_t_k_iCallback: WorkshopEULAStatus_t__bindgen_ty_1 = WorkshopEULAStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum WorkshopEULAStatus_t__bindgen_ty_1 {
    k_iCallback = 3420,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of WorkshopEULAStatus_t",
    ][::std::mem::size_of::<WorkshopEULAStatus_t>() - 20usize];
    [
        "Alignment of WorkshopEULAStatus_t",
    ][::std::mem::align_of::<WorkshopEULAStatus_t>() - 4usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_eResult",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_eResult) - 0usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_nAppID",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_nAppID) - 4usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_unVersion",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_unVersion) - 8usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_rtAction",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_rtAction) - 12usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_bAccepted",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_bAccepted) - 16usize];
    [
        "Offset of field: WorkshopEULAStatus_t::m_bNeedsAction",
    ][::std::mem::offset_of!(WorkshopEULAStatus_t, m_bNeedsAction) - 17usize];
};
pub type HHTMLBrowser = uint32;
pub const INVALID_HTMLBROWSER: uint32 = 0;
#[repr(C)]
pub struct ISteamHTMLSurface__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct ISteamHTMLSurface {
    pub vtable_: *const ISteamHTMLSurface__bindgen_vtable,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ISteamHTMLSurface_EHTMLMouseButton {
    eHTMLMouseButton_Left = 0,
    eHTMLMouseButton_Right = 1,
    eHTMLMouseButton_Middle = 2,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ISteamHTMLSurface_EHTMLMouseCursor {
    k_EHTMLMouseCursor_User = 0,
    k_EHTMLMouseCursor_None = 1,
    k_EHTMLMouseCursor_Arrow = 2,
    k_EHTMLMouseCursor_IBeam = 3,
    k_EHTMLMouseCursor_Hourglass = 4,
    k_EHTMLMouseCursor_WaitArrow = 5,
    k_EHTMLMouseCursor_Crosshair = 6,
    k_EHTMLMouseCursor_Up = 7,
    k_EHTMLMouseCursor_SizeNW = 8,
    k_EHTMLMouseCursor_SizeSE = 9,
    k_EHTMLMouseCursor_SizeNE = 10,
    k_EHTMLMouseCursor_SizeSW = 11,
    k_EHTMLMouseCursor_SizeW = 12,
    k_EHTMLMouseCursor_SizeE = 13,
    k_EHTMLMouseCursor_SizeN = 14,
    k_EHTMLMouseCursor_SizeS = 15,
    k_EHTMLMouseCursor_SizeWE = 16,
    k_EHTMLMouseCursor_SizeNS = 17,
    k_EHTMLMouseCursor_SizeAll = 18,
    k_EHTMLMouseCursor_No = 19,
    k_EHTMLMouseCursor_Hand = 20,
    k_EHTMLMouseCursor_Blank = 21,
    k_EHTMLMouseCursor_MiddlePan = 22,
    k_EHTMLMouseCursor_NorthPan = 23,
    k_EHTMLMouseCursor_NorthEastPan = 24,
    k_EHTMLMouseCursor_EastPan = 25,
    k_EHTMLMouseCursor_SouthEastPan = 26,
    k_EHTMLMouseCursor_SouthPan = 27,
    k_EHTMLMouseCursor_SouthWestPan = 28,
    k_EHTMLMouseCursor_WestPan = 29,
    k_EHTMLMouseCursor_NorthWestPan = 30,
    k_EHTMLMouseCursor_Alias = 31,
    k_EHTMLMouseCursor_Cell = 32,
    k_EHTMLMouseCursor_ColResize = 33,
    k_EHTMLMouseCursor_CopyCur = 34,
    k_EHTMLMouseCursor_VerticalText = 35,
    k_EHTMLMouseCursor_RowResize = 36,
    k_EHTMLMouseCursor_ZoomIn = 37,
    k_EHTMLMouseCursor_ZoomOut = 38,
    k_EHTMLMouseCursor_Help = 39,
    k_EHTMLMouseCursor_Custom = 40,
    k_EHTMLMouseCursor_SizeNWSE = 41,
    k_EHTMLMouseCursor_SizeNESW = 42,
    k_EHTMLMouseCursor_last = 43,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ISteamHTMLSurface_EHTMLKeyModifiers {
    k_eHTMLKeyModifier_None = 0,
    k_eHTMLKeyModifier_AltDown = 1,
    k_eHTMLKeyModifier_CtrlDown = 2,
    k_eHTMLKeyModifier_ShiftDown = 4,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamHTMLSurface"][::std::mem::size_of::<ISteamHTMLSurface>() - 8usize];
    [
        "Alignment of ISteamHTMLSurface",
    ][::std::mem::align_of::<ISteamHTMLSurface>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_BrowserReady_t {
    pub unBrowserHandle: HHTMLBrowser,
}
pub const HTML_BrowserReady_t_k_iCallback: HTML_BrowserReady_t__bindgen_ty_1 = HTML_BrowserReady_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_BrowserReady_t__bindgen_ty_1 {
    k_iCallback = 4501,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_BrowserReady_t",
    ][::std::mem::size_of::<HTML_BrowserReady_t>() - 4usize];
    [
        "Alignment of HTML_BrowserReady_t",
    ][::std::mem::align_of::<HTML_BrowserReady_t>() - 4usize];
    [
        "Offset of field: HTML_BrowserReady_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_BrowserReady_t, unBrowserHandle) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_NeedsPaint_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pBGRA: *const ::std::os::raw::c_char,
    pub unWide: uint32,
    pub unTall: uint32,
    pub unUpdateX: uint32,
    pub unUpdateY: uint32,
    pub unUpdateWide: uint32,
    pub unUpdateTall: uint32,
    pub unScrollX: uint32,
    pub unScrollY: uint32,
    pub flPageScale: f32,
    pub unPageSerial: uint32,
}
pub const HTML_NeedsPaint_t_k_iCallback: HTML_NeedsPaint_t__bindgen_ty_1 = HTML_NeedsPaint_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_NeedsPaint_t__bindgen_ty_1 {
    k_iCallback = 4502,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_NeedsPaint_t"][::std::mem::size_of::<HTML_NeedsPaint_t>() - 52usize];
    [
        "Alignment of HTML_NeedsPaint_t",
    ][::std::mem::align_of::<HTML_NeedsPaint_t>() - 4usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_NeedsPaint_t::pBGRA",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, pBGRA) - 4usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unWide",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unWide) - 12usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unTall",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unTall) - 16usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unUpdateX",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unUpdateX) - 20usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unUpdateY",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unUpdateY) - 24usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unUpdateWide",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unUpdateWide) - 28usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unUpdateTall",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unUpdateTall) - 32usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unScrollX",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unScrollX) - 36usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unScrollY",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unScrollY) - 40usize];
    [
        "Offset of field: HTML_NeedsPaint_t::flPageScale",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, flPageScale) - 44usize];
    [
        "Offset of field: HTML_NeedsPaint_t::unPageSerial",
    ][::std::mem::offset_of!(HTML_NeedsPaint_t, unPageSerial) - 48usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_StartRequest_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchURL: *const ::std::os::raw::c_char,
    pub pchTarget: *const ::std::os::raw::c_char,
    pub pchPostData: *const ::std::os::raw::c_char,
    pub bIsRedirect: bool,
}
pub const HTML_StartRequest_t_k_iCallback: HTML_StartRequest_t__bindgen_ty_1 = HTML_StartRequest_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_StartRequest_t__bindgen_ty_1 {
    k_iCallback = 4503,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_StartRequest_t",
    ][::std::mem::size_of::<HTML_StartRequest_t>() - 32usize];
    [
        "Alignment of HTML_StartRequest_t",
    ][::std::mem::align_of::<HTML_StartRequest_t>() - 4usize];
    [
        "Offset of field: HTML_StartRequest_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_StartRequest_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_StartRequest_t::pchURL",
    ][::std::mem::offset_of!(HTML_StartRequest_t, pchURL) - 4usize];
    [
        "Offset of field: HTML_StartRequest_t::pchTarget",
    ][::std::mem::offset_of!(HTML_StartRequest_t, pchTarget) - 12usize];
    [
        "Offset of field: HTML_StartRequest_t::pchPostData",
    ][::std::mem::offset_of!(HTML_StartRequest_t, pchPostData) - 20usize];
    [
        "Offset of field: HTML_StartRequest_t::bIsRedirect",
    ][::std::mem::offset_of!(HTML_StartRequest_t, bIsRedirect) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_CloseBrowser_t {
    pub unBrowserHandle: HHTMLBrowser,
}
pub const HTML_CloseBrowser_t_k_iCallback: HTML_CloseBrowser_t__bindgen_ty_1 = HTML_CloseBrowser_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_CloseBrowser_t__bindgen_ty_1 {
    k_iCallback = 4504,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_CloseBrowser_t",
    ][::std::mem::size_of::<HTML_CloseBrowser_t>() - 4usize];
    [
        "Alignment of HTML_CloseBrowser_t",
    ][::std::mem::align_of::<HTML_CloseBrowser_t>() - 4usize];
    [
        "Offset of field: HTML_CloseBrowser_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_CloseBrowser_t, unBrowserHandle) - 0usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_URLChanged_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchURL: *const ::std::os::raw::c_char,
    pub pchPostData: *const ::std::os::raw::c_char,
    pub bIsRedirect: bool,
    pub pchPageTitle: *const ::std::os::raw::c_char,
    pub bNewNavigation: bool,
}
pub const HTML_URLChanged_t_k_iCallback: HTML_URLChanged_t__bindgen_ty_1 = HTML_URLChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_URLChanged_t__bindgen_ty_1 {
    k_iCallback = 4505,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_URLChanged_t"][::std::mem::size_of::<HTML_URLChanged_t>() - 36usize];
    [
        "Alignment of HTML_URLChanged_t",
    ][::std::mem::align_of::<HTML_URLChanged_t>() - 4usize];
    [
        "Offset of field: HTML_URLChanged_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_URLChanged_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_URLChanged_t::pchURL",
    ][::std::mem::offset_of!(HTML_URLChanged_t, pchURL) - 4usize];
    [
        "Offset of field: HTML_URLChanged_t::pchPostData",
    ][::std::mem::offset_of!(HTML_URLChanged_t, pchPostData) - 12usize];
    [
        "Offset of field: HTML_URLChanged_t::bIsRedirect",
    ][::std::mem::offset_of!(HTML_URLChanged_t, bIsRedirect) - 20usize];
    [
        "Offset of field: HTML_URLChanged_t::pchPageTitle",
    ][::std::mem::offset_of!(HTML_URLChanged_t, pchPageTitle) - 24usize];
    [
        "Offset of field: HTML_URLChanged_t::bNewNavigation",
    ][::std::mem::offset_of!(HTML_URLChanged_t, bNewNavigation) - 32usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_FinishedRequest_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchURL: *const ::std::os::raw::c_char,
    pub pchPageTitle: *const ::std::os::raw::c_char,
}
pub const HTML_FinishedRequest_t_k_iCallback: HTML_FinishedRequest_t__bindgen_ty_1 = HTML_FinishedRequest_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_FinishedRequest_t__bindgen_ty_1 {
    k_iCallback = 4506,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_FinishedRequest_t",
    ][::std::mem::size_of::<HTML_FinishedRequest_t>() - 20usize];
    [
        "Alignment of HTML_FinishedRequest_t",
    ][::std::mem::align_of::<HTML_FinishedRequest_t>() - 4usize];
    [
        "Offset of field: HTML_FinishedRequest_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_FinishedRequest_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_FinishedRequest_t::pchURL",
    ][::std::mem::offset_of!(HTML_FinishedRequest_t, pchURL) - 4usize];
    [
        "Offset of field: HTML_FinishedRequest_t::pchPageTitle",
    ][::std::mem::offset_of!(HTML_FinishedRequest_t, pchPageTitle) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_OpenLinkInNewTab_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchURL: *const ::std::os::raw::c_char,
}
pub const HTML_OpenLinkInNewTab_t_k_iCallback: HTML_OpenLinkInNewTab_t__bindgen_ty_1 = HTML_OpenLinkInNewTab_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_OpenLinkInNewTab_t__bindgen_ty_1 {
    k_iCallback = 4507,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_OpenLinkInNewTab_t",
    ][::std::mem::size_of::<HTML_OpenLinkInNewTab_t>() - 12usize];
    [
        "Alignment of HTML_OpenLinkInNewTab_t",
    ][::std::mem::align_of::<HTML_OpenLinkInNewTab_t>() - 4usize];
    [
        "Offset of field: HTML_OpenLinkInNewTab_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_OpenLinkInNewTab_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_OpenLinkInNewTab_t::pchURL",
    ][::std::mem::offset_of!(HTML_OpenLinkInNewTab_t, pchURL) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_ChangedTitle_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchTitle: *const ::std::os::raw::c_char,
}
pub const HTML_ChangedTitle_t_k_iCallback: HTML_ChangedTitle_t__bindgen_ty_1 = HTML_ChangedTitle_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_ChangedTitle_t__bindgen_ty_1 {
    k_iCallback = 4508,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_ChangedTitle_t",
    ][::std::mem::size_of::<HTML_ChangedTitle_t>() - 12usize];
    [
        "Alignment of HTML_ChangedTitle_t",
    ][::std::mem::align_of::<HTML_ChangedTitle_t>() - 4usize];
    [
        "Offset of field: HTML_ChangedTitle_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_ChangedTitle_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_ChangedTitle_t::pchTitle",
    ][::std::mem::offset_of!(HTML_ChangedTitle_t, pchTitle) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_SearchResults_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub unResults: uint32,
    pub unCurrentMatch: uint32,
}
pub const HTML_SearchResults_t_k_iCallback: HTML_SearchResults_t__bindgen_ty_1 = HTML_SearchResults_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_SearchResults_t__bindgen_ty_1 {
    k_iCallback = 4509,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_SearchResults_t",
    ][::std::mem::size_of::<HTML_SearchResults_t>() - 12usize];
    [
        "Alignment of HTML_SearchResults_t",
    ][::std::mem::align_of::<HTML_SearchResults_t>() - 4usize];
    [
        "Offset of field: HTML_SearchResults_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_SearchResults_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_SearchResults_t::unResults",
    ][::std::mem::offset_of!(HTML_SearchResults_t, unResults) - 4usize];
    [
        "Offset of field: HTML_SearchResults_t::unCurrentMatch",
    ][::std::mem::offset_of!(HTML_SearchResults_t, unCurrentMatch) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_CanGoBackAndForward_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub bCanGoBack: bool,
    pub bCanGoForward: bool,
}
pub const HTML_CanGoBackAndForward_t_k_iCallback: HTML_CanGoBackAndForward_t__bindgen_ty_1 = HTML_CanGoBackAndForward_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_CanGoBackAndForward_t__bindgen_ty_1 {
    k_iCallback = 4510,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_CanGoBackAndForward_t",
    ][::std::mem::size_of::<HTML_CanGoBackAndForward_t>() - 8usize];
    [
        "Alignment of HTML_CanGoBackAndForward_t",
    ][::std::mem::align_of::<HTML_CanGoBackAndForward_t>() - 4usize];
    [
        "Offset of field: HTML_CanGoBackAndForward_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_CanGoBackAndForward_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_CanGoBackAndForward_t::bCanGoBack",
    ][::std::mem::offset_of!(HTML_CanGoBackAndForward_t, bCanGoBack) - 4usize];
    [
        "Offset of field: HTML_CanGoBackAndForward_t::bCanGoForward",
    ][::std::mem::offset_of!(HTML_CanGoBackAndForward_t, bCanGoForward) - 5usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_HorizontalScroll_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub unScrollMax: uint32,
    pub unScrollCurrent: uint32,
    pub flPageScale: f32,
    pub bVisible: bool,
    pub unPageSize: uint32,
}
pub const HTML_HorizontalScroll_t_k_iCallback: HTML_HorizontalScroll_t__bindgen_ty_1 = HTML_HorizontalScroll_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_HorizontalScroll_t__bindgen_ty_1 {
    k_iCallback = 4511,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_HorizontalScroll_t",
    ][::std::mem::size_of::<HTML_HorizontalScroll_t>() - 24usize];
    [
        "Alignment of HTML_HorizontalScroll_t",
    ][::std::mem::align_of::<HTML_HorizontalScroll_t>() - 4usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::unScrollMax",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, unScrollMax) - 4usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::unScrollCurrent",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, unScrollCurrent) - 8usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::flPageScale",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, flPageScale) - 12usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::bVisible",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, bVisible) - 16usize];
    [
        "Offset of field: HTML_HorizontalScroll_t::unPageSize",
    ][::std::mem::offset_of!(HTML_HorizontalScroll_t, unPageSize) - 20usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_VerticalScroll_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub unScrollMax: uint32,
    pub unScrollCurrent: uint32,
    pub flPageScale: f32,
    pub bVisible: bool,
    pub unPageSize: uint32,
}
pub const HTML_VerticalScroll_t_k_iCallback: HTML_VerticalScroll_t__bindgen_ty_1 = HTML_VerticalScroll_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_VerticalScroll_t__bindgen_ty_1 {
    k_iCallback = 4512,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_VerticalScroll_t",
    ][::std::mem::size_of::<HTML_VerticalScroll_t>() - 24usize];
    [
        "Alignment of HTML_VerticalScroll_t",
    ][::std::mem::align_of::<HTML_VerticalScroll_t>() - 4usize];
    [
        "Offset of field: HTML_VerticalScroll_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_VerticalScroll_t::unScrollMax",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, unScrollMax) - 4usize];
    [
        "Offset of field: HTML_VerticalScroll_t::unScrollCurrent",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, unScrollCurrent) - 8usize];
    [
        "Offset of field: HTML_VerticalScroll_t::flPageScale",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, flPageScale) - 12usize];
    [
        "Offset of field: HTML_VerticalScroll_t::bVisible",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, bVisible) - 16usize];
    [
        "Offset of field: HTML_VerticalScroll_t::unPageSize",
    ][::std::mem::offset_of!(HTML_VerticalScroll_t, unPageSize) - 20usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_LinkAtPosition_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub x: uint32,
    pub y: uint32,
    pub pchURL: *const ::std::os::raw::c_char,
    pub bInput: bool,
    pub bLiveLink: bool,
}
pub const HTML_LinkAtPosition_t_k_iCallback: HTML_LinkAtPosition_t__bindgen_ty_1 = HTML_LinkAtPosition_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_LinkAtPosition_t__bindgen_ty_1 {
    k_iCallback = 4513,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_LinkAtPosition_t",
    ][::std::mem::size_of::<HTML_LinkAtPosition_t>() - 24usize];
    [
        "Alignment of HTML_LinkAtPosition_t",
    ][::std::mem::align_of::<HTML_LinkAtPosition_t>() - 4usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::x",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, x) - 4usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::y",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, y) - 8usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::pchURL",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, pchURL) - 12usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::bInput",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, bInput) - 20usize];
    [
        "Offset of field: HTML_LinkAtPosition_t::bLiveLink",
    ][::std::mem::offset_of!(HTML_LinkAtPosition_t, bLiveLink) - 21usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_JSAlert_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchMessage: *const ::std::os::raw::c_char,
}
pub const HTML_JSAlert_t_k_iCallback: HTML_JSAlert_t__bindgen_ty_1 = HTML_JSAlert_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_JSAlert_t__bindgen_ty_1 {
    k_iCallback = 4514,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_JSAlert_t"][::std::mem::size_of::<HTML_JSAlert_t>() - 12usize];
    ["Alignment of HTML_JSAlert_t"][::std::mem::align_of::<HTML_JSAlert_t>() - 4usize];
    [
        "Offset of field: HTML_JSAlert_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_JSAlert_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_JSAlert_t::pchMessage",
    ][::std::mem::offset_of!(HTML_JSAlert_t, pchMessage) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_JSConfirm_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchMessage: *const ::std::os::raw::c_char,
}
pub const HTML_JSConfirm_t_k_iCallback: HTML_JSConfirm_t__bindgen_ty_1 = HTML_JSConfirm_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_JSConfirm_t__bindgen_ty_1 {
    k_iCallback = 4515,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_JSConfirm_t"][::std::mem::size_of::<HTML_JSConfirm_t>() - 12usize];
    [
        "Alignment of HTML_JSConfirm_t",
    ][::std::mem::align_of::<HTML_JSConfirm_t>() - 4usize];
    [
        "Offset of field: HTML_JSConfirm_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_JSConfirm_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_JSConfirm_t::pchMessage",
    ][::std::mem::offset_of!(HTML_JSConfirm_t, pchMessage) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_FileOpenDialog_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchTitle: *const ::std::os::raw::c_char,
    pub pchInitialFile: *const ::std::os::raw::c_char,
}
pub const HTML_FileOpenDialog_t_k_iCallback: HTML_FileOpenDialog_t__bindgen_ty_1 = HTML_FileOpenDialog_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_FileOpenDialog_t__bindgen_ty_1 {
    k_iCallback = 4516,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_FileOpenDialog_t",
    ][::std::mem::size_of::<HTML_FileOpenDialog_t>() - 20usize];
    [
        "Alignment of HTML_FileOpenDialog_t",
    ][::std::mem::align_of::<HTML_FileOpenDialog_t>() - 4usize];
    [
        "Offset of field: HTML_FileOpenDialog_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_FileOpenDialog_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_FileOpenDialog_t::pchTitle",
    ][::std::mem::offset_of!(HTML_FileOpenDialog_t, pchTitle) - 4usize];
    [
        "Offset of field: HTML_FileOpenDialog_t::pchInitialFile",
    ][::std::mem::offset_of!(HTML_FileOpenDialog_t, pchInitialFile) - 12usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_NewWindow_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchURL: *const ::std::os::raw::c_char,
    pub unX: uint32,
    pub unY: uint32,
    pub unWide: uint32,
    pub unTall: uint32,
    pub unNewWindow_BrowserHandle_IGNORE: HHTMLBrowser,
}
pub const HTML_NewWindow_t_k_iCallback: HTML_NewWindow_t__bindgen_ty_1 = HTML_NewWindow_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_NewWindow_t__bindgen_ty_1 {
    k_iCallback = 4521,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_NewWindow_t"][::std::mem::size_of::<HTML_NewWindow_t>() - 32usize];
    [
        "Alignment of HTML_NewWindow_t",
    ][::std::mem::align_of::<HTML_NewWindow_t>() - 4usize];
    [
        "Offset of field: HTML_NewWindow_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_NewWindow_t::pchURL",
    ][::std::mem::offset_of!(HTML_NewWindow_t, pchURL) - 4usize];
    [
        "Offset of field: HTML_NewWindow_t::unX",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unX) - 12usize];
    [
        "Offset of field: HTML_NewWindow_t::unY",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unY) - 16usize];
    [
        "Offset of field: HTML_NewWindow_t::unWide",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unWide) - 20usize];
    [
        "Offset of field: HTML_NewWindow_t::unTall",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unTall) - 24usize];
    [
        "Offset of field: HTML_NewWindow_t::unNewWindow_BrowserHandle_IGNORE",
    ][::std::mem::offset_of!(HTML_NewWindow_t, unNewWindow_BrowserHandle_IGNORE)
        - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_SetCursor_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub eMouseCursor: uint32,
}
pub const HTML_SetCursor_t_k_iCallback: HTML_SetCursor_t__bindgen_ty_1 = HTML_SetCursor_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_SetCursor_t__bindgen_ty_1 {
    k_iCallback = 4522,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_SetCursor_t"][::std::mem::size_of::<HTML_SetCursor_t>() - 8usize];
    [
        "Alignment of HTML_SetCursor_t",
    ][::std::mem::align_of::<HTML_SetCursor_t>() - 4usize];
    [
        "Offset of field: HTML_SetCursor_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_SetCursor_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_SetCursor_t::eMouseCursor",
    ][::std::mem::offset_of!(HTML_SetCursor_t, eMouseCursor) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_StatusText_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchMsg: *const ::std::os::raw::c_char,
}
pub const HTML_StatusText_t_k_iCallback: HTML_StatusText_t__bindgen_ty_1 = HTML_StatusText_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_StatusText_t__bindgen_ty_1 {
    k_iCallback = 4523,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_StatusText_t"][::std::mem::size_of::<HTML_StatusText_t>() - 12usize];
    [
        "Alignment of HTML_StatusText_t",
    ][::std::mem::align_of::<HTML_StatusText_t>() - 4usize];
    [
        "Offset of field: HTML_StatusText_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_StatusText_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_StatusText_t::pchMsg",
    ][::std::mem::offset_of!(HTML_StatusText_t, pchMsg) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_ShowToolTip_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchMsg: *const ::std::os::raw::c_char,
}
pub const HTML_ShowToolTip_t_k_iCallback: HTML_ShowToolTip_t__bindgen_ty_1 = HTML_ShowToolTip_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_ShowToolTip_t__bindgen_ty_1 {
    k_iCallback = 4524,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_ShowToolTip_t",
    ][::std::mem::size_of::<HTML_ShowToolTip_t>() - 12usize];
    [
        "Alignment of HTML_ShowToolTip_t",
    ][::std::mem::align_of::<HTML_ShowToolTip_t>() - 4usize];
    [
        "Offset of field: HTML_ShowToolTip_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_ShowToolTip_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_ShowToolTip_t::pchMsg",
    ][::std::mem::offset_of!(HTML_ShowToolTip_t, pchMsg) - 4usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct HTML_UpdateToolTip_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub pchMsg: *const ::std::os::raw::c_char,
}
pub const HTML_UpdateToolTip_t_k_iCallback: HTML_UpdateToolTip_t__bindgen_ty_1 = HTML_UpdateToolTip_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_UpdateToolTip_t__bindgen_ty_1 {
    k_iCallback = 4525,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_UpdateToolTip_t",
    ][::std::mem::size_of::<HTML_UpdateToolTip_t>() - 12usize];
    [
        "Alignment of HTML_UpdateToolTip_t",
    ][::std::mem::align_of::<HTML_UpdateToolTip_t>() - 4usize];
    [
        "Offset of field: HTML_UpdateToolTip_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_UpdateToolTip_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_UpdateToolTip_t::pchMsg",
    ][::std::mem::offset_of!(HTML_UpdateToolTip_t, pchMsg) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_HideToolTip_t {
    pub unBrowserHandle: HHTMLBrowser,
}
pub const HTML_HideToolTip_t_k_iCallback: HTML_HideToolTip_t__bindgen_ty_1 = HTML_HideToolTip_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_HideToolTip_t__bindgen_ty_1 {
    k_iCallback = 4526,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of HTML_HideToolTip_t"][::std::mem::size_of::<HTML_HideToolTip_t>() - 4usize];
    [
        "Alignment of HTML_HideToolTip_t",
    ][::std::mem::align_of::<HTML_HideToolTip_t>() - 4usize];
    [
        "Offset of field: HTML_HideToolTip_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_HideToolTip_t, unBrowserHandle) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct HTML_BrowserRestarted_t {
    pub unBrowserHandle: HHTMLBrowser,
    pub unOldBrowserHandle: HHTMLBrowser,
}
pub const HTML_BrowserRestarted_t_k_iCallback: HTML_BrowserRestarted_t__bindgen_ty_1 = HTML_BrowserRestarted_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum HTML_BrowserRestarted_t__bindgen_ty_1 {
    k_iCallback = 4527,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of HTML_BrowserRestarted_t",
    ][::std::mem::size_of::<HTML_BrowserRestarted_t>() - 8usize];
    [
        "Alignment of HTML_BrowserRestarted_t",
    ][::std::mem::align_of::<HTML_BrowserRestarted_t>() - 4usize];
    [
        "Offset of field: HTML_BrowserRestarted_t::unBrowserHandle",
    ][::std::mem::offset_of!(HTML_BrowserRestarted_t, unBrowserHandle) - 0usize];
    [
        "Offset of field: HTML_BrowserRestarted_t::unOldBrowserHandle",
    ][::std::mem::offset_of!(HTML_BrowserRestarted_t, unOldBrowserHandle) - 4usize];
};
pub type SteamItemInstanceID_t = uint64;
unsafe extern "C" {
    #[link_name = "\u{1}_ZL28k_SteamItemInstanceIDInvalid"]
    pub static k_SteamItemInstanceIDInvalid: SteamItemInstanceID_t;
}
pub type SteamItemDef_t = int32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamItemFlags {
    k_ESteamItemNoTrade = 1,
    k_ESteamItemRemoved = 256,
    k_ESteamItemConsumed = 512,
}
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamItemDetails_t {
    pub m_itemId: SteamItemInstanceID_t,
    pub m_iDefinition: SteamItemDef_t,
    pub m_unQuantity: uint16,
    pub m_unFlags: uint16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamItemDetails_t",
    ][::std::mem::size_of::<SteamItemDetails_t>() - 16usize];
    [
        "Alignment of SteamItemDetails_t",
    ][::std::mem::align_of::<SteamItemDetails_t>() - 4usize];
    [
        "Offset of field: SteamItemDetails_t::m_itemId",
    ][::std::mem::offset_of!(SteamItemDetails_t, m_itemId) - 0usize];
    [
        "Offset of field: SteamItemDetails_t::m_iDefinition",
    ][::std::mem::offset_of!(SteamItemDetails_t, m_iDefinition) - 8usize];
    [
        "Offset of field: SteamItemDetails_t::m_unQuantity",
    ][::std::mem::offset_of!(SteamItemDetails_t, m_unQuantity) - 12usize];
    [
        "Offset of field: SteamItemDetails_t::m_unFlags",
    ][::std::mem::offset_of!(SteamItemDetails_t, m_unFlags) - 14usize];
};
pub type SteamInventoryResult_t = int32;
pub const k_SteamInventoryResultInvalid: SteamInventoryResult_t = -1;
pub type SteamInventoryUpdateHandle_t = uint64;
pub const k_SteamInventoryUpdateHandleInvalid: SteamInventoryUpdateHandle_t = 18446744073709551615;
#[repr(C)]
pub struct ISteamInventory__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamInventory {
    pub vtable_: *const ISteamInventory__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamInventory"][::std::mem::size_of::<ISteamInventory>() - 8usize];
    ["Alignment of ISteamInventory"][::std::mem::align_of::<ISteamInventory>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInventoryResultReady_t {
    pub m_handle: SteamInventoryResult_t,
    pub m_result: EResult,
}
pub const SteamInventoryResultReady_t_k_iCallback: SteamInventoryResultReady_t__bindgen_ty_1 = SteamInventoryResultReady_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryResultReady_t__bindgen_ty_1 {
    k_iCallback = 4700,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryResultReady_t",
    ][::std::mem::size_of::<SteamInventoryResultReady_t>() - 8usize];
    [
        "Alignment of SteamInventoryResultReady_t",
    ][::std::mem::align_of::<SteamInventoryResultReady_t>() - 4usize];
    [
        "Offset of field: SteamInventoryResultReady_t::m_handle",
    ][::std::mem::offset_of!(SteamInventoryResultReady_t, m_handle) - 0usize];
    [
        "Offset of field: SteamInventoryResultReady_t::m_result",
    ][::std::mem::offset_of!(SteamInventoryResultReady_t, m_result) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInventoryFullUpdate_t {
    pub m_handle: SteamInventoryResult_t,
}
pub const SteamInventoryFullUpdate_t_k_iCallback: SteamInventoryFullUpdate_t__bindgen_ty_1 = SteamInventoryFullUpdate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryFullUpdate_t__bindgen_ty_1 {
    k_iCallback = 4701,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryFullUpdate_t",
    ][::std::mem::size_of::<SteamInventoryFullUpdate_t>() - 4usize];
    [
        "Alignment of SteamInventoryFullUpdate_t",
    ][::std::mem::align_of::<SteamInventoryFullUpdate_t>() - 4usize];
    [
        "Offset of field: SteamInventoryFullUpdate_t::m_handle",
    ][::std::mem::offset_of!(SteamInventoryFullUpdate_t, m_handle) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInventoryDefinitionUpdate_t {
    pub _address: u8,
}
pub const SteamInventoryDefinitionUpdate_t_k_iCallback: SteamInventoryDefinitionUpdate_t__bindgen_ty_1 = SteamInventoryDefinitionUpdate_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryDefinitionUpdate_t__bindgen_ty_1 {
    k_iCallback = 4702,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryDefinitionUpdate_t",
    ][::std::mem::size_of::<SteamInventoryDefinitionUpdate_t>() - 1usize];
    [
        "Alignment of SteamInventoryDefinitionUpdate_t",
    ][::std::mem::align_of::<SteamInventoryDefinitionUpdate_t>() - 1usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SteamInventoryEligiblePromoItemDefIDs_t {
    pub m_result: EResult,
    pub m_steamID: CSteamID,
    pub m_numEligiblePromoItemDefs: ::std::os::raw::c_int,
    pub m_bCachedData: bool,
}
pub const SteamInventoryEligiblePromoItemDefIDs_t_k_iCallback: SteamInventoryEligiblePromoItemDefIDs_t__bindgen_ty_1 = SteamInventoryEligiblePromoItemDefIDs_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryEligiblePromoItemDefIDs_t__bindgen_ty_1 {
    k_iCallback = 4703,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryEligiblePromoItemDefIDs_t",
    ][::std::mem::size_of::<SteamInventoryEligiblePromoItemDefIDs_t>() - 20usize];
    [
        "Alignment of SteamInventoryEligiblePromoItemDefIDs_t",
    ][::std::mem::align_of::<SteamInventoryEligiblePromoItemDefIDs_t>() - 4usize];
    [
        "Offset of field: SteamInventoryEligiblePromoItemDefIDs_t::m_result",
    ][::std::mem::offset_of!(SteamInventoryEligiblePromoItemDefIDs_t, m_result)
        - 0usize];
    [
        "Offset of field: SteamInventoryEligiblePromoItemDefIDs_t::m_steamID",
    ][::std::mem::offset_of!(SteamInventoryEligiblePromoItemDefIDs_t, m_steamID)
        - 4usize];
    [
        "Offset of field: SteamInventoryEligiblePromoItemDefIDs_t::m_numEligiblePromoItemDefs",
    ][::std::mem::offset_of!(
        SteamInventoryEligiblePromoItemDefIDs_t, m_numEligiblePromoItemDefs
    ) - 12usize];
    [
        "Offset of field: SteamInventoryEligiblePromoItemDefIDs_t::m_bCachedData",
    ][::std::mem::offset_of!(SteamInventoryEligiblePromoItemDefIDs_t, m_bCachedData)
        - 16usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamInventoryStartPurchaseResult_t {
    pub m_result: EResult,
    pub m_ulOrderID: uint64,
    pub m_ulTransID: uint64,
}
pub const SteamInventoryStartPurchaseResult_t_k_iCallback: SteamInventoryStartPurchaseResult_t__bindgen_ty_1 = SteamInventoryStartPurchaseResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryStartPurchaseResult_t__bindgen_ty_1 {
    k_iCallback = 4704,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryStartPurchaseResult_t",
    ][::std::mem::size_of::<SteamInventoryStartPurchaseResult_t>() - 20usize];
    [
        "Alignment of SteamInventoryStartPurchaseResult_t",
    ][::std::mem::align_of::<SteamInventoryStartPurchaseResult_t>() - 4usize];
    [
        "Offset of field: SteamInventoryStartPurchaseResult_t::m_result",
    ][::std::mem::offset_of!(SteamInventoryStartPurchaseResult_t, m_result) - 0usize];
    [
        "Offset of field: SteamInventoryStartPurchaseResult_t::m_ulOrderID",
    ][::std::mem::offset_of!(SteamInventoryStartPurchaseResult_t, m_ulOrderID) - 4usize];
    [
        "Offset of field: SteamInventoryStartPurchaseResult_t::m_ulTransID",
    ][::std::mem::offset_of!(SteamInventoryStartPurchaseResult_t, m_ulTransID)
        - 12usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamInventoryRequestPricesResult_t {
    pub m_result: EResult,
    pub m_rgchCurrency: [::std::os::raw::c_char; 4usize],
}
pub const SteamInventoryRequestPricesResult_t_k_iCallback: SteamInventoryRequestPricesResult_t__bindgen_ty_1 = SteamInventoryRequestPricesResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamInventoryRequestPricesResult_t__bindgen_ty_1 {
    k_iCallback = 4705,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamInventoryRequestPricesResult_t",
    ][::std::mem::size_of::<SteamInventoryRequestPricesResult_t>() - 8usize];
    [
        "Alignment of SteamInventoryRequestPricesResult_t",
    ][::std::mem::align_of::<SteamInventoryRequestPricesResult_t>() - 4usize];
    [
        "Offset of field: SteamInventoryRequestPricesResult_t::m_result",
    ][::std::mem::offset_of!(SteamInventoryRequestPricesResult_t, m_result) - 0usize];
    [
        "Offset of field: SteamInventoryRequestPricesResult_t::m_rgchCurrency",
    ][::std::mem::offset_of!(SteamInventoryRequestPricesResult_t, m_rgchCurrency)
        - 4usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ETimelineGameMode {
    k_ETimelineGameMode_Invalid = 0,
    k_ETimelineGameMode_Playing = 1,
    k_ETimelineGameMode_Staging = 2,
    k_ETimelineGameMode_Menus = 3,
    k_ETimelineGameMode_LoadingScreen = 4,
    k_ETimelineGameMode_Max = 5,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ETimelineEventClipPriority {
    k_ETimelineEventClipPriority_Invalid = 0,
    k_ETimelineEventClipPriority_None = 1,
    k_ETimelineEventClipPriority_Standard = 2,
    k_ETimelineEventClipPriority_Featured = 3,
}
pub const k_unMaxTimelinePriority: uint32 = 1000;
pub const k_unTimelinePriority_KeepCurrentValue: uint32 = 1000000;
pub const k_flMaxTimelineEventDuration: f32 = 600.0;
pub const k_cchMaxPhaseIDLength: uint32 = 64;
pub type TimelineEventHandle_t = uint64;
#[repr(C)]
pub struct ISteamTimeline__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamTimeline {
    pub vtable_: *const ISteamTimeline__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamTimeline"][::std::mem::size_of::<ISteamTimeline>() - 8usize];
    ["Alignment of ISteamTimeline"][::std::mem::align_of::<ISteamTimeline>() - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamTimelineGamePhaseRecordingExists_t {
    pub m_rgchPhaseID: [::std::os::raw::c_char; 64usize],
    pub m_ulRecordingMS: uint64,
    pub m_ulLongestClipMS: uint64,
    pub m_unClipCount: uint32,
    pub m_unScreenshotCount: uint32,
}
pub const SteamTimelineGamePhaseRecordingExists_t_k_iCallback: SteamTimelineGamePhaseRecordingExists_t__bindgen_ty_1 = SteamTimelineGamePhaseRecordingExists_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamTimelineGamePhaseRecordingExists_t__bindgen_ty_1 {
    k_iCallback = 6001,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamTimelineGamePhaseRecordingExists_t",
    ][::std::mem::size_of::<SteamTimelineGamePhaseRecordingExists_t>() - 88usize];
    [
        "Alignment of SteamTimelineGamePhaseRecordingExists_t",
    ][::std::mem::align_of::<SteamTimelineGamePhaseRecordingExists_t>() - 4usize];
    [
        "Offset of field: SteamTimelineGamePhaseRecordingExists_t::m_rgchPhaseID",
    ][::std::mem::offset_of!(SteamTimelineGamePhaseRecordingExists_t, m_rgchPhaseID)
        - 0usize];
    [
        "Offset of field: SteamTimelineGamePhaseRecordingExists_t::m_ulRecordingMS",
    ][::std::mem::offset_of!(SteamTimelineGamePhaseRecordingExists_t, m_ulRecordingMS)
        - 64usize];
    [
        "Offset of field: SteamTimelineGamePhaseRecordingExists_t::m_ulLongestClipMS",
    ][::std::mem::offset_of!(SteamTimelineGamePhaseRecordingExists_t, m_ulLongestClipMS)
        - 72usize];
    [
        "Offset of field: SteamTimelineGamePhaseRecordingExists_t::m_unClipCount",
    ][::std::mem::offset_of!(SteamTimelineGamePhaseRecordingExists_t, m_unClipCount)
        - 80usize];
    [
        "Offset of field: SteamTimelineGamePhaseRecordingExists_t::m_unScreenshotCount",
    ][::std::mem::offset_of!(
        SteamTimelineGamePhaseRecordingExists_t, m_unScreenshotCount
    ) - 84usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamTimelineEventRecordingExists_t {
    pub m_ulEventID: uint64,
    pub m_bRecordingExists: bool,
}
pub const SteamTimelineEventRecordingExists_t_k_iCallback: SteamTimelineEventRecordingExists_t__bindgen_ty_1 = SteamTimelineEventRecordingExists_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamTimelineEventRecordingExists_t__bindgen_ty_1 {
    k_iCallback = 6002,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamTimelineEventRecordingExists_t",
    ][::std::mem::size_of::<SteamTimelineEventRecordingExists_t>() - 12usize];
    [
        "Alignment of SteamTimelineEventRecordingExists_t",
    ][::std::mem::align_of::<SteamTimelineEventRecordingExists_t>() - 4usize];
    [
        "Offset of field: SteamTimelineEventRecordingExists_t::m_ulEventID",
    ][::std::mem::offset_of!(SteamTimelineEventRecordingExists_t, m_ulEventID) - 0usize];
    [
        "Offset of field: SteamTimelineEventRecordingExists_t::m_bRecordingExists",
    ][::std::mem::offset_of!(SteamTimelineEventRecordingExists_t, m_bRecordingExists)
        - 8usize];
};
#[repr(C)]
pub struct ISteamVideo__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamVideo {
    pub vtable_: *const ISteamVideo__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamVideo"][::std::mem::size_of::<ISteamVideo>() - 8usize];
    ["Alignment of ISteamVideo"][::std::mem::align_of::<ISteamVideo>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GetVideoURLResult_t {
    pub m_eResult: EResult,
    pub m_unVideoAppID: AppId_t,
    pub m_rgchURL: [::std::os::raw::c_char; 256usize],
}
pub const GetVideoURLResult_t_k_iCallback: GetVideoURLResult_t__bindgen_ty_1 = GetVideoURLResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetVideoURLResult_t__bindgen_ty_1 {
    k_iCallback = 4611,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetVideoURLResult_t",
    ][::std::mem::size_of::<GetVideoURLResult_t>() - 264usize];
    [
        "Alignment of GetVideoURLResult_t",
    ][::std::mem::align_of::<GetVideoURLResult_t>() - 4usize];
    [
        "Offset of field: GetVideoURLResult_t::m_eResult",
    ][::std::mem::offset_of!(GetVideoURLResult_t, m_eResult) - 0usize];
    [
        "Offset of field: GetVideoURLResult_t::m_unVideoAppID",
    ][::std::mem::offset_of!(GetVideoURLResult_t, m_unVideoAppID) - 4usize];
    [
        "Offset of field: GetVideoURLResult_t::m_rgchURL",
    ][::std::mem::offset_of!(GetVideoURLResult_t, m_rgchURL) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GetOPFSettingsResult_t {
    pub m_eResult: EResult,
    pub m_unVideoAppID: AppId_t,
}
pub const GetOPFSettingsResult_t_k_iCallback: GetOPFSettingsResult_t__bindgen_ty_1 = GetOPFSettingsResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GetOPFSettingsResult_t__bindgen_ty_1 {
    k_iCallback = 4624,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GetOPFSettingsResult_t",
    ][::std::mem::size_of::<GetOPFSettingsResult_t>() - 8usize];
    [
        "Alignment of GetOPFSettingsResult_t",
    ][::std::mem::align_of::<GetOPFSettingsResult_t>() - 4usize];
    [
        "Offset of field: GetOPFSettingsResult_t::m_eResult",
    ][::std::mem::offset_of!(GetOPFSettingsResult_t, m_eResult) - 0usize];
    [
        "Offset of field: GetOPFSettingsResult_t::m_unVideoAppID",
    ][::std::mem::offset_of!(GetOPFSettingsResult_t, m_unVideoAppID) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BroadcastUploadStart_t {
    pub m_bIsRTMP: bool,
}
pub const BroadcastUploadStart_t_k_iCallback: BroadcastUploadStart_t__bindgen_ty_1 = BroadcastUploadStart_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BroadcastUploadStart_t__bindgen_ty_1 {
    k_iCallback = 4604,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of BroadcastUploadStart_t",
    ][::std::mem::size_of::<BroadcastUploadStart_t>() - 1usize];
    [
        "Alignment of BroadcastUploadStart_t",
    ][::std::mem::align_of::<BroadcastUploadStart_t>() - 1usize];
    [
        "Offset of field: BroadcastUploadStart_t::m_bIsRTMP",
    ][::std::mem::offset_of!(BroadcastUploadStart_t, m_bIsRTMP) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BroadcastUploadStop_t {
    pub m_eResult: EBroadcastUploadResult,
}
pub const BroadcastUploadStop_t_k_iCallback: BroadcastUploadStop_t__bindgen_ty_1 = BroadcastUploadStop_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum BroadcastUploadStop_t__bindgen_ty_1 {
    k_iCallback = 4605,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of BroadcastUploadStop_t",
    ][::std::mem::size_of::<BroadcastUploadStop_t>() - 4usize];
    [
        "Alignment of BroadcastUploadStop_t",
    ][::std::mem::align_of::<BroadcastUploadStop_t>() - 4usize];
    [
        "Offset of field: BroadcastUploadStop_t::m_eResult",
    ][::std::mem::offset_of!(BroadcastUploadStop_t, m_eResult) - 0usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EParentalFeature {
    k_EFeatureInvalid = 0,
    k_EFeatureStore = 1,
    k_EFeatureCommunity = 2,
    k_EFeatureProfile = 3,
    k_EFeatureFriends = 4,
    k_EFeatureNews = 5,
    k_EFeatureTrading = 6,
    k_EFeatureSettings = 7,
    k_EFeatureConsole = 8,
    k_EFeatureBrowser = 9,
    k_EFeatureParentalSetup = 10,
    k_EFeatureLibrary = 11,
    k_EFeatureTest = 12,
    k_EFeatureSiteLicense = 13,
    k_EFeatureKioskMode_Deprecated = 14,
    k_EFeatureBlockAlways = 15,
    k_EFeatureMax = 16,
}
#[repr(C)]
pub struct ISteamParentalSettings__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamParentalSettings {
    pub vtable_: *const ISteamParentalSettings__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamParentalSettings",
    ][::std::mem::size_of::<ISteamParentalSettings>() - 8usize];
    [
        "Alignment of ISteamParentalSettings",
    ][::std::mem::align_of::<ISteamParentalSettings>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamParentalSettingsChanged_t {
    pub _address: u8,
}
pub const SteamParentalSettingsChanged_t_k_iCallback: SteamParentalSettingsChanged_t__bindgen_ty_1 = SteamParentalSettingsChanged_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamParentalSettingsChanged_t__bindgen_ty_1 {
    k_iCallback = 5001,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamParentalSettingsChanged_t",
    ][::std::mem::size_of::<SteamParentalSettingsChanged_t>() - 1usize];
    [
        "Alignment of SteamParentalSettingsChanged_t",
    ][::std::mem::align_of::<SteamParentalSettingsChanged_t>() - 1usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamDeviceFormFactor {
    k_ESteamDeviceFormFactorUnknown = 0,
    k_ESteamDeviceFormFactorPhone = 1,
    k_ESteamDeviceFormFactorTablet = 2,
    k_ESteamDeviceFormFactorComputer = 3,
    k_ESteamDeviceFormFactorTV = 4,
    k_ESteamDeviceFormFactorVRHeadset = 5,
}
pub type RemotePlaySessionID_t = uint32;
pub type RemotePlayCursorID_t = uint32;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemotePlayInputType {
    k_ERemotePlayInputUnknown = 0,
    k_ERemotePlayInputMouseMotion = 1,
    k_ERemotePlayInputMouseButtonDown = 2,
    k_ERemotePlayInputMouseButtonUp = 3,
    k_ERemotePlayInputMouseWheel = 4,
    k_ERemotePlayInputKeyDown = 5,
    k_ERemotePlayInputKeyUp = 6,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemotePlayMouseButton {
    k_ERemotePlayMouseButtonLeft = 1,
    k_ERemotePlayMouseButtonRight = 2,
    k_ERemotePlayMouseButtonMiddle = 16,
    k_ERemotePlayMouseButtonX1 = 32,
    k_ERemotePlayMouseButtonX2 = 64,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemotePlayMouseWheelDirection {
    k_ERemotePlayMouseWheelUp = 1,
    k_ERemotePlayMouseWheelDown = 2,
    k_ERemotePlayMouseWheelLeft = 3,
    k_ERemotePlayMouseWheelRight = 4,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemotePlayScancode {
    k_ERemotePlayScancodeUnknown = 0,
    k_ERemotePlayScancodeA = 4,
    k_ERemotePlayScancodeB = 5,
    k_ERemotePlayScancodeC = 6,
    k_ERemotePlayScancodeD = 7,
    k_ERemotePlayScancodeE = 8,
    k_ERemotePlayScancodeF = 9,
    k_ERemotePlayScancodeG = 10,
    k_ERemotePlayScancodeH = 11,
    k_ERemotePlayScancodeI = 12,
    k_ERemotePlayScancodeJ = 13,
    k_ERemotePlayScancodeK = 14,
    k_ERemotePlayScancodeL = 15,
    k_ERemotePlayScancodeM = 16,
    k_ERemotePlayScancodeN = 17,
    k_ERemotePlayScancodeO = 18,
    k_ERemotePlayScancodeP = 19,
    k_ERemotePlayScancodeQ = 20,
    k_ERemotePlayScancodeR = 21,
    k_ERemotePlayScancodeS = 22,
    k_ERemotePlayScancodeT = 23,
    k_ERemotePlayScancodeU = 24,
    k_ERemotePlayScancodeV = 25,
    k_ERemotePlayScancodeW = 26,
    k_ERemotePlayScancodeX = 27,
    k_ERemotePlayScancodeY = 28,
    k_ERemotePlayScancodeZ = 29,
    k_ERemotePlayScancode1 = 30,
    k_ERemotePlayScancode2 = 31,
    k_ERemotePlayScancode3 = 32,
    k_ERemotePlayScancode4 = 33,
    k_ERemotePlayScancode5 = 34,
    k_ERemotePlayScancode6 = 35,
    k_ERemotePlayScancode7 = 36,
    k_ERemotePlayScancode8 = 37,
    k_ERemotePlayScancode9 = 38,
    k_ERemotePlayScancode0 = 39,
    k_ERemotePlayScancodeReturn = 40,
    k_ERemotePlayScancodeEscape = 41,
    k_ERemotePlayScancodeBackspace = 42,
    k_ERemotePlayScancodeTab = 43,
    k_ERemotePlayScancodeSpace = 44,
    k_ERemotePlayScancodeMinus = 45,
    k_ERemotePlayScancodeEquals = 46,
    k_ERemotePlayScancodeLeftBracket = 47,
    k_ERemotePlayScancodeRightBracket = 48,
    k_ERemotePlayScancodeBackslash = 49,
    k_ERemotePlayScancodeSemicolon = 51,
    k_ERemotePlayScancodeApostrophe = 52,
    k_ERemotePlayScancodeGrave = 53,
    k_ERemotePlayScancodeComma = 54,
    k_ERemotePlayScancodePeriod = 55,
    k_ERemotePlayScancodeSlash = 56,
    k_ERemotePlayScancodeCapsLock = 57,
    k_ERemotePlayScancodeF1 = 58,
    k_ERemotePlayScancodeF2 = 59,
    k_ERemotePlayScancodeF3 = 60,
    k_ERemotePlayScancodeF4 = 61,
    k_ERemotePlayScancodeF5 = 62,
    k_ERemotePlayScancodeF6 = 63,
    k_ERemotePlayScancodeF7 = 64,
    k_ERemotePlayScancodeF8 = 65,
    k_ERemotePlayScancodeF9 = 66,
    k_ERemotePlayScancodeF10 = 67,
    k_ERemotePlayScancodeF11 = 68,
    k_ERemotePlayScancodeF12 = 69,
    k_ERemotePlayScancodeInsert = 73,
    k_ERemotePlayScancodeHome = 74,
    k_ERemotePlayScancodePageUp = 75,
    k_ERemotePlayScancodeDelete = 76,
    k_ERemotePlayScancodeEnd = 77,
    k_ERemotePlayScancodePageDown = 78,
    k_ERemotePlayScancodeRight = 79,
    k_ERemotePlayScancodeLeft = 80,
    k_ERemotePlayScancodeDown = 81,
    k_ERemotePlayScancodeUp = 82,
    k_ERemotePlayScancodeLeftControl = 224,
    k_ERemotePlayScancodeLeftShift = 225,
    k_ERemotePlayScancodeLeftAlt = 226,
    k_ERemotePlayScancodeLeftGUI = 227,
    k_ERemotePlayScancodeRightControl = 228,
    k_ERemotePlayScancodeRightShift = 229,
    k_ERemotePlayScancodeRightALT = 230,
    k_ERemotePlayScancodeRightGUI = 231,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ERemotePlayKeyModifier {
    k_ERemotePlayKeyModifierNone = 0,
    k_ERemotePlayKeyModifierLeftShift = 1,
    k_ERemotePlayKeyModifierRightShift = 2,
    k_ERemotePlayKeyModifierLeftControl = 64,
    k_ERemotePlayKeyModifierRightControl = 128,
    k_ERemotePlayKeyModifierLeftAlt = 256,
    k_ERemotePlayKeyModifierRightAlt = 512,
    k_ERemotePlayKeyModifierLeftGUI = 1024,
    k_ERemotePlayKeyModifierRightGUI = 2048,
    k_ERemotePlayKeyModifierNumLock = 4096,
    k_ERemotePlayKeyModifierCapsLock = 8192,
    k_ERemotePlayKeyModifierMask = 65535,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RemotePlayInputMouseMotion_t {
    pub m_bAbsolute: bool,
    pub m_flNormalizedX: f32,
    pub m_flNormalizedY: f32,
    pub m_nDeltaX: ::std::os::raw::c_int,
    pub m_nDeltaY: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemotePlayInputMouseMotion_t",
    ][::std::mem::size_of::<RemotePlayInputMouseMotion_t>() - 20usize];
    [
        "Alignment of RemotePlayInputMouseMotion_t",
    ][::std::mem::align_of::<RemotePlayInputMouseMotion_t>() - 4usize];
    [
        "Offset of field: RemotePlayInputMouseMotion_t::m_bAbsolute",
    ][::std::mem::offset_of!(RemotePlayInputMouseMotion_t, m_bAbsolute) - 0usize];
    [
        "Offset of field: RemotePlayInputMouseMotion_t::m_flNormalizedX",
    ][::std::mem::offset_of!(RemotePlayInputMouseMotion_t, m_flNormalizedX) - 4usize];
    [
        "Offset of field: RemotePlayInputMouseMotion_t::m_flNormalizedY",
    ][::std::mem::offset_of!(RemotePlayInputMouseMotion_t, m_flNormalizedY) - 8usize];
    [
        "Offset of field: RemotePlayInputMouseMotion_t::m_nDeltaX",
    ][::std::mem::offset_of!(RemotePlayInputMouseMotion_t, m_nDeltaX) - 12usize];
    [
        "Offset of field: RemotePlayInputMouseMotion_t::m_nDeltaY",
    ][::std::mem::offset_of!(RemotePlayInputMouseMotion_t, m_nDeltaY) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RemotePlayInputMouseWheel_t {
    pub m_eDirection: ERemotePlayMouseWheelDirection,
    pub m_flAmount: f32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemotePlayInputMouseWheel_t",
    ][::std::mem::size_of::<RemotePlayInputMouseWheel_t>() - 8usize];
    [
        "Alignment of RemotePlayInputMouseWheel_t",
    ][::std::mem::align_of::<RemotePlayInputMouseWheel_t>() - 4usize];
    [
        "Offset of field: RemotePlayInputMouseWheel_t::m_eDirection",
    ][::std::mem::offset_of!(RemotePlayInputMouseWheel_t, m_eDirection) - 0usize];
    [
        "Offset of field: RemotePlayInputMouseWheel_t::m_flAmount",
    ][::std::mem::offset_of!(RemotePlayInputMouseWheel_t, m_flAmount) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RemotePlayInputKey_t {
    pub m_eScancode: ::std::os::raw::c_int,
    pub m_unModifiers: uint32,
    pub m_unKeycode: uint32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemotePlayInputKey_t",
    ][::std::mem::size_of::<RemotePlayInputKey_t>() - 12usize];
    [
        "Alignment of RemotePlayInputKey_t",
    ][::std::mem::align_of::<RemotePlayInputKey_t>() - 4usize];
    [
        "Offset of field: RemotePlayInputKey_t::m_eScancode",
    ][::std::mem::offset_of!(RemotePlayInputKey_t, m_eScancode) - 0usize];
    [
        "Offset of field: RemotePlayInputKey_t::m_unModifiers",
    ][::std::mem::offset_of!(RemotePlayInputKey_t, m_unModifiers) - 4usize];
    [
        "Offset of field: RemotePlayInputKey_t::m_unKeycode",
    ][::std::mem::offset_of!(RemotePlayInputKey_t, m_unKeycode) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RemotePlayInput_t {
    pub m_unSessionID: RemotePlaySessionID_t,
    pub m_eType: ERemotePlayInputType,
    pub __bindgen_anon_1: RemotePlayInput_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union RemotePlayInput_t__bindgen_ty_1 {
    pub m_MouseMotion: RemotePlayInputMouseMotion_t,
    pub m_eMouseButton: ERemotePlayMouseButton,
    pub m_MouseWheel: RemotePlayInputMouseWheel_t,
    pub m_Key: RemotePlayInputKey_t,
    pub padding: [::std::os::raw::c_char; 56usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of RemotePlayInput_t__bindgen_ty_1",
    ][::std::mem::size_of::<RemotePlayInput_t__bindgen_ty_1>() - 56usize];
    [
        "Alignment of RemotePlayInput_t__bindgen_ty_1",
    ][::std::mem::align_of::<RemotePlayInput_t__bindgen_ty_1>() - 4usize];
    [
        "Offset of field: RemotePlayInput_t__bindgen_ty_1::m_MouseMotion",
    ][::std::mem::offset_of!(RemotePlayInput_t__bindgen_ty_1, m_MouseMotion) - 0usize];
    [
        "Offset of field: RemotePlayInput_t__bindgen_ty_1::m_eMouseButton",
    ][::std::mem::offset_of!(RemotePlayInput_t__bindgen_ty_1, m_eMouseButton) - 0usize];
    [
        "Offset of field: RemotePlayInput_t__bindgen_ty_1::m_MouseWheel",
    ][::std::mem::offset_of!(RemotePlayInput_t__bindgen_ty_1, m_MouseWheel) - 0usize];
    [
        "Offset of field: RemotePlayInput_t__bindgen_ty_1::m_Key",
    ][::std::mem::offset_of!(RemotePlayInput_t__bindgen_ty_1, m_Key) - 0usize];
    [
        "Offset of field: RemotePlayInput_t__bindgen_ty_1::padding",
    ][::std::mem::offset_of!(RemotePlayInput_t__bindgen_ty_1, padding) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of RemotePlayInput_t"][::std::mem::size_of::<RemotePlayInput_t>() - 64usize];
    [
        "Alignment of RemotePlayInput_t",
    ][::std::mem::align_of::<RemotePlayInput_t>() - 4usize];
    [
        "Offset of field: RemotePlayInput_t::m_unSessionID",
    ][::std::mem::offset_of!(RemotePlayInput_t, m_unSessionID) - 0usize];
    [
        "Offset of field: RemotePlayInput_t::m_eType",
    ][::std::mem::offset_of!(RemotePlayInput_t, m_eType) - 4usize];
};
#[repr(C)]
pub struct ISteamRemotePlay__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamRemotePlay {
    pub vtable_: *const ISteamRemotePlay__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamRemotePlay"][::std::mem::size_of::<ISteamRemotePlay>() - 8usize];
    [
        "Alignment of ISteamRemotePlay",
    ][::std::mem::align_of::<ISteamRemotePlay>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamRemotePlaySessionConnected_t {
    pub m_unSessionID: RemotePlaySessionID_t,
}
pub const SteamRemotePlaySessionConnected_t_k_iCallback: SteamRemotePlaySessionConnected_t__bindgen_ty_1 = SteamRemotePlaySessionConnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamRemotePlaySessionConnected_t__bindgen_ty_1 {
    k_iCallback = 5701,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamRemotePlaySessionConnected_t",
    ][::std::mem::size_of::<SteamRemotePlaySessionConnected_t>() - 4usize];
    [
        "Alignment of SteamRemotePlaySessionConnected_t",
    ][::std::mem::align_of::<SteamRemotePlaySessionConnected_t>() - 4usize];
    [
        "Offset of field: SteamRemotePlaySessionConnected_t::m_unSessionID",
    ][::std::mem::offset_of!(SteamRemotePlaySessionConnected_t, m_unSessionID) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamRemotePlaySessionDisconnected_t {
    pub m_unSessionID: RemotePlaySessionID_t,
}
pub const SteamRemotePlaySessionDisconnected_t_k_iCallback: SteamRemotePlaySessionDisconnected_t__bindgen_ty_1 = SteamRemotePlaySessionDisconnected_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamRemotePlaySessionDisconnected_t__bindgen_ty_1 {
    k_iCallback = 5702,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamRemotePlaySessionDisconnected_t",
    ][::std::mem::size_of::<SteamRemotePlaySessionDisconnected_t>() - 4usize];
    [
        "Alignment of SteamRemotePlaySessionDisconnected_t",
    ][::std::mem::align_of::<SteamRemotePlaySessionDisconnected_t>() - 4usize];
    [
        "Offset of field: SteamRemotePlaySessionDisconnected_t::m_unSessionID",
    ][::std::mem::offset_of!(SteamRemotePlaySessionDisconnected_t, m_unSessionID)
        - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamRemotePlayTogetherGuestInvite_t {
    pub m_szConnectURL: [::std::os::raw::c_char; 1024usize],
}
pub const SteamRemotePlayTogetherGuestInvite_t_k_iCallback: SteamRemotePlayTogetherGuestInvite_t__bindgen_ty_1 = SteamRemotePlayTogetherGuestInvite_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamRemotePlayTogetherGuestInvite_t__bindgen_ty_1 {
    k_iCallback = 5703,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamRemotePlayTogetherGuestInvite_t",
    ][::std::mem::size_of::<SteamRemotePlayTogetherGuestInvite_t>() - 1024usize];
    [
        "Alignment of SteamRemotePlayTogetherGuestInvite_t",
    ][::std::mem::align_of::<SteamRemotePlayTogetherGuestInvite_t>() - 1usize];
    [
        "Offset of field: SteamRemotePlayTogetherGuestInvite_t::m_szConnectURL",
    ][::std::mem::offset_of!(SteamRemotePlayTogetherGuestInvite_t, m_szConnectURL)
        - 0usize];
};
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamDatagramRelayAuthTicket {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamDatagramHostedAddress {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamDatagramGameCoordinatorServerLogin {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkingFakeIPResult_t {
    _unused: [u8; 0],
}
pub type FnSteamNetConnectionStatusChanged = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamNetConnectionStatusChangedCallback_t),
>;
pub type FnSteamNetAuthenticationStatusChanged = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamNetAuthenticationStatus_t),
>;
pub type FnSteamRelayNetworkStatusChanged = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamRelayNetworkStatus_t),
>;
pub type FnSteamNetworkingMessagesSessionRequest = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamNetworkingMessagesSessionRequest_t),
>;
pub type FnSteamNetworkingMessagesSessionFailed = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamNetworkingMessagesSessionFailed_t),
>;
pub type FnSteamNetworkingFakeIPResult = ::std::option::Option<
    unsafe extern "C" fn(arg1: *mut SteamNetworkingFakeIPResult_t),
>;
pub type HSteamNetConnection = uint32;
pub const k_HSteamNetConnection_Invalid: HSteamNetConnection = 0;
pub type HSteamListenSocket = uint32;
pub const k_HSteamListenSocket_Invalid: HSteamListenSocket = 0;
pub type HSteamNetPollGroup = uint32;
pub const k_HSteamNetPollGroup_Invalid: HSteamNetPollGroup = 0;
pub const k_cchMaxSteamNetworkingErrMsg: ::std::os::raw::c_int = 1024;
pub type SteamNetworkingErrMsg = [::std::os::raw::c_char; 1024usize];
pub type SteamNetworkingPOPID = uint32;
pub type SteamNetworkingMicroseconds = int64;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingAvailability {
    k_ESteamNetworkingAvailability_CannotTry = -102,
    k_ESteamNetworkingAvailability_Failed = -101,
    k_ESteamNetworkingAvailability_Previously = -100,
    k_ESteamNetworkingAvailability_Retrying = -10,
    k_ESteamNetworkingAvailability_NeverTried = 1,
    k_ESteamNetworkingAvailability_Waiting = 2,
    k_ESteamNetworkingAvailability_Attempting = 3,
    k_ESteamNetworkingAvailability_Current = 100,
    k_ESteamNetworkingAvailability_Unknown = 0,
    k_ESteamNetworkingAvailability__Force32bit = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingIdentityType {
    k_ESteamNetworkingIdentityType_Invalid = 0,
    k_ESteamNetworkingIdentityType_SteamID = 16,
    k_ESteamNetworkingIdentityType_XboxPairwiseID = 17,
    k_ESteamNetworkingIdentityType_SonyPSN = 18,
    k_ESteamNetworkingIdentityType_IPAddress = 1,
    k_ESteamNetworkingIdentityType_GenericString = 2,
    k_ESteamNetworkingIdentityType_GenericBytes = 3,
    k_ESteamNetworkingIdentityType_UnknownType = 4,
    k_ESteamNetworkingIdentityType__Force32bit = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingFakeIPType {
    k_ESteamNetworkingFakeIPType_Invalid = 0,
    k_ESteamNetworkingFakeIPType_NotFake = 1,
    k_ESteamNetworkingFakeIPType_GlobalIPv4 = 2,
    k_ESteamNetworkingFakeIPType_LocalIPv4 = 3,
    k_ESteamNetworkingFakeIPType__Force32Bit = 2147483647,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SteamNetworkingIPAddr {
    pub __bindgen_anon_1: SteamNetworkingIPAddr__bindgen_ty_2,
    pub m_port: uint16,
}
pub const SteamNetworkingIPAddr_k_cchMaxString: SteamNetworkingIPAddr__bindgen_ty_1 = SteamNetworkingIPAddr__bindgen_ty_1::k_cchMaxString;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetworkingIPAddr__bindgen_ty_1 {
    k_cchMaxString = 48,
}
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkingIPAddr_IPv4MappedAddress {
    pub m_8zeros: uint64,
    pub m_0000: uint16,
    pub m_ffff: uint16,
    pub m_ip: [uint8; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIPAddr_IPv4MappedAddress",
    ][::std::mem::size_of::<SteamNetworkingIPAddr_IPv4MappedAddress>() - 16usize];
    [
        "Alignment of SteamNetworkingIPAddr_IPv4MappedAddress",
    ][::std::mem::align_of::<SteamNetworkingIPAddr_IPv4MappedAddress>() - 1usize];
    [
        "Offset of field: SteamNetworkingIPAddr_IPv4MappedAddress::m_8zeros",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr_IPv4MappedAddress, m_8zeros)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIPAddr_IPv4MappedAddress::m_0000",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr_IPv4MappedAddress, m_0000) - 8usize];
    [
        "Offset of field: SteamNetworkingIPAddr_IPv4MappedAddress::m_ffff",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr_IPv4MappedAddress, m_ffff) - 10usize];
    [
        "Offset of field: SteamNetworkingIPAddr_IPv4MappedAddress::m_ip",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr_IPv4MappedAddress, m_ip) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub union SteamNetworkingIPAddr__bindgen_ty_2 {
    pub m_ipv6: [uint8; 16usize],
    pub m_ipv4: SteamNetworkingIPAddr_IPv4MappedAddress,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIPAddr__bindgen_ty_2",
    ][::std::mem::size_of::<SteamNetworkingIPAddr__bindgen_ty_2>() - 16usize];
    [
        "Alignment of SteamNetworkingIPAddr__bindgen_ty_2",
    ][::std::mem::align_of::<SteamNetworkingIPAddr__bindgen_ty_2>() - 1usize];
    [
        "Offset of field: SteamNetworkingIPAddr__bindgen_ty_2::m_ipv6",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr__bindgen_ty_2, m_ipv6) - 0usize];
    [
        "Offset of field: SteamNetworkingIPAddr__bindgen_ty_2::m_ipv4",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr__bindgen_ty_2, m_ipv4) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIPAddr",
    ][::std::mem::size_of::<SteamNetworkingIPAddr>() - 18usize];
    [
        "Alignment of SteamNetworkingIPAddr",
    ][::std::mem::align_of::<SteamNetworkingIPAddr>() - 1usize];
    [
        "Offset of field: SteamNetworkingIPAddr::m_port",
    ][::std::mem::offset_of!(SteamNetworkingIPAddr, m_port) - 16usize];
};
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SteamNetworkingIdentity {
    pub m_eType: ESteamNetworkingIdentityType,
    pub m_cbSize: ::std::os::raw::c_int,
    pub __bindgen_anon_1: SteamNetworkingIdentity__bindgen_ty_2,
}
pub const SteamNetworkingIdentity_k_cchMaxString: SteamNetworkingIdentity__bindgen_ty_1 = SteamNetworkingIdentity__bindgen_ty_1::k_cchMaxString;
pub const SteamNetworkingIdentity_k_cchMaxGenericString: SteamNetworkingIdentity__bindgen_ty_1 = SteamNetworkingIdentity__bindgen_ty_1::k_cchMaxGenericString;
pub const SteamNetworkingIdentity_k_cchMaxXboxPairwiseID: SteamNetworkingIdentity__bindgen_ty_1 = SteamNetworkingIdentity__bindgen_ty_1::k_cchMaxXboxPairwiseID;
pub const SteamNetworkingIdentity_k_cbMaxGenericBytes: SteamNetworkingIdentity__bindgen_ty_1 = SteamNetworkingIdentity__bindgen_ty_1::k_cchMaxGenericString;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetworkingIdentity__bindgen_ty_1 {
    k_cchMaxString = 128,
    k_cchMaxGenericString = 32,
    k_cchMaxXboxPairwiseID = 33,
}
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub union SteamNetworkingIdentity__bindgen_ty_2 {
    pub m_steamID64: uint64,
    pub m_PSNID: uint64,
    pub m_szGenericString: [::std::os::raw::c_char; 32usize],
    pub m_szXboxPairwiseID: [::std::os::raw::c_char; 33usize],
    pub m_genericBytes: [uint8; 32usize],
    pub m_szUnknownRawString: [::std::os::raw::c_char; 128usize],
    pub m_ip: SteamNetworkingIPAddr,
    pub m_reserved: [uint32; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIdentity__bindgen_ty_2",
    ][::std::mem::size_of::<SteamNetworkingIdentity__bindgen_ty_2>() - 128usize];
    [
        "Alignment of SteamNetworkingIdentity__bindgen_ty_2",
    ][::std::mem::align_of::<SteamNetworkingIdentity__bindgen_ty_2>() - 1usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_steamID64",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_steamID64)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_PSNID",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_PSNID) - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_szGenericString",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_szGenericString)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_szXboxPairwiseID",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_szXboxPairwiseID)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_genericBytes",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_genericBytes)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_szUnknownRawString",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_szUnknownRawString)
        - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_ip",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_ip) - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity__bindgen_ty_2::m_reserved",
    ][::std::mem::offset_of!(SteamNetworkingIdentity__bindgen_ty_2, m_reserved)
        - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIdentity",
    ][::std::mem::size_of::<SteamNetworkingIdentity>() - 136usize];
    [
        "Alignment of SteamNetworkingIdentity",
    ][::std::mem::align_of::<SteamNetworkingIdentity>() - 1usize];
    [
        "Offset of field: SteamNetworkingIdentity::m_eType",
    ][::std::mem::offset_of!(SteamNetworkingIdentity, m_eType) - 0usize];
    [
        "Offset of field: SteamNetworkingIdentity::m_cbSize",
    ][::std::mem::offset_of!(SteamNetworkingIdentity, m_cbSize) - 4usize];
};
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingConnectionState {
    k_ESteamNetworkingConnectionState_None = 0,
    k_ESteamNetworkingConnectionState_Connecting = 1,
    k_ESteamNetworkingConnectionState_FindingRoute = 2,
    k_ESteamNetworkingConnectionState_Connected = 3,
    k_ESteamNetworkingConnectionState_ClosedByPeer = 4,
    k_ESteamNetworkingConnectionState_ProblemDetectedLocally = 5,
    k_ESteamNetworkingConnectionState_FinWait = -1,
    k_ESteamNetworkingConnectionState_Linger = -2,
    k_ESteamNetworkingConnectionState_Dead = -3,
    k_ESteamNetworkingConnectionState__Force32Bit = 2147483647,
}
impl ESteamNetConnectionEnd {
    pub const k_ESteamNetConnectionEnd_App_Generic: ESteamNetConnectionEnd = ESteamNetConnectionEnd::k_ESteamNetConnectionEnd_App_Min;
}
impl ESteamNetConnectionEnd {
    pub const k_ESteamNetConnectionEnd_AppException_Generic: ESteamNetConnectionEnd = ESteamNetConnectionEnd::k_ESteamNetConnectionEnd_AppException_Min;
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetConnectionEnd {
    k_ESteamNetConnectionEnd_Invalid = 0,
    k_ESteamNetConnectionEnd_App_Min = 1000,
    k_ESteamNetConnectionEnd_App_Max = 1999,
    k_ESteamNetConnectionEnd_AppException_Min = 2000,
    k_ESteamNetConnectionEnd_AppException_Max = 2999,
    k_ESteamNetConnectionEnd_Local_Min = 3000,
    k_ESteamNetConnectionEnd_Local_OfflineMode = 3001,
    k_ESteamNetConnectionEnd_Local_ManyRelayConnectivity = 3002,
    k_ESteamNetConnectionEnd_Local_HostedServerPrimaryRelay = 3003,
    k_ESteamNetConnectionEnd_Local_NetworkConfig = 3004,
    k_ESteamNetConnectionEnd_Local_Rights = 3005,
    k_ESteamNetConnectionEnd_Local_P2P_ICE_NoPublicAddresses = 3006,
    k_ESteamNetConnectionEnd_Local_Max = 3999,
    k_ESteamNetConnectionEnd_Remote_Min = 4000,
    k_ESteamNetConnectionEnd_Remote_Timeout = 4001,
    k_ESteamNetConnectionEnd_Remote_BadCrypt = 4002,
    k_ESteamNetConnectionEnd_Remote_BadCert = 4003,
    k_ESteamNetConnectionEnd_Remote_BadProtocolVersion = 4006,
    k_ESteamNetConnectionEnd_Remote_P2P_ICE_NoPublicAddresses = 4007,
    k_ESteamNetConnectionEnd_Remote_Max = 4999,
    k_ESteamNetConnectionEnd_Misc_Min = 5000,
    k_ESteamNetConnectionEnd_Misc_Generic = 5001,
    k_ESteamNetConnectionEnd_Misc_InternalError = 5002,
    k_ESteamNetConnectionEnd_Misc_Timeout = 5003,
    k_ESteamNetConnectionEnd_Misc_SteamConnectivity = 5005,
    k_ESteamNetConnectionEnd_Misc_NoRelaySessionsToClient = 5006,
    k_ESteamNetConnectionEnd_Misc_P2P_Rendezvous = 5008,
    k_ESteamNetConnectionEnd_Misc_P2P_NAT_Firewall = 5009,
    k_ESteamNetConnectionEnd_Misc_PeerSentNoConnection = 5010,
    k_ESteamNetConnectionEnd_Misc_Max = 5999,
    k_ESteamNetConnectionEnd__Force32Bit = 2147483647,
}
pub const k_cchSteamNetworkingMaxConnectionCloseReason: ::std::os::raw::c_int = 128;
pub const k_cchSteamNetworkingMaxConnectionDescription: ::std::os::raw::c_int = 128;
pub const k_cchSteamNetworkingMaxConnectionAppName: ::std::os::raw::c_int = 32;
pub const k_nSteamNetworkConnectionInfoFlags_Unauthenticated: ::std::os::raw::c_int = 1;
pub const k_nSteamNetworkConnectionInfoFlags_Unencrypted: ::std::os::raw::c_int = 2;
pub const k_nSteamNetworkConnectionInfoFlags_LoopbackBuffers: ::std::os::raw::c_int = 4;
pub const k_nSteamNetworkConnectionInfoFlags_Fast: ::std::os::raw::c_int = 8;
pub const k_nSteamNetworkConnectionInfoFlags_Relayed: ::std::os::raw::c_int = 16;
pub const k_nSteamNetworkConnectionInfoFlags_DualWifi: ::std::os::raw::c_int = 32;
#[repr(C, packed(4))]
#[derive(Copy, Clone)]
pub struct SteamNetConnectionInfo_t {
    pub m_identityRemote: SteamNetworkingIdentity,
    pub m_nUserData: int64,
    pub m_hListenSocket: HSteamListenSocket,
    pub m_addrRemote: SteamNetworkingIPAddr,
    pub m__pad1: uint16,
    pub m_idPOPRemote: SteamNetworkingPOPID,
    pub m_idPOPRelay: SteamNetworkingPOPID,
    pub m_eState: ESteamNetworkingConnectionState,
    pub m_eEndReason: ::std::os::raw::c_int,
    pub m_szEndDebug: [::std::os::raw::c_char; 128usize],
    pub m_szConnectionDescription: [::std::os::raw::c_char; 128usize],
    pub m_nFlags: ::std::os::raw::c_int,
    pub reserved: [uint32; 63usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetConnectionInfo_t",
    ][::std::mem::size_of::<SteamNetConnectionInfo_t>() - 696usize];
    [
        "Alignment of SteamNetConnectionInfo_t",
    ][::std::mem::align_of::<SteamNetConnectionInfo_t>() - 4usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_identityRemote",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_identityRemote) - 0usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_nUserData",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_nUserData) - 136usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_hListenSocket",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_hListenSocket) - 144usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_addrRemote",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_addrRemote) - 148usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m__pad1",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m__pad1) - 166usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_idPOPRemote",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_idPOPRemote) - 168usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_idPOPRelay",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_idPOPRelay) - 172usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_eState",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_eState) - 176usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_eEndReason",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_eEndReason) - 180usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_szEndDebug",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_szEndDebug) - 184usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_szConnectionDescription",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_szConnectionDescription)
        - 312usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::m_nFlags",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, m_nFlags) - 440usize];
    [
        "Offset of field: SteamNetConnectionInfo_t::reserved",
    ][::std::mem::offset_of!(SteamNetConnectionInfo_t, reserved) - 444usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetConnectionRealTimeStatus_t {
    pub m_eState: ESteamNetworkingConnectionState,
    pub m_nPing: ::std::os::raw::c_int,
    pub m_flConnectionQualityLocal: f32,
    pub m_flConnectionQualityRemote: f32,
    pub m_flOutPacketsPerSec: f32,
    pub m_flOutBytesPerSec: f32,
    pub m_flInPacketsPerSec: f32,
    pub m_flInBytesPerSec: f32,
    pub m_nSendRateBytesPerSecond: ::std::os::raw::c_int,
    pub m_cbPendingUnreliable: ::std::os::raw::c_int,
    pub m_cbPendingReliable: ::std::os::raw::c_int,
    pub m_cbSentUnackedReliable: ::std::os::raw::c_int,
    pub m_usecQueueTime: SteamNetworkingMicroseconds,
    pub reserved: [uint32; 16usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetConnectionRealTimeStatus_t",
    ][::std::mem::size_of::<SteamNetConnectionRealTimeStatus_t>() - 120usize];
    [
        "Alignment of SteamNetConnectionRealTimeStatus_t",
    ][::std::mem::align_of::<SteamNetConnectionRealTimeStatus_t>() - 4usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_eState",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_eState) - 0usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_nPing",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_nPing) - 4usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flConnectionQualityLocal",
    ][::std::mem::offset_of!(
        SteamNetConnectionRealTimeStatus_t, m_flConnectionQualityLocal
    ) - 8usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flConnectionQualityRemote",
    ][::std::mem::offset_of!(
        SteamNetConnectionRealTimeStatus_t, m_flConnectionQualityRemote
    ) - 12usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flOutPacketsPerSec",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_flOutPacketsPerSec)
        - 16usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flOutBytesPerSec",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_flOutBytesPerSec)
        - 20usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flInPacketsPerSec",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_flInPacketsPerSec)
        - 24usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_flInBytesPerSec",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_flInBytesPerSec)
        - 28usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_nSendRateBytesPerSecond",
    ][::std::mem::offset_of!(
        SteamNetConnectionRealTimeStatus_t, m_nSendRateBytesPerSecond
    ) - 32usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_cbPendingUnreliable",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_cbPendingUnreliable)
        - 36usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_cbPendingReliable",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_cbPendingReliable)
        - 40usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_cbSentUnackedReliable",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_cbSentUnackedReliable)
        - 44usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::m_usecQueueTime",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, m_usecQueueTime)
        - 48usize];
    [
        "Offset of field: SteamNetConnectionRealTimeStatus_t::reserved",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeStatus_t, reserved) - 56usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetConnectionRealTimeLaneStatus_t {
    pub m_cbPendingUnreliable: ::std::os::raw::c_int,
    pub m_cbPendingReliable: ::std::os::raw::c_int,
    pub m_cbSentUnackedReliable: ::std::os::raw::c_int,
    pub _reservePad1: ::std::os::raw::c_int,
    pub m_usecQueueTime: SteamNetworkingMicroseconds,
    pub reserved: [uint32; 10usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetConnectionRealTimeLaneStatus_t",
    ][::std::mem::size_of::<SteamNetConnectionRealTimeLaneStatus_t>() - 64usize];
    [
        "Alignment of SteamNetConnectionRealTimeLaneStatus_t",
    ][::std::mem::align_of::<SteamNetConnectionRealTimeLaneStatus_t>() - 4usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::m_cbPendingUnreliable",
    ][::std::mem::offset_of!(
        SteamNetConnectionRealTimeLaneStatus_t, m_cbPendingUnreliable
    ) - 0usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::m_cbPendingReliable",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeLaneStatus_t, m_cbPendingReliable)
        - 4usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::m_cbSentUnackedReliable",
    ][::std::mem::offset_of!(
        SteamNetConnectionRealTimeLaneStatus_t, m_cbSentUnackedReliable
    ) - 8usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::_reservePad1",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeLaneStatus_t, _reservePad1)
        - 12usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::m_usecQueueTime",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeLaneStatus_t, m_usecQueueTime)
        - 16usize];
    [
        "Offset of field: SteamNetConnectionRealTimeLaneStatus_t::reserved",
    ][::std::mem::offset_of!(SteamNetConnectionRealTimeLaneStatus_t, reserved)
        - 24usize];
};
pub const k_cbMaxSteamNetworkingSocketsMessageSizeSend: ::std::os::raw::c_int = 524288;
#[repr(C)]
pub struct SteamNetworkingMessage_t {
    pub m_pData: *mut ::std::os::raw::c_void,
    pub m_cbSize: ::std::os::raw::c_int,
    pub m_conn: HSteamNetConnection,
    pub m_identityPeer: SteamNetworkingIdentity,
    pub m_nConnUserData: int64,
    pub m_usecTimeReceived: SteamNetworkingMicroseconds,
    pub m_nMessageNumber: int64,
    pub m_pfnFreeData: ::std::option::Option<
        unsafe extern "C" fn(pMsg: *mut SteamNetworkingMessage_t),
    >,
    pub m_pfnRelease: ::std::option::Option<
        unsafe extern "C" fn(pMsg: *mut SteamNetworkingMessage_t),
    >,
    pub m_nChannel: ::std::os::raw::c_int,
    pub m_nFlags: ::std::os::raw::c_int,
    pub m_nUserData: int64,
    pub m_idxLane: uint16,
    pub _pad1__: uint16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingMessage_t",
    ][::std::mem::size_of::<SteamNetworkingMessage_t>() - 216usize];
    [
        "Alignment of SteamNetworkingMessage_t",
    ][::std::mem::align_of::<SteamNetworkingMessage_t>() - 8usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_pData",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_pData) - 0usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_cbSize",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_cbSize) - 8usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_conn",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_conn) - 12usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_identityPeer",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_identityPeer) - 16usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_nConnUserData",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_nConnUserData) - 152usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_usecTimeReceived",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_usecTimeReceived) - 160usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_nMessageNumber",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_nMessageNumber) - 168usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_pfnFreeData",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_pfnFreeData) - 176usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_pfnRelease",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_pfnRelease) - 184usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_nChannel",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_nChannel) - 192usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_nFlags",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_nFlags) - 196usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_nUserData",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_nUserData) - 200usize];
    [
        "Offset of field: SteamNetworkingMessage_t::m_idxLane",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, m_idxLane) - 208usize];
    [
        "Offset of field: SteamNetworkingMessage_t::_pad1__",
    ][::std::mem::offset_of!(SteamNetworkingMessage_t, _pad1__) - 210usize];
};
pub const k_nSteamNetworkingSend_Unreliable: ::std::os::raw::c_int = 0;
pub const k_nSteamNetworkingSend_NoNagle: ::std::os::raw::c_int = 1;
pub const k_nSteamNetworkingSend_UnreliableNoNagle: ::std::os::raw::c_int = 1;
pub const k_nSteamNetworkingSend_NoDelay: ::std::os::raw::c_int = 4;
pub const k_nSteamNetworkingSend_UnreliableNoDelay: ::std::os::raw::c_int = 5;
pub const k_nSteamNetworkingSend_Reliable: ::std::os::raw::c_int = 8;
pub const k_nSteamNetworkingSend_ReliableNoNagle: ::std::os::raw::c_int = 9;
pub const k_nSteamNetworkingSend_UseCurrentThread: ::std::os::raw::c_int = 16;
pub const k_nSteamNetworkingSend_AutoRestartBrokenSession: ::std::os::raw::c_int = 32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkPingLocation_t {
    pub m_data: [uint8; 512usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkPingLocation_t",
    ][::std::mem::size_of::<SteamNetworkPingLocation_t>() - 512usize];
    [
        "Alignment of SteamNetworkPingLocation_t",
    ][::std::mem::align_of::<SteamNetworkPingLocation_t>() - 1usize];
    [
        "Offset of field: SteamNetworkPingLocation_t::m_data",
    ][::std::mem::offset_of!(SteamNetworkPingLocation_t, m_data) - 0usize];
};
pub const k_cchMaxSteamNetworkingPingLocationString: ::std::os::raw::c_int = 1024;
pub const k_nSteamNetworkingPing_Failed: ::std::os::raw::c_int = -1;
pub const k_nSteamNetworkingPing_Unknown: ::std::os::raw::c_int = -2;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingConfigScope {
    k_ESteamNetworkingConfig_Global = 1,
    k_ESteamNetworkingConfig_SocketsInterface = 2,
    k_ESteamNetworkingConfig_ListenSocket = 3,
    k_ESteamNetworkingConfig_Connection = 4,
    k_ESteamNetworkingConfigScope__Force32Bit = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingConfigDataType {
    k_ESteamNetworkingConfig_Int32 = 1,
    k_ESteamNetworkingConfig_Int64 = 2,
    k_ESteamNetworkingConfig_Float = 3,
    k_ESteamNetworkingConfig_String = 4,
    k_ESteamNetworkingConfig_Ptr = 5,
    k_ESteamNetworkingConfigDataType__Force32Bit = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingConfigValue {
    k_ESteamNetworkingConfig_Invalid = 0,
    k_ESteamNetworkingConfig_TimeoutInitial = 24,
    k_ESteamNetworkingConfig_TimeoutConnected = 25,
    k_ESteamNetworkingConfig_SendBufferSize = 9,
    k_ESteamNetworkingConfig_RecvBufferSize = 47,
    k_ESteamNetworkingConfig_RecvBufferMessages = 48,
    k_ESteamNetworkingConfig_RecvMaxMessageSize = 49,
    k_ESteamNetworkingConfig_RecvMaxSegmentsPerPacket = 50,
    k_ESteamNetworkingConfig_ConnectionUserData = 40,
    k_ESteamNetworkingConfig_SendRateMin = 10,
    k_ESteamNetworkingConfig_SendRateMax = 11,
    k_ESteamNetworkingConfig_NagleTime = 12,
    k_ESteamNetworkingConfig_IP_AllowWithoutAuth = 23,
    k_ESteamNetworkingConfig_IPLocalHost_AllowWithoutAuth = 52,
    k_ESteamNetworkingConfig_MTU_PacketSize = 32,
    k_ESteamNetworkingConfig_MTU_DataSize = 33,
    k_ESteamNetworkingConfig_Unencrypted = 34,
    k_ESteamNetworkingConfig_SymmetricConnect = 37,
    k_ESteamNetworkingConfig_LocalVirtualPort = 38,
    k_ESteamNetworkingConfig_DualWifi_Enable = 39,
    k_ESteamNetworkingConfig_EnableDiagnosticsUI = 46,
    k_ESteamNetworkingConfig_SendTimeSincePreviousPacket = 59,
    k_ESteamNetworkingConfig_FakePacketLoss_Send = 2,
    k_ESteamNetworkingConfig_FakePacketLoss_Recv = 3,
    k_ESteamNetworkingConfig_FakePacketLag_Send = 4,
    k_ESteamNetworkingConfig_FakePacketLag_Recv = 5,
    k_ESteamNetworkingConfig_FakePacketJitter_Send_Avg = 53,
    k_ESteamNetworkingConfig_FakePacketJitter_Send_Max = 54,
    k_ESteamNetworkingConfig_FakePacketJitter_Send_Pct = 55,
    k_ESteamNetworkingConfig_FakePacketJitter_Recv_Avg = 56,
    k_ESteamNetworkingConfig_FakePacketJitter_Recv_Max = 57,
    k_ESteamNetworkingConfig_FakePacketJitter_Recv_Pct = 58,
    k_ESteamNetworkingConfig_FakePacketReorder_Send = 6,
    k_ESteamNetworkingConfig_FakePacketReorder_Recv = 7,
    k_ESteamNetworkingConfig_FakePacketReorder_Time = 8,
    k_ESteamNetworkingConfig_FakePacketDup_Send = 26,
    k_ESteamNetworkingConfig_FakePacketDup_Recv = 27,
    k_ESteamNetworkingConfig_FakePacketDup_TimeMax = 28,
    k_ESteamNetworkingConfig_PacketTraceMaxBytes = 41,
    k_ESteamNetworkingConfig_FakeRateLimit_Send_Rate = 42,
    k_ESteamNetworkingConfig_FakeRateLimit_Send_Burst = 43,
    k_ESteamNetworkingConfig_FakeRateLimit_Recv_Rate = 44,
    k_ESteamNetworkingConfig_FakeRateLimit_Recv_Burst = 45,
    k_ESteamNetworkingConfig_OutOfOrderCorrectionWindowMicroseconds = 51,
    k_ESteamNetworkingConfig_Callback_ConnectionStatusChanged = 201,
    k_ESteamNetworkingConfig_Callback_AuthStatusChanged = 202,
    k_ESteamNetworkingConfig_Callback_RelayNetworkStatusChanged = 203,
    k_ESteamNetworkingConfig_Callback_MessagesSessionRequest = 204,
    k_ESteamNetworkingConfig_Callback_MessagesSessionFailed = 205,
    k_ESteamNetworkingConfig_Callback_CreateConnectionSignaling = 206,
    k_ESteamNetworkingConfig_Callback_FakeIPResult = 207,
    k_ESteamNetworkingConfig_P2P_STUN_ServerList = 103,
    k_ESteamNetworkingConfig_P2P_Transport_ICE_Enable = 104,
    k_ESteamNetworkingConfig_P2P_Transport_ICE_Penalty = 105,
    k_ESteamNetworkingConfig_P2P_Transport_SDR_Penalty = 106,
    k_ESteamNetworkingConfig_P2P_TURN_ServerList = 107,
    k_ESteamNetworkingConfig_P2P_TURN_UserList = 108,
    k_ESteamNetworkingConfig_P2P_TURN_PassList = 109,
    k_ESteamNetworkingConfig_P2P_Transport_ICE_Implementation = 110,
    k_ESteamNetworkingConfig_SDRClient_ConsecutitivePingTimeoutsFailInitial = 19,
    k_ESteamNetworkingConfig_SDRClient_ConsecutitivePingTimeoutsFail = 20,
    k_ESteamNetworkingConfig_SDRClient_MinPingsBeforePingAccurate = 21,
    k_ESteamNetworkingConfig_SDRClient_SingleSocket = 22,
    k_ESteamNetworkingConfig_SDRClient_ForceRelayCluster = 29,
    k_ESteamNetworkingConfig_SDRClient_DevTicket = 30,
    k_ESteamNetworkingConfig_SDRClient_ForceProxyAddr = 31,
    k_ESteamNetworkingConfig_SDRClient_FakeClusterPing = 36,
    k_ESteamNetworkingConfig_SDRClient_LimitPingProbesToNearestN = 60,
    k_ESteamNetworkingConfig_LogLevel_AckRTT = 13,
    k_ESteamNetworkingConfig_LogLevel_PacketDecode = 14,
    k_ESteamNetworkingConfig_LogLevel_Message = 15,
    k_ESteamNetworkingConfig_LogLevel_PacketGaps = 16,
    k_ESteamNetworkingConfig_LogLevel_P2PRendezvous = 17,
    k_ESteamNetworkingConfig_LogLevel_SDRRelayPings = 18,
    k_ESteamNetworkingConfig_ECN = 999,
    k_ESteamNetworkingConfig_DELETED_EnumerateDevVars = 35,
    k_ESteamNetworkingConfigValue__Force32Bit = 2147483647,
}
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_Default: ::std::os::raw::c_int = -1;
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_Disable: ::std::os::raw::c_int = 0;
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_Relay: ::std::os::raw::c_int = 1;
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_Private: ::std::os::raw::c_int = 2;
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_Public: ::std::os::raw::c_int = 4;
pub const k_nSteamNetworkingConfig_P2P_Transport_ICE_Enable_All: ::std::os::raw::c_int = 2147483647;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SteamNetworkingConfigValue_t {
    pub m_eValue: ESteamNetworkingConfigValue,
    pub m_eDataType: ESteamNetworkingConfigDataType,
    pub m_val: SteamNetworkingConfigValue_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SteamNetworkingConfigValue_t__bindgen_ty_1 {
    pub m_int32: i32,
    pub m_int64: i64,
    pub m_float: f32,
    pub m_string: *const ::std::os::raw::c_char,
    pub m_ptr: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingConfigValue_t__bindgen_ty_1",
    ][::std::mem::size_of::<SteamNetworkingConfigValue_t__bindgen_ty_1>() - 8usize];
    [
        "Alignment of SteamNetworkingConfigValue_t__bindgen_ty_1",
    ][::std::mem::align_of::<SteamNetworkingConfigValue_t__bindgen_ty_1>() - 8usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t__bindgen_ty_1::m_int32",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t__bindgen_ty_1, m_int32)
        - 0usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t__bindgen_ty_1::m_int64",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t__bindgen_ty_1, m_int64)
        - 0usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t__bindgen_ty_1::m_float",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t__bindgen_ty_1, m_float)
        - 0usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t__bindgen_ty_1::m_string",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t__bindgen_ty_1, m_string)
        - 0usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t__bindgen_ty_1::m_ptr",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t__bindgen_ty_1, m_ptr)
        - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingConfigValue_t",
    ][::std::mem::size_of::<SteamNetworkingConfigValue_t>() - 16usize];
    [
        "Alignment of SteamNetworkingConfigValue_t",
    ][::std::mem::align_of::<SteamNetworkingConfigValue_t>() - 8usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t::m_eValue",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t, m_eValue) - 0usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t::m_eDataType",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t, m_eDataType) - 4usize];
    [
        "Offset of field: SteamNetworkingConfigValue_t::m_val",
    ][::std::mem::offset_of!(SteamNetworkingConfigValue_t, m_val) - 8usize];
};
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingGetConfigValueResult {
    k_ESteamNetworkingGetConfigValue_BadValue = -1,
    k_ESteamNetworkingGetConfigValue_BadScopeObj = -2,
    k_ESteamNetworkingGetConfigValue_BufferTooSmall = -3,
    k_ESteamNetworkingGetConfigValue_OK = 1,
    k_ESteamNetworkingGetConfigValue_OKInherited = 2,
    k_ESteamNetworkingGetConfigValueResult__Force32Bit = 2147483647,
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamNetworkingSocketsDebugOutputType {
    k_ESteamNetworkingSocketsDebugOutputType_None = 0,
    k_ESteamNetworkingSocketsDebugOutputType_Bug = 1,
    k_ESteamNetworkingSocketsDebugOutputType_Error = 2,
    k_ESteamNetworkingSocketsDebugOutputType_Important = 3,
    k_ESteamNetworkingSocketsDebugOutputType_Warning = 4,
    k_ESteamNetworkingSocketsDebugOutputType_Msg = 5,
    k_ESteamNetworkingSocketsDebugOutputType_Verbose = 6,
    k_ESteamNetworkingSocketsDebugOutputType_Debug = 7,
    k_ESteamNetworkingSocketsDebugOutputType_Everything = 8,
    k_ESteamNetworkingSocketsDebugOutputType__Force32Bit = 2147483647,
}
pub type FSteamNetworkingSocketsDebugOutput = ::std::option::Option<
    unsafe extern "C" fn(
        nType: ESteamNetworkingSocketsDebugOutputType,
        pszMsg: *const ::std::os::raw::c_char,
    ),
>;
pub const k_SteamDatagramPOPID_dev: SteamNetworkingPOPID = 6579574;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkingPOPIDRender {
    pub buf: [::std::os::raw::c_char; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingPOPIDRender",
    ][::std::mem::size_of::<SteamNetworkingPOPIDRender>() - 8usize];
    [
        "Alignment of SteamNetworkingPOPIDRender",
    ][::std::mem::align_of::<SteamNetworkingPOPIDRender>() - 1usize];
    [
        "Offset of field: SteamNetworkingPOPIDRender::buf",
    ][::std::mem::offset_of!(SteamNetworkingPOPIDRender, buf) - 0usize];
};
pub type ISteamNetworkingMessage = SteamNetworkingMessage_t;
pub type SteamDatagramErrMsg = SteamNetworkingErrMsg;
#[repr(C)]
pub struct ISteamNetworkingMessages__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamNetworkingMessages {
    pub vtable_: *const ISteamNetworkingMessages__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamNetworkingMessages",
    ][::std::mem::size_of::<ISteamNetworkingMessages>() - 8usize];
    [
        "Alignment of ISteamNetworkingMessages",
    ][::std::mem::align_of::<ISteamNetworkingMessages>() - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SteamNetworkingMessagesSessionRequest_t {
    pub m_identityRemote: SteamNetworkingIdentity,
}
pub const SteamNetworkingMessagesSessionRequest_t_k_iCallback: SteamNetworkingMessagesSessionRequest_t__bindgen_ty_1 = SteamNetworkingMessagesSessionRequest_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetworkingMessagesSessionRequest_t__bindgen_ty_1 {
    k_iCallback = 1251,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingMessagesSessionRequest_t",
    ][::std::mem::size_of::<SteamNetworkingMessagesSessionRequest_t>() - 136usize];
    [
        "Alignment of SteamNetworkingMessagesSessionRequest_t",
    ][::std::mem::align_of::<SteamNetworkingMessagesSessionRequest_t>() - 1usize];
    [
        "Offset of field: SteamNetworkingMessagesSessionRequest_t::m_identityRemote",
    ][::std::mem::offset_of!(SteamNetworkingMessagesSessionRequest_t, m_identityRemote)
        - 0usize];
};
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SteamNetworkingMessagesSessionFailed_t {
    pub m_info: SteamNetConnectionInfo_t,
}
pub const SteamNetworkingMessagesSessionFailed_t_k_iCallback: SteamNetworkingMessagesSessionFailed_t__bindgen_ty_1 = SteamNetworkingMessagesSessionFailed_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetworkingMessagesSessionFailed_t__bindgen_ty_1 {
    k_iCallback = 1252,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingMessagesSessionFailed_t",
    ][::std::mem::size_of::<SteamNetworkingMessagesSessionFailed_t>() - 696usize];
    [
        "Alignment of SteamNetworkingMessagesSessionFailed_t",
    ][::std::mem::align_of::<SteamNetworkingMessagesSessionFailed_t>() - 1usize];
    [
        "Offset of field: SteamNetworkingMessagesSessionFailed_t::m_info",
    ][::std::mem::offset_of!(SteamNetworkingMessagesSessionFailed_t, m_info) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamNetworkingConnectionSignaling {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamNetworkingSignalingRecvContext {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamNetworkingFakeUDPPort {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct ISteamNetworkingSockets__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct ISteamNetworkingSockets {
    pub vtable_: *const ISteamNetworkingSockets__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamNetworkingSockets",
    ][::std::mem::size_of::<ISteamNetworkingSockets>() - 8usize];
    [
        "Alignment of ISteamNetworkingSockets",
    ][::std::mem::align_of::<ISteamNetworkingSockets>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN23ISteamNetworkingSocketsD1Ev"]
    pub fn ISteamNetworkingSockets_ISteamNetworkingSockets_destructor(
        this: *mut ISteamNetworkingSockets,
    );
}
impl ISteamNetworkingSockets {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ISteamNetworkingSockets_ISteamNetworkingSockets_destructor(self)
    }
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SteamNetConnectionStatusChangedCallback_t {
    pub m_hConn: HSteamNetConnection,
    pub m_info: SteamNetConnectionInfo_t,
    pub m_eOldState: ESteamNetworkingConnectionState,
}
pub const SteamNetConnectionStatusChangedCallback_t_k_iCallback: SteamNetConnectionStatusChangedCallback_t__bindgen_ty_1 = SteamNetConnectionStatusChangedCallback_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetConnectionStatusChangedCallback_t__bindgen_ty_1 {
    k_iCallback = 1221,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetConnectionStatusChangedCallback_t",
    ][::std::mem::size_of::<SteamNetConnectionStatusChangedCallback_t>() - 704usize];
    [
        "Alignment of SteamNetConnectionStatusChangedCallback_t",
    ][::std::mem::align_of::<SteamNetConnectionStatusChangedCallback_t>() - 4usize];
    [
        "Offset of field: SteamNetConnectionStatusChangedCallback_t::m_hConn",
    ][::std::mem::offset_of!(SteamNetConnectionStatusChangedCallback_t, m_hConn)
        - 0usize];
    [
        "Offset of field: SteamNetConnectionStatusChangedCallback_t::m_info",
    ][::std::mem::offset_of!(SteamNetConnectionStatusChangedCallback_t, m_info)
        - 4usize];
    [
        "Offset of field: SteamNetConnectionStatusChangedCallback_t::m_eOldState",
    ][::std::mem::offset_of!(SteamNetConnectionStatusChangedCallback_t, m_eOldState)
        - 700usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetAuthenticationStatus_t {
    pub m_eAvail: ESteamNetworkingAvailability,
    pub m_debugMsg: [::std::os::raw::c_char; 256usize],
}
pub const SteamNetAuthenticationStatus_t_k_iCallback: SteamNetAuthenticationStatus_t__bindgen_ty_1 = SteamNetAuthenticationStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamNetAuthenticationStatus_t__bindgen_ty_1 {
    k_iCallback = 1222,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetAuthenticationStatus_t",
    ][::std::mem::size_of::<SteamNetAuthenticationStatus_t>() - 260usize];
    [
        "Alignment of SteamNetAuthenticationStatus_t",
    ][::std::mem::align_of::<SteamNetAuthenticationStatus_t>() - 4usize];
    [
        "Offset of field: SteamNetAuthenticationStatus_t::m_eAvail",
    ][::std::mem::offset_of!(SteamNetAuthenticationStatus_t, m_eAvail) - 0usize];
    [
        "Offset of field: SteamNetAuthenticationStatus_t::m_debugMsg",
    ][::std::mem::offset_of!(SteamNetAuthenticationStatus_t, m_debugMsg) - 4usize];
};
#[repr(C)]
pub struct ISteamNetworkingUtils__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug)]
pub struct ISteamNetworkingUtils {
    pub vtable_: *const ISteamNetworkingUtils__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamNetworkingUtils",
    ][::std::mem::size_of::<ISteamNetworkingUtils>() - 8usize];
    [
        "Alignment of ISteamNetworkingUtils",
    ][::std::mem::align_of::<ISteamNetworkingUtils>() - 8usize];
};
unsafe extern "C" {
    #[link_name = "\u{1}_ZN21ISteamNetworkingUtilsD1Ev"]
    pub fn ISteamNetworkingUtils_ISteamNetworkingUtils_destructor(
        this: *mut ISteamNetworkingUtils,
    );
}
impl ISteamNetworkingUtils {
    #[inline]
    pub unsafe fn destruct(&mut self) {
        ISteamNetworkingUtils_ISteamNetworkingUtils_destructor(self)
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamRelayNetworkStatus_t {
    pub m_eAvail: ESteamNetworkingAvailability,
    pub m_bPingMeasurementInProgress: ::std::os::raw::c_int,
    pub m_eAvailNetworkConfig: ESteamNetworkingAvailability,
    pub m_eAvailAnyRelay: ESteamNetworkingAvailability,
    pub m_debugMsg: [::std::os::raw::c_char; 256usize],
}
pub const SteamRelayNetworkStatus_t_k_iCallback: SteamRelayNetworkStatus_t__bindgen_ty_1 = SteamRelayNetworkStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SteamRelayNetworkStatus_t__bindgen_ty_1 {
    k_iCallback = 1281,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamRelayNetworkStatus_t",
    ][::std::mem::size_of::<SteamRelayNetworkStatus_t>() - 272usize];
    [
        "Alignment of SteamRelayNetworkStatus_t",
    ][::std::mem::align_of::<SteamRelayNetworkStatus_t>() - 4usize];
    [
        "Offset of field: SteamRelayNetworkStatus_t::m_eAvail",
    ][::std::mem::offset_of!(SteamRelayNetworkStatus_t, m_eAvail) - 0usize];
    [
        "Offset of field: SteamRelayNetworkStatus_t::m_bPingMeasurementInProgress",
    ][::std::mem::offset_of!(SteamRelayNetworkStatus_t, m_bPingMeasurementInProgress)
        - 4usize];
    [
        "Offset of field: SteamRelayNetworkStatus_t::m_eAvailNetworkConfig",
    ][::std::mem::offset_of!(SteamRelayNetworkStatus_t, m_eAvailNetworkConfig) - 8usize];
    [
        "Offset of field: SteamRelayNetworkStatus_t::m_eAvailAnyRelay",
    ][::std::mem::offset_of!(SteamRelayNetworkStatus_t, m_eAvailAnyRelay) - 12usize];
    [
        "Offset of field: SteamRelayNetworkStatus_t::m_debugMsg",
    ][::std::mem::offset_of!(SteamRelayNetworkStatus_t, m_debugMsg) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkingIdentityRender {
    pub buf: [::std::os::raw::c_char; 128usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIdentityRender",
    ][::std::mem::size_of::<SteamNetworkingIdentityRender>() - 128usize];
    [
        "Alignment of SteamNetworkingIdentityRender",
    ][::std::mem::align_of::<SteamNetworkingIdentityRender>() - 1usize];
    [
        "Offset of field: SteamNetworkingIdentityRender::buf",
    ][::std::mem::offset_of!(SteamNetworkingIdentityRender, buf) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SteamNetworkingIPAddrRender {
    pub buf: [::std::os::raw::c_char; 48usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of SteamNetworkingIPAddrRender",
    ][::std::mem::size_of::<SteamNetworkingIPAddrRender>() - 48usize];
    [
        "Alignment of SteamNetworkingIPAddrRender",
    ][::std::mem::align_of::<SteamNetworkingIPAddrRender>() - 1usize];
    [
        "Offset of field: SteamNetworkingIPAddrRender::buf",
    ][::std::mem::offset_of!(SteamNetworkingIPAddrRender, buf) - 0usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ESteamAPIInitResult {
    k_ESteamAPIInitResult_OK = 0,
    k_ESteamAPIInitResult_FailedGeneric = 1,
    k_ESteamAPIInitResult_NoSteamClient = 2,
    k_ESteamAPIInitResult_VersionMismatch = 3,
}
unsafe extern "C" {
    pub fn SteamAPI_InitFlat(pOutErrMsg: *mut SteamErrMsg) -> ESteamAPIInitResult;
    pub fn SteamAPI_Shutdown();
    pub fn SteamAPI_RestartAppIfNecessary(unOwnAppID: uint32) -> bool;
    pub fn SteamAPI_ReleaseCurrentThreadMemory();
    pub fn SteamAPI_WriteMiniDump(
        uStructuredExceptionCode: uint32,
        pvExceptionInfo: *mut ::std::os::raw::c_void,
        uBuildID: uint32,
    );
    pub fn SteamAPI_SetMiniDumpComment(pchMsg: *const ::std::os::raw::c_char);
    pub fn SteamAPI_IsSteamRunning() -> bool;
    pub fn SteamAPI_GetSteamInstallPath() -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_SetTryCatchCallbacks(bTryCatchCallbacks: bool);
    pub fn SteamAPI_ManualDispatch_Init();
    pub fn SteamAPI_ManualDispatch_RunFrame(hSteamPipe: HSteamPipe);
    pub fn SteamAPI_ManualDispatch_GetNextCallback(
        hSteamPipe: HSteamPipe,
        pCallbackMsg: *mut CallbackMsg_t,
    ) -> bool;
    pub fn SteamAPI_ManualDispatch_FreeLastCallback(hSteamPipe: HSteamPipe);
    pub fn SteamAPI_ManualDispatch_GetAPICallResult(
        hSteamPipe: HSteamPipe,
        hSteamAPICall: SteamAPICall_t,
        pCallback: *mut ::std::os::raw::c_void,
        cubCallback: ::std::os::raw::c_int,
        iCallbackExpected: ::std::os::raw::c_int,
        pbFailed: *mut bool,
    ) -> bool;
    pub fn SteamInternal_SteamAPI_Init(
        pszInternalCheckInterfaceVersions: *const ::std::os::raw::c_char,
        pOutErrMsg: *mut SteamErrMsg,
    ) -> ESteamAPIInitResult;
}
#[repr(C)]
pub struct ISteamGameServer__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamGameServer {
    pub vtable_: *const ISteamGameServer__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamGameServer"][::std::mem::size_of::<ISteamGameServer>() - 8usize];
    [
        "Alignment of ISteamGameServer",
    ][::std::mem::align_of::<ISteamGameServer>() - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSClientApprove_t {
    pub m_SteamID: CSteamID,
    pub m_OwnerSteamID: CSteamID,
}
pub const GSClientApprove_t_k_iCallback: GSClientApprove_t__bindgen_ty_1 = GSClientApprove_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSClientApprove_t__bindgen_ty_1 {
    k_iCallback = 201,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSClientApprove_t"][::std::mem::size_of::<GSClientApprove_t>() - 16usize];
    [
        "Alignment of GSClientApprove_t",
    ][::std::mem::align_of::<GSClientApprove_t>() - 1usize];
    [
        "Offset of field: GSClientApprove_t::m_SteamID",
    ][::std::mem::offset_of!(GSClientApprove_t, m_SteamID) - 0usize];
    [
        "Offset of field: GSClientApprove_t::m_OwnerSteamID",
    ][::std::mem::offset_of!(GSClientApprove_t, m_OwnerSteamID) - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSClientDeny_t {
    pub m_SteamID: CSteamID,
    pub m_eDenyReason: EDenyReason,
    pub m_rgchOptionalText: [::std::os::raw::c_char; 128usize],
}
pub const GSClientDeny_t_k_iCallback: GSClientDeny_t__bindgen_ty_1 = GSClientDeny_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSClientDeny_t__bindgen_ty_1 {
    k_iCallback = 202,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSClientDeny_t"][::std::mem::size_of::<GSClientDeny_t>() - 140usize];
    ["Alignment of GSClientDeny_t"][::std::mem::align_of::<GSClientDeny_t>() - 4usize];
    [
        "Offset of field: GSClientDeny_t::m_SteamID",
    ][::std::mem::offset_of!(GSClientDeny_t, m_SteamID) - 0usize];
    [
        "Offset of field: GSClientDeny_t::m_eDenyReason",
    ][::std::mem::offset_of!(GSClientDeny_t, m_eDenyReason) - 8usize];
    [
        "Offset of field: GSClientDeny_t::m_rgchOptionalText",
    ][::std::mem::offset_of!(GSClientDeny_t, m_rgchOptionalText) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSClientKick_t {
    pub m_SteamID: CSteamID,
    pub m_eDenyReason: EDenyReason,
}
pub const GSClientKick_t_k_iCallback: GSClientKick_t__bindgen_ty_1 = GSClientKick_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSClientKick_t__bindgen_ty_1 {
    k_iCallback = 203,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSClientKick_t"][::std::mem::size_of::<GSClientKick_t>() - 12usize];
    ["Alignment of GSClientKick_t"][::std::mem::align_of::<GSClientKick_t>() - 4usize];
    [
        "Offset of field: GSClientKick_t::m_SteamID",
    ][::std::mem::offset_of!(GSClientKick_t, m_SteamID) - 0usize];
    [
        "Offset of field: GSClientKick_t::m_eDenyReason",
    ][::std::mem::offset_of!(GSClientKick_t, m_eDenyReason) - 8usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GSClientAchievementStatus_t {
    pub m_SteamID: uint64,
    pub m_pchAchievement: [::std::os::raw::c_char; 128usize],
    pub m_bUnlocked: bool,
}
pub const GSClientAchievementStatus_t_k_iCallback: GSClientAchievementStatus_t__bindgen_ty_1 = GSClientAchievementStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSClientAchievementStatus_t__bindgen_ty_1 {
    k_iCallback = 206,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GSClientAchievementStatus_t",
    ][::std::mem::size_of::<GSClientAchievementStatus_t>() - 140usize];
    [
        "Alignment of GSClientAchievementStatus_t",
    ][::std::mem::align_of::<GSClientAchievementStatus_t>() - 4usize];
    [
        "Offset of field: GSClientAchievementStatus_t::m_SteamID",
    ][::std::mem::offset_of!(GSClientAchievementStatus_t, m_SteamID) - 0usize];
    [
        "Offset of field: GSClientAchievementStatus_t::m_pchAchievement",
    ][::std::mem::offset_of!(GSClientAchievementStatus_t, m_pchAchievement) - 8usize];
    [
        "Offset of field: GSClientAchievementStatus_t::m_bUnlocked",
    ][::std::mem::offset_of!(GSClientAchievementStatus_t, m_bUnlocked) - 136usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GSPolicyResponse_t {
    pub m_bSecure: uint8,
}
pub const GSPolicyResponse_t_k_iCallback: GSPolicyResponse_t__bindgen_ty_1 = GSPolicyResponse_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSPolicyResponse_t__bindgen_ty_1 {
    k_iCallback = 115,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSPolicyResponse_t"][::std::mem::size_of::<GSPolicyResponse_t>() - 1usize];
    [
        "Alignment of GSPolicyResponse_t",
    ][::std::mem::align_of::<GSPolicyResponse_t>() - 1usize];
    [
        "Offset of field: GSPolicyResponse_t::m_bSecure",
    ][::std::mem::offset_of!(GSPolicyResponse_t, m_bSecure) - 0usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GSGameplayStats_t {
    pub m_eResult: EResult,
    pub m_nRank: int32,
    pub m_unTotalConnects: uint32,
    pub m_unTotalMinutesPlayed: uint32,
}
pub const GSGameplayStats_t_k_iCallback: GSGameplayStats_t__bindgen_ty_1 = GSGameplayStats_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSGameplayStats_t__bindgen_ty_1 {
    k_iCallback = 207,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSGameplayStats_t"][::std::mem::size_of::<GSGameplayStats_t>() - 16usize];
    [
        "Alignment of GSGameplayStats_t",
    ][::std::mem::align_of::<GSGameplayStats_t>() - 4usize];
    [
        "Offset of field: GSGameplayStats_t::m_eResult",
    ][::std::mem::offset_of!(GSGameplayStats_t, m_eResult) - 0usize];
    [
        "Offset of field: GSGameplayStats_t::m_nRank",
    ][::std::mem::offset_of!(GSGameplayStats_t, m_nRank) - 4usize];
    [
        "Offset of field: GSGameplayStats_t::m_unTotalConnects",
    ][::std::mem::offset_of!(GSGameplayStats_t, m_unTotalConnects) - 8usize];
    [
        "Offset of field: GSGameplayStats_t::m_unTotalMinutesPlayed",
    ][::std::mem::offset_of!(GSGameplayStats_t, m_unTotalMinutesPlayed) - 12usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSClientGroupStatus_t {
    pub m_SteamIDUser: CSteamID,
    pub m_SteamIDGroup: CSteamID,
    pub m_bMember: bool,
    pub m_bOfficer: bool,
}
pub const GSClientGroupStatus_t_k_iCallback: GSClientGroupStatus_t__bindgen_ty_1 = GSClientGroupStatus_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSClientGroupStatus_t__bindgen_ty_1 {
    k_iCallback = 208,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of GSClientGroupStatus_t",
    ][::std::mem::size_of::<GSClientGroupStatus_t>() - 18usize];
    [
        "Alignment of GSClientGroupStatus_t",
    ][::std::mem::align_of::<GSClientGroupStatus_t>() - 1usize];
    [
        "Offset of field: GSClientGroupStatus_t::m_SteamIDUser",
    ][::std::mem::offset_of!(GSClientGroupStatus_t, m_SteamIDUser) - 0usize];
    [
        "Offset of field: GSClientGroupStatus_t::m_SteamIDGroup",
    ][::std::mem::offset_of!(GSClientGroupStatus_t, m_SteamIDGroup) - 8usize];
    [
        "Offset of field: GSClientGroupStatus_t::m_bMember",
    ][::std::mem::offset_of!(GSClientGroupStatus_t, m_bMember) - 16usize];
    [
        "Offset of field: GSClientGroupStatus_t::m_bOfficer",
    ][::std::mem::offset_of!(GSClientGroupStatus_t, m_bOfficer) - 17usize];
};
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct GSReputation_t {
    pub m_eResult: EResult,
    pub m_unReputationScore: uint32,
    pub m_bBanned: bool,
    pub m_unBannedIP: uint32,
    pub m_usBannedPort: uint16,
    pub m_ulBannedGameID: uint64,
    pub m_unBanExpires: uint32,
}
pub const GSReputation_t_k_iCallback: GSReputation_t__bindgen_ty_1 = GSReputation_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSReputation_t__bindgen_ty_1 {
    k_iCallback = 209,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSReputation_t"][::std::mem::size_of::<GSReputation_t>() - 32usize];
    ["Alignment of GSReputation_t"][::std::mem::align_of::<GSReputation_t>() - 4usize];
    [
        "Offset of field: GSReputation_t::m_eResult",
    ][::std::mem::offset_of!(GSReputation_t, m_eResult) - 0usize];
    [
        "Offset of field: GSReputation_t::m_unReputationScore",
    ][::std::mem::offset_of!(GSReputation_t, m_unReputationScore) - 4usize];
    [
        "Offset of field: GSReputation_t::m_bBanned",
    ][::std::mem::offset_of!(GSReputation_t, m_bBanned) - 8usize];
    [
        "Offset of field: GSReputation_t::m_unBannedIP",
    ][::std::mem::offset_of!(GSReputation_t, m_unBannedIP) - 12usize];
    [
        "Offset of field: GSReputation_t::m_usBannedPort",
    ][::std::mem::offset_of!(GSReputation_t, m_usBannedPort) - 16usize];
    [
        "Offset of field: GSReputation_t::m_ulBannedGameID",
    ][::std::mem::offset_of!(GSReputation_t, m_ulBannedGameID) - 20usize];
    [
        "Offset of field: GSReputation_t::m_unBanExpires",
    ][::std::mem::offset_of!(GSReputation_t, m_unBanExpires) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AssociateWithClanResult_t {
    pub m_eResult: EResult,
}
pub const AssociateWithClanResult_t_k_iCallback: AssociateWithClanResult_t__bindgen_ty_1 = AssociateWithClanResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum AssociateWithClanResult_t__bindgen_ty_1 {
    k_iCallback = 210,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of AssociateWithClanResult_t",
    ][::std::mem::size_of::<AssociateWithClanResult_t>() - 4usize];
    [
        "Alignment of AssociateWithClanResult_t",
    ][::std::mem::align_of::<AssociateWithClanResult_t>() - 4usize];
    [
        "Offset of field: AssociateWithClanResult_t::m_eResult",
    ][::std::mem::offset_of!(AssociateWithClanResult_t, m_eResult) - 0usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ComputeNewPlayerCompatibilityResult_t {
    pub m_eResult: EResult,
    pub m_cPlayersThatDontLikeCandidate: ::std::os::raw::c_int,
    pub m_cPlayersThatCandidateDoesntLike: ::std::os::raw::c_int,
    pub m_cClanPlayersThatDontLikeCandidate: ::std::os::raw::c_int,
    pub m_SteamIDCandidate: CSteamID,
}
pub const ComputeNewPlayerCompatibilityResult_t_k_iCallback: ComputeNewPlayerCompatibilityResult_t__bindgen_ty_1 = ComputeNewPlayerCompatibilityResult_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ComputeNewPlayerCompatibilityResult_t__bindgen_ty_1 {
    k_iCallback = 211,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ComputeNewPlayerCompatibilityResult_t",
    ][::std::mem::size_of::<ComputeNewPlayerCompatibilityResult_t>() - 24usize];
    [
        "Alignment of ComputeNewPlayerCompatibilityResult_t",
    ][::std::mem::align_of::<ComputeNewPlayerCompatibilityResult_t>() - 4usize];
    [
        "Offset of field: ComputeNewPlayerCompatibilityResult_t::m_eResult",
    ][::std::mem::offset_of!(ComputeNewPlayerCompatibilityResult_t, m_eResult) - 0usize];
    [
        "Offset of field: ComputeNewPlayerCompatibilityResult_t::m_cPlayersThatDontLikeCandidate",
    ][::std::mem::offset_of!(
        ComputeNewPlayerCompatibilityResult_t, m_cPlayersThatDontLikeCandidate
    ) - 4usize];
    [
        "Offset of field: ComputeNewPlayerCompatibilityResult_t::m_cPlayersThatCandidateDoesntLike",
    ][::std::mem::offset_of!(
        ComputeNewPlayerCompatibilityResult_t, m_cPlayersThatCandidateDoesntLike
    ) - 8usize];
    [
        "Offset of field: ComputeNewPlayerCompatibilityResult_t::m_cClanPlayersThatDontLikeCandidate",
    ][::std::mem::offset_of!(
        ComputeNewPlayerCompatibilityResult_t, m_cClanPlayersThatDontLikeCandidate
    ) - 12usize];
    [
        "Offset of field: ComputeNewPlayerCompatibilityResult_t::m_SteamIDCandidate",
    ][::std::mem::offset_of!(ComputeNewPlayerCompatibilityResult_t, m_SteamIDCandidate)
        - 16usize];
};
#[repr(C)]
pub struct ISteamGameServerStats__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamGameServerStats {
    pub vtable_: *const ISteamGameServerStats__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of ISteamGameServerStats",
    ][::std::mem::size_of::<ISteamGameServerStats>() - 8usize];
    [
        "Alignment of ISteamGameServerStats",
    ][::std::mem::align_of::<ISteamGameServerStats>() - 8usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSStatsReceived_t {
    pub m_eResult: EResult,
    pub m_steamIDUser: CSteamID,
}
pub const GSStatsReceived_t_k_iCallback: GSStatsReceived_t__bindgen_ty_1 = GSStatsReceived_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSStatsReceived_t__bindgen_ty_1 {
    k_iCallback = 1800,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSStatsReceived_t"][::std::mem::size_of::<GSStatsReceived_t>() - 12usize];
    [
        "Alignment of GSStatsReceived_t",
    ][::std::mem::align_of::<GSStatsReceived_t>() - 4usize];
    [
        "Offset of field: GSStatsReceived_t::m_eResult",
    ][::std::mem::offset_of!(GSStatsReceived_t, m_eResult) - 0usize];
    [
        "Offset of field: GSStatsReceived_t::m_steamIDUser",
    ][::std::mem::offset_of!(GSStatsReceived_t, m_steamIDUser) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSStatsStored_t {
    pub m_eResult: EResult,
    pub m_steamIDUser: CSteamID,
}
pub const GSStatsStored_t_k_iCallback: GSStatsStored_t__bindgen_ty_1 = GSStatsStored_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSStatsStored_t__bindgen_ty_1 {
    k_iCallback = 1801,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSStatsStored_t"][::std::mem::size_of::<GSStatsStored_t>() - 12usize];
    ["Alignment of GSStatsStored_t"][::std::mem::align_of::<GSStatsStored_t>() - 4usize];
    [
        "Offset of field: GSStatsStored_t::m_eResult",
    ][::std::mem::offset_of!(GSStatsStored_t, m_eResult) - 0usize];
    [
        "Offset of field: GSStatsStored_t::m_steamIDUser",
    ][::std::mem::offset_of!(GSStatsStored_t, m_steamIDUser) - 4usize];
};
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GSStatsUnloaded_t {
    pub m_steamIDUser: CSteamID,
}
pub const GSStatsUnloaded_t_k_iCallback: GSStatsUnloaded_t__bindgen_ty_1 = GSStatsUnloaded_t__bindgen_ty_1::k_iCallback;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GSStatsUnloaded_t__bindgen_ty_1 {
    k_iCallback = 1108,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of GSStatsUnloaded_t"][::std::mem::size_of::<GSStatsUnloaded_t>() - 8usize];
    [
        "Alignment of GSStatsUnloaded_t",
    ][::std::mem::align_of::<GSStatsUnloaded_t>() - 1usize];
    [
        "Offset of field: GSStatsUnloaded_t::m_steamIDUser",
    ][::std::mem::offset_of!(GSStatsUnloaded_t, m_steamIDUser) - 0usize];
};
pub type uint64_steamid = uint64;
pub type uint64_gameid = uint64;
unsafe extern "C" {
    pub fn SteamAPI_ISteamClient_CreateSteamPipe(self_: *mut ISteamClient) -> HSteamPipe;
    pub fn SteamAPI_ISteamClient_BReleaseSteamPipe(
        self_: *mut ISteamClient,
        hSteamPipe: HSteamPipe,
    ) -> bool;
    pub fn SteamAPI_ISteamClient_ConnectToGlobalUser(
        self_: *mut ISteamClient,
        hSteamPipe: HSteamPipe,
    ) -> HSteamUser;
    pub fn SteamAPI_ISteamClient_CreateLocalUser(
        self_: *mut ISteamClient,
        phSteamPipe: *mut HSteamPipe,
        eAccountType: EAccountType,
    ) -> HSteamUser;
    pub fn SteamAPI_ISteamClient_ReleaseUser(
        self_: *mut ISteamClient,
        hSteamPipe: HSteamPipe,
        hUser: HSteamUser,
    );
    pub fn SteamAPI_ISteamClient_GetISteamUser(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamUser;
    pub fn SteamAPI_ISteamClient_GetISteamGameServer(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamGameServer;
    pub fn SteamAPI_ISteamClient_SetLocalIPBinding(
        self_: *mut ISteamClient,
        unIP: *const SteamIPAddress_t,
        usPort: uint16,
    );
    pub fn SteamAPI_ISteamClient_GetISteamFriends(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamFriends;
    pub fn SteamAPI_ISteamClient_GetISteamUtils(
        self_: *mut ISteamClient,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamUtils;
    pub fn SteamAPI_ISteamClient_GetISteamMatchmaking(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamMatchmaking;
    pub fn SteamAPI_ISteamClient_GetISteamMatchmakingServers(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamMatchmakingServers;
    pub fn SteamAPI_ISteamClient_GetISteamGenericInterface(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void;
    pub fn SteamAPI_ISteamClient_GetISteamUserStats(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamUserStats;
    pub fn SteamAPI_ISteamClient_GetISteamGameServerStats(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamGameServerStats;
    pub fn SteamAPI_ISteamClient_GetISteamApps(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamApps;
    pub fn SteamAPI_ISteamClient_GetISteamNetworking(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamNetworking;
    pub fn SteamAPI_ISteamClient_GetISteamRemoteStorage(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamRemoteStorage;
    pub fn SteamAPI_ISteamClient_GetISteamScreenshots(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamScreenshots;
    pub fn SteamAPI_ISteamClient_GetISteamGameSearch(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamGameSearch;
    pub fn SteamAPI_ISteamClient_GetIPCCallCount(self_: *mut ISteamClient) -> uint32;
    pub fn SteamAPI_ISteamClient_SetWarningMessageHook(
        self_: *mut ISteamClient,
        pFunction: SteamAPIWarningMessageHook_t,
    );
    pub fn SteamAPI_ISteamClient_BShutdownIfAllPipesClosed(
        self_: *mut ISteamClient,
    ) -> bool;
    pub fn SteamAPI_ISteamClient_GetISteamHTTP(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamHTTP;
    pub fn SteamAPI_ISteamClient_GetISteamController(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamController;
    pub fn SteamAPI_ISteamClient_GetISteamUGC(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamUGC;
    pub fn SteamAPI_ISteamClient_GetISteamMusic(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamMusic;
    pub fn SteamAPI_ISteamClient_GetISteamMusicRemote(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamMusicRemote;
    pub fn SteamAPI_ISteamClient_GetISteamHTMLSurface(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamHTMLSurface;
    pub fn SteamAPI_ISteamClient_GetISteamInventory(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamInventory;
    pub fn SteamAPI_ISteamClient_GetISteamVideo(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamVideo;
    pub fn SteamAPI_ISteamClient_GetISteamParentalSettings(
        self_: *mut ISteamClient,
        hSteamuser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamParentalSettings;
    pub fn SteamAPI_ISteamClient_GetISteamInput(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamInput;
    pub fn SteamAPI_ISteamClient_GetISteamParties(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamParties;
    pub fn SteamAPI_ISteamClient_GetISteamRemotePlay(
        self_: *mut ISteamClient,
        hSteamUser: HSteamUser,
        hSteamPipe: HSteamPipe,
        pchVersion: *const ::std::os::raw::c_char,
    ) -> *mut ISteamRemotePlay;
    pub fn SteamAPI_SteamUser_v023() -> *mut ISteamUser;
    pub fn SteamAPI_ISteamUser_GetHSteamUser(self_: *mut ISteamUser) -> HSteamUser;
    pub fn SteamAPI_ISteamUser_BLoggedOn(self_: *mut ISteamUser) -> bool;
    pub fn SteamAPI_ISteamUser_GetSteamID(self_: *mut ISteamUser) -> uint64_steamid;
    pub fn SteamAPI_ISteamUser_InitiateGameConnection_DEPRECATED(
        self_: *mut ISteamUser,
        pAuthBlob: *mut ::std::os::raw::c_void,
        cbMaxAuthBlob: ::std::os::raw::c_int,
        steamIDGameServer: uint64_steamid,
        unIPServer: uint32,
        usPortServer: uint16,
        bSecure: bool,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUser_TerminateGameConnection_DEPRECATED(
        self_: *mut ISteamUser,
        unIPServer: uint32,
        usPortServer: uint16,
    );
    pub fn SteamAPI_ISteamUser_TrackAppUsageEvent(
        self_: *mut ISteamUser,
        gameID: uint64_gameid,
        eAppUsageEvent: ::std::os::raw::c_int,
        pchExtraInfo: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamUser_GetUserDataFolder(
        self_: *mut ISteamUser,
        pchBuffer: *mut ::std::os::raw::c_char,
        cubBuffer: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamUser_StartVoiceRecording(self_: *mut ISteamUser);
    pub fn SteamAPI_ISteamUser_StopVoiceRecording(self_: *mut ISteamUser);
    pub fn SteamAPI_ISteamUser_GetAvailableVoice(
        self_: *mut ISteamUser,
        pcbCompressed: *mut uint32,
        pcbUncompressed_Deprecated: *mut uint32,
        nUncompressedVoiceDesiredSampleRate_Deprecated: uint32,
    ) -> EVoiceResult;
    pub fn SteamAPI_ISteamUser_GetVoice(
        self_: *mut ISteamUser,
        bWantCompressed: bool,
        pDestBuffer: *mut ::std::os::raw::c_void,
        cbDestBufferSize: uint32,
        nBytesWritten: *mut uint32,
        bWantUncompressed_Deprecated: bool,
        pUncompressedDestBuffer_Deprecated: *mut ::std::os::raw::c_void,
        cbUncompressedDestBufferSize_Deprecated: uint32,
        nUncompressBytesWritten_Deprecated: *mut uint32,
        nUncompressedVoiceDesiredSampleRate_Deprecated: uint32,
    ) -> EVoiceResult;
    pub fn SteamAPI_ISteamUser_DecompressVoice(
        self_: *mut ISteamUser,
        pCompressed: *const ::std::os::raw::c_void,
        cbCompressed: uint32,
        pDestBuffer: *mut ::std::os::raw::c_void,
        cbDestBufferSize: uint32,
        nBytesWritten: *mut uint32,
        nDesiredSampleRate: uint32,
    ) -> EVoiceResult;
    pub fn SteamAPI_ISteamUser_GetVoiceOptimalSampleRate(
        self_: *mut ISteamUser,
    ) -> uint32;
    pub fn SteamAPI_ISteamUser_GetAuthSessionTicket(
        self_: *mut ISteamUser,
        pTicket: *mut ::std::os::raw::c_void,
        cbMaxTicket: ::std::os::raw::c_int,
        pcbTicket: *mut uint32,
        pSteamNetworkingIdentity: *const SteamNetworkingIdentity,
    ) -> HAuthTicket;
    pub fn SteamAPI_ISteamUser_GetAuthTicketForWebApi(
        self_: *mut ISteamUser,
        pchIdentity: *const ::std::os::raw::c_char,
    ) -> HAuthTicket;
    pub fn SteamAPI_ISteamUser_BeginAuthSession(
        self_: *mut ISteamUser,
        pAuthTicket: *const ::std::os::raw::c_void,
        cbAuthTicket: ::std::os::raw::c_int,
        steamID: uint64_steamid,
    ) -> EBeginAuthSessionResult;
    pub fn SteamAPI_ISteamUser_EndAuthSession(
        self_: *mut ISteamUser,
        steamID: uint64_steamid,
    );
    pub fn SteamAPI_ISteamUser_CancelAuthTicket(
        self_: *mut ISteamUser,
        hAuthTicket: HAuthTicket,
    );
    pub fn SteamAPI_ISteamUser_UserHasLicenseForApp(
        self_: *mut ISteamUser,
        steamID: uint64_steamid,
        appID: AppId_t,
    ) -> EUserHasLicenseForAppResult;
    pub fn SteamAPI_ISteamUser_BIsBehindNAT(self_: *mut ISteamUser) -> bool;
    pub fn SteamAPI_ISteamUser_AdvertiseGame(
        self_: *mut ISteamUser,
        steamIDGameServer: uint64_steamid,
        unIPServer: uint32,
        usPortServer: uint16,
    );
    pub fn SteamAPI_ISteamUser_RequestEncryptedAppTicket(
        self_: *mut ISteamUser,
        pDataToInclude: *mut ::std::os::raw::c_void,
        cbDataToInclude: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUser_GetEncryptedAppTicket(
        self_: *mut ISteamUser,
        pTicket: *mut ::std::os::raw::c_void,
        cbMaxTicket: ::std::os::raw::c_int,
        pcbTicket: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUser_GetGameBadgeLevel(
        self_: *mut ISteamUser,
        nSeries: ::std::os::raw::c_int,
        bFoil: bool,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUser_GetPlayerSteamLevel(
        self_: *mut ISteamUser,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUser_RequestStoreAuthURL(
        self_: *mut ISteamUser,
        pchRedirectURL: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUser_BIsPhoneVerified(self_: *mut ISteamUser) -> bool;
    pub fn SteamAPI_ISteamUser_BIsTwoFactorEnabled(self_: *mut ISteamUser) -> bool;
    pub fn SteamAPI_ISteamUser_BIsPhoneIdentifying(self_: *mut ISteamUser) -> bool;
    pub fn SteamAPI_ISteamUser_BIsPhoneRequiringVerification(
        self_: *mut ISteamUser,
    ) -> bool;
    pub fn SteamAPI_ISteamUser_GetMarketEligibility(
        self_: *mut ISteamUser,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUser_GetDurationControl(
        self_: *mut ISteamUser,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUser_BSetDurationControlOnlineState(
        self_: *mut ISteamUser,
        eNewState: EDurationControlOnlineState,
    ) -> bool;
    pub fn SteamAPI_SteamFriends_v018() -> *mut ISteamFriends;
    pub fn SteamAPI_ISteamFriends_GetPersonaName(
        self_: *mut ISteamFriends,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetPersonaState(
        self_: *mut ISteamFriends,
    ) -> EPersonaState;
    pub fn SteamAPI_ISteamFriends_GetFriendCount(
        self_: *mut ISteamFriends,
        iFriendFlags: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendByIndex(
        self_: *mut ISteamFriends,
        iFriend: ::std::os::raw::c_int,
        iFriendFlags: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_GetFriendRelationship(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> EFriendRelationship;
    pub fn SteamAPI_ISteamFriends_GetFriendPersonaState(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> EPersonaState;
    pub fn SteamAPI_ISteamFriends_GetFriendPersonaName(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetFriendGamePlayed(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        pFriendGameInfo: *mut FriendGameInfo_t,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetFriendPersonaNameHistory(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        iPersonaName: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetFriendSteamLevel(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetPlayerNickname(
        self_: *mut ISteamFriends,
        steamIDPlayer: uint64_steamid,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetFriendsGroupCount(
        self_: *mut ISteamFriends,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendsGroupIDByIndex(
        self_: *mut ISteamFriends,
        iFG: ::std::os::raw::c_int,
    ) -> FriendsGroupID_t;
    pub fn SteamAPI_ISteamFriends_GetFriendsGroupName(
        self_: *mut ISteamFriends,
        friendsGroupID: FriendsGroupID_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetFriendsGroupMembersCount(
        self_: *mut ISteamFriends,
        friendsGroupID: FriendsGroupID_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendsGroupMembersList(
        self_: *mut ISteamFriends,
        friendsGroupID: FriendsGroupID_t,
        pOutSteamIDMembers: *mut CSteamID,
        nMembersCount: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamFriends_HasFriend(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        iFriendFlags: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetClanCount(
        self_: *mut ISteamFriends,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetClanByIndex(
        self_: *mut ISteamFriends,
        iClan: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_GetClanName(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetClanTag(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetClanActivityCounts(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
        pnOnline: *mut ::std::os::raw::c_int,
        pnInGame: *mut ::std::os::raw::c_int,
        pnChatting: *mut ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_DownloadClanActivityCounts(
        self_: *mut ISteamFriends,
        psteamIDClans: *mut CSteamID,
        cClansToRequest: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_GetFriendCountFromSource(
        self_: *mut ISteamFriends,
        steamIDSource: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendFromSourceByIndex(
        self_: *mut ISteamFriends,
        steamIDSource: uint64_steamid,
        iFriend: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_IsUserInSource(
        self_: *mut ISteamFriends,
        steamIDUser: uint64_steamid,
        steamIDSource: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(
        self_: *mut ISteamFriends,
        steamIDUser: uint64_steamid,
        bSpeaking: bool,
    );
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlay(
        self_: *mut ISteamFriends,
        pchDialog: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayToUser(
        self_: *mut ISteamFriends,
        pchDialog: *const ::std::os::raw::c_char,
        steamID: uint64_steamid,
    );
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayToWebPage(
        self_: *mut ISteamFriends,
        pchURL: *const ::std::os::raw::c_char,
        eMode: EActivateGameOverlayToWebPageMode,
    );
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayToStore(
        self_: *mut ISteamFriends,
        nAppID: AppId_t,
        eFlag: EOverlayToStoreFlag,
    );
    pub fn SteamAPI_ISteamFriends_SetPlayedWith(
        self_: *mut ISteamFriends,
        steamIDUserPlayedWith: uint64_steamid,
    );
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialog(
        self_: *mut ISteamFriends,
        steamIDLobby: uint64_steamid,
    );
    pub fn SteamAPI_ISteamFriends_GetSmallFriendAvatar(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetMediumFriendAvatar(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetLargeFriendAvatar(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_RequestUserInformation(
        self_: *mut ISteamFriends,
        steamIDUser: uint64_steamid,
        bRequireNameOnly: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_RequestClanOfficerList(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_GetClanOwner(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_GetClanOfficerCount(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetClanOfficerByIndex(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
        iOfficer: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_SetRichPresence(
        self_: *mut ISteamFriends,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_ClearRichPresence(self_: *mut ISteamFriends);
    pub fn SteamAPI_ISteamFriends_GetFriendRichPresence(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetFriendRichPresenceKeyCount(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendRichPresenceKeyByIndex(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        iKey: ::std::os::raw::c_int,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_RequestFriendRichPresence(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    );
    pub fn SteamAPI_ISteamFriends_InviteUserToGame(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        pchConnectString: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetCoplayFriendCount(
        self_: *mut ISteamFriends,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetCoplayFriend(
        self_: *mut ISteamFriends,
        iCoplayFriend: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_GetFriendCoplayTime(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFriendCoplayGame(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
    ) -> AppId_t;
    pub fn SteamAPI_ISteamFriends_JoinClanChatRoom(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_LeaveClanChatRoom(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetClanChatMemberCount(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetChatMemberByIndex(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
        iUser: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamFriends_SendClanChatMessage(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
        pchText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetClanChatMessage(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
        iMessage: ::std::os::raw::c_int,
        prgchText: *mut ::std::os::raw::c_void,
        cchTextMax: ::std::os::raw::c_int,
        peChatEntryType: *mut EChatEntryType,
        psteamidChatter: *mut CSteamID,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_IsClanChatAdmin(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
        steamIDUser: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_IsClanChatWindowOpenInSteam(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_OpenClanChatWindowInSteam(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_CloseClanChatWindowInSteam(
        self_: *mut ISteamFriends,
        steamIDClanChat: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_SetListenForFriendsMessages(
        self_: *mut ISteamFriends,
        bInterceptEnabled: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_ReplyToFriendMessage(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        pchMsgToSend: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetFriendMessage(
        self_: *mut ISteamFriends,
        steamIDFriend: uint64_steamid,
        iMessageID: ::std::os::raw::c_int,
        pvData: *mut ::std::os::raw::c_void,
        cubData: ::std::os::raw::c_int,
        peChatEntryType: *mut EChatEntryType,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_GetFollowerCount(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_IsFollowing(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_EnumerateFollowingList(
        self_: *mut ISteamFriends,
        unStartIndex: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_IsClanPublic(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_IsClanOfficialGameGroup(
        self_: *mut ISteamFriends,
        steamIDClan: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetNumChatsWithUnreadPriorityMessages(
        self_: *mut ISteamFriends,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayRemotePlayTogetherInviteDialog(
        self_: *mut ISteamFriends,
        steamIDLobby: uint64_steamid,
    );
    pub fn SteamAPI_ISteamFriends_RegisterProtocolInOverlayBrowser(
        self_: *mut ISteamFriends,
        pchProtocol: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialogConnectString(
        self_: *mut ISteamFriends,
        pchConnectString: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamFriends_RequestEquippedProfileItems(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamFriends_BHasEquippedProfileItem(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
        itemType: ECommunityProfileItemType,
    ) -> bool;
    pub fn SteamAPI_ISteamFriends_GetProfileItemPropertyString(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
        itemType: ECommunityProfileItemType,
        prop: ECommunityProfileItemProperty,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamFriends_GetProfileItemPropertyUint(
        self_: *mut ISteamFriends,
        steamID: uint64_steamid,
        itemType: ECommunityProfileItemType,
        prop: ECommunityProfileItemProperty,
    ) -> uint32;
    pub fn SteamAPI_SteamUtils_v010() -> *mut ISteamUtils;
    pub fn SteamAPI_SteamGameServerUtils_v010() -> *mut ISteamUtils;
    pub fn SteamAPI_ISteamUtils_GetSecondsSinceAppActive(
        self_: *mut ISteamUtils,
    ) -> uint32;
    pub fn SteamAPI_ISteamUtils_GetSecondsSinceComputerActive(
        self_: *mut ISteamUtils,
    ) -> uint32;
    pub fn SteamAPI_ISteamUtils_GetConnectedUniverse(
        self_: *mut ISteamUtils,
    ) -> EUniverse;
    pub fn SteamAPI_ISteamUtils_GetServerRealTime(self_: *mut ISteamUtils) -> uint32;
    pub fn SteamAPI_ISteamUtils_GetIPCountry(
        self_: *mut ISteamUtils,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamUtils_GetImageSize(
        self_: *mut ISteamUtils,
        iImage: ::std::os::raw::c_int,
        pnWidth: *mut uint32,
        pnHeight: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetImageRGBA(
        self_: *mut ISteamUtils,
        iImage: ::std::os::raw::c_int,
        pubDest: *mut uint8,
        nDestBufferSize: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetCurrentBatteryPower(self_: *mut ISteamUtils) -> uint8;
    pub fn SteamAPI_ISteamUtils_GetAppID(self_: *mut ISteamUtils) -> uint32;
    pub fn SteamAPI_ISteamUtils_SetOverlayNotificationPosition(
        self_: *mut ISteamUtils,
        eNotificationPosition: ENotificationPosition,
    );
    pub fn SteamAPI_ISteamUtils_IsAPICallCompleted(
        self_: *mut ISteamUtils,
        hSteamAPICall: SteamAPICall_t,
        pbFailed: *mut bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetAPICallFailureReason(
        self_: *mut ISteamUtils,
        hSteamAPICall: SteamAPICall_t,
    ) -> ESteamAPICallFailure;
    pub fn SteamAPI_ISteamUtils_GetAPICallResult(
        self_: *mut ISteamUtils,
        hSteamAPICall: SteamAPICall_t,
        pCallback: *mut ::std::os::raw::c_void,
        cubCallback: ::std::os::raw::c_int,
        iCallbackExpected: ::std::os::raw::c_int,
        pbFailed: *mut bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetIPCCallCount(self_: *mut ISteamUtils) -> uint32;
    pub fn SteamAPI_ISteamUtils_SetWarningMessageHook(
        self_: *mut ISteamUtils,
        pFunction: SteamAPIWarningMessageHook_t,
    );
    pub fn SteamAPI_ISteamUtils_IsOverlayEnabled(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_ISteamUtils_BOverlayNeedsPresent(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_ISteamUtils_CheckFileSignature(
        self_: *mut ISteamUtils,
        szFileName: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUtils_ShowGamepadTextInput(
        self_: *mut ISteamUtils,
        eInputMode: EGamepadTextInputMode,
        eLineInputMode: EGamepadTextInputLineMode,
        pchDescription: *const ::std::os::raw::c_char,
        unCharMax: uint32,
        pchExistingText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetEnteredGamepadTextLength(
        self_: *mut ISteamUtils,
    ) -> uint32;
    pub fn SteamAPI_ISteamUtils_GetEnteredGamepadTextInput(
        self_: *mut ISteamUtils,
        pchText: *mut ::std::os::raw::c_char,
        cchText: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_GetSteamUILanguage(
        self_: *mut ISteamUtils,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamUtils_IsSteamRunningInVR(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_ISteamUtils_SetOverlayNotificationInset(
        self_: *mut ISteamUtils,
        nHorizontalInset: ::std::os::raw::c_int,
        nVerticalInset: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamUtils_IsSteamInBigPictureMode(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_ISteamUtils_StartVRDashboard(self_: *mut ISteamUtils);
    pub fn SteamAPI_ISteamUtils_IsVRHeadsetStreamingEnabled(
        self_: *mut ISteamUtils,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_SetVRHeadsetStreamingEnabled(
        self_: *mut ISteamUtils,
        bEnabled: bool,
    );
    pub fn SteamAPI_ISteamUtils_IsSteamChinaLauncher(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_ISteamUtils_InitFilterText(
        self_: *mut ISteamUtils,
        unFilterOptions: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_FilterText(
        self_: *mut ISteamUtils,
        eContext: ETextFilteringContext,
        sourceSteamID: uint64_steamid,
        pchInputMessage: *const ::std::os::raw::c_char,
        pchOutFilteredText: *mut ::std::os::raw::c_char,
        nByteSizeOutFilteredText: uint32,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUtils_GetIPv6ConnectivityState(
        self_: *mut ISteamUtils,
        eProtocol: ESteamIPv6ConnectivityProtocol,
    ) -> ESteamIPv6ConnectivityState;
    pub fn SteamAPI_ISteamUtils_IsSteamRunningOnSteamDeck(
        self_: *mut ISteamUtils,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_ShowFloatingGamepadTextInput(
        self_: *mut ISteamUtils,
        eKeyboardMode: EFloatingGamepadTextInputMode,
        nTextFieldXPosition: ::std::os::raw::c_int,
        nTextFieldYPosition: ::std::os::raw::c_int,
        nTextFieldWidth: ::std::os::raw::c_int,
        nTextFieldHeight: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_SetGameLauncherMode(
        self_: *mut ISteamUtils,
        bLauncherMode: bool,
    );
    pub fn SteamAPI_ISteamUtils_DismissFloatingGamepadTextInput(
        self_: *mut ISteamUtils,
    ) -> bool;
    pub fn SteamAPI_ISteamUtils_DismissGamepadTextInput(self_: *mut ISteamUtils) -> bool;
    pub fn SteamAPI_SteamMatchmaking_v009() -> *mut ISteamMatchmaking;
    pub fn SteamAPI_ISteamMatchmaking_GetFavoriteGameCount(
        self_: *mut ISteamMatchmaking,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_GetFavoriteGame(
        self_: *mut ISteamMatchmaking,
        iGame: ::std::os::raw::c_int,
        pnAppID: *mut AppId_t,
        pnIP: *mut uint32,
        pnConnPort: *mut uint16,
        pnQueryPort: *mut uint16,
        punFlags: *mut uint32,
        pRTime32LastPlayedOnServer: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_AddFavoriteGame(
        self_: *mut ISteamMatchmaking,
        nAppID: AppId_t,
        nIP: uint32,
        nConnPort: uint16,
        nQueryPort: uint16,
        unFlags: uint32,
        rTime32LastPlayedOnServer: uint32,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_RemoveFavoriteGame(
        self_: *mut ISteamMatchmaking,
        nAppID: AppId_t,
        nIP: uint32,
        nConnPort: uint16,
        nQueryPort: uint16,
        unFlags: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_RequestLobbyList(
        self_: *mut ISteamMatchmaking,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListStringFilter(
        self_: *mut ISteamMatchmaking,
        pchKeyToMatch: *const ::std::os::raw::c_char,
        pchValueToMatch: *const ::std::os::raw::c_char,
        eComparisonType: ELobbyComparison,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListNumericalFilter(
        self_: *mut ISteamMatchmaking,
        pchKeyToMatch: *const ::std::os::raw::c_char,
        nValueToMatch: ::std::os::raw::c_int,
        eComparisonType: ELobbyComparison,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListNearValueFilter(
        self_: *mut ISteamMatchmaking,
        pchKeyToMatch: *const ::std::os::raw::c_char,
        nValueToBeCloseTo: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListFilterSlotsAvailable(
        self_: *mut ISteamMatchmaking,
        nSlotsAvailable: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListDistanceFilter(
        self_: *mut ISteamMatchmaking,
        eLobbyDistanceFilter: ELobbyDistanceFilter,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListResultCountFilter(
        self_: *mut ISteamMatchmaking,
        cMaxResults: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmaking_AddRequestLobbyListCompatibleMembersFilter(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    );
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyByIndex(
        self_: *mut ISteamMatchmaking,
        iLobby: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamMatchmaking_CreateLobby(
        self_: *mut ISteamMatchmaking,
        eLobbyType: ELobbyType,
        cMaxMembers: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamMatchmaking_JoinLobby(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamMatchmaking_LeaveLobby(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    );
    pub fn SteamAPI_ISteamMatchmaking_InviteUserToLobby(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        steamIDInvitee: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetNumLobbyMembers(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyMemberByIndex(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        iMember: ::std::os::raw::c_int,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyDataCount(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyDataByIndex(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        iLobbyData: ::std::os::raw::c_int,
        pchKey: *mut ::std::os::raw::c_char,
        cchKeyBufferSize: ::std::os::raw::c_int,
        pchValue: *mut ::std::os::raw::c_char,
        cchValueBufferSize: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_DeleteLobbyData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyMemberData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        steamIDUser: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyMemberData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamMatchmaking_SendLobbyChatMsg(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        pvMsgBody: *const ::std::os::raw::c_void,
        cubMsgBody: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyChatEntry(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        iChatID: ::std::os::raw::c_int,
        pSteamIDUser: *mut CSteamID,
        pvData: *mut ::std::os::raw::c_void,
        cubData: ::std::os::raw::c_int,
        peChatEntryType: *mut EChatEntryType,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_RequestLobbyData(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyGameServer(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        unGameServerIP: uint32,
        unGameServerPort: uint16,
        steamIDGameServer: uint64_steamid,
    );
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyGameServer(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        punGameServerIP: *mut uint32,
        punGameServerPort: *mut uint16,
        psteamIDGameServer: *mut CSteamID,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyMemberLimit(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        cMaxMembers: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyMemberLimit(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyType(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        eLobbyType: ELobbyType,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyJoinable(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        bLobbyJoinable: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_GetLobbyOwner(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamMatchmaking_SetLobbyOwner(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        steamIDNewOwner: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmaking_SetLinkedLobby(
        self_: *mut ISteamMatchmaking,
        steamIDLobby: uint64_steamid,
        steamIDLobbyDependent: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmakingServerListResponse_ServerResponded(
        self_: *mut ISteamMatchmakingServerListResponse,
        hRequest: HServerListRequest,
        iServer: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmakingServerListResponse_ServerFailedToRespond(
        self_: *mut ISteamMatchmakingServerListResponse,
        hRequest: HServerListRequest,
        iServer: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmakingServerListResponse_RefreshComplete(
        self_: *mut ISteamMatchmakingServerListResponse,
        hRequest: HServerListRequest,
        response: EMatchMakingServerResponse,
    );
    pub fn SteamAPI_ISteamMatchmakingPingResponse_ServerResponded(
        self_: *mut ISteamMatchmakingPingResponse,
        server: *mut gameserveritem_t,
    );
    pub fn SteamAPI_ISteamMatchmakingPingResponse_ServerFailedToRespond(
        self_: *mut ISteamMatchmakingPingResponse,
    );
    pub fn SteamAPI_ISteamMatchmakingPlayersResponse_AddPlayerToList(
        self_: *mut ISteamMatchmakingPlayersResponse,
        pchName: *const ::std::os::raw::c_char,
        nScore: ::std::os::raw::c_int,
        flTimePlayed: f32,
    );
    pub fn SteamAPI_ISteamMatchmakingPlayersResponse_PlayersFailedToRespond(
        self_: *mut ISteamMatchmakingPlayersResponse,
    );
    pub fn SteamAPI_ISteamMatchmakingPlayersResponse_PlayersRefreshComplete(
        self_: *mut ISteamMatchmakingPlayersResponse,
    );
    pub fn SteamAPI_ISteamMatchmakingRulesResponse_RulesResponded(
        self_: *mut ISteamMatchmakingRulesResponse,
        pchRule: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamMatchmakingRulesResponse_RulesFailedToRespond(
        self_: *mut ISteamMatchmakingRulesResponse,
    );
    pub fn SteamAPI_ISteamMatchmakingRulesResponse_RulesRefreshComplete(
        self_: *mut ISteamMatchmakingRulesResponse,
    );
    pub fn SteamAPI_SteamMatchmakingServers_v002() -> *mut ISteamMatchmakingServers;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestInternetServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        ppchFilters: *mut *mut MatchMakingKeyValuePair_t,
        nFilters: uint32,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestLANServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestFriendsServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        ppchFilters: *mut *mut MatchMakingKeyValuePair_t,
        nFilters: uint32,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestFavoritesServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        ppchFilters: *mut *mut MatchMakingKeyValuePair_t,
        nFilters: uint32,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestHistoryServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        ppchFilters: *mut *mut MatchMakingKeyValuePair_t,
        nFilters: uint32,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_RequestSpectatorServerList(
        self_: *mut ISteamMatchmakingServers,
        iApp: AppId_t,
        ppchFilters: *mut *mut MatchMakingKeyValuePair_t,
        nFilters: uint32,
        pRequestServersResponse: *mut ISteamMatchmakingServerListResponse,
    ) -> HServerListRequest;
    pub fn SteamAPI_ISteamMatchmakingServers_ReleaseRequest(
        self_: *mut ISteamMatchmakingServers,
        hServerListRequest: HServerListRequest,
    );
    pub fn SteamAPI_ISteamMatchmakingServers_GetServerDetails(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
        iServer: ::std::os::raw::c_int,
    ) -> *mut gameserveritem_t;
    pub fn SteamAPI_ISteamMatchmakingServers_CancelQuery(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
    );
    pub fn SteamAPI_ISteamMatchmakingServers_RefreshQuery(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
    );
    pub fn SteamAPI_ISteamMatchmakingServers_IsRefreshing(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
    ) -> bool;
    pub fn SteamAPI_ISteamMatchmakingServers_GetServerCount(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamMatchmakingServers_RefreshServer(
        self_: *mut ISteamMatchmakingServers,
        hRequest: HServerListRequest,
        iServer: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamMatchmakingServers_PingServer(
        self_: *mut ISteamMatchmakingServers,
        unIP: uint32,
        usPort: uint16,
        pRequestServersResponse: *mut ISteamMatchmakingPingResponse,
    ) -> HServerQuery;
    pub fn SteamAPI_ISteamMatchmakingServers_PlayerDetails(
        self_: *mut ISteamMatchmakingServers,
        unIP: uint32,
        usPort: uint16,
        pRequestServersResponse: *mut ISteamMatchmakingPlayersResponse,
    ) -> HServerQuery;
    pub fn SteamAPI_ISteamMatchmakingServers_ServerRules(
        self_: *mut ISteamMatchmakingServers,
        unIP: uint32,
        usPort: uint16,
        pRequestServersResponse: *mut ISteamMatchmakingRulesResponse,
    ) -> HServerQuery;
    pub fn SteamAPI_ISteamMatchmakingServers_CancelServerQuery(
        self_: *mut ISteamMatchmakingServers,
        hServerQuery: HServerQuery,
    );
    pub fn SteamAPI_SteamGameSearch_v001() -> *mut ISteamGameSearch;
    pub fn SteamAPI_ISteamGameSearch_AddGameSearchParams(
        self_: *mut ISteamGameSearch,
        pchKeyToFind: *const ::std::os::raw::c_char,
        pchValuesToFind: *const ::std::os::raw::c_char,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_SearchForGameWithLobby(
        self_: *mut ISteamGameSearch,
        steamIDLobby: uint64_steamid,
        nPlayerMin: ::std::os::raw::c_int,
        nPlayerMax: ::std::os::raw::c_int,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_SearchForGameSolo(
        self_: *mut ISteamGameSearch,
        nPlayerMin: ::std::os::raw::c_int,
        nPlayerMax: ::std::os::raw::c_int,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_AcceptGame(
        self_: *mut ISteamGameSearch,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_DeclineGame(
        self_: *mut ISteamGameSearch,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_RetrieveConnectionDetails(
        self_: *mut ISteamGameSearch,
        steamIDHost: uint64_steamid,
        pchConnectionDetails: *mut ::std::os::raw::c_char,
        cubConnectionDetails: ::std::os::raw::c_int,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_EndGameSearch(
        self_: *mut ISteamGameSearch,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_SetGameHostParams(
        self_: *mut ISteamGameSearch,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_SetConnectionDetails(
        self_: *mut ISteamGameSearch,
        pchConnectionDetails: *const ::std::os::raw::c_char,
        cubConnectionDetails: ::std::os::raw::c_int,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_RequestPlayersForGame(
        self_: *mut ISteamGameSearch,
        nPlayerMin: ::std::os::raw::c_int,
        nPlayerMax: ::std::os::raw::c_int,
        nMaxTeamSize: ::std::os::raw::c_int,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_HostConfirmGameStart(
        self_: *mut ISteamGameSearch,
        ullUniqueGameID: uint64,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_CancelRequestPlayersForGame(
        self_: *mut ISteamGameSearch,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_SubmitPlayerResult(
        self_: *mut ISteamGameSearch,
        ullUniqueGameID: uint64,
        steamIDPlayer: uint64_steamid,
        EPlayerResult: EPlayerResult_t,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_ISteamGameSearch_EndGame(
        self_: *mut ISteamGameSearch,
        ullUniqueGameID: uint64,
    ) -> EGameSearchErrorCode_t;
    pub fn SteamAPI_SteamParties_v002() -> *mut ISteamParties;
    pub fn SteamAPI_ISteamParties_GetNumActiveBeacons(
        self_: *mut ISteamParties,
    ) -> uint32;
    pub fn SteamAPI_ISteamParties_GetBeaconByIndex(
        self_: *mut ISteamParties,
        unIndex: uint32,
    ) -> PartyBeaconID_t;
    pub fn SteamAPI_ISteamParties_GetBeaconDetails(
        self_: *mut ISteamParties,
        ulBeaconID: PartyBeaconID_t,
        pSteamIDBeaconOwner: *mut CSteamID,
        pLocation: *mut SteamPartyBeaconLocation_t,
        pchMetadata: *mut ::std::os::raw::c_char,
        cchMetadata: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamParties_JoinParty(
        self_: *mut ISteamParties,
        ulBeaconID: PartyBeaconID_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamParties_GetNumAvailableBeaconLocations(
        self_: *mut ISteamParties,
        puNumLocations: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamParties_GetAvailableBeaconLocations(
        self_: *mut ISteamParties,
        pLocationList: *mut SteamPartyBeaconLocation_t,
        uMaxNumLocations: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamParties_CreateBeacon(
        self_: *mut ISteamParties,
        unOpenSlots: uint32,
        pBeaconLocation: *mut SteamPartyBeaconLocation_t,
        pchConnectString: *const ::std::os::raw::c_char,
        pchMetadata: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamParties_OnReservationCompleted(
        self_: *mut ISteamParties,
        ulBeacon: PartyBeaconID_t,
        steamIDUser: uint64_steamid,
    );
    pub fn SteamAPI_ISteamParties_CancelReservation(
        self_: *mut ISteamParties,
        ulBeacon: PartyBeaconID_t,
        steamIDUser: uint64_steamid,
    );
    pub fn SteamAPI_ISteamParties_ChangeNumOpenSlots(
        self_: *mut ISteamParties,
        ulBeacon: PartyBeaconID_t,
        unOpenSlots: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamParties_DestroyBeacon(
        self_: *mut ISteamParties,
        ulBeacon: PartyBeaconID_t,
    ) -> bool;
    pub fn SteamAPI_ISteamParties_GetBeaconLocationData(
        self_: *mut ISteamParties,
        BeaconLocation: SteamPartyBeaconLocation_t,
        eData: ESteamPartyBeaconLocationData,
        pchDataStringOut: *mut ::std::os::raw::c_char,
        cchDataStringOut: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_SteamRemoteStorage_v016() -> *mut ISteamRemoteStorage;
    pub fn SteamAPI_ISteamRemoteStorage_FileWrite(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        pvData: *const ::std::os::raw::c_void,
        cubData: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileRead(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        pvData: *mut ::std::os::raw::c_void,
        cubDataToRead: int32,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_FileWriteAsync(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        pvData: *const ::std::os::raw::c_void,
        cubData: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_FileReadAsync(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        nOffset: uint32,
        cubToRead: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_FileReadAsyncComplete(
        self_: *mut ISteamRemoteStorage,
        hReadCall: SteamAPICall_t,
        pvBuffer: *mut ::std::os::raw::c_void,
        cubToRead: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileForget(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileDelete(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileShare(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_SetSyncPlatforms(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        eRemoteStoragePlatform: ERemoteStoragePlatform,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileWriteStreamOpen(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> UGCFileWriteStreamHandle_t;
    pub fn SteamAPI_ISteamRemoteStorage_FileWriteStreamWriteChunk(
        self_: *mut ISteamRemoteStorage,
        writeHandle: UGCFileWriteStreamHandle_t,
        pvData: *const ::std::os::raw::c_void,
        cubData: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileWriteStreamClose(
        self_: *mut ISteamRemoteStorage,
        writeHandle: UGCFileWriteStreamHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileWriteStreamCancel(
        self_: *mut ISteamRemoteStorage,
        writeHandle: UGCFileWriteStreamHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FileExists(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_FilePersisted(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_GetFileSize(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_GetFileTimestamp(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> int64;
    pub fn SteamAPI_ISteamRemoteStorage_GetSyncPlatforms(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
    ) -> ERemoteStoragePlatform;
    pub fn SteamAPI_ISteamRemoteStorage_GetFileCount(
        self_: *mut ISteamRemoteStorage,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_GetFileNameAndSize(
        self_: *mut ISteamRemoteStorage,
        iFile: ::std::os::raw::c_int,
        pnFileSizeInBytes: *mut int32,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamRemoteStorage_GetQuota(
        self_: *mut ISteamRemoteStorage,
        pnTotalBytes: *mut uint64,
        puAvailableBytes: *mut uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_IsCloudEnabledForAccount(
        self_: *mut ISteamRemoteStorage,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_IsCloudEnabledForApp(
        self_: *mut ISteamRemoteStorage,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_SetCloudEnabledForApp(
        self_: *mut ISteamRemoteStorage,
        bEnabled: bool,
    );
    pub fn SteamAPI_ISteamRemoteStorage_UGCDownload(
        self_: *mut ISteamRemoteStorage,
        hContent: UGCHandle_t,
        unPriority: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_GetUGCDownloadProgress(
        self_: *mut ISteamRemoteStorage,
        hContent: UGCHandle_t,
        pnBytesDownloaded: *mut int32,
        pnBytesExpected: *mut int32,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_GetUGCDetails(
        self_: *mut ISteamRemoteStorage,
        hContent: UGCHandle_t,
        pnAppID: *mut AppId_t,
        ppchName: *mut *mut ::std::os::raw::c_char,
        pnFileSizeInBytes: *mut int32,
        pSteamIDOwner: *mut CSteamID,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UGCRead(
        self_: *mut ISteamRemoteStorage,
        hContent: UGCHandle_t,
        pvData: *mut ::std::os::raw::c_void,
        cubDataToRead: int32,
        cOffset: uint32,
        eAction: EUGCReadAction,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_GetCachedUGCCount(
        self_: *mut ISteamRemoteStorage,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_GetCachedUGCHandle(
        self_: *mut ISteamRemoteStorage,
        iCachedContent: int32,
    ) -> UGCHandle_t;
    pub fn SteamAPI_ISteamRemoteStorage_PublishWorkshopFile(
        self_: *mut ISteamRemoteStorage,
        pchFile: *const ::std::os::raw::c_char,
        pchPreviewFile: *const ::std::os::raw::c_char,
        nConsumerAppId: AppId_t,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        eVisibility: ERemoteStoragePublishedFileVisibility,
        pTags: *mut SteamParamStringArray_t,
        eWorkshopFileType: EWorkshopFileType,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_CreatePublishedFileUpdateRequest(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> PublishedFileUpdateHandle_t;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileFile(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pchFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFilePreviewFile(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pchPreviewFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTitle(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pchTitle: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileDescription(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pchDescription: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileVisibility(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        eVisibility: ERemoteStoragePublishedFileVisibility,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileTags(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pTags: *mut SteamParamStringArray_t,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_CommitPublishedFileUpdate(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_GetPublishedFileDetails(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
        unMaxSecondsOld: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_DeletePublishedFile(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_EnumerateUserPublishedFiles(
        self_: *mut ISteamRemoteStorage,
        unStartIndex: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_SubscribePublishedFile(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_EnumerateUserSubscribedFiles(
        self_: *mut ISteamRemoteStorage,
        unStartIndex: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_UnsubscribePublishedFile(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_UpdatePublishedFileSetChangeDescription(
        self_: *mut ISteamRemoteStorage,
        updateHandle: PublishedFileUpdateHandle_t,
        pchChangeDescription: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_GetPublishedItemVoteDetails(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_UpdateUserPublishedItemVote(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
        bVoteUp: bool,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_GetUserPublishedItemVoteDetails(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_EnumerateUserSharedWorkshopFiles(
        self_: *mut ISteamRemoteStorage,
        steamId: uint64_steamid,
        unStartIndex: uint32,
        pRequiredTags: *mut SteamParamStringArray_t,
        pExcludedTags: *mut SteamParamStringArray_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_PublishVideo(
        self_: *mut ISteamRemoteStorage,
        eVideoProvider: EWorkshopVideoProvider,
        pchVideoAccount: *const ::std::os::raw::c_char,
        pchVideoIdentifier: *const ::std::os::raw::c_char,
        pchPreviewFile: *const ::std::os::raw::c_char,
        nConsumerAppId: AppId_t,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        eVisibility: ERemoteStoragePublishedFileVisibility,
        pTags: *mut SteamParamStringArray_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_SetUserPublishedFileAction(
        self_: *mut ISteamRemoteStorage,
        unPublishedFileId: PublishedFileId_t,
        eAction: EWorkshopFileAction,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_EnumeratePublishedFilesByUserAction(
        self_: *mut ISteamRemoteStorage,
        eAction: EWorkshopFileAction,
        unStartIndex: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_EnumeratePublishedWorkshopFiles(
        self_: *mut ISteamRemoteStorage,
        eEnumerationType: EWorkshopEnumerationType,
        unStartIndex: uint32,
        unCount: uint32,
        unDays: uint32,
        pTags: *mut SteamParamStringArray_t,
        pUserTags: *mut SteamParamStringArray_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_UGCDownloadToLocation(
        self_: *mut ISteamRemoteStorage,
        hContent: UGCHandle_t,
        pchLocation: *const ::std::os::raw::c_char,
        unPriority: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamRemoteStorage_GetLocalFileChangeCount(
        self_: *mut ISteamRemoteStorage,
    ) -> int32;
    pub fn SteamAPI_ISteamRemoteStorage_GetLocalFileChange(
        self_: *mut ISteamRemoteStorage,
        iFile: ::std::os::raw::c_int,
        pEChangeType: *mut ERemoteStorageLocalFileChange,
        pEFilePathType: *mut ERemoteStorageFilePathType,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamRemoteStorage_BeginFileWriteBatch(
        self_: *mut ISteamRemoteStorage,
    ) -> bool;
    pub fn SteamAPI_ISteamRemoteStorage_EndFileWriteBatch(
        self_: *mut ISteamRemoteStorage,
    ) -> bool;
    pub fn SteamAPI_SteamUserStats_v013() -> *mut ISteamUserStats;
    pub fn SteamAPI_ISteamUserStats_GetStatInt32(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut int32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetStatFloat(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut f32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_SetStatInt32(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        nData: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_SetStatFloat(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        fData: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_UpdateAvgRateStat(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        flCountThisSession: f32,
        dSessionLength: f64,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetAchievement(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pbAchieved: *mut bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_SetAchievement(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_ClearAchievement(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetAchievementAndUnlockTime(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pbAchieved: *mut bool,
        punUnlockTime: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_StoreStats(self_: *mut ISteamUserStats) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetAchievementIcon(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUserStats_GetAchievementDisplayAttribute(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pchKey: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamUserStats_IndicateAchievementProgress(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        nCurProgress: uint32,
        nMaxProgress: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetNumAchievements(
        self_: *mut ISteamUserStats,
    ) -> uint32;
    pub fn SteamAPI_ISteamUserStats_GetAchievementName(
        self_: *mut ISteamUserStats,
        iAchievement: uint32,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamUserStats_RequestUserStats(
        self_: *mut ISteamUserStats,
        steamIDUser: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetUserStatInt32(
        self_: *mut ISteamUserStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut int32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetUserStatFloat(
        self_: *mut ISteamUserStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut f32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetUserAchievement(
        self_: *mut ISteamUserStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pbAchieved: *mut bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetUserAchievementAndUnlockTime(
        self_: *mut ISteamUserStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pbAchieved: *mut bool,
        punUnlockTime: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_ResetAllStats(
        self_: *mut ISteamUserStats,
        bAchievementsToo: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_FindOrCreateLeaderboard(
        self_: *mut ISteamUserStats,
        pchLeaderboardName: *const ::std::os::raw::c_char,
        eLeaderboardSortMethod: ELeaderboardSortMethod,
        eLeaderboardDisplayType: ELeaderboardDisplayType,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_FindLeaderboard(
        self_: *mut ISteamUserStats,
        pchLeaderboardName: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetLeaderboardName(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamUserStats_GetLeaderboardEntryCount(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUserStats_GetLeaderboardSortMethod(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
    ) -> ELeaderboardSortMethod;
    pub fn SteamAPI_ISteamUserStats_GetLeaderboardDisplayType(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
    ) -> ELeaderboardDisplayType;
    pub fn SteamAPI_ISteamUserStats_DownloadLeaderboardEntries(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
        eLeaderboardDataRequest: ELeaderboardDataRequest,
        nRangeStart: ::std::os::raw::c_int,
        nRangeEnd: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_DownloadLeaderboardEntriesForUsers(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
        prgUsers: *mut CSteamID,
        cUsers: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetDownloadedLeaderboardEntry(
        self_: *mut ISteamUserStats,
        hSteamLeaderboardEntries: SteamLeaderboardEntries_t,
        index: ::std::os::raw::c_int,
        pLeaderboardEntry: *mut LeaderboardEntry_t,
        pDetails: *mut int32,
        cDetailsMax: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_UploadLeaderboardScore(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
        eLeaderboardUploadScoreMethod: ELeaderboardUploadScoreMethod,
        nScore: int32,
        pScoreDetails: *const int32,
        cScoreDetailsCount: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_AttachLeaderboardUGC(
        self_: *mut ISteamUserStats,
        hSteamLeaderboard: SteamLeaderboard_t,
        hUGC: UGCHandle_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetNumberOfCurrentPlayers(
        self_: *mut ISteamUserStats,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_RequestGlobalAchievementPercentages(
        self_: *mut ISteamUserStats,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetMostAchievedAchievementInfo(
        self_: *mut ISteamUserStats,
        pchName: *mut ::std::os::raw::c_char,
        unNameBufLen: uint32,
        pflPercent: *mut f32,
        pbAchieved: *mut bool,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUserStats_GetNextMostAchievedAchievementInfo(
        self_: *mut ISteamUserStats,
        iIteratorPrevious: ::std::os::raw::c_int,
        pchName: *mut ::std::os::raw::c_char,
        unNameBufLen: uint32,
        pflPercent: *mut f32,
        pbAchieved: *mut bool,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamUserStats_GetAchievementAchievedPercent(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pflPercent: *mut f32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_RequestGlobalStats(
        self_: *mut ISteamUserStats,
        nHistoryDays: ::std::os::raw::c_int,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUserStats_GetGlobalStatInt64(
        self_: *mut ISteamUserStats,
        pchStatName: *const ::std::os::raw::c_char,
        pData: *mut int64,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetGlobalStatDouble(
        self_: *mut ISteamUserStats,
        pchStatName: *const ::std::os::raw::c_char,
        pData: *mut f64,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetGlobalStatHistoryInt64(
        self_: *mut ISteamUserStats,
        pchStatName: *const ::std::os::raw::c_char,
        pData: *mut int64,
        cubData: uint32,
    ) -> int32;
    pub fn SteamAPI_ISteamUserStats_GetGlobalStatHistoryDouble(
        self_: *mut ISteamUserStats,
        pchStatName: *const ::std::os::raw::c_char,
        pData: *mut f64,
        cubData: uint32,
    ) -> int32;
    pub fn SteamAPI_ISteamUserStats_GetAchievementProgressLimitsInt32(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pnMinProgress: *mut int32,
        pnMaxProgress: *mut int32,
    ) -> bool;
    pub fn SteamAPI_ISteamUserStats_GetAchievementProgressLimitsFloat(
        self_: *mut ISteamUserStats,
        pchName: *const ::std::os::raw::c_char,
        pfMinProgress: *mut f32,
        pfMaxProgress: *mut f32,
    ) -> bool;
    pub fn SteamAPI_SteamApps_v008() -> *mut ISteamApps;
    pub fn SteamAPI_ISteamApps_BIsSubscribed(self_: *mut ISteamApps) -> bool;
    pub fn SteamAPI_ISteamApps_BIsLowViolence(self_: *mut ISteamApps) -> bool;
    pub fn SteamAPI_ISteamApps_BIsCybercafe(self_: *mut ISteamApps) -> bool;
    pub fn SteamAPI_ISteamApps_BIsVACBanned(self_: *mut ISteamApps) -> bool;
    pub fn SteamAPI_ISteamApps_GetCurrentGameLanguage(
        self_: *mut ISteamApps,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamApps_GetAvailableGameLanguages(
        self_: *mut ISteamApps,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamApps_BIsSubscribedApp(
        self_: *mut ISteamApps,
        appID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_BIsDlcInstalled(
        self_: *mut ISteamApps,
        appID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetEarliestPurchaseUnixTime(
        self_: *mut ISteamApps,
        nAppID: AppId_t,
    ) -> uint32;
    pub fn SteamAPI_ISteamApps_BIsSubscribedFromFreeWeekend(
        self_: *mut ISteamApps,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetDLCCount(
        self_: *mut ISteamApps,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamApps_BGetDLCDataByIndex(
        self_: *mut ISteamApps,
        iDLC: ::std::os::raw::c_int,
        pAppID: *mut AppId_t,
        pbAvailable: *mut bool,
        pchName: *mut ::std::os::raw::c_char,
        cchNameBufferSize: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_InstallDLC(self_: *mut ISteamApps, nAppID: AppId_t);
    pub fn SteamAPI_ISteamApps_UninstallDLC(self_: *mut ISteamApps, nAppID: AppId_t);
    pub fn SteamAPI_ISteamApps_RequestAppProofOfPurchaseKey(
        self_: *mut ISteamApps,
        nAppID: AppId_t,
    );
    pub fn SteamAPI_ISteamApps_GetCurrentBetaName(
        self_: *mut ISteamApps,
        pchName: *mut ::std::os::raw::c_char,
        cchNameBufferSize: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_MarkContentCorrupt(
        self_: *mut ISteamApps,
        bMissingFilesOnly: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetInstalledDepots(
        self_: *mut ISteamApps,
        appID: AppId_t,
        pvecDepots: *mut DepotId_t,
        cMaxDepots: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamApps_GetAppInstallDir(
        self_: *mut ISteamApps,
        appID: AppId_t,
        pchFolder: *mut ::std::os::raw::c_char,
        cchFolderBufferSize: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamApps_BIsAppInstalled(
        self_: *mut ISteamApps,
        appID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetAppOwner(self_: *mut ISteamApps) -> uint64_steamid;
    pub fn SteamAPI_ISteamApps_GetLaunchQueryParam(
        self_: *mut ISteamApps,
        pchKey: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamApps_GetDlcDownloadProgress(
        self_: *mut ISteamApps,
        nAppID: AppId_t,
        punBytesDownloaded: *mut uint64,
        punBytesTotal: *mut uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetAppBuildId(
        self_: *mut ISteamApps,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamApps_RequestAllProofOfPurchaseKeys(self_: *mut ISteamApps);
    pub fn SteamAPI_ISteamApps_GetFileDetails(
        self_: *mut ISteamApps,
        pszFileName: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamApps_GetLaunchCommandLine(
        self_: *mut ISteamApps,
        pszCommandLine: *mut ::std::os::raw::c_char,
        cubCommandLine: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamApps_BIsSubscribedFromFamilySharing(
        self_: *mut ISteamApps,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_BIsTimedTrial(
        self_: *mut ISteamApps,
        punSecondsAllowed: *mut uint32,
        punSecondsPlayed: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_SetDlcContext(
        self_: *mut ISteamApps,
        nAppID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_GetNumBetas(
        self_: *mut ISteamApps,
        pnAvailable: *mut ::std::os::raw::c_int,
        pnPrivate: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamApps_GetBetaInfo(
        self_: *mut ISteamApps,
        iBetaIndex: ::std::os::raw::c_int,
        punFlags: *mut uint32,
        punBuildID: *mut uint32,
        pchBetaName: *mut ::std::os::raw::c_char,
        cchBetaName: ::std::os::raw::c_int,
        pchDescription: *mut ::std::os::raw::c_char,
        cchDescription: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamApps_SetActiveBeta(
        self_: *mut ISteamApps,
        pchBetaName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamNetworking_v006() -> *mut ISteamNetworking;
    pub fn SteamAPI_SteamGameServerNetworking_v006() -> *mut ISteamNetworking;
    pub fn SteamAPI_ISteamNetworking_SendP2PPacket(
        self_: *mut ISteamNetworking,
        steamIDRemote: uint64_steamid,
        pubData: *const ::std::os::raw::c_void,
        cubData: uint32,
        eP2PSendType: EP2PSend,
        nChannel: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_IsP2PPacketAvailable(
        self_: *mut ISteamNetworking,
        pcubMsgSize: *mut uint32,
        nChannel: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_ReadP2PPacket(
        self_: *mut ISteamNetworking,
        pubDest: *mut ::std::os::raw::c_void,
        cubDest: uint32,
        pcubMsgSize: *mut uint32,
        psteamIDRemote: *mut CSteamID,
        nChannel: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_AcceptP2PSessionWithUser(
        self_: *mut ISteamNetworking,
        steamIDRemote: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_CloseP2PSessionWithUser(
        self_: *mut ISteamNetworking,
        steamIDRemote: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_CloseP2PChannelWithUser(
        self_: *mut ISteamNetworking,
        steamIDRemote: uint64_steamid,
        nChannel: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_GetP2PSessionState(
        self_: *mut ISteamNetworking,
        steamIDRemote: uint64_steamid,
        pConnectionState: *mut P2PSessionState_t,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_AllowP2PPacketRelay(
        self_: *mut ISteamNetworking,
        bAllow: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_CreateListenSocket(
        self_: *mut ISteamNetworking,
        nVirtualP2PPort: ::std::os::raw::c_int,
        nIP: SteamIPAddress_t,
        nPort: uint16,
        bAllowUseOfPacketRelay: bool,
    ) -> SNetListenSocket_t;
    pub fn SteamAPI_ISteamNetworking_CreateP2PConnectionSocket(
        self_: *mut ISteamNetworking,
        steamIDTarget: uint64_steamid,
        nVirtualPort: ::std::os::raw::c_int,
        nTimeoutSec: ::std::os::raw::c_int,
        bAllowUseOfPacketRelay: bool,
    ) -> SNetSocket_t;
    pub fn SteamAPI_ISteamNetworking_CreateConnectionSocket(
        self_: *mut ISteamNetworking,
        nIP: SteamIPAddress_t,
        nPort: uint16,
        nTimeoutSec: ::std::os::raw::c_int,
    ) -> SNetSocket_t;
    pub fn SteamAPI_ISteamNetworking_DestroySocket(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
        bNotifyRemoteEnd: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_DestroyListenSocket(
        self_: *mut ISteamNetworking,
        hSocket: SNetListenSocket_t,
        bNotifyRemoteEnd: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_SendDataOnSocket(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
        pubData: *mut ::std::os::raw::c_void,
        cubData: uint32,
        bReliable: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_IsDataAvailableOnSocket(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
        pcubMsgSize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_RetrieveDataFromSocket(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
        pubDest: *mut ::std::os::raw::c_void,
        cubDest: uint32,
        pcubMsgSize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_IsDataAvailable(
        self_: *mut ISteamNetworking,
        hListenSocket: SNetListenSocket_t,
        pcubMsgSize: *mut uint32,
        phSocket: *mut SNetSocket_t,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_RetrieveData(
        self_: *mut ISteamNetworking,
        hListenSocket: SNetListenSocket_t,
        pubDest: *mut ::std::os::raw::c_void,
        cubDest: uint32,
        pcubMsgSize: *mut uint32,
        phSocket: *mut SNetSocket_t,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_GetSocketInfo(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
        pSteamIDRemote: *mut CSteamID,
        peSocketStatus: *mut ::std::os::raw::c_int,
        punIPRemote: *mut SteamIPAddress_t,
        punPortRemote: *mut uint16,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_GetListenSocketInfo(
        self_: *mut ISteamNetworking,
        hListenSocket: SNetListenSocket_t,
        pnIP: *mut SteamIPAddress_t,
        pnPort: *mut uint16,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworking_GetSocketConnectionType(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
    ) -> ESNetSocketConnectionType;
    pub fn SteamAPI_ISteamNetworking_GetMaxPacketSize(
        self_: *mut ISteamNetworking,
        hSocket: SNetSocket_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_SteamScreenshots_v003() -> *mut ISteamScreenshots;
    pub fn SteamAPI_ISteamScreenshots_WriteScreenshot(
        self_: *mut ISteamScreenshots,
        pubRGB: *mut ::std::os::raw::c_void,
        cubRGB: uint32,
        nWidth: ::std::os::raw::c_int,
        nHeight: ::std::os::raw::c_int,
    ) -> ScreenshotHandle;
    pub fn SteamAPI_ISteamScreenshots_AddScreenshotToLibrary(
        self_: *mut ISteamScreenshots,
        pchFilename: *const ::std::os::raw::c_char,
        pchThumbnailFilename: *const ::std::os::raw::c_char,
        nWidth: ::std::os::raw::c_int,
        nHeight: ::std::os::raw::c_int,
    ) -> ScreenshotHandle;
    pub fn SteamAPI_ISteamScreenshots_TriggerScreenshot(self_: *mut ISteamScreenshots);
    pub fn SteamAPI_ISteamScreenshots_HookScreenshots(
        self_: *mut ISteamScreenshots,
        bHook: bool,
    );
    pub fn SteamAPI_ISteamScreenshots_SetLocation(
        self_: *mut ISteamScreenshots,
        hScreenshot: ScreenshotHandle,
        pchLocation: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamScreenshots_TagUser(
        self_: *mut ISteamScreenshots,
        hScreenshot: ScreenshotHandle,
        steamID: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamScreenshots_TagPublishedFile(
        self_: *mut ISteamScreenshots,
        hScreenshot: ScreenshotHandle,
        unPublishedFileID: PublishedFileId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamScreenshots_IsScreenshotsHooked(
        self_: *mut ISteamScreenshots,
    ) -> bool;
    pub fn SteamAPI_ISteamScreenshots_AddVRScreenshotToLibrary(
        self_: *mut ISteamScreenshots,
        eType: EVRScreenshotType,
        pchFilename: *const ::std::os::raw::c_char,
        pchVRFilename: *const ::std::os::raw::c_char,
    ) -> ScreenshotHandle;
    pub fn SteamAPI_SteamMusic_v001() -> *mut ISteamMusic;
    pub fn SteamAPI_ISteamMusic_BIsEnabled(self_: *mut ISteamMusic) -> bool;
    pub fn SteamAPI_ISteamMusic_BIsPlaying(self_: *mut ISteamMusic) -> bool;
    pub fn SteamAPI_ISteamMusic_GetPlaybackStatus(
        self_: *mut ISteamMusic,
    ) -> AudioPlayback_Status;
    pub fn SteamAPI_ISteamMusic_Play(self_: *mut ISteamMusic);
    pub fn SteamAPI_ISteamMusic_Pause(self_: *mut ISteamMusic);
    pub fn SteamAPI_ISteamMusic_PlayPrevious(self_: *mut ISteamMusic);
    pub fn SteamAPI_ISteamMusic_PlayNext(self_: *mut ISteamMusic);
    pub fn SteamAPI_ISteamMusic_SetVolume(self_: *mut ISteamMusic, flVolume: f32);
    pub fn SteamAPI_ISteamMusic_GetVolume(self_: *mut ISteamMusic) -> f32;
    pub fn SteamAPI_SteamMusicRemote_v001() -> *mut ISteamMusicRemote;
    pub fn SteamAPI_ISteamMusicRemote_RegisterSteamMusicRemote(
        self_: *mut ISteamMusicRemote,
        pchName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_DeregisterSteamMusicRemote(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_BIsCurrentMusicRemote(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_BActivationSuccess(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetDisplayName(
        self_: *mut ISteamMusicRemote,
        pchDisplayName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetPNGIcon_64x64(
        self_: *mut ISteamMusicRemote,
        pvBuffer: *mut ::std::os::raw::c_void,
        cbBufferLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnablePlayPrevious(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnablePlayNext(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnableShuffled(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnableLooped(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnableQueue(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_EnablePlaylists(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdatePlaybackStatus(
        self_: *mut ISteamMusicRemote,
        nStatus: AudioPlayback_Status,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateShuffled(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateLooped(
        self_: *mut ISteamMusicRemote,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateVolume(
        self_: *mut ISteamMusicRemote,
        flValue: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_CurrentEntryWillChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_CurrentEntryIsAvailable(
        self_: *mut ISteamMusicRemote,
        bAvailable: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateCurrentEntryText(
        self_: *mut ISteamMusicRemote,
        pchText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateCurrentEntryElapsedSeconds(
        self_: *mut ISteamMusicRemote,
        nValue: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_UpdateCurrentEntryCoverArt(
        self_: *mut ISteamMusicRemote,
        pvBuffer: *mut ::std::os::raw::c_void,
        cbBufferLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_CurrentEntryDidChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_QueueWillChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_ResetQueueEntries(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetQueueEntry(
        self_: *mut ISteamMusicRemote,
        nID: ::std::os::raw::c_int,
        nPosition: ::std::os::raw::c_int,
        pchEntryText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetCurrentQueueEntry(
        self_: *mut ISteamMusicRemote,
        nID: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_QueueDidChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_PlaylistWillChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_ResetPlaylistEntries(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetPlaylistEntry(
        self_: *mut ISteamMusicRemote,
        nID: ::std::os::raw::c_int,
        nPosition: ::std::os::raw::c_int,
        pchEntryText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_SetCurrentPlaylistEntry(
        self_: *mut ISteamMusicRemote,
        nID: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamMusicRemote_PlaylistDidChange(
        self_: *mut ISteamMusicRemote,
    ) -> bool;
    pub fn SteamAPI_SteamHTTP_v003() -> *mut ISteamHTTP;
    pub fn SteamAPI_SteamGameServerHTTP_v003() -> *mut ISteamHTTP;
    pub fn SteamAPI_ISteamHTTP_CreateHTTPRequest(
        self_: *mut ISteamHTTP,
        eHTTPRequestMethod: EHTTPMethod,
        pchAbsoluteURL: *const ::std::os::raw::c_char,
    ) -> HTTPRequestHandle;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestContextValue(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        ulContextValue: uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestNetworkActivityTimeout(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        unTimeoutSeconds: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestHeaderValue(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchHeaderName: *const ::std::os::raw::c_char,
        pchHeaderValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestGetOrPostParameter(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchParamName: *const ::std::os::raw::c_char,
        pchParamValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SendHTTPRequest(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pCallHandle: *mut SteamAPICall_t,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SendHTTPRequestAndStreamResponse(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pCallHandle: *mut SteamAPICall_t,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_DeferHTTPRequest(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_PrioritizeHTTPRequest(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPResponseHeaderSize(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchHeaderName: *const ::std::os::raw::c_char,
        unResponseHeaderSize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPResponseHeaderValue(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchHeaderName: *const ::std::os::raw::c_char,
        pHeaderValueBuffer: *mut uint8,
        unBufferSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPResponseBodySize(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        unBodySize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPResponseBodyData(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pBodyDataBuffer: *mut uint8,
        unBufferSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPStreamingResponseBodyData(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        cOffset: uint32,
        pBodyDataBuffer: *mut uint8,
        unBufferSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_ReleaseHTTPRequest(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPDownloadProgressPct(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pflPercentOut: *mut f32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestRawPostBody(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchContentType: *const ::std::os::raw::c_char,
        pubBody: *mut uint8,
        unBodyLen: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_CreateCookieContainer(
        self_: *mut ISteamHTTP,
        bAllowResponsesToModify: bool,
    ) -> HTTPCookieContainerHandle;
    pub fn SteamAPI_ISteamHTTP_ReleaseCookieContainer(
        self_: *mut ISteamHTTP,
        hCookieContainer: HTTPCookieContainerHandle,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetCookie(
        self_: *mut ISteamHTTP,
        hCookieContainer: HTTPCookieContainerHandle,
        pchHost: *const ::std::os::raw::c_char,
        pchUrl: *const ::std::os::raw::c_char,
        pchCookie: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestCookieContainer(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        hCookieContainer: HTTPCookieContainerHandle,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestUserAgentInfo(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pchUserAgentInfo: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestRequiresVerifiedCertificate(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        bRequireVerifiedCertificate: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_SetHTTPRequestAbsoluteTimeoutMS(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        unMilliseconds: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamHTTP_GetHTTPRequestWasTimedOut(
        self_: *mut ISteamHTTP,
        hRequest: HTTPRequestHandle,
        pbWasTimedOut: *mut bool,
    ) -> bool;
    pub fn SteamAPI_SteamInput_v006() -> *mut ISteamInput;
    pub fn SteamAPI_ISteamInput_Init(
        self_: *mut ISteamInput,
        bExplicitlyCallRunFrame: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamInput_Shutdown(self_: *mut ISteamInput) -> bool;
    pub fn SteamAPI_ISteamInput_SetInputActionManifestFilePath(
        self_: *mut ISteamInput,
        pchInputActionManifestAbsolutePath: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamInput_RunFrame(self_: *mut ISteamInput, bReservedValue: bool);
    pub fn SteamAPI_ISteamInput_BWaitForData(
        self_: *mut ISteamInput,
        bWaitForever: bool,
        unTimeout: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInput_BNewDataAvailable(self_: *mut ISteamInput) -> bool;
    pub fn SteamAPI_ISteamInput_GetConnectedControllers(
        self_: *mut ISteamInput,
        handlesOut: *mut InputHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamInput_EnableDeviceCallbacks(self_: *mut ISteamInput);
    pub fn SteamAPI_ISteamInput_EnableActionEventCallbacks(
        self_: *mut ISteamInput,
        pCallback: SteamInputActionEventCallbackPointer,
    );
    pub fn SteamAPI_ISteamInput_GetActionSetHandle(
        self_: *mut ISteamInput,
        pszActionSetName: *const ::std::os::raw::c_char,
    ) -> InputActionSetHandle_t;
    pub fn SteamAPI_ISteamInput_ActivateActionSet(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        actionSetHandle: InputActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamInput_GetCurrentActionSet(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    ) -> InputActionSetHandle_t;
    pub fn SteamAPI_ISteamInput_ActivateActionSetLayer(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        actionSetLayerHandle: InputActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamInput_DeactivateActionSetLayer(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        actionSetLayerHandle: InputActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamInput_DeactivateAllActionSetLayers(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    );
    pub fn SteamAPI_ISteamInput_GetActiveActionSetLayers(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        handlesOut: *mut InputActionSetHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamInput_GetDigitalActionHandle(
        self_: *mut ISteamInput,
        pszActionName: *const ::std::os::raw::c_char,
    ) -> InputDigitalActionHandle_t;
    pub fn SteamAPI_ISteamInput_GetDigitalActionData(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        digitalActionHandle: InputDigitalActionHandle_t,
    ) -> InputDigitalActionData_t;
    pub fn SteamAPI_ISteamInput_GetDigitalActionOrigins(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        actionSetHandle: InputActionSetHandle_t,
        digitalActionHandle: InputDigitalActionHandle_t,
        originsOut: *mut EInputActionOrigin,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamInput_GetStringForDigitalActionName(
        self_: *mut ISteamInput,
        eActionHandle: InputDigitalActionHandle_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetAnalogActionHandle(
        self_: *mut ISteamInput,
        pszActionName: *const ::std::os::raw::c_char,
    ) -> InputAnalogActionHandle_t;
    pub fn SteamAPI_ISteamInput_GetAnalogActionData(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        analogActionHandle: InputAnalogActionHandle_t,
    ) -> InputAnalogActionData_t;
    pub fn SteamAPI_ISteamInput_GetAnalogActionOrigins(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        actionSetHandle: InputActionSetHandle_t,
        analogActionHandle: InputAnalogActionHandle_t,
        originsOut: *mut EInputActionOrigin,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamInput_GetGlyphPNGForActionOrigin(
        self_: *mut ISteamInput,
        eOrigin: EInputActionOrigin,
        eSize: ESteamInputGlyphSize,
        unFlags: uint32,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetGlyphSVGForActionOrigin(
        self_: *mut ISteamInput,
        eOrigin: EInputActionOrigin,
        unFlags: uint32,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetGlyphForActionOrigin_Legacy(
        self_: *mut ISteamInput,
        eOrigin: EInputActionOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetStringForActionOrigin(
        self_: *mut ISteamInput,
        eOrigin: EInputActionOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetStringForAnalogActionName(
        self_: *mut ISteamInput,
        eActionHandle: InputAnalogActionHandle_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_StopAnalogActionMomentum(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        eAction: InputAnalogActionHandle_t,
    );
    pub fn SteamAPI_ISteamInput_GetMotionData(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    ) -> InputMotionData_t;
    pub fn SteamAPI_ISteamInput_TriggerVibration(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        usLeftSpeed: ::std::os::raw::c_ushort,
        usRightSpeed: ::std::os::raw::c_ushort,
    );
    pub fn SteamAPI_ISteamInput_TriggerVibrationExtended(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        usLeftSpeed: ::std::os::raw::c_ushort,
        usRightSpeed: ::std::os::raw::c_ushort,
        usLeftTriggerSpeed: ::std::os::raw::c_ushort,
        usRightTriggerSpeed: ::std::os::raw::c_ushort,
    );
    pub fn SteamAPI_ISteamInput_TriggerSimpleHapticEvent(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        eHapticLocation: EControllerHapticLocation,
        nIntensity: uint8,
        nGainDB: ::std::os::raw::c_char,
        nOtherIntensity: uint8,
        nOtherGainDB: ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamInput_SetLEDColor(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        nColorR: uint8,
        nColorG: uint8,
        nColorB: uint8,
        nFlags: ::std::os::raw::c_uint,
    );
    pub fn SteamAPI_ISteamInput_Legacy_TriggerHapticPulse(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        eTargetPad: ESteamControllerPad,
        usDurationMicroSec: ::std::os::raw::c_ushort,
    );
    pub fn SteamAPI_ISteamInput_Legacy_TriggerRepeatedHapticPulse(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        eTargetPad: ESteamControllerPad,
        usDurationMicroSec: ::std::os::raw::c_ushort,
        usOffMicroSec: ::std::os::raw::c_ushort,
        unRepeat: ::std::os::raw::c_ushort,
        nFlags: ::std::os::raw::c_uint,
    );
    pub fn SteamAPI_ISteamInput_ShowBindingPanel(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInput_GetInputTypeForHandle(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    ) -> ESteamInputType;
    pub fn SteamAPI_ISteamInput_GetControllerForGamepadIndex(
        self_: *mut ISteamInput,
        nIndex: ::std::os::raw::c_int,
    ) -> InputHandle_t;
    pub fn SteamAPI_ISteamInput_GetGamepadIndexForController(
        self_: *mut ISteamInput,
        ulinputHandle: InputHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamInput_GetStringForXboxOrigin(
        self_: *mut ISteamInput,
        eOrigin: EXboxOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetGlyphForXboxOrigin(
        self_: *mut ISteamInput,
        eOrigin: EXboxOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamInput_GetActionOriginFromXboxOrigin(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        eOrigin: EXboxOrigin,
    ) -> EInputActionOrigin;
    pub fn SteamAPI_ISteamInput_TranslateActionOrigin(
        self_: *mut ISteamInput,
        eDestinationInputType: ESteamInputType,
        eSourceOrigin: EInputActionOrigin,
    ) -> EInputActionOrigin;
    pub fn SteamAPI_ISteamInput_GetDeviceBindingRevision(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        pMajor: *mut ::std::os::raw::c_int,
        pMinor: *mut ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamInput_GetRemotePlaySessionID(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
    ) -> uint32;
    pub fn SteamAPI_ISteamInput_GetSessionInputConfigurationSettings(
        self_: *mut ISteamInput,
    ) -> uint16;
    pub fn SteamAPI_ISteamInput_SetDualSenseTriggerEffect(
        self_: *mut ISteamInput,
        inputHandle: InputHandle_t,
        pParam: *const ScePadTriggerEffectParam,
    );
    pub fn SteamAPI_SteamController_v008() -> *mut ISteamController;
    pub fn SteamAPI_ISteamController_Init(self_: *mut ISteamController) -> bool;
    pub fn SteamAPI_ISteamController_Shutdown(self_: *mut ISteamController) -> bool;
    pub fn SteamAPI_ISteamController_RunFrame(self_: *mut ISteamController);
    pub fn SteamAPI_ISteamController_GetConnectedControllers(
        self_: *mut ISteamController,
        handlesOut: *mut ControllerHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamController_GetActionSetHandle(
        self_: *mut ISteamController,
        pszActionSetName: *const ::std::os::raw::c_char,
    ) -> ControllerActionSetHandle_t;
    pub fn SteamAPI_ISteamController_ActivateActionSet(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        actionSetHandle: ControllerActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamController_GetCurrentActionSet(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
    ) -> ControllerActionSetHandle_t;
    pub fn SteamAPI_ISteamController_ActivateActionSetLayer(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        actionSetLayerHandle: ControllerActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamController_DeactivateActionSetLayer(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        actionSetLayerHandle: ControllerActionSetHandle_t,
    );
    pub fn SteamAPI_ISteamController_DeactivateAllActionSetLayers(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
    );
    pub fn SteamAPI_ISteamController_GetActiveActionSetLayers(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        handlesOut: *mut ControllerActionSetHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamController_GetDigitalActionHandle(
        self_: *mut ISteamController,
        pszActionName: *const ::std::os::raw::c_char,
    ) -> ControllerDigitalActionHandle_t;
    pub fn SteamAPI_ISteamController_GetDigitalActionData(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        digitalActionHandle: ControllerDigitalActionHandle_t,
    ) -> InputDigitalActionData_t;
    pub fn SteamAPI_ISteamController_GetDigitalActionOrigins(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        actionSetHandle: ControllerActionSetHandle_t,
        digitalActionHandle: ControllerDigitalActionHandle_t,
        originsOut: *mut EControllerActionOrigin,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamController_GetAnalogActionHandle(
        self_: *mut ISteamController,
        pszActionName: *const ::std::os::raw::c_char,
    ) -> ControllerAnalogActionHandle_t;
    pub fn SteamAPI_ISteamController_GetAnalogActionData(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        analogActionHandle: ControllerAnalogActionHandle_t,
    ) -> InputAnalogActionData_t;
    pub fn SteamAPI_ISteamController_GetAnalogActionOrigins(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        actionSetHandle: ControllerActionSetHandle_t,
        analogActionHandle: ControllerAnalogActionHandle_t,
        originsOut: *mut EControllerActionOrigin,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamController_GetGlyphForActionOrigin(
        self_: *mut ISteamController,
        eOrigin: EControllerActionOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamController_GetStringForActionOrigin(
        self_: *mut ISteamController,
        eOrigin: EControllerActionOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamController_StopAnalogActionMomentum(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        eAction: ControllerAnalogActionHandle_t,
    );
    pub fn SteamAPI_ISteamController_GetMotionData(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
    ) -> InputMotionData_t;
    pub fn SteamAPI_ISteamController_TriggerHapticPulse(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        eTargetPad: ESteamControllerPad,
        usDurationMicroSec: ::std::os::raw::c_ushort,
    );
    pub fn SteamAPI_ISteamController_TriggerRepeatedHapticPulse(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        eTargetPad: ESteamControllerPad,
        usDurationMicroSec: ::std::os::raw::c_ushort,
        usOffMicroSec: ::std::os::raw::c_ushort,
        unRepeat: ::std::os::raw::c_ushort,
        nFlags: ::std::os::raw::c_uint,
    );
    pub fn SteamAPI_ISteamController_TriggerVibration(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        usLeftSpeed: ::std::os::raw::c_ushort,
        usRightSpeed: ::std::os::raw::c_ushort,
    );
    pub fn SteamAPI_ISteamController_SetLEDColor(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        nColorR: uint8,
        nColorG: uint8,
        nColorB: uint8,
        nFlags: ::std::os::raw::c_uint,
    );
    pub fn SteamAPI_ISteamController_ShowBindingPanel(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamController_GetInputTypeForHandle(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
    ) -> ESteamInputType;
    pub fn SteamAPI_ISteamController_GetControllerForGamepadIndex(
        self_: *mut ISteamController,
        nIndex: ::std::os::raw::c_int,
    ) -> ControllerHandle_t;
    pub fn SteamAPI_ISteamController_GetGamepadIndexForController(
        self_: *mut ISteamController,
        ulControllerHandle: ControllerHandle_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamController_GetStringForXboxOrigin(
        self_: *mut ISteamController,
        eOrigin: EXboxOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamController_GetGlyphForXboxOrigin(
        self_: *mut ISteamController,
        eOrigin: EXboxOrigin,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamController_GetActionOriginFromXboxOrigin(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        eOrigin: EXboxOrigin,
    ) -> EControllerActionOrigin;
    pub fn SteamAPI_ISteamController_TranslateActionOrigin(
        self_: *mut ISteamController,
        eDestinationInputType: ESteamInputType,
        eSourceOrigin: EControllerActionOrigin,
    ) -> EControllerActionOrigin;
    pub fn SteamAPI_ISteamController_GetControllerBindingRevision(
        self_: *mut ISteamController,
        controllerHandle: ControllerHandle_t,
        pMajor: *mut ::std::os::raw::c_int,
        pMinor: *mut ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_SteamUGC_v021() -> *mut ISteamUGC;
    pub fn SteamAPI_SteamGameServerUGC_v021() -> *mut ISteamUGC;
    pub fn SteamAPI_ISteamUGC_CreateQueryUserUGCRequest(
        self_: *mut ISteamUGC,
        unAccountID: AccountID_t,
        eListType: EUserUGCList,
        eMatchingUGCType: EUGCMatchingUGCType,
        eSortOrder: EUserUGCListSortOrder,
        nCreatorAppID: AppId_t,
        nConsumerAppID: AppId_t,
        unPage: uint32,
    ) -> UGCQueryHandle_t;
    pub fn SteamAPI_ISteamUGC_CreateQueryAllUGCRequestPage(
        self_: *mut ISteamUGC,
        eQueryType: EUGCQuery,
        eMatchingeMatchingUGCTypeFileType: EUGCMatchingUGCType,
        nCreatorAppID: AppId_t,
        nConsumerAppID: AppId_t,
        unPage: uint32,
    ) -> UGCQueryHandle_t;
    pub fn SteamAPI_ISteamUGC_CreateQueryAllUGCRequestCursor(
        self_: *mut ISteamUGC,
        eQueryType: EUGCQuery,
        eMatchingeMatchingUGCTypeFileType: EUGCMatchingUGCType,
        nCreatorAppID: AppId_t,
        nConsumerAppID: AppId_t,
        pchCursor: *const ::std::os::raw::c_char,
    ) -> UGCQueryHandle_t;
    pub fn SteamAPI_ISteamUGC_CreateQueryUGCDetailsRequest(
        self_: *mut ISteamUGC,
        pvecPublishedFileID: *mut PublishedFileId_t,
        unNumPublishedFileIDs: uint32,
    ) -> UGCQueryHandle_t;
    pub fn SteamAPI_ISteamUGC_SendQueryUGCRequest(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCResult(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pDetails: *mut SteamUGCDetails_t,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCNumTags(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        indexTag: uint32,
        pchValue: *mut ::std::os::raw::c_char,
        cchValueSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCTagDisplayName(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        indexTag: uint32,
        pchValue: *mut ::std::os::raw::c_char,
        cchValueSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCPreviewURL(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pchURL: *mut ::std::os::raw::c_char,
        cchURLSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCMetadata(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pchMetadata: *mut ::std::os::raw::c_char,
        cchMetadatasize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCChildren(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pvecPublishedFileID: *mut PublishedFileId_t,
        cMaxEntries: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCStatistic(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        eStatType: EItemStatistic,
        pStatValue: *mut uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCNumAdditionalPreviews(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCAdditionalPreview(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        previewIndex: uint32,
        pchURLOrVideoID: *mut ::std::os::raw::c_char,
        cchURLSize: uint32,
        pchOriginalFileName: *mut ::std::os::raw::c_char,
        cchOriginalFileNameSize: uint32,
        pPreviewType: *mut EItemPreviewType,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCNumKeyValueTags(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCKeyValueTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        keyValueTagIndex: uint32,
        pchKey: *mut ::std::os::raw::c_char,
        cchKeySize: uint32,
        pchValue: *mut ::std::os::raw::c_char,
        cchValueSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryFirstUGCKeyValueTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *mut ::std::os::raw::c_char,
        cchValueSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetNumSupportedGameVersions(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetSupportedGameVersionData(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        versionIndex: uint32,
        pchGameBranchMin: *mut ::std::os::raw::c_char,
        pchGameBranchMax: *mut ::std::os::raw::c_char,
        cchGameBranchSize: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetQueryUGCContentDescriptors(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        index: uint32,
        pvecDescriptors: *mut EUGCContentDescriptorID,
        cMaxEntries: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_ReleaseQueryUGCRequest(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddRequiredTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pTagName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddRequiredTagGroup(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pTagGroups: *const SteamParamStringArray_t,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddExcludedTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pTagName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnOnlyIDs(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnOnlyIDs: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnKeyValueTags(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnKeyValueTags: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnLongDescription(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnLongDescription: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnMetadata(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnMetadata: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnChildren(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnChildren: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnAdditionalPreviews(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnAdditionalPreviews: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnTotalOnly(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bReturnTotalOnly: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetReturnPlaytimeStats(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        unDays: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetLanguage(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pchLanguage: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetAllowCachedResponse(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        unMaxAgeSeconds: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetAdminQuery(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        bAdminQuery: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetCloudFileNameFilter(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pMatchCloudFileName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetMatchAnyTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        bMatchAnyTag: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetSearchText(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pSearchText: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetRankedByTrendDays(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        unDays: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetTimeCreatedDateRange(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        rtStart: RTime32,
        rtEnd: RTime32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetTimeUpdatedDateRange(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        rtStart: RTime32,
        rtEnd: RTime32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddRequiredKeyValueTag(
        self_: *mut ISteamUGC,
        handle: UGCQueryHandle_t,
        pKey: *const ::std::os::raw::c_char,
        pValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_RequestUGCDetails(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        unMaxAgeSeconds: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_CreateItem(
        self_: *mut ISteamUGC,
        nConsumerAppId: AppId_t,
        eFileType: EWorkshopFileType,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_StartItemUpdate(
        self_: *mut ISteamUGC,
        nConsumerAppId: AppId_t,
        nPublishedFileID: PublishedFileId_t,
    ) -> UGCUpdateHandle_t;
    pub fn SteamAPI_ISteamUGC_SetItemTitle(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchTitle: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemDescription(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchDescription: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemUpdateLanguage(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchLanguage: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemMetadata(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchMetaData: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemVisibility(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        eVisibility: ERemoteStoragePublishedFileVisibility,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemTags(
        self_: *mut ISteamUGC,
        updateHandle: UGCUpdateHandle_t,
        pTags: *const SteamParamStringArray_t,
        bAllowAdminTags: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemContent(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pszContentFolder: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetItemPreview(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pszPreviewFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetAllowLegacyUpload(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        bAllowLegacyUpload: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_RemoveAllItemKeyValueTags(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_RemoveItemKeyValueTags(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchKey: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddItemKeyValueTag(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddItemPreviewFile(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pszPreviewFile: *const ::std::os::raw::c_char,
        type_: EItemPreviewType,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddItemPreviewVideo(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pszVideoID: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_UpdateItemPreviewFile(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        index: uint32,
        pszPreviewFile: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_UpdateItemPreviewVideo(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        index: uint32,
        pszVideoID: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_RemoveItemPreview(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        index: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_AddContentDescriptor(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        descid: EUGCContentDescriptorID,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_RemoveContentDescriptor(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        descid: EUGCContentDescriptorID,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetRequiredGameVersions(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pszGameBranchMin: *const ::std::os::raw::c_char,
        pszGameBranchMax: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SubmitItemUpdate(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        pchChangeNote: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetItemUpdateProgress(
        self_: *mut ISteamUGC,
        handle: UGCUpdateHandle_t,
        punBytesProcessed: *mut uint64,
        punBytesTotal: *mut uint64,
    ) -> EItemUpdateStatus;
    pub fn SteamAPI_ISteamUGC_SetUserItemVote(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        bVoteUp: bool,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetUserItemVote(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_AddItemToFavorites(
        self_: *mut ISteamUGC,
        nAppId: AppId_t,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_RemoveItemFromFavorites(
        self_: *mut ISteamUGC,
        nAppId: AppId_t,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_SubscribeItem(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_UnsubscribeItem(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetNumSubscribedItems(
        self_: *mut ISteamUGC,
        bIncludeLocallyDisabled: bool,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetSubscribedItems(
        self_: *mut ISteamUGC,
        pvecPublishedFileID: *mut PublishedFileId_t,
        cMaxEntries: uint32,
        bIncludeLocallyDisabled: bool,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetItemState(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_GetItemInstallInfo(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        punSizeOnDisk: *mut uint64,
        pchFolder: *mut ::std::os::raw::c_char,
        cchFolderSize: uint32,
        punTimeStamp: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_GetItemDownloadInfo(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        punBytesDownloaded: *mut uint64,
        punBytesTotal: *mut uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_DownloadItem(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        bHighPriority: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_BInitWorkshopForGameServer(
        self_: *mut ISteamUGC,
        unWorkshopDepotID: DepotId_t,
        pszFolder: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SuspendDownloads(self_: *mut ISteamUGC, bSuspend: bool);
    pub fn SteamAPI_ISteamUGC_StartPlaytimeTracking(
        self_: *mut ISteamUGC,
        pvecPublishedFileID: *mut PublishedFileId_t,
        unNumPublishedFileIDs: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_StopPlaytimeTracking(
        self_: *mut ISteamUGC,
        pvecPublishedFileID: *mut PublishedFileId_t,
        unNumPublishedFileIDs: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_StopPlaytimeTrackingForAllItems(
        self_: *mut ISteamUGC,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_AddDependency(
        self_: *mut ISteamUGC,
        nParentPublishedFileID: PublishedFileId_t,
        nChildPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_RemoveDependency(
        self_: *mut ISteamUGC,
        nParentPublishedFileID: PublishedFileId_t,
        nChildPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_AddAppDependency(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        nAppID: AppId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_RemoveAppDependency(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
        nAppID: AppId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetAppDependencies(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_DeleteItem(
        self_: *mut ISteamUGC,
        nPublishedFileID: PublishedFileId_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_ShowWorkshopEULA(self_: *mut ISteamUGC) -> bool;
    pub fn SteamAPI_ISteamUGC_GetWorkshopEULAStatus(
        self_: *mut ISteamUGC,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamUGC_GetUserContentDescriptorPreferences(
        self_: *mut ISteamUGC,
        pvecDescriptors: *mut EUGCContentDescriptorID,
        cMaxEntries: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamUGC_SetItemsDisabledLocally(
        self_: *mut ISteamUGC,
        pvecPublishedFileIDs: *mut PublishedFileId_t,
        unNumPublishedFileIDs: uint32,
        bDisabledLocally: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamUGC_SetSubscriptionsLoadOrder(
        self_: *mut ISteamUGC,
        pvecPublishedFileIDs: *mut PublishedFileId_t,
        unNumPublishedFileIDs: uint32,
    ) -> bool;
    pub fn SteamAPI_SteamHTMLSurface_v005() -> *mut ISteamHTMLSurface;
    pub fn SteamAPI_ISteamHTMLSurface_Init(self_: *mut ISteamHTMLSurface) -> bool;
    pub fn SteamAPI_ISteamHTMLSurface_Shutdown(self_: *mut ISteamHTMLSurface) -> bool;
    pub fn SteamAPI_ISteamHTMLSurface_CreateBrowser(
        self_: *mut ISteamHTMLSurface,
        pchUserAgent: *const ::std::os::raw::c_char,
        pchUserCSS: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamHTMLSurface_RemoveBrowser(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_LoadURL(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        pchURL: *const ::std::os::raw::c_char,
        pchPostData: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetSize(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        unWidth: uint32,
        unHeight: uint32,
    );
    pub fn SteamAPI_ISteamHTMLSurface_StopLoad(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_Reload(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_GoBack(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_GoForward(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_AddHeader(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamHTMLSurface_ExecuteJavascript(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        pchScript: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamHTMLSurface_MouseUp(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        eMouseButton: ISteamHTMLSurface_EHTMLMouseButton,
    );
    pub fn SteamAPI_ISteamHTMLSurface_MouseDown(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        eMouseButton: ISteamHTMLSurface_EHTMLMouseButton,
    );
    pub fn SteamAPI_ISteamHTMLSurface_MouseDoubleClick(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        eMouseButton: ISteamHTMLSurface_EHTMLMouseButton,
    );
    pub fn SteamAPI_ISteamHTMLSurface_MouseMove(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamHTMLSurface_MouseWheel(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        nDelta: int32,
    );
    pub fn SteamAPI_ISteamHTMLSurface_KeyDown(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        nNativeKeyCode: uint32,
        eHTMLKeyModifiers: ISteamHTMLSurface_EHTMLKeyModifiers,
        bIsSystemKey: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_KeyUp(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        nNativeKeyCode: uint32,
        eHTMLKeyModifiers: ISteamHTMLSurface_EHTMLKeyModifiers,
    );
    pub fn SteamAPI_ISteamHTMLSurface_KeyChar(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        cUnicodeChar: uint32,
        eHTMLKeyModifiers: ISteamHTMLSurface_EHTMLKeyModifiers,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetHorizontalScroll(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        nAbsolutePixelScroll: uint32,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetVerticalScroll(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        nAbsolutePixelScroll: uint32,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetKeyFocus(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        bHasKeyFocus: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_ViewSource(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_CopyToClipboard(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_PasteFromClipboard(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_Find(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        pchSearchStr: *const ::std::os::raw::c_char,
        bCurrentlyInFind: bool,
        bReverse: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_StopFind(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_GetLinkAtPosition(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        x: ::std::os::raw::c_int,
        y: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetCookie(
        self_: *mut ISteamHTMLSurface,
        pchHostname: *const ::std::os::raw::c_char,
        pchKey: *const ::std::os::raw::c_char,
        pchValue: *const ::std::os::raw::c_char,
        pchPath: *const ::std::os::raw::c_char,
        nExpires: RTime32,
        bSecure: bool,
        bHTTPOnly: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetPageScaleFactor(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        flZoom: f32,
        nPointX: ::std::os::raw::c_int,
        nPointY: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetBackgroundMode(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        bBackgroundMode: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_SetDPIScalingFactor(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        flDPIScaling: f32,
    );
    pub fn SteamAPI_ISteamHTMLSurface_OpenDeveloperTools(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
    );
    pub fn SteamAPI_ISteamHTMLSurface_AllowStartRequest(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        bAllowed: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_JSDialogResponse(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        bResult: bool,
    );
    pub fn SteamAPI_ISteamHTMLSurface_FileLoadDialogResponse(
        self_: *mut ISteamHTMLSurface,
        unBrowserHandle: HHTMLBrowser,
        pchSelectedFiles: *mut *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_SteamInventory_v003() -> *mut ISteamInventory;
    pub fn SteamAPI_SteamGameServerInventory_v003() -> *mut ISteamInventory;
    pub fn SteamAPI_ISteamInventory_GetResultStatus(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
    ) -> EResult;
    pub fn SteamAPI_ISteamInventory_GetResultItems(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
        pOutItemsArray: *mut SteamItemDetails_t,
        punOutItemsArraySize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetResultItemProperty(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
        unItemIndex: uint32,
        pchPropertyName: *const ::std::os::raw::c_char,
        pchValueBuffer: *mut ::std::os::raw::c_char,
        punValueBufferSizeOut: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetResultTimestamp(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
    ) -> uint32;
    pub fn SteamAPI_ISteamInventory_CheckResultSteamID(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
        steamIDExpected: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_DestroyResult(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
    );
    pub fn SteamAPI_ISteamInventory_GetAllItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetItemsByID(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        pInstanceIDs: *const SteamItemInstanceID_t,
        unCountInstanceIDs: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SerializeResult(
        self_: *mut ISteamInventory,
        resultHandle: SteamInventoryResult_t,
        pOutBuffer: *mut ::std::os::raw::c_void,
        punOutBufferSize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_DeserializeResult(
        self_: *mut ISteamInventory,
        pOutResultHandle: *mut SteamInventoryResult_t,
        pBuffer: *const ::std::os::raw::c_void,
        unBufferSize: uint32,
        bRESERVED_MUST_BE_FALSE: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GenerateItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        pArrayItemDefs: *const SteamItemDef_t,
        punArrayQuantity: *const uint32,
        unArrayLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GrantPromoItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_AddPromoItem(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        itemDef: SteamItemDef_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_AddPromoItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        pArrayItemDefs: *const SteamItemDef_t,
        unArrayLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_ConsumeItem(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        itemConsume: SteamItemInstanceID_t,
        unQuantity: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_ExchangeItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        pArrayGenerate: *const SteamItemDef_t,
        punArrayGenerateQuantity: *const uint32,
        unArrayGenerateLength: uint32,
        pArrayDestroy: *const SteamItemInstanceID_t,
        punArrayDestroyQuantity: *const uint32,
        unArrayDestroyLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_TransferItemQuantity(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        itemIdSource: SteamItemInstanceID_t,
        unQuantity: uint32,
        itemIdDest: SteamItemInstanceID_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SendItemDropHeartbeat(self_: *mut ISteamInventory);
    pub fn SteamAPI_ISteamInventory_TriggerItemDrop(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        dropListDefinition: SteamItemDef_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_TradeItems(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        steamIDTradePartner: uint64_steamid,
        pArrayGive: *const SteamItemInstanceID_t,
        pArrayGiveQuantity: *const uint32,
        nArrayGiveLength: uint32,
        pArrayGet: *const SteamItemInstanceID_t,
        pArrayGetQuantity: *const uint32,
        nArrayGetLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_LoadItemDefinitions(
        self_: *mut ISteamInventory,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetItemDefinitionIDs(
        self_: *mut ISteamInventory,
        pItemDefIDs: *mut SteamItemDef_t,
        punItemDefIDsArraySize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetItemDefinitionProperty(
        self_: *mut ISteamInventory,
        iDefinition: SteamItemDef_t,
        pchPropertyName: *const ::std::os::raw::c_char,
        pchValueBuffer: *mut ::std::os::raw::c_char,
        punValueBufferSizeOut: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_RequestEligiblePromoItemDefinitionsIDs(
        self_: *mut ISteamInventory,
        steamID: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamInventory_GetEligiblePromoItemDefinitionIDs(
        self_: *mut ISteamInventory,
        steamID: uint64_steamid,
        pItemDefIDs: *mut SteamItemDef_t,
        punItemDefIDsArraySize: *mut uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_StartPurchase(
        self_: *mut ISteamInventory,
        pArrayItemDefs: *const SteamItemDef_t,
        punArrayQuantity: *const uint32,
        unArrayLength: uint32,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamInventory_RequestPrices(
        self_: *mut ISteamInventory,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamInventory_GetNumItemsWithPrices(
        self_: *mut ISteamInventory,
    ) -> uint32;
    pub fn SteamAPI_ISteamInventory_GetItemsWithPrices(
        self_: *mut ISteamInventory,
        pArrayItemDefs: *mut SteamItemDef_t,
        pCurrentPrices: *mut uint64,
        pBasePrices: *mut uint64,
        unArrayLength: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_GetItemPrice(
        self_: *mut ISteamInventory,
        iDefinition: SteamItemDef_t,
        pCurrentPrice: *mut uint64,
        pBasePrice: *mut uint64,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_StartUpdateProperties(
        self_: *mut ISteamInventory,
    ) -> SteamInventoryUpdateHandle_t;
    pub fn SteamAPI_ISteamInventory_RemoveProperty(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        nItemID: SteamItemInstanceID_t,
        pchPropertyName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SetPropertyString(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        nItemID: SteamItemInstanceID_t,
        pchPropertyName: *const ::std::os::raw::c_char,
        pchPropertyValue: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SetPropertyBool(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        nItemID: SteamItemInstanceID_t,
        pchPropertyName: *const ::std::os::raw::c_char,
        bValue: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SetPropertyInt64(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        nItemID: SteamItemInstanceID_t,
        pchPropertyName: *const ::std::os::raw::c_char,
        nValue: int64,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SetPropertyFloat(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        nItemID: SteamItemInstanceID_t,
        pchPropertyName: *const ::std::os::raw::c_char,
        flValue: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_SubmitUpdateProperties(
        self_: *mut ISteamInventory,
        handle: SteamInventoryUpdateHandle_t,
        pResultHandle: *mut SteamInventoryResult_t,
    ) -> bool;
    pub fn SteamAPI_ISteamInventory_InspectItem(
        self_: *mut ISteamInventory,
        pResultHandle: *mut SteamInventoryResult_t,
        pchItemToken: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamTimeline_v004() -> *mut ISteamTimeline;
    pub fn SteamAPI_ISteamTimeline_SetTimelineTooltip(
        self_: *mut ISteamTimeline,
        pchDescription: *const ::std::os::raw::c_char,
        flTimeDelta: f32,
    );
    pub fn SteamAPI_ISteamTimeline_ClearTimelineTooltip(
        self_: *mut ISteamTimeline,
        flTimeDelta: f32,
    );
    pub fn SteamAPI_ISteamTimeline_SetTimelineGameMode(
        self_: *mut ISteamTimeline,
        eMode: ETimelineGameMode,
    );
    pub fn SteamAPI_ISteamTimeline_AddInstantaneousTimelineEvent(
        self_: *mut ISteamTimeline,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        pchIcon: *const ::std::os::raw::c_char,
        unIconPriority: uint32,
        flStartOffsetSeconds: f32,
        ePossibleClip: ETimelineEventClipPriority,
    ) -> TimelineEventHandle_t;
    pub fn SteamAPI_ISteamTimeline_AddRangeTimelineEvent(
        self_: *mut ISteamTimeline,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        pchIcon: *const ::std::os::raw::c_char,
        unIconPriority: uint32,
        flStartOffsetSeconds: f32,
        flDuration: f32,
        ePossibleClip: ETimelineEventClipPriority,
    ) -> TimelineEventHandle_t;
    pub fn SteamAPI_ISteamTimeline_StartRangeTimelineEvent(
        self_: *mut ISteamTimeline,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        pchIcon: *const ::std::os::raw::c_char,
        unPriority: uint32,
        flStartOffsetSeconds: f32,
        ePossibleClip: ETimelineEventClipPriority,
    ) -> TimelineEventHandle_t;
    pub fn SteamAPI_ISteamTimeline_UpdateRangeTimelineEvent(
        self_: *mut ISteamTimeline,
        ulEvent: TimelineEventHandle_t,
        pchTitle: *const ::std::os::raw::c_char,
        pchDescription: *const ::std::os::raw::c_char,
        pchIcon: *const ::std::os::raw::c_char,
        unPriority: uint32,
        ePossibleClip: ETimelineEventClipPriority,
    );
    pub fn SteamAPI_ISteamTimeline_EndRangeTimelineEvent(
        self_: *mut ISteamTimeline,
        ulEvent: TimelineEventHandle_t,
        flEndOffsetSeconds: f32,
    );
    pub fn SteamAPI_ISteamTimeline_RemoveTimelineEvent(
        self_: *mut ISteamTimeline,
        ulEvent: TimelineEventHandle_t,
    );
    pub fn SteamAPI_ISteamTimeline_DoesEventRecordingExist(
        self_: *mut ISteamTimeline,
        ulEvent: TimelineEventHandle_t,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamTimeline_StartGamePhase(self_: *mut ISteamTimeline);
    pub fn SteamAPI_ISteamTimeline_EndGamePhase(self_: *mut ISteamTimeline);
    pub fn SteamAPI_ISteamTimeline_SetGamePhaseID(
        self_: *mut ISteamTimeline,
        pchPhaseID: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamTimeline_DoesGamePhaseRecordingExist(
        self_: *mut ISteamTimeline,
        pchPhaseID: *const ::std::os::raw::c_char,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamTimeline_AddGamePhaseTag(
        self_: *mut ISteamTimeline,
        pchTagName: *const ::std::os::raw::c_char,
        pchTagIcon: *const ::std::os::raw::c_char,
        pchTagGroup: *const ::std::os::raw::c_char,
        unPriority: uint32,
    );
    pub fn SteamAPI_ISteamTimeline_SetGamePhaseAttribute(
        self_: *mut ISteamTimeline,
        pchAttributeGroup: *const ::std::os::raw::c_char,
        pchAttributeValue: *const ::std::os::raw::c_char,
        unPriority: uint32,
    );
    pub fn SteamAPI_ISteamTimeline_OpenOverlayToGamePhase(
        self_: *mut ISteamTimeline,
        pchPhaseID: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamTimeline_OpenOverlayToTimelineEvent(
        self_: *mut ISteamTimeline,
        ulEvent: TimelineEventHandle_t,
    );
    pub fn SteamAPI_SteamVideo_v007() -> *mut ISteamVideo;
    pub fn SteamAPI_ISteamVideo_GetVideoURL(
        self_: *mut ISteamVideo,
        unVideoAppID: AppId_t,
    );
    pub fn SteamAPI_ISteamVideo_IsBroadcasting(
        self_: *mut ISteamVideo,
        pnNumViewers: *mut ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamVideo_GetOPFSettings(
        self_: *mut ISteamVideo,
        unVideoAppID: AppId_t,
    );
    pub fn SteamAPI_ISteamVideo_GetOPFStringForApp(
        self_: *mut ISteamVideo,
        unVideoAppID: AppId_t,
        pchBuffer: *mut ::std::os::raw::c_char,
        pnBufferSize: *mut int32,
    ) -> bool;
    pub fn SteamAPI_SteamParentalSettings_v001() -> *mut ISteamParentalSettings;
    pub fn SteamAPI_ISteamParentalSettings_BIsParentalLockEnabled(
        self_: *mut ISteamParentalSettings,
    ) -> bool;
    pub fn SteamAPI_ISteamParentalSettings_BIsParentalLockLocked(
        self_: *mut ISteamParentalSettings,
    ) -> bool;
    pub fn SteamAPI_ISteamParentalSettings_BIsAppBlocked(
        self_: *mut ISteamParentalSettings,
        nAppID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamParentalSettings_BIsAppInBlockList(
        self_: *mut ISteamParentalSettings,
        nAppID: AppId_t,
    ) -> bool;
    pub fn SteamAPI_ISteamParentalSettings_BIsFeatureBlocked(
        self_: *mut ISteamParentalSettings,
        eFeature: EParentalFeature,
    ) -> bool;
    pub fn SteamAPI_ISteamParentalSettings_BIsFeatureInBlockList(
        self_: *mut ISteamParentalSettings,
        eFeature: EParentalFeature,
    ) -> bool;
    pub fn SteamAPI_SteamRemotePlay_v003() -> *mut ISteamRemotePlay;
    pub fn SteamAPI_ISteamRemotePlay_GetSessionCount(
        self_: *mut ISteamRemotePlay,
    ) -> uint32;
    pub fn SteamAPI_ISteamRemotePlay_GetSessionID(
        self_: *mut ISteamRemotePlay,
        iSessionIndex: ::std::os::raw::c_int,
    ) -> RemotePlaySessionID_t;
    pub fn SteamAPI_ISteamRemotePlay_GetSessionSteamID(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamRemotePlay_GetSessionClientName(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamRemotePlay_GetSessionClientFormFactor(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
    ) -> ESteamDeviceFormFactor;
    pub fn SteamAPI_ISteamRemotePlay_BGetSessionClientResolution(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
        pnResolutionX: *mut ::std::os::raw::c_int,
        pnResolutionY: *mut ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamRemotePlay_ShowRemotePlayTogetherUI(
        self_: *mut ISteamRemotePlay,
    ) -> bool;
    pub fn SteamAPI_ISteamRemotePlay_BSendRemotePlayTogetherInvite(
        self_: *mut ISteamRemotePlay,
        steamIDFriend: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamRemotePlay_BEnableRemotePlayTogetherDirectInput(
        self_: *mut ISteamRemotePlay,
    ) -> bool;
    pub fn SteamAPI_ISteamRemotePlay_DisableRemotePlayTogetherDirectInput(
        self_: *mut ISteamRemotePlay,
    );
    pub fn SteamAPI_ISteamRemotePlay_GetInput(
        self_: *mut ISteamRemotePlay,
        pInput: *mut RemotePlayInput_t,
        unMaxEvents: uint32,
    ) -> uint32;
    pub fn SteamAPI_ISteamRemotePlay_SetMouseVisibility(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
        bVisible: bool,
    );
    pub fn SteamAPI_ISteamRemotePlay_SetMousePosition(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
        flNormalizedX: f32,
        flNormalizedY: f32,
    );
    pub fn SteamAPI_ISteamRemotePlay_CreateMouseCursor(
        self_: *mut ISteamRemotePlay,
        nWidth: ::std::os::raw::c_int,
        nHeight: ::std::os::raw::c_int,
        nHotX: ::std::os::raw::c_int,
        nHotY: ::std::os::raw::c_int,
        pBGRA: *const ::std::os::raw::c_void,
        nPitch: ::std::os::raw::c_int,
    ) -> RemotePlayCursorID_t;
    pub fn SteamAPI_ISteamRemotePlay_SetMouseCursor(
        self_: *mut ISteamRemotePlay,
        unSessionID: RemotePlaySessionID_t,
        unCursorID: RemotePlayCursorID_t,
    );
    pub fn SteamAPI_SteamNetworkingMessages_SteamAPI_v002() -> *mut ISteamNetworkingMessages;
    pub fn SteamAPI_SteamGameServerNetworkingMessages_SteamAPI_v002() -> *mut ISteamNetworkingMessages;
    pub fn SteamAPI_ISteamNetworkingMessages_SendMessageToUser(
        self_: *mut ISteamNetworkingMessages,
        identityRemote: *const SteamNetworkingIdentity,
        pubData: *const ::std::os::raw::c_void,
        cubData: uint32,
        nSendFlags: ::std::os::raw::c_int,
        nRemoteChannel: ::std::os::raw::c_int,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingMessages_ReceiveMessagesOnChannel(
        self_: *mut ISteamNetworkingMessages,
        nLocalChannel: ::std::os::raw::c_int,
        ppOutMessages: *mut *mut SteamNetworkingMessage_t,
        nMaxMessages: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingMessages_AcceptSessionWithUser(
        self_: *mut ISteamNetworkingMessages,
        identityRemote: *const SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingMessages_CloseSessionWithUser(
        self_: *mut ISteamNetworkingMessages,
        identityRemote: *const SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingMessages_CloseChannelWithUser(
        self_: *mut ISteamNetworkingMessages,
        identityRemote: *const SteamNetworkingIdentity,
        nLocalChannel: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingMessages_GetSessionConnectionInfo(
        self_: *mut ISteamNetworkingMessages,
        identityRemote: *const SteamNetworkingIdentity,
        pConnectionInfo: *mut SteamNetConnectionInfo_t,
        pQuickStatus: *mut SteamNetConnectionRealTimeStatus_t,
    ) -> ESteamNetworkingConnectionState;
    pub fn SteamAPI_SteamNetworkingSockets_SteamAPI_v012() -> *mut ISteamNetworkingSockets;
    pub fn SteamAPI_SteamGameServerNetworkingSockets_SteamAPI_v012() -> *mut ISteamNetworkingSockets;
    pub fn SteamAPI_ISteamNetworkingSockets_CreateListenSocketIP(
        self_: *mut ISteamNetworkingSockets,
        localAddress: *const SteamNetworkingIPAddr,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamListenSocket;
    pub fn SteamAPI_ISteamNetworkingSockets_ConnectByIPAddress(
        self_: *mut ISteamNetworkingSockets,
        address: *const SteamNetworkingIPAddr,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamNetConnection;
    pub fn SteamAPI_ISteamNetworkingSockets_CreateListenSocketP2P(
        self_: *mut ISteamNetworkingSockets,
        nLocalVirtualPort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamListenSocket;
    pub fn SteamAPI_ISteamNetworkingSockets_ConnectP2P(
        self_: *mut ISteamNetworkingSockets,
        identityRemote: *const SteamNetworkingIdentity,
        nRemoteVirtualPort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamNetConnection;
    pub fn SteamAPI_ISteamNetworkingSockets_AcceptConnection(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_CloseConnection(
        self_: *mut ISteamNetworkingSockets,
        hPeer: HSteamNetConnection,
        nReason: ::std::os::raw::c_int,
        pszDebug: *const ::std::os::raw::c_char,
        bEnableLinger: bool,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_CloseListenSocket(
        self_: *mut ISteamNetworkingSockets,
        hSocket: HSteamListenSocket,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_SetConnectionUserData(
        self_: *mut ISteamNetworkingSockets,
        hPeer: HSteamNetConnection,
        nUserData: int64,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_GetConnectionUserData(
        self_: *mut ISteamNetworkingSockets,
        hPeer: HSteamNetConnection,
    ) -> int64;
    pub fn SteamAPI_ISteamNetworkingSockets_SetConnectionName(
        self_: *mut ISteamNetworkingSockets,
        hPeer: HSteamNetConnection,
        pszName: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamNetworkingSockets_GetConnectionName(
        self_: *mut ISteamNetworkingSockets,
        hPeer: HSteamNetConnection,
        pszName: *mut ::std::os::raw::c_char,
        nMaxLen: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_SendMessageToConnection(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        pData: *const ::std::os::raw::c_void,
        cbData: uint32,
        nSendFlags: ::std::os::raw::c_int,
        pOutMessageNumber: *mut int64,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_SendMessages(
        self_: *mut ISteamNetworkingSockets,
        nMessages: ::std::os::raw::c_int,
        pMessages: *const *mut SteamNetworkingMessage_t,
        pOutMessageNumberOrResult: *mut int64,
    );
    pub fn SteamAPI_ISteamNetworkingSockets_FlushMessagesOnConnection(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_ReceiveMessagesOnConnection(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        ppOutMessages: *mut *mut SteamNetworkingMessage_t,
        nMaxMessages: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingSockets_GetConnectionInfo(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        pInfo: *mut SteamNetConnectionInfo_t,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_GetConnectionRealTimeStatus(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        pStatus: *mut SteamNetConnectionRealTimeStatus_t,
        nLanes: ::std::os::raw::c_int,
        pLanes: *mut SteamNetConnectionRealTimeLaneStatus_t,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_GetDetailedConnectionStatus(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        pszBuf: *mut ::std::os::raw::c_char,
        cbBuf: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingSockets_GetListenSocketAddress(
        self_: *mut ISteamNetworkingSockets,
        hSocket: HSteamListenSocket,
        address: *mut SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_CreateSocketPair(
        self_: *mut ISteamNetworkingSockets,
        pOutConnection1: *mut HSteamNetConnection,
        pOutConnection2: *mut HSteamNetConnection,
        bUseNetworkLoopback: bool,
        pIdentity1: *const SteamNetworkingIdentity,
        pIdentity2: *const SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_ConfigureConnectionLanes(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        nNumLanes: ::std::os::raw::c_int,
        pLanePriorities: *const ::std::os::raw::c_int,
        pLaneWeights: *const uint16,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_GetIdentity(
        self_: *mut ISteamNetworkingSockets,
        pIdentity: *mut SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_InitAuthentication(
        self_: *mut ISteamNetworkingSockets,
    ) -> ESteamNetworkingAvailability;
    pub fn SteamAPI_ISteamNetworkingSockets_GetAuthenticationStatus(
        self_: *mut ISteamNetworkingSockets,
        pDetails: *mut SteamNetAuthenticationStatus_t,
    ) -> ESteamNetworkingAvailability;
    pub fn SteamAPI_ISteamNetworkingSockets_CreatePollGroup(
        self_: *mut ISteamNetworkingSockets,
    ) -> HSteamNetPollGroup;
    pub fn SteamAPI_ISteamNetworkingSockets_DestroyPollGroup(
        self_: *mut ISteamNetworkingSockets,
        hPollGroup: HSteamNetPollGroup,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_SetConnectionPollGroup(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        hPollGroup: HSteamNetPollGroup,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_ReceiveMessagesOnPollGroup(
        self_: *mut ISteamNetworkingSockets,
        hPollGroup: HSteamNetPollGroup,
        ppOutMessages: *mut *mut SteamNetworkingMessage_t,
        nMaxMessages: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingSockets_ReceivedRelayAuthTicket(
        self_: *mut ISteamNetworkingSockets,
        pvTicket: *const ::std::os::raw::c_void,
        cbTicket: ::std::os::raw::c_int,
        pOutParsedTicket: *mut SteamDatagramRelayAuthTicket,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_FindRelayAuthTicketForServer(
        self_: *mut ISteamNetworkingSockets,
        identityGameServer: *const SteamNetworkingIdentity,
        nRemoteVirtualPort: ::std::os::raw::c_int,
        pOutParsedTicket: *mut SteamDatagramRelayAuthTicket,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingSockets_ConnectToHostedDedicatedServer(
        self_: *mut ISteamNetworkingSockets,
        identityTarget: *const SteamNetworkingIdentity,
        nRemoteVirtualPort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamNetConnection;
    pub fn SteamAPI_ISteamNetworkingSockets_GetHostedDedicatedServerPort(
        self_: *mut ISteamNetworkingSockets,
    ) -> uint16;
    pub fn SteamAPI_ISteamNetworkingSockets_GetHostedDedicatedServerPOPID(
        self_: *mut ISteamNetworkingSockets,
    ) -> SteamNetworkingPOPID;
    pub fn SteamAPI_ISteamNetworkingSockets_GetHostedDedicatedServerAddress(
        self_: *mut ISteamNetworkingSockets,
        pRouting: *mut SteamDatagramHostedAddress,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_CreateHostedDedicatedServerListenSocket(
        self_: *mut ISteamNetworkingSockets,
        nLocalVirtualPort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamListenSocket;
    pub fn SteamAPI_ISteamNetworkingSockets_GetGameCoordinatorServerLogin(
        self_: *mut ISteamNetworkingSockets,
        pLoginInfo: *mut SteamDatagramGameCoordinatorServerLogin,
        pcbSignedBlob: *mut ::std::os::raw::c_int,
        pBlob: *mut ::std::os::raw::c_void,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_ConnectP2PCustomSignaling(
        self_: *mut ISteamNetworkingSockets,
        pSignaling: *mut ISteamNetworkingConnectionSignaling,
        pPeerIdentity: *const SteamNetworkingIdentity,
        nRemoteVirtualPort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamNetConnection;
    pub fn SteamAPI_ISteamNetworkingSockets_ReceivedP2PCustomSignal(
        self_: *mut ISteamNetworkingSockets,
        pMsg: *const ::std::os::raw::c_void,
        cbMsg: ::std::os::raw::c_int,
        pContext: *mut ISteamNetworkingSignalingRecvContext,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_GetCertificateRequest(
        self_: *mut ISteamNetworkingSockets,
        pcbBlob: *mut ::std::os::raw::c_int,
        pBlob: *mut ::std::os::raw::c_void,
        errMsg: *mut SteamNetworkingErrMsg,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_SetCertificate(
        self_: *mut ISteamNetworkingSockets,
        pCertificate: *const ::std::os::raw::c_void,
        cbCertificate: ::std::os::raw::c_int,
        errMsg: *mut SteamNetworkingErrMsg,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_ResetIdentity(
        self_: *mut ISteamNetworkingSockets,
        pIdentity: *const SteamNetworkingIdentity,
    );
    pub fn SteamAPI_ISteamNetworkingSockets_RunCallbacks(
        self_: *mut ISteamNetworkingSockets,
    );
    pub fn SteamAPI_ISteamNetworkingSockets_BeginAsyncRequestFakeIP(
        self_: *mut ISteamNetworkingSockets,
        nNumPorts: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingSockets_GetFakeIP(
        self_: *mut ISteamNetworkingSockets,
        idxFirstPort: ::std::os::raw::c_int,
        pInfo: *mut SteamNetworkingFakeIPResult_t,
    );
    pub fn SteamAPI_ISteamNetworkingSockets_CreateListenSocketP2PFakeIP(
        self_: *mut ISteamNetworkingSockets,
        idxFakePort: ::std::os::raw::c_int,
        nOptions: ::std::os::raw::c_int,
        pOptions: *const SteamNetworkingConfigValue_t,
    ) -> HSteamListenSocket;
    pub fn SteamAPI_ISteamNetworkingSockets_GetRemoteFakeIPForConnection(
        self_: *mut ISteamNetworkingSockets,
        hConn: HSteamNetConnection,
        pOutAddr: *mut SteamNetworkingIPAddr,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingSockets_CreateFakeUDPPort(
        self_: *mut ISteamNetworkingSockets,
        idxFakeServerPort: ::std::os::raw::c_int,
    ) -> *mut ISteamNetworkingFakeUDPPort;
    pub fn SteamAPI_SteamNetworkingUtils_SteamAPI_v004() -> *mut ISteamNetworkingUtils;
    pub fn SteamAPI_ISteamNetworkingUtils_AllocateMessage(
        self_: *mut ISteamNetworkingUtils,
        cbAllocateBuffer: ::std::os::raw::c_int,
    ) -> *mut SteamNetworkingMessage_t;
    pub fn SteamAPI_ISteamNetworkingUtils_InitRelayNetworkAccess(
        self_: *mut ISteamNetworkingUtils,
    );
    pub fn SteamAPI_ISteamNetworkingUtils_GetRelayNetworkStatus(
        self_: *mut ISteamNetworkingUtils,
        pDetails: *mut SteamRelayNetworkStatus_t,
    ) -> ESteamNetworkingAvailability;
    pub fn SteamAPI_ISteamNetworkingUtils_GetLocalPingLocation(
        self_: *mut ISteamNetworkingUtils,
        result: *mut SteamNetworkPingLocation_t,
    ) -> f32;
    pub fn SteamAPI_ISteamNetworkingUtils_EstimatePingTimeBetweenTwoLocations(
        self_: *mut ISteamNetworkingUtils,
        location1: *const SteamNetworkPingLocation_t,
        location2: *const SteamNetworkPingLocation_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_EstimatePingTimeFromLocalHost(
        self_: *mut ISteamNetworkingUtils,
        remoteLocation: *const SteamNetworkPingLocation_t,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_ConvertPingLocationToString(
        self_: *mut ISteamNetworkingUtils,
        location: *const SteamNetworkPingLocation_t,
        pszBuf: *mut ::std::os::raw::c_char,
        cchBufSize: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamNetworkingUtils_ParsePingLocationString(
        self_: *mut ISteamNetworkingUtils,
        pszString: *const ::std::os::raw::c_char,
        result: *mut SteamNetworkPingLocation_t,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_CheckPingDataUpToDate(
        self_: *mut ISteamNetworkingUtils,
        flMaxAgeSeconds: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_GetPingToDataCenter(
        self_: *mut ISteamNetworkingUtils,
        popID: SteamNetworkingPOPID,
        pViaRelayPoP: *mut SteamNetworkingPOPID,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_GetDirectPingToPOP(
        self_: *mut ISteamNetworkingUtils,
        popID: SteamNetworkingPOPID,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_GetPOPCount(
        self_: *mut ISteamNetworkingUtils,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_GetPOPList(
        self_: *mut ISteamNetworkingUtils,
        list: *mut SteamNetworkingPOPID,
        nListSz: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingUtils_GetLocalTimestamp(
        self_: *mut ISteamNetworkingUtils,
    ) -> SteamNetworkingMicroseconds;
    pub fn SteamAPI_ISteamNetworkingUtils_SetDebugOutputFunction(
        self_: *mut ISteamNetworkingUtils,
        eDetailLevel: ESteamNetworkingSocketsDebugOutputType,
        pfnFunc: FSteamNetworkingSocketsDebugOutput,
    );
    pub fn SteamAPI_ISteamNetworkingUtils_IsFakeIPv4(
        self_: *mut ISteamNetworkingUtils,
        nIPv4: uint32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_GetIPv4FakeIPType(
        self_: *mut ISteamNetworkingUtils,
        nIPv4: uint32,
    ) -> ESteamNetworkingFakeIPType;
    pub fn SteamAPI_ISteamNetworkingUtils_GetRealIdentityForFakeIP(
        self_: *mut ISteamNetworkingUtils,
        fakeIP: *const SteamNetworkingIPAddr,
        pOutRealIdentity: *mut SteamNetworkingIdentity,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValueInt32(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        val: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValueFloat(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        val: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValueString(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        val: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalConfigValuePtr(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        val: *mut ::std::os::raw::c_void,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetConnectionConfigValueInt32(
        self_: *mut ISteamNetworkingUtils,
        hConn: HSteamNetConnection,
        eValue: ESteamNetworkingConfigValue,
        val: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetConnectionConfigValueFloat(
        self_: *mut ISteamNetworkingUtils,
        hConn: HSteamNetConnection,
        eValue: ESteamNetworkingConfigValue,
        val: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetConnectionConfigValueString(
        self_: *mut ISteamNetworkingUtils,
        hConn: HSteamNetConnection,
        eValue: ESteamNetworkingConfigValue,
        val: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_SteamNetConnectionStatusChanged(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamNetConnectionStatusChanged,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_SteamNetAuthenticationStatusChanged(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamNetAuthenticationStatusChanged,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_SteamRelayNetworkStatusChanged(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamRelayNetworkStatusChanged,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_FakeIPResult(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamNetworkingFakeIPResult,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_MessagesSessionRequest(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamNetworkingMessagesSessionRequest,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetGlobalCallback_MessagesSessionFailed(
        self_: *mut ISteamNetworkingUtils,
        fnCallback: FnSteamNetworkingMessagesSessionFailed,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetConfigValue(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        eScopeType: ESteamNetworkingConfigScope,
        scopeObj: isize,
        eDataType: ESteamNetworkingConfigDataType,
        pArg: *const ::std::os::raw::c_void,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SetConfigValueStruct(
        self_: *mut ISteamNetworkingUtils,
        opt: *const SteamNetworkingConfigValue_t,
        eScopeType: ESteamNetworkingConfigScope,
        scopeObj: isize,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_GetConfigValue(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        eScopeType: ESteamNetworkingConfigScope,
        scopeObj: isize,
        pOutDataType: *mut ESteamNetworkingConfigDataType,
        pResult: *mut ::std::os::raw::c_void,
        cbResult: *mut usize,
    ) -> ESteamNetworkingGetConfigValueResult;
    pub fn SteamAPI_ISteamNetworkingUtils_GetConfigValueInfo(
        self_: *mut ISteamNetworkingUtils,
        eValue: ESteamNetworkingConfigValue,
        pOutDataType: *mut ESteamNetworkingConfigDataType,
        pOutScope: *mut ESteamNetworkingConfigScope,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_ISteamNetworkingUtils_IterateGenericEditableConfigValues(
        self_: *mut ISteamNetworkingUtils,
        eCurrent: ESteamNetworkingConfigValue,
        bEnumerateDevVars: bool,
    ) -> ESteamNetworkingConfigValue;
    pub fn SteamAPI_ISteamNetworkingUtils_SteamNetworkingIPAddr_ToString(
        self_: *mut ISteamNetworkingUtils,
        addr: *const SteamNetworkingIPAddr,
        buf: *mut ::std::os::raw::c_char,
        cbBuf: uint32,
        bWithPort: bool,
    );
    pub fn SteamAPI_ISteamNetworkingUtils_SteamNetworkingIPAddr_ParseString(
        self_: *mut ISteamNetworkingUtils,
        pAddr: *mut SteamNetworkingIPAddr,
        pszStr: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamNetworkingUtils_SteamNetworkingIPAddr_GetFakeIPType(
        self_: *mut ISteamNetworkingUtils,
        addr: *const SteamNetworkingIPAddr,
    ) -> ESteamNetworkingFakeIPType;
    pub fn SteamAPI_ISteamNetworkingUtils_SteamNetworkingIdentity_ToString(
        self_: *mut ISteamNetworkingUtils,
        identity: *const SteamNetworkingIdentity,
        buf: *mut ::std::os::raw::c_char,
        cbBuf: uint32,
    );
    pub fn SteamAPI_ISteamNetworkingUtils_SteamNetworkingIdentity_ParseString(
        self_: *mut ISteamNetworkingUtils,
        pIdentity: *mut SteamNetworkingIdentity,
        pszStr: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamGameServer_v015() -> *mut ISteamGameServer;
    pub fn SteamAPI_ISteamGameServer_SetProduct(
        self_: *mut ISteamGameServer,
        pszProduct: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetGameDescription(
        self_: *mut ISteamGameServer,
        pszGameDescription: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetModDir(
        self_: *mut ISteamGameServer,
        pszModDir: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetDedicatedServer(
        self_: *mut ISteamGameServer,
        bDedicated: bool,
    );
    pub fn SteamAPI_ISteamGameServer_LogOn(
        self_: *mut ISteamGameServer,
        pszToken: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_LogOnAnonymous(self_: *mut ISteamGameServer);
    pub fn SteamAPI_ISteamGameServer_LogOff(self_: *mut ISteamGameServer);
    pub fn SteamAPI_ISteamGameServer_BLoggedOn(self_: *mut ISteamGameServer) -> bool;
    pub fn SteamAPI_ISteamGameServer_BSecure(self_: *mut ISteamGameServer) -> bool;
    pub fn SteamAPI_ISteamGameServer_GetSteamID(
        self_: *mut ISteamGameServer,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamGameServer_WasRestartRequested(
        self_: *mut ISteamGameServer,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServer_SetMaxPlayerCount(
        self_: *mut ISteamGameServer,
        cPlayersMax: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamGameServer_SetBotPlayerCount(
        self_: *mut ISteamGameServer,
        cBotplayers: ::std::os::raw::c_int,
    );
    pub fn SteamAPI_ISteamGameServer_SetServerName(
        self_: *mut ISteamGameServer,
        pszServerName: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetMapName(
        self_: *mut ISteamGameServer,
        pszMapName: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetPasswordProtected(
        self_: *mut ISteamGameServer,
        bPasswordProtected: bool,
    );
    pub fn SteamAPI_ISteamGameServer_SetSpectatorPort(
        self_: *mut ISteamGameServer,
        unSpectatorPort: uint16,
    );
    pub fn SteamAPI_ISteamGameServer_SetSpectatorServerName(
        self_: *mut ISteamGameServer,
        pszSpectatorServerName: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_ClearAllKeyValues(self_: *mut ISteamGameServer);
    pub fn SteamAPI_ISteamGameServer_SetKeyValue(
        self_: *mut ISteamGameServer,
        pKey: *const ::std::os::raw::c_char,
        pValue: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetGameTags(
        self_: *mut ISteamGameServer,
        pchGameTags: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetGameData(
        self_: *mut ISteamGameServer,
        pchGameData: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetRegion(
        self_: *mut ISteamGameServer,
        pszRegion: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_ISteamGameServer_SetAdvertiseServerActive(
        self_: *mut ISteamGameServer,
        bActive: bool,
    );
    pub fn SteamAPI_ISteamGameServer_GetAuthSessionTicket(
        self_: *mut ISteamGameServer,
        pTicket: *mut ::std::os::raw::c_void,
        cbMaxTicket: ::std::os::raw::c_int,
        pcbTicket: *mut uint32,
        pSnid: *const SteamNetworkingIdentity,
    ) -> HAuthTicket;
    pub fn SteamAPI_ISteamGameServer_BeginAuthSession(
        self_: *mut ISteamGameServer,
        pAuthTicket: *const ::std::os::raw::c_void,
        cbAuthTicket: ::std::os::raw::c_int,
        steamID: uint64_steamid,
    ) -> EBeginAuthSessionResult;
    pub fn SteamAPI_ISteamGameServer_EndAuthSession(
        self_: *mut ISteamGameServer,
        steamID: uint64_steamid,
    );
    pub fn SteamAPI_ISteamGameServer_CancelAuthTicket(
        self_: *mut ISteamGameServer,
        hAuthTicket: HAuthTicket,
    );
    pub fn SteamAPI_ISteamGameServer_UserHasLicenseForApp(
        self_: *mut ISteamGameServer,
        steamID: uint64_steamid,
        appID: AppId_t,
    ) -> EUserHasLicenseForAppResult;
    pub fn SteamAPI_ISteamGameServer_RequestUserGroupStatus(
        self_: *mut ISteamGameServer,
        steamIDUser: uint64_steamid,
        steamIDGroup: uint64_steamid,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServer_GetGameplayStats(self_: *mut ISteamGameServer);
    pub fn SteamAPI_ISteamGameServer_GetServerReputation(
        self_: *mut ISteamGameServer,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamGameServer_GetPublicIP(
        self_: *mut ISteamGameServer,
    ) -> SteamIPAddress_t;
    pub fn SteamAPI_ISteamGameServer_HandleIncomingPacket(
        self_: *mut ISteamGameServer,
        pData: *const ::std::os::raw::c_void,
        cbData: ::std::os::raw::c_int,
        srcIP: uint32,
        srcPort: uint16,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServer_GetNextOutgoingPacket(
        self_: *mut ISteamGameServer,
        pOut: *mut ::std::os::raw::c_void,
        cbMaxOut: ::std::os::raw::c_int,
        pNetAdr: *mut uint32,
        pPort: *mut uint16,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamGameServer_AssociateWithClan(
        self_: *mut ISteamGameServer,
        steamIDClan: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamGameServer_ComputeNewPlayerCompatibility(
        self_: *mut ISteamGameServer,
        steamIDNewPlayer: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamGameServer_SendUserConnectAndAuthenticate_DEPRECATED(
        self_: *mut ISteamGameServer,
        unIPClient: uint32,
        pvAuthBlob: *const ::std::os::raw::c_void,
        cubAuthBlobSize: uint32,
        pSteamIDUser: *mut CSteamID,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServer_CreateUnauthenticatedUserConnection(
        self_: *mut ISteamGameServer,
    ) -> uint64_steamid;
    pub fn SteamAPI_ISteamGameServer_SendUserDisconnect_DEPRECATED(
        self_: *mut ISteamGameServer,
        steamIDUser: uint64_steamid,
    );
    pub fn SteamAPI_ISteamGameServer_BUpdateUserData(
        self_: *mut ISteamGameServer,
        steamIDUser: uint64_steamid,
        pchPlayerName: *const ::std::os::raw::c_char,
        uScore: uint32,
    ) -> bool;
    pub fn SteamAPI_SteamGameServerStats_v001() -> *mut ISteamGameServerStats;
    pub fn SteamAPI_ISteamGameServerStats_RequestUserStats(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamGameServerStats_GetUserStatInt32(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut int32,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_GetUserStatFloat(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pData: *mut f32,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_GetUserAchievement(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        pbAchieved: *mut bool,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_SetUserStatInt32(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        nData: int32,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_SetUserStatFloat(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        fData: f32,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_UpdateUserAvgRateStat(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
        flCountThisSession: f32,
        dSessionLength: f64,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_SetUserAchievement(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_ClearUserAchievement(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
        pchName: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_ISteamGameServerStats_StoreUserStats(
        self_: *mut ISteamGameServerStats,
        steamIDUser: uint64_steamid,
    ) -> SteamAPICall_t;
    pub fn SteamAPI_ISteamNetworkingFakeUDPPort_DestroyFakeUDPPort(
        self_: *mut ISteamNetworkingFakeUDPPort,
    );
    pub fn SteamAPI_ISteamNetworkingFakeUDPPort_SendMessageToFakeIP(
        self_: *mut ISteamNetworkingFakeUDPPort,
        remoteAddress: *const SteamNetworkingIPAddr,
        pData: *const ::std::os::raw::c_void,
        cbData: uint32,
        nSendFlags: ::std::os::raw::c_int,
    ) -> EResult;
    pub fn SteamAPI_ISteamNetworkingFakeUDPPort_ReceiveMessages(
        self_: *mut ISteamNetworkingFakeUDPPort,
        ppOutMessages: *mut *mut SteamNetworkingMessage_t,
        nMaxMessages: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
    pub fn SteamAPI_ISteamNetworkingFakeUDPPort_ScheduleCleanup(
        self_: *mut ISteamNetworkingFakeUDPPort,
        remoteAddress: *const SteamNetworkingIPAddr,
    );
    pub fn SteamAPI_SteamIPAddress_t_IsSet(self_: *mut SteamIPAddress_t) -> bool;
    pub fn SteamAPI_MatchMakingKeyValuePair_t_Construct(
        self_: *mut MatchMakingKeyValuePair_t,
    );
    pub fn SteamAPI_servernetadr_t_Construct(self_: *mut servernetadr_t);
    pub fn SteamAPI_servernetadr_t_Init(
        self_: *mut servernetadr_t,
        ip: ::std::os::raw::c_uint,
        usQueryPort: uint16,
        usConnectionPort: uint16,
    );
    pub fn SteamAPI_servernetadr_t_GetQueryPort(self_: *mut servernetadr_t) -> uint16;
    pub fn SteamAPI_servernetadr_t_SetQueryPort(
        self_: *mut servernetadr_t,
        usPort: uint16,
    );
    pub fn SteamAPI_servernetadr_t_GetConnectionPort(
        self_: *mut servernetadr_t,
    ) -> uint16;
    pub fn SteamAPI_servernetadr_t_SetConnectionPort(
        self_: *mut servernetadr_t,
        usPort: uint16,
    );
    pub fn SteamAPI_servernetadr_t_GetIP(self_: *mut servernetadr_t) -> uint32;
    pub fn SteamAPI_servernetadr_t_SetIP(self_: *mut servernetadr_t, unIP: uint32);
    pub fn SteamAPI_servernetadr_t_GetConnectionAddressString(
        self_: *mut servernetadr_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_servernetadr_t_GetQueryAddressString(
        self_: *mut servernetadr_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_servernetadr_t_IsLessThan(
        self_: *mut servernetadr_t,
        netadr: *const servernetadr_t,
    ) -> bool;
    pub fn SteamAPI_servernetadr_t_Assign(
        self_: *mut servernetadr_t,
        that: *const servernetadr_t,
    );
    pub fn SteamAPI_gameserveritem_t_Construct(self_: *mut gameserveritem_t);
    pub fn SteamAPI_gameserveritem_t_GetName(
        self_: *mut gameserveritem_t,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_gameserveritem_t_SetName(
        self_: *mut gameserveritem_t,
        pName: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_SteamNetworkingIPAddr_Clear(self_: *mut SteamNetworkingIPAddr);
    pub fn SteamAPI_SteamNetworkingIPAddr_IsIPv6AllZeros(
        self_: *mut SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIPAddr_SetIPv6(
        self_: *mut SteamNetworkingIPAddr,
        ipv6: *const uint8,
        nPort: uint16,
    );
    pub fn SteamAPI_SteamNetworkingIPAddr_SetIPv4(
        self_: *mut SteamNetworkingIPAddr,
        nIP: uint32,
        nPort: uint16,
    );
    pub fn SteamAPI_SteamNetworkingIPAddr_IsIPv4(
        self_: *mut SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIPAddr_GetIPv4(
        self_: *mut SteamNetworkingIPAddr,
    ) -> uint32;
    pub fn SteamAPI_SteamNetworkingIPAddr_SetIPv6LocalHost(
        self_: *mut SteamNetworkingIPAddr,
        nPort: uint16,
    );
    pub fn SteamAPI_SteamNetworkingIPAddr_IsLocalHost(
        self_: *mut SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIPAddr_ToString(
        self_: *mut SteamNetworkingIPAddr,
        buf: *mut ::std::os::raw::c_char,
        cbBuf: uint32,
        bWithPort: bool,
    );
    pub fn SteamAPI_SteamNetworkingIPAddr_ParseString(
        self_: *mut SteamNetworkingIPAddr,
        pszStr: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIPAddr_IsEqualTo(
        self_: *mut SteamNetworkingIPAddr,
        x: *const SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIPAddr_GetFakeIPType(
        self_: *mut SteamNetworkingIPAddr,
    ) -> ESteamNetworkingFakeIPType;
    pub fn SteamAPI_SteamNetworkingIPAddr_IsFakeIP(
        self_: *mut SteamNetworkingIPAddr,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_Clear(self_: *mut SteamNetworkingIdentity);
    pub fn SteamAPI_SteamNetworkingIdentity_IsInvalid(
        self_: *mut SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_SetSteamID(
        self_: *mut SteamNetworkingIdentity,
        steamID: uint64_steamid,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_GetSteamID(
        self_: *mut SteamNetworkingIdentity,
    ) -> uint64_steamid;
    pub fn SteamAPI_SteamNetworkingIdentity_SetSteamID64(
        self_: *mut SteamNetworkingIdentity,
        steamID: uint64,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_GetSteamID64(
        self_: *mut SteamNetworkingIdentity,
    ) -> uint64;
    pub fn SteamAPI_SteamNetworkingIdentity_SetXboxPairwiseID(
        self_: *mut SteamNetworkingIdentity,
        pszString: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_GetXboxPairwiseID(
        self_: *mut SteamNetworkingIdentity,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_SteamNetworkingIdentity_SetPSNID(
        self_: *mut SteamNetworkingIdentity,
        id: uint64,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_GetPSNID(
        self_: *mut SteamNetworkingIdentity,
    ) -> uint64;
    pub fn SteamAPI_SteamNetworkingIdentity_SetIPAddr(
        self_: *mut SteamNetworkingIdentity,
        addr: *const SteamNetworkingIPAddr,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_GetIPAddr(
        self_: *mut SteamNetworkingIdentity,
    ) -> *const SteamNetworkingIPAddr;
    pub fn SteamAPI_SteamNetworkingIdentity_SetIPv4Addr(
        self_: *mut SteamNetworkingIdentity,
        nIPv4: uint32,
        nPort: uint16,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_GetIPv4(
        self_: *mut SteamNetworkingIdentity,
    ) -> uint32;
    pub fn SteamAPI_SteamNetworkingIdentity_GetFakeIPType(
        self_: *mut SteamNetworkingIdentity,
    ) -> ESteamNetworkingFakeIPType;
    pub fn SteamAPI_SteamNetworkingIdentity_IsFakeIP(
        self_: *mut SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_SetLocalHost(
        self_: *mut SteamNetworkingIdentity,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_IsLocalHost(
        self_: *mut SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_SetGenericString(
        self_: *mut SteamNetworkingIdentity,
        pszString: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_GetGenericString(
        self_: *mut SteamNetworkingIdentity,
    ) -> *const ::std::os::raw::c_char;
    pub fn SteamAPI_SteamNetworkingIdentity_SetGenericBytes(
        self_: *mut SteamNetworkingIdentity,
        data: *const ::std::os::raw::c_void,
        cbLen: uint32,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_GetGenericBytes(
        self_: *mut SteamNetworkingIdentity,
        cbLen: *mut ::std::os::raw::c_int,
    ) -> *const uint8;
    pub fn SteamAPI_SteamNetworkingIdentity_IsEqualTo(
        self_: *mut SteamNetworkingIdentity,
        x: *const SteamNetworkingIdentity,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingIdentity_ToString(
        self_: *mut SteamNetworkingIdentity,
        buf: *mut ::std::os::raw::c_char,
        cbBuf: uint32,
    );
    pub fn SteamAPI_SteamNetworkingIdentity_ParseString(
        self_: *mut SteamNetworkingIdentity,
        pszStr: *const ::std::os::raw::c_char,
    ) -> bool;
    pub fn SteamAPI_SteamNetworkingMessage_t_Release(
        self_: *mut SteamNetworkingMessage_t,
    );
    pub fn SteamAPI_SteamNetworkingConfigValue_t_SetInt32(
        self_: *mut SteamNetworkingConfigValue_t,
        eVal: ESteamNetworkingConfigValue,
        data: i32,
    );
    pub fn SteamAPI_SteamNetworkingConfigValue_t_SetInt64(
        self_: *mut SteamNetworkingConfigValue_t,
        eVal: ESteamNetworkingConfigValue,
        data: i64,
    );
    pub fn SteamAPI_SteamNetworkingConfigValue_t_SetFloat(
        self_: *mut SteamNetworkingConfigValue_t,
        eVal: ESteamNetworkingConfigValue,
        data: f32,
    );
    pub fn SteamAPI_SteamNetworkingConfigValue_t_SetPtr(
        self_: *mut SteamNetworkingConfigValue_t,
        eVal: ESteamNetworkingConfigValue,
        data: *mut ::std::os::raw::c_void,
    );
    pub fn SteamAPI_SteamNetworkingConfigValue_t_SetString(
        self_: *mut SteamNetworkingConfigValue_t,
        eVal: ESteamNetworkingConfigValue,
        data: *const ::std::os::raw::c_char,
    );
    pub fn SteamAPI_SteamDatagramHostedAddress_Clear(
        self_: *mut SteamDatagramHostedAddress,
    );
    pub fn SteamAPI_SteamDatagramHostedAddress_GetPopID(
        self_: *mut SteamDatagramHostedAddress,
    ) -> SteamNetworkingPOPID;
    pub fn SteamAPI_SteamDatagramHostedAddress_SetDevAddress(
        self_: *mut SteamDatagramHostedAddress,
        nIP: uint32,
        nPort: uint16,
        popid: SteamNetworkingPOPID,
    );
}
pub const k_nSteamEncryptedAppTicketSymmetricKeyLen: ::std::os::raw::c_int = 32;
unsafe extern "C" {
    pub fn SteamEncryptedAppTicket_BDecryptTicket(
        rgubTicketEncrypted: *const uint8,
        cubTicketEncrypted: uint32,
        rgubTicketDecrypted: *mut uint8,
        pcubTicketDecrypted: *mut uint32,
        rgubKey: *const uint8,
        cubKey: ::std::os::raw::c_int,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_BIsTicketForApp(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        nAppID: AppId_t,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_GetTicketIssueTime(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
    ) -> RTime32;
    pub fn SteamEncryptedAppTicket_GetTicketSteamID(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        psteamID: *mut CSteamID,
    );
    pub fn SteamEncryptedAppTicket_GetTicketAppID(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
    ) -> AppId_t;
    pub fn SteamEncryptedAppTicket_BUserOwnsAppInTicket(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        nAppID: AppId_t,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_BUserIsVacBanned(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_BGetAppDefinedValue(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        pValue: *mut uint32,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_GetUserVariableData(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        pcubUserData: *mut uint32,
    ) -> *const uint8;
    pub fn SteamEncryptedAppTicket_BIsTicketSigned(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
        pubRSAKey: *const uint8,
        cubRSAKey: uint32,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_BIsLicenseBorrowed(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
    ) -> bool;
    pub fn SteamEncryptedAppTicket_BIsLicenseTemporary(
        rgubTicketDecrypted: *mut uint8,
        cubTicketDecrypted: uint32,
    ) -> bool;
}
#[repr(C)]
pub struct ISteamAppTicket__bindgen_vtable(::std::os::raw::c_void);
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ISteamAppTicket {
    pub vtable_: *const ISteamAppTicket__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ISteamAppTicket"][::std::mem::size_of::<ISteamAppTicket>() - 8usize];
    ["Alignment of ISteamAppTicket"][::std::mem::align_of::<ISteamAppTicket>() - 8usize];
};
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EServerMode {
    eServerModeInvalid = 0,
    eServerModeNoAuthentication = 1,
    eServerModeAuthentication = 2,
    eServerModeAuthenticationAndSecure = 3,
}
pub const STEAMGAMESERVER_QUERY_PORT_SHARED: uint16 = 65535;
pub const MASTERSERVERUPDATERPORT_USEGAMESOCKETSHARE: uint16 = 65535;
unsafe extern "C" {
    pub fn SteamGameServer_Shutdown();
    pub fn SteamGameServer_BSecure() -> bool;
    pub fn SteamGameServer_GetSteamID() -> uint64;
    pub fn SteamInternal_GameServer_Init_V2(
        unIP: uint32,
        usGamePort: uint16,
        usQueryPort: uint16,
        eServerMode: EServerMode,
        pchVersionString: *const ::std::os::raw::c_char,
        pszInternalCheckInterfaceVersions: *const ::std::os::raw::c_char,
        pOutErrMsg: *mut SteamErrMsg,
    ) -> ESteamAPIInitResult;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of __va_list_tag"][::std::mem::size_of::<__va_list_tag>() - 24usize];
    ["Alignment of __va_list_tag"][::std::mem::align_of::<__va_list_tag>() - 8usize];
    [
        "Offset of field: __va_list_tag::gp_offset",
    ][::std::mem::offset_of!(__va_list_tag, gp_offset) - 0usize];
    [
        "Offset of field: __va_list_tag::fp_offset",
    ][::std::mem::offset_of!(__va_list_tag, fp_offset) - 4usize];
    [
        "Offset of field: __va_list_tag::overflow_arg_area",
    ][::std::mem::offset_of!(__va_list_tag, overflow_arg_area) - 8usize];
    [
        "Offset of field: __va_list_tag::reg_save_area",
    ][::std::mem::offset_of!(__va_list_tag, reg_save_area) - 16usize];
};
