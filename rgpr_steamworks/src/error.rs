use crate::sys;
use crate::util::CStrArray;
use std::error::Error as StdError;
use std::ffi::NulError;
use std::fmt::{Debug, Display, Formatter};
use std::num::TryFromIntError;

#[derive(Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum CallError<E: Debug + StdError> {
	/// Steamworks call failed.
	#[error("SteamAPI gave failure flag from callback")]
	Failed,

	/// The call manager shutdown.
	#[error("the CallManager is being shutdown")]
	Shutdown,

	/// Error from the dispatcher's implementation.
	#[error("Dispatch-specific error {0:?}")]
	Specific(E),
}

impl<E: Debug + StdError + Clone> Clone for CallError<E> {
	fn clone(&self) -> Self {
		match self {
			Self::Failed => Self::Failed,
			Self::Shutdown => Self::Shutdown,
			Self::Specific(error) => Self::Specific(error.clone()),
		}
	}
}

impl<E: Debug + StdError + Copy> Copy for CallError<E> {}

impl<E: Debug + StdError> From<CallFutureError> for CallError<E> {
	fn from(value: CallFutureError) -> Self {
		match value {
			CallFutureError::Failed => CallError::Failed,
			CallFutureError::Shutdown => CallError::Shutdown,
			CallFutureError::Moved => panic!("attempted to use CallFutureError::Moved"),
			CallFutureError::EarlyDrop => panic!("attempted to use CallFutureError::EarlyDrop"),
		}
	}
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum CallFutureError {
	/// Steamworks call failed.
	#[error("SteamAPI gave failure flag from callback")]
	Failed,

	/// The call manager shutdown.CallErrorPrivate::Moved
	#[error("the CallManager is being shutdown")]
	Shutdown,

	/// Put in the place of good data once it is moved.
	/// Flags a memory location has having fulfilled its purpose.
	#[error("CallFuture's result has been moved")]
	Moved,

	/// Used when the CallFuture's result was never yielded by the future.
	#[error("CallFuture was dropped before its result could be polled")]
	EarlyDrop,
}

/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, thiserror::Error)]
pub enum ChatRoomEnterError {
	/// > Chat doesn't exist (probably closed).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Chat doesn't exist")]
	DoesntExist,

	/// > General denied - You don't have the permissions needed to join the chat.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("You don't have the permissions needed to join the chat")]
	NotAllowed,

	/// > Chat room has reached its maximum size.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Chat room has reached its maximum size")]
	Full,

	/// > Unexpected Error.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Unexpected Error")]
	UnexpectedError,

	/// > You are banned from this chat room and may not join.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("You are banned from this chat room and may not join")]
	Banned,

	/// > Joining this chat is not allowed because you are a limited user (no value on account).
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Joining this chat is not allowed because you are a limited user (no value on account)")]
	Limited,

	/// > Attempt to join a clan chat when the clan is locked or disabled.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Attempt to join a clan chat when the clan is locked or disabled")]
	ClanDisabled,

	/// > Attempt to join a chat when the user has a community lock on their account.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Attempt to join a chat when the user has a community lock on their account")]
	CommunityBan,

	/// > Join failed - a user that is in the chat has blocked you from joining.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Join failed - a user that is in the chat has blocked you from joining")]
	MemberBlockedYou,

	/// > Join failed - you have blocked a user that is already in the chat.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Join failed - you have blocked a user that is already in the chat")]
	YouBlockedMember,

	/// > Join failed - too many join attempts in a very short period of time.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/steam_api#EChatRoomEnterResponse)
	#[error("Join failed - too many join attempts in a very short period of time")]
	RateLimitExceeded,

	#[error("Unknown error: {0}")]
	Unknown(u32),
}

impl TryFrom<u32> for ChatRoomEnterError {
	type Error = ();

	fn try_from(value: u32) -> Result<Self, Self::Error> {
		use sys::EChatRoomEnterResponse::*;

		const SUCCESS: u32 = k_EChatRoomEnterResponseSuccess as u32;
		const DOESNTEXIST: u32 = k_EChatRoomEnterResponseDoesntExist as u32;
		const NOTALLOWED: u32 = k_EChatRoomEnterResponseNotAllowed as u32;
		const FULL: u32 = k_EChatRoomEnterResponseFull as u32;
		const ERROR: u32 = k_EChatRoomEnterResponseError as u32;
		const BANNED: u32 = k_EChatRoomEnterResponseBanned as u32;
		const LIMITED: u32 = k_EChatRoomEnterResponseLimited as u32;
		const CLANDISABLED: u32 = k_EChatRoomEnterResponseClanDisabled as u32;
		const COMMUNITYBAN: u32 = k_EChatRoomEnterResponseCommunityBan as u32;
		const MEMBERBLOCKEDYOU: u32 = k_EChatRoomEnterResponseMemberBlockedYou as u32;
		const YOUBLOCKEDMEMBER: u32 = k_EChatRoomEnterResponseYouBlockedMember as u32;
		const RATELIMITEXCEEDED: u32 = k_EChatRoomEnterResponseRatelimitExceeded as u32;

		Ok(match value {
			SUCCESS => return Err(()),
			DOESNTEXIST => Self::DoesntExist,
			NOTALLOWED => Self::NotAllowed,
			FULL => Self::Full,
			ERROR => Self::UnexpectedError,
			BANNED => Self::Banned,
			LIMITED => Self::Limited,
			CLANDISABLED => Self::ClanDisabled,
			COMMUNITYBAN => Self::CommunityBan,
			MEMBERBLOCKEDYOU => Self::MemberBlockedYou,
			YOUBLOCKEDMEMBER => Self::YouBlockedMember,
			RATELIMITEXCEEDED => Self::RateLimitExceeded,
			unknown => Self::Unknown(unknown),
		})
	}
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
	#[error("interface to the Steam API already exsists, use Steam::get instead")]
	AlreadyExists,

	#[error("locations for data were not filled")]
	DataUnfulfilled,

	#[error("general error: {0:?}")]
	General(#[from] GeneralError),

	#[error("Steam API initialization error {0:?}: \"{1}\"")]
	SteamInit(InitErrorEnum, String),

	#[error("string conversion error, string must not contain nulls")]
	StrNulError(#[from] NulError),

	/// Failed to init steamworks as the executable was not started through steam,
	/// and the executable is now being started through steam.
	/// You should exit entirely if this error is received.
	#[error("restarting through steam")]
	RestartingThroughSteam,

	#[error("failed, no error message from the Steam API is available")]
	SilentFailure,
}

impl Error {
	pub(crate) fn steam_init(init_result: sys::ESteamAPIInitResult, message: CStrArray<1024>) -> Option<Self> {
		Some(Self::SteamInit(InitErrorEnum::new(init_result)?, message.to_string()))
	}
}

impl From<SilentFailure> for Error {
	fn from(_: SilentFailure) -> Self {
		Self::SilentFailure
	}
}

/// The unsuccessful variants of [EResult](https://partner.steamgames.com/doc/api/steam_api#EResult).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, thiserror::Error)]
pub enum GeneralError {
	#[error("SteamAPI error: unknown error {0}")]
	Unknown(i32),

	#[error("SteamAPI error: Fail")]
	Fail,

	#[error("SteamAPI error: NoConnection")]
	NoConnection,

	#[error("SteamAPI error: InvalidPassword")]
	InvalidPassword,

	#[error("SteamAPI error: LoggedInElsewhere")]
	LoggedInElsewhere,

	#[error("SteamAPI error: InvalidProtocolVer")]
	InvalidProtocolVer,

	#[error("SteamAPI error: InvalidParam")]
	InvalidParam,

	#[error("SteamAPI error: FileNotFound")]
	FileNotFound,

	#[error("SteamAPI error: Busy")]
	Busy,

	#[error("SteamAPI error: InvalidState")]
	InvalidState,

	#[error("SteamAPI error: InvalidName")]
	InvalidName,

	#[error("SteamAPI error: InvalidEmail")]
	InvalidEmail,

	#[error("SteamAPI error: DuplicateName")]
	DuplicateName,

	#[error("SteamAPI error: AccessDenied")]
	AccessDenied,

	#[error("SteamAPI error: Timeout")]
	Timeout,

	#[error("SteamAPI error: Banned")]
	Banned,

	#[error("SteamAPI error: AccountNotFound")]
	AccountNotFound,

	#[error("SteamAPI error: InvalidSteamID")]
	InvalidSteamID,

	#[error("SteamAPI error: ServiceUnavailable")]
	ServiceUnavailable,

	#[error("SteamAPI error: NotLoggedOn")]
	NotLoggedOn,

	#[error("SteamAPI error: Pending")]
	Pending,

	#[error("SteamAPI error: EncryptionFailure")]
	EncryptionFailure,

	#[error("SteamAPI error: InsufficientPrivilege")]
	InsufficientPrivilege,

	#[error("SteamAPI error: LimitExceeded")]
	LimitExceeded,

	#[error("SteamAPI error: Revoked")]
	Revoked,

	#[error("SteamAPI error: Expired")]
	Expired,

	#[error("SteamAPI error: AlreadyRedeemed")]
	AlreadyRedeemed,

	#[error("SteamAPI error: DuplicateRequest")]
	DuplicateRequest,

	#[error("SteamAPI error: AlreadyOwned")]
	AlreadyOwned,

	#[error("SteamAPI error: IPNotFound")]
	IPNotFound,

	#[error("SteamAPI error: PersistFailed")]
	PersistFailed,

	#[error("SteamAPI error: LockingFailed")]
	LockingFailed,

	#[error("SteamAPI error: LogonSessionReplaced")]
	LogonSessionReplaced,

	#[error("SteamAPI error: ConnectFailed")]
	ConnectFailed,

	#[error("SteamAPI error: HandshakeFailed")]
	HandshakeFailed,

	#[error("SteamAPI error: IOFailure")]
	IOFailure,

	#[error("SteamAPI error: RemoteDisconnect")]
	RemoteDisconnect,

	#[error("SteamAPI error: ShoppingCartNotFound")]
	ShoppingCartNotFound,

	#[error("SteamAPI error: Blocked")]
	Blocked,

	#[error("SteamAPI error: Ignored")]
	Ignored,

	#[error("SteamAPI error: NoMatch")]
	NoMatch,

	#[error("SteamAPI error: AccountDisabled")]
	AccountDisabled,

	#[error("SteamAPI error: ServiceReadOnly")]
	ServiceReadOnly,

	#[error("SteamAPI error: AccountNotFeatured")]
	AccountNotFeatured,

	#[error("SteamAPI error: AdministratorOK")]
	AdministratorOK,

	#[error("SteamAPI error: ContentVersion")]
	ContentVersion,

	#[error("SteamAPI error: TryAnotherCM")]
	TryAnotherCM,

	#[error("SteamAPI error: PasswordRequiredToKickSession")]
	PasswordRequiredToKickSession,

	#[error("SteamAPI error: AlreadyLoggedInElsewhere")]
	AlreadyLoggedInElsewhere,

	#[error("SteamAPI error: Suspended")]
	Suspended,

	#[error("SteamAPI error: Cancelled")]
	Cancelled,

	#[error("SteamAPI error: DataCorruption")]
	DataCorruption,

	#[error("SteamAPI error: DiskFull")]
	DiskFull,

	#[error("SteamAPI error: RemoteCallFailed")]
	RemoteCallFailed,

	#[error("SteamAPI error: PasswordUnset")]
	PasswordUnset,

	#[error("SteamAPI error: ExternalAccountUnlinked")]
	ExternalAccountUnlinked,

	#[error("SteamAPI error: PSNTicketInvalid")]
	PSNTicketInvalid,

	#[error("SteamAPI error: ExternalAccountAlreadyLinked")]
	ExternalAccountAlreadyLinked,

	#[error("SteamAPI error: RemoteFileConflict")]
	RemoteFileConflict,

	#[error("SteamAPI error: IllegalPassword")]
	IllegalPassword,

	#[error("SteamAPI error: SameAsPreviousValue")]
	SameAsPreviousValue,

	#[error("SteamAPI error: AccountLogonDenied")]
	AccountLogonDenied,

	#[error("SteamAPI error: CannotUseOldPassword")]
	CannotUseOldPassword,

	#[error("SteamAPI error: InvalidLoginAuthCode")]
	InvalidLoginAuthCode,

	#[error("SteamAPI error: AccountLogonDeniedNoMail")]
	AccountLogonDeniedNoMail,

	#[error("SteamAPI error: HardwareNotCapableOfIPT")]
	HardwareNotCapableOfIPT,

	#[error("SteamAPI error: IPTInitError")]
	IPTInitError,

	#[error("SteamAPI error: ParentalControlRestricted")]
	ParentalControlRestricted,

	#[error("SteamAPI error: FacebookQueryError")]
	FacebookQueryError,

	#[error("SteamAPI error: ExpiredLoginAuthCode")]
	ExpiredLoginAuthCode,

	#[error("SteamAPI error: IPLoginRestrictionFailed")]
	IPLoginRestrictionFailed,

	#[error("SteamAPI error: AccountLockedDown")]
	AccountLockedDown,

	#[error("SteamAPI error: AccountLogonDeniedVerifiedEmailRequired")]
	AccountLogonDeniedVerifiedEmailRequired,

	#[error("SteamAPI error: NoMatchingURL")]
	NoMatchingURL,

	#[error("SteamAPI error: BadResponse")]
	BadResponse,

	#[error("SteamAPI error: RequirePasswordReEntry")]
	RequirePasswordReEntry,

	#[error("SteamAPI error: ValueOutOfRange")]
	ValueOutOfRange,

	#[error("SteamAPI error: UnexpectedError")]
	UnexpectedError,

	#[error("SteamAPI error: Disabled")]
	Disabled,

	#[error("SteamAPI error: InvalidCEGSubmission")]
	InvalidCEGSubmission,

	#[error("SteamAPI error: RestrictedDevice")]
	RestrictedDevice,

	#[error("SteamAPI error: RegionLocked")]
	RegionLocked,

	#[error("SteamAPI error: RateLimitExceeded")]
	RateLimitExceeded,

	#[error("SteamAPI error: AccountLoginDeniedNeedTwoFactor")]
	AccountLoginDeniedNeedTwoFactor,

	#[error("SteamAPI error: ItemDeleted")]
	ItemDeleted,

	#[error("SteamAPI error: AccountLoginDeniedThrottle")]
	AccountLoginDeniedThrottle,

	#[error("SteamAPI error: TwoFactorCodeMismatch")]
	TwoFactorCodeMismatch,

	#[error("SteamAPI error: TwoFactorActivationCodeMismatch")]
	TwoFactorActivationCodeMismatch,

	#[error("SteamAPI error: AccountAssociatedToMultiplePartners")]
	AccountAssociatedToMultiplePartners,

	#[error("SteamAPI error: NotModified")]
	NotModified,

	#[error("SteamAPI error: NoMobileDevice")]
	NoMobileDevice,

	#[error("SteamAPI error: TimeNotSynced")]
	TimeNotSynced,

	#[error("SteamAPI error: SmsCodeFailed")]
	SmsCodeFailed,

	#[error("SteamAPI error: AccountLimitExceeded")]
	AccountLimitExceeded,

	#[error("SteamAPI error: AccountActivityLimitExceeded")]
	AccountActivityLimitExceeded,

	#[error("SteamAPI error: PhoneActivityLimitExceeded")]
	PhoneActivityLimitExceeded,

	#[error("SteamAPI error: RefundToWallet")]
	RefundToWallet,

	#[error("SteamAPI error: EmailSendFailure")]
	EmailSendFailure,

	#[error("SteamAPI error: NotSettled")]
	NotSettled,

	#[error("SteamAPI error: NeedCaptcha")]
	NeedCaptcha,

	#[error("SteamAPI error: GSLTDenied")]
	GSLTDenied,

	#[error("SteamAPI error: GSOwnerDenied")]
	GSOwnerDenied,

	#[error("SteamAPI error: InvalidItemType")]
	InvalidItemType,

	#[error("SteamAPI error: IPBanned")]
	IPBanned,

	#[error("SteamAPI error: GSLTExpired")]
	GSLTExpired,

	#[error("SteamAPI error: InsufficientFunds")]
	InsufficientFunds,

	#[error("SteamAPI error: TooManyPending")]
	TooManyPending,

	#[error("SteamAPI error: NoSiteLicensesFound")]
	NoSiteLicensesFound,

	#[error("SteamAPI error: WGNetworkSendExceeded")]
	WGNetworkSendExceeded,

	#[error("SteamAPI error: AccountNotFriends")]
	AccountNotFriends,

	#[error("SteamAPI error: LimitedUserAccount")]
	LimitedUserAccount,

	#[error("SteamAPI error: CantRemoveItem")]
	CantRemoveItem,

	#[error("SteamAPI error: AccountDeleted")]
	AccountDeleted,

	#[error("SteamAPI error: ExistingUserCancelledLicense")]
	ExistingUserCancelledLicense,

	#[error("SteamAPI error: CommunityCooldown")]
	CommunityCooldown,

	#[error("SteamAPI error: NoLauncherSpecified")]
	NoLauncherSpecified,

	#[error("SteamAPI error: MustAgreeToSSA")]
	MustAgreeToSSA,

	#[error("SteamAPI error: LauncherMigrated")]
	LauncherMigrated,

	#[error("SteamAPI error: SteamRealmMismatch")]
	SteamRealmMismatch,

	#[error("SteamAPI error: InvalidSignature")]
	InvalidSignature,

	#[error("SteamAPI error: ParseFailure")]
	ParseFailure,

	#[error("SteamAPI error: NoVerifiedPhone")]
	NoVerifiedPhone,

	#[error("SteamAPI error: InsufficientBattery")]
	InsufficientBattery,

	#[error("SteamAPI error: ChargerRequired")]
	ChargerRequired,

	#[error("SteamAPI error: CachedCredentialInvalid")]
	CachedCredentialInvalid,

	#[error("SteamAPI error: PhoneNumberIsVOIP")]
	PhoneNumberIsVOIP,

	#[error("SteamAPI error: NotSupported")]
	NotSupported,

	#[error("SteamAPI error: FamilySizeLimitExceeded")]
	FamilySizeLimitExceeded,

	#[error("SteamAPI error: OfflineAppCacheInvalid")]
	OfflineAppCacheInvalid,
}

impl GeneralError {
	pub(crate) fn new(steam_e_result: sys::EResult) -> Option<Self> {
		use sys::EResult::*;
		use GeneralError::*;

		Some(match steam_e_result {
			k_EResultNone | k_EResultOK => return None,
			k_EResultFail => Fail,
			k_EResultNoConnection => NoConnection,
			k_EResultInvalidPassword => InvalidPassword,
			k_EResultLoggedInElsewhere => LoggedInElsewhere,
			k_EResultInvalidProtocolVer => InvalidProtocolVer,
			k_EResultInvalidParam => InvalidParam,
			k_EResultFileNotFound => FileNotFound,
			k_EResultBusy => Busy,
			k_EResultInvalidState => InvalidState,
			k_EResultInvalidName => InvalidName,
			k_EResultInvalidEmail => InvalidEmail,
			k_EResultDuplicateName => DuplicateName,
			k_EResultAccessDenied => AccessDenied,
			k_EResultTimeout => Timeout,
			k_EResultBanned => Banned,
			k_EResultAccountNotFound => AccountNotFound,
			k_EResultInvalidSteamID => InvalidSteamID,
			k_EResultServiceUnavailable => ServiceUnavailable,
			k_EResultNotLoggedOn => NotLoggedOn,
			k_EResultPending => Pending,
			k_EResultEncryptionFailure => EncryptionFailure,
			k_EResultInsufficientPrivilege => InsufficientPrivilege,
			k_EResultLimitExceeded => LimitExceeded,
			k_EResultRevoked => Revoked,
			k_EResultExpired => Expired,
			k_EResultAlreadyRedeemed => AlreadyRedeemed,
			k_EResultDuplicateRequest => DuplicateRequest,
			k_EResultAlreadyOwned => AlreadyOwned,
			k_EResultIPNotFound => IPNotFound,
			k_EResultPersistFailed => PersistFailed,
			k_EResultLockingFailed => LockingFailed,
			k_EResultLogonSessionReplaced => LogonSessionReplaced,
			k_EResultConnectFailed => ConnectFailed,
			k_EResultHandshakeFailed => HandshakeFailed,
			k_EResultIOFailure => IOFailure,
			k_EResultRemoteDisconnect => RemoteDisconnect,
			k_EResultShoppingCartNotFound => ShoppingCartNotFound,
			k_EResultBlocked => Blocked,
			k_EResultIgnored => Ignored,
			k_EResultNoMatch => NoMatch,
			k_EResultAccountDisabled => AccountDisabled,
			k_EResultServiceReadOnly => ServiceReadOnly,
			k_EResultAccountNotFeatured => AccountNotFeatured,
			k_EResultAdministratorOK => AdministratorOK,
			k_EResultContentVersion => ContentVersion,
			k_EResultTryAnotherCM => TryAnotherCM,
			k_EResultPasswordRequiredToKickSession => PasswordRequiredToKickSession,
			k_EResultAlreadyLoggedInElsewhere => AlreadyLoggedInElsewhere,
			k_EResultSuspended => Suspended,
			k_EResultCancelled => Cancelled,
			k_EResultDataCorruption => DataCorruption,
			k_EResultDiskFull => DiskFull,
			k_EResultRemoteCallFailed => RemoteCallFailed,
			k_EResultPasswordUnset => PasswordUnset,
			k_EResultExternalAccountUnlinked => ExternalAccountUnlinked,
			k_EResultPSNTicketInvalid => PSNTicketInvalid,
			k_EResultExternalAccountAlreadyLinked => ExternalAccountAlreadyLinked,
			k_EResultRemoteFileConflict => RemoteFileConflict,
			k_EResultIllegalPassword => IllegalPassword,
			k_EResultSameAsPreviousValue => SameAsPreviousValue,
			k_EResultAccountLogonDenied => AccountLogonDenied,
			k_EResultCannotUseOldPassword => CannotUseOldPassword,
			k_EResultInvalidLoginAuthCode => InvalidLoginAuthCode,
			k_EResultAccountLogonDeniedNoMail => AccountLogonDeniedNoMail,
			k_EResultHardwareNotCapableOfIPT => HardwareNotCapableOfIPT,
			k_EResultIPTInitError => IPTInitError,
			k_EResultParentalControlRestricted => ParentalControlRestricted,
			k_EResultFacebookQueryError => FacebookQueryError,
			k_EResultExpiredLoginAuthCode => ExpiredLoginAuthCode,
			k_EResultIPLoginRestrictionFailed => IPLoginRestrictionFailed,
			k_EResultAccountLockedDown => AccountLockedDown,
			k_EResultAccountLogonDeniedVerifiedEmailRequired => AccountLogonDeniedVerifiedEmailRequired,
			k_EResultNoMatchingURL => NoMatchingURL,
			k_EResultBadResponse => BadResponse,
			k_EResultRequirePasswordReEntry => RequirePasswordReEntry,
			k_EResultValueOutOfRange => ValueOutOfRange,
			k_EResultUnexpectedError => UnexpectedError,
			k_EResultDisabled => Disabled,
			k_EResultInvalidCEGSubmission => InvalidCEGSubmission,
			k_EResultRestrictedDevice => RestrictedDevice,
			k_EResultRegionLocked => RegionLocked,
			k_EResultRateLimitExceeded => RateLimitExceeded,
			k_EResultAccountLoginDeniedNeedTwoFactor => AccountLoginDeniedNeedTwoFactor,
			k_EResultItemDeleted => ItemDeleted,
			k_EResultAccountLoginDeniedThrottle => AccountLoginDeniedThrottle,
			k_EResultTwoFactorCodeMismatch => TwoFactorCodeMismatch,
			k_EResultTwoFactorActivationCodeMismatch => TwoFactorActivationCodeMismatch,
			k_EResultAccountAssociatedToMultiplePartners => AccountAssociatedToMultiplePartners,
			k_EResultNotModified => NotModified,
			k_EResultNoMobileDevice => NoMobileDevice,
			k_EResultTimeNotSynced => TimeNotSynced,
			k_EResultSmsCodeFailed => SmsCodeFailed,
			k_EResultAccountLimitExceeded => AccountLimitExceeded,
			k_EResultAccountActivityLimitExceeded => AccountActivityLimitExceeded,
			k_EResultPhoneActivityLimitExceeded => PhoneActivityLimitExceeded,
			k_EResultRefundToWallet => RefundToWallet,
			k_EResultEmailSendFailure => EmailSendFailure,
			k_EResultNotSettled => NotSettled,
			k_EResultNeedCaptcha => NeedCaptcha,
			k_EResultGSLTDenied => GSLTDenied,
			k_EResultGSOwnerDenied => GSOwnerDenied,
			k_EResultInvalidItemType => InvalidItemType,
			k_EResultIPBanned => IPBanned,
			k_EResultGSLTExpired => GSLTExpired,
			k_EResultInsufficientFunds => InsufficientFunds,
			k_EResultTooManyPending => TooManyPending,
			k_EResultNoSiteLicensesFound => NoSiteLicensesFound,
			k_EResultWGNetworkSendExceeded => WGNetworkSendExceeded,
			k_EResultAccountNotFriends => AccountNotFriends,
			k_EResultLimitedUserAccount => LimitedUserAccount,
			k_EResultCantRemoveItem => CantRemoveItem,
			k_EResultAccountDeleted => AccountDeleted,
			k_EResultExistingUserCancelledLicense => ExistingUserCancelledLicense,
			k_EResultCommunityCooldown => CommunityCooldown,
			k_EResultNoLauncherSpecified => NoLauncherSpecified,
			k_EResultMustAgreeToSSA => MustAgreeToSSA,
			k_EResultLauncherMigrated => LauncherMigrated,
			k_EResultSteamRealmMismatch => SteamRealmMismatch,
			k_EResultInvalidSignature => InvalidSignature,
			k_EResultParseFailure => ParseFailure,
			k_EResultNoVerifiedPhone => NoVerifiedPhone,
			k_EResultInsufficientBattery => InsufficientBattery,
			k_EResultChargerRequired => ChargerRequired,
			k_EResultCachedCredentialInvalid => CachedCredentialInvalid,
			K_EResultPhoneNumberIsVOIP => PhoneNumberIsVOIP,
			k_EResultNotSupported => NotSupported,
			k_EResultFamilySizeLimitExceeded => FamilySizeLimitExceeded,
			k_EResultOfflineAppCacheInvalid => OfflineAppCacheInvalid,
			c_enum => Unknown(c_enum as i32),
		})
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum InitErrorEnum {
	FailedGeneric,
	NoSteamClient,
	VersionMismatch,
	Unknown(i32),
}

impl InitErrorEnum {
	pub(crate) fn new(init_result: sys::ESteamAPIInitResult) -> Option<Self> {
		use sys::ESteamAPIInitResult::*;

		match init_result {
			k_ESteamAPIInitResult_OK => None,
			k_ESteamAPIInitResult_FailedGeneric => Some(Self::FailedGeneric),
			k_ESteamAPIInitResult_NoSteamClient => Some(Self::NoSteamClient),
			k_ESteamAPIInitResult_VersionMismatch => Some(Self::VersionMismatch),
			unknown => Some(Self::Unknown(unknown as i32)),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, thiserror::Error)]
pub enum IntoCIndexError {
	#[error("index cannot be negative")]
	Negative,

	#[error("{0:?}")]
	TryFromIntError(#[from] TryFromIntError),
}

/// Unfortunately, a common pattern with the Steam API is returning a
/// bool to indicate the success or fail state of a functional call.  
/// When a function in the Steam API has a possible fail state
/// but no error code or message is given,
/// this struct is used.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SilentFailure;

impl Display for SilentFailure {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.write_str("failed, no error message from the Steam API is available")
	}
}

impl StdError for SilentFailure {}
