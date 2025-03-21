//! See [`FriendsInterface`].

pub mod rich_presence;

use crate::call::{CallFuture, Callback, CallbackRaw, Dispatch};
use crate::dt::{AppId, GameId, ImageHandle, LobbyId, SteamId};
use crate::error::{CallError, GeneralError, UnspecifiedError};
use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild, SteamInterface};
use crate::iter::{SteamApiIterator, SteamApiStream, Unreliable};
use crate::util::{empty_cstr_ptr, some_string, success, FiniteStr, RequestQueue};
use crate::{sys, Private};
use bitflags::bitflags;
use lru::LruCache;
use rich_presence::RichPresenceInterface;
use std::collections::{HashSet, VecDeque};
use std::default::Default;
use std::ffi::{c_int, c_short, c_uint, CString};
use std::future::Future;
use std::mem::MaybeUninit;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::num::{NonZero, NonZeroUsize};
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};
use std::task::{Context, Poll};

impl AsRef<FriendsInterface> for super::ClientInterfaces {
	fn as_ref(&self) -> &FriendsInterface {
		&self.friends
	}
}

/// > Interface to access information about individual users and interact with the [Steam Overlay].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends)
///
/// [Steam Overlay]: https://partner.steamgames.com/doc/features/overlay
#[derive(Debug)]
pub struct FriendsInterface {
	fip: FixedInterfacePtr<sys::ISteamFriends>,
	clan_lru: Mutex<LruCache<SteamId, ClanRecord>>,
	in_game_speaking: Mutex<HashSet<SteamId>>,
	large_avatars: Mutex<RequestQueue<SteamId, (), ImageHandle>>,
	members_lock: Mutex<()>,
	user_info_requests: Mutex<RequestQueue<SteamId, bool, ()>>,
	steam: SteamChild,
}

impl FriendsInterface {
	/// The maximum amount of retained clan records.
	const CLAN_LRU_CAP: usize = 64;

	/// Opens the Steam overlay to a specific window.
	///
	/// # Panics
	/// If the `url` field of [`ActivateGameOverlay::WebPage`] contains a null character.
	pub fn activate_game_overlay(&self, instruction: ActivateGameOverlay) {
		use ActivateGameOverlay::*;

		let fip = *self.fip;

		unsafe {
			match instruction {
				//ActivateGameOverlay
				Friends => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Friends".as_ptr()),
				Community => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Community".as_ptr()),
				Players => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Players".as_ptr()),
				Settings => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Settings".as_ptr()),
				OfficialGameGroup => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"OfficialGameGroup".as_ptr()),
				Stats(None) => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Stats".as_ptr()),
				Achievements(None) => sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c"Achievements".as_ptr()),

				//still ActivateGameOverlay, just a little special
				ChatRoomGroup(steam_id) => {
					let string = format!("chatroomgroup/{}", steam_id.0);
					let c_string = CString::new(string).unwrap();

					sys::SteamAPI_ISteamFriends_ActivateGameOverlay(fip, c_string.as_ptr())
				}

				//ActivateGameOverlayToUser
				Profile(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"steamid".as_ptr(), steam_id.0),
				Chat(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"chat".as_ptr(), steam_id.0),
				JoinTrade(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"jointrade".as_ptr(), steam_id.0),
				Stats(Some(steam_id)) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"stats".as_ptr(), steam_id.0),
				Achievements(Some(steam_id)) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"achievements".as_ptr(), steam_id.0),
				FriendAdd(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"friendadd".as_ptr(), steam_id.0),
				FriendRemove(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"friendremove".as_ptr(), steam_id.0),
				FriendRequestAccept(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"friendrequestaccept".as_ptr(), steam_id.0),
				FriendRequestIgnore(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayToUser(fip, c"friendrequestignore".as_ptr(), steam_id.0),

				//ActivateGameOverlayInviteDialog
				InviteDialog(steam_id) => sys::SteamAPI_ISteamFriends_ActivateGameOverlayInviteDialog(fip, steam_id.0),

				//ActivateGameOverlayToStore
				Store { app_id, show_in_cart } => {
					sys::SteamAPI_ISteamFriends_ActivateGameOverlayToStore(
						fip,
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
						fip,
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

	/// Returns a [`Category`] which can yield some information
	/// such as its [`name`] and [`members`].
	///
	/// Use [`category_iter`] to get a list of all the current user's created [`CategoryId`]s.
	///
	/// [`category_iter`]: Self::category_iter
	/// [`name`]: Category::name
	/// [`members`]: Category::members
	pub fn category(&self, id: CategoryId) -> Category {
		Category { id: id.get(), ifc: self }
	}

	/// Returns an iterator yielding the [`CategoryId`]s the user has created in their friends list.
	pub fn category_iter(&self) -> Unreliable<CategoryIter> {
		CategoryIter { cursor: 0, ifc: self }.wrap()
	}

	/// Clears all cached [`Clans`] data.
	/// Calling this will not deallocate the cache or reduce memory usage.
	pub fn clear_clan_cache(&self) {
		self.clan_lru.lock().unwrap().clear();
	}

	/// > Gets the number of Steam groups that the current user is a member of.
	///
	/// The function to iterate with is deprecated according to the docs.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanCount)
	pub fn clan_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamFriends_GetClanCount(*self.fip) as u32 }
	}

	/// Accessor for a clan's chat window.
	/// Can be used to check if the chat is open, open the chat, and close the chat.
	pub fn clan_chat_window(&self, steam_id: impl Into<SteamId>) -> ClanChatWindow {
		ClanChatWindow { ifc: self, steam_id: steam_id.into() }
	}

	/// Returns a lock on the [`Clans`] cache where Steam Group information can be queried or loaded.
	pub fn clans(&self) -> Clans {
		Clans {
			ifc: &self,
			guard_cache: self.clan_lru.lock().unwrap(),
		}
	}

	/// > Gets the number of players that the current user has recently played with, across all games.
	/// This is used for iteration, after calling this then GetCoplayFriend can be used to get the Steam ID of each player.
	/// These players have been set with previous calls to [`set_played_with`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetCoplayFriendCount)
	///
	/// [`set_played_with`]: Self::set_played_with
	pub fn coplay_friend_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamFriends_GetCoplayFriendCount(*self.fip) as u32 }
	}

	/// Returns an iterator which yields the [`SteamId`]s of players that were recently played with.
	/// Using [`set_played_with`] during iteration will cause skipped or duplicated.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetCoplayFriendCount)
	///
	/// [`set_played_with`]: Self::set_played_with
	pub fn coplay_friend_iter(&self) -> Unreliable<CoplayFriendIter> {
		CoplayFriendIter { cursor: 0, ifc: &self }.wrap()
	}

	/// Downloads and caches [`ClanActivityCounts`] for the provided clans' [`SteamId`]s.
	/// Use [`Clans::load`] alongside [`Clan::activity_counts`] if you just want one.
	/// If you want to use an existing cached count, use [`Clans::get`] instead lieu of [`Clans::load`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#DownloadClanActivityCounts)
	pub async fn download_clan_activity_counts(&self, clan_ids: impl Into<Vec<SteamId>>) -> Result<Vec<Option<ClanActivityCounts>>, CallError<UnspecifiedError>> {
		let clan_ids = clan_ids.into();
		let steam = self.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(DownloadClanActivityCounts {
			steam: steam.child(),
			clan_ids: clan_ids.clone(),
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		let counts_vec = future.await?;
		let iter = counts_vec.iter().zip(clan_ids.iter());
		let mut guard_clan = self.clan_lru.lock().unwrap();

		if clan_ids.len() > Self::CLAN_LRU_CAP {
			//only update existing
			//we can't push in new entries as we could immediately push them out
			for (activity_counts, steam_id) in iter {
				if let Some(existing) = guard_clan.get_mut(steam_id) {
					existing.activity_counts = *activity_counts;
				}
			}
		} else {
			//update and insert
			for (activity_counts, steam_id) in iter {
				//update
				if let Some(existing) = guard_clan.get_mut(steam_id) {
					existing.activity_counts = *activity_counts;

					continue;
				}

				//insert
				guard_clan.push(
					*steam_id,
					ClanRecord {
						activity_counts: *activity_counts,
						officer_count: None,
					},
				);
			}
		}

		//explicit drop for significant drop
		drop(guard_clan);

		Ok(counts_vec)
	}

	/// > After calling [`request_equipped_profile_items`],
	/// you can use this function to check if the user has a type of profile item equipped or not.
	///
	/// Will always return `false` if [`request_equipped_profile_items`] has not been called.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#BHasEquippedProfileItem)
	///
	/// [`request_equipped_profile_items`]: Self::request_equipped_profile_items
	pub fn equipped_profile_item(&self, steam_id: impl Into<SteamId>, item_type: ProfileItemType) -> bool {
		unsafe { sys::SteamAPI_ISteamFriends_BHasEquippedProfileItem(*self.fip, steam_id.into().0 as _, item_type.into()) }
	}

	/// Returns an asynchronous stream for querying the current user's followed users.
	///
	/// > Gets the list of users that the current user is following.
	/// You can be following people that are not your friends.
	/// Following allows you to receive updates when the person does things like post a new piece of content to the Steam Workshop.
	///
	/// Dispatches will be created as the stream is polled.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EnumerateFollowingList)
	pub fn following_list(&self) -> Unreliable<FollowingListStream> {
		FollowingListStream {
			allow_dispatch: true,
			call_future: None,
			dispatch_cursor: 0,
			queue: VecDeque::new(),
			steam: self.steam.clone(),
			terminated: false,
		}
		.wrap()
	}

	/// Returns an iterator the gets the current user's friends.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendByIndex)
	pub fn friend_iter(&self, flags: FriendFlags) -> Unreliable<FriendIter> {
		FriendIter { cursor: 0, flags, ifc: &self }.wrap()
	}

	/// > Gets the Steam level of the specified user.
	///
	/// May return 0 upon failure, or if the profile is private.
	/// Some Steam users, typically limited users, are level 0 naturally.
	///
	/// > If the Steam level is not immediately available for the specified user then this returns 0 and queues it to be downloaded from the Steam servers.
	/// When it gets downloaded a [`PersonaStateChange`] callback will be posted with [`PersonaChange`] including [`PersonaChange::STEAM_LEVEL`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendSteamLevel)
	pub fn friend_steam_level(&self, steam_id: impl Into<SteamId>) -> u32 {
		unsafe { sys::SteamAPI_ISteamFriends_GetFriendSteamLevel(*self.fip, steam_id.into().0) as u32 }
	}

	/// > Gets the app ID of the game that user played with someone on their recently-played-with list.
	///
	/// Returns `None` if the `AppId` is invalid.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendCoplayGame)
	pub fn get_coplay_friend_game(&self, steam_id: impl Into<SteamId>) -> Option<AppId> {
		AppId::valid_from(unsafe { sys::SteamAPI_ISteamFriends_GetFriendCoplayGame(*self.fip, steam_id.into().0) })
	}

	/// > Checks if the specified friend is in a game,
	/// and gets info about the game if they are.
	///
	/// `None` if the friend is not playing a game,
	/// or their game info is not accessible.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendGamePlayed)
	pub fn get_friend_game_played(&self, steam_id: impl Into<SteamId>) -> Option<FriendGameInfo> {
		let mut c_data = MaybeUninit::<sys::FriendGameInfo_t>::uninit();

		if unsafe { sys::SteamAPI_ISteamFriends_GetFriendGamePlayed(*self.fip, steam_id.into().0, c_data.as_mut_ptr()) } {
			let c_data = unsafe { c_data.assume_init() };

			//TODO!
			Some(FriendGameInfo {
				game_id: c_data.m_gameID.into(),
				game_ip: SocketAddrV4::new(Ipv4Addr::from(c_data.m_unGameIP as u32), c_data.m_usGamePort as u16),
				query_port: 0,
				lobby_id: SteamId::from(c_data.m_steamIDLobby),
			})
		} else {
			None
		}
	}

	/// Use [`request_large_avatar`] to load the avatar.
	///
	/// [`request_large_avatar`]: Self::request_large_avatar
	pub fn get_large_avatar(&self, steam_id: impl Into<SteamId>) -> Result<ImageHandle, AvatarError> {
		match unsafe { sys::SteamAPI_ISteamFriends_GetLargeFriendAvatar(*self.fip, steam_id.into().0) } {
			-1 => Err(AvatarError::Downloading), //avatar is loading
			0 => Err(AvatarError::NoAvatar),     //no avatar is set
			handle => Ok(ImageHandle::new(handle, None)),
		}
	}

	/// > Gets the specified user's persona (display) name.
	/// This will only be known to the current user if the other user is in their friends list,
	/// on the same game server,
	/// in a chat room or lobby,
	/// or in a small Steam group with the local user.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendPersonaName)
	pub fn get_persona_name(&self, steam_id: impl Into<SteamId>) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamFriends_GetFriendPersonaName(*self.fip, steam_id.into().0)) }
	}

	/// > Checks if the user meets the specified criteria.
	/// (Friends, blocked, users on the same server, etc)
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#HasFriend)
	pub fn has_friend(&self, steam_id: impl Into<SteamId>, flags: FriendFlags) -> bool {
		unsafe { sys::SteamAPI_ISteamFriends_HasFriend(*self.fip, steam_id.into().0, flags.bits()) }
	}

	/// Accessor for marking users as speaking-in-game.
	/// If the marked user is currently in a Steam voice chat, they will be muted for the local user.
	///
	/// Use this to prevent the local user from hearing another in both your game's voice chat and Steam's at the same time.
	///
	/// Only one [`InGameSpeaking`] accessor can exist at a time.
	/// Other threads calling this function while an [`InGameSpeaking`] exists will be blocked until the accessor drops.
	pub fn in_game_speaking(&self) -> InGameSpeaking {
		InGameSpeaking {
			ifc: self,
			speakers: self.in_game_speaking.lock().unwrap(),
		}
	}

	/// > Invites a friend or clan member to the current game using a special invite string.
	/// If the target user accepts the invite then the `connect_string` gets added to the command-line when launching the game.
	/// If the game is already running for that user,
	/// then they will receive a [`GameRichPresenceJoinRequested`] callback with the connect string.
	///
	/// # Panics
	/// If `connect_string` is `Some` and contains a nul character.
	///
	/// [`GameRichPresenceJoinRequested`]: rich_presence::GameRichPresenceJoinRequested
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#InviteUserToGame)
	pub fn invite_user_to_game(&self, steam_id: impl Into<SteamId>, connect_string: Option<impl FiniteStr<{ rich_presence::VALUE_MAX }>>) {
		if let Some(connect_string) = connect_string {
			let connect_string = connect_string.to_finite_cstring().unwrap();

			unsafe { sys::SteamAPI_ISteamFriends_InviteUserToGame(*self.fip, steam_id.into().0, connect_string.as_ptr()) };
		} else {
			//TODO: check if null() is okay instead of empty_cstr_ptr()
			unsafe { sys::SteamAPI_ISteamFriends_InviteUserToGame(*self.fip, steam_id.into().0, empty_cstr_ptr()) };
		}
	}

	/// > Get the members of users in a source (Steam group, chat room, or game server).
	/// Large Steam groups cannot be iterated by the local user.
	///
	/// # Blocking
	/// Only one [`MembersIter`] can exist at a time.
	/// Calling this function again while a [`MembersIter`] already exists
	/// will cause the current thread to be blocked until the iterator can be made.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendCountFromSource)
	pub fn members_iter(&self, steam_id: impl Into<SteamId>) -> MembersIter {
		MembersIter {
			cursor: 0,
			ifc: self,
			steam_id: steam_id.into(),
			_lock: self.members_lock.lock().unwrap(),
		}
	}

	/// > Gets one of the previous display names for the specified user.
	/// This only works for display names that the current user has seen on the local computer.
	///
	/// The first item (if any) of ther iterator will always be the current name.
	/// If the current name is needed, use [`get_persona_name`] instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendPersonaNameHistory)
	///
	/// [`get_persona_name`]: Self::get_persona_name
	pub fn persona_name_history_iter(&self, steam_id: impl Into<SteamId>) -> PersonaNameHistoryIter {
		PersonaNameHistoryIter {
			ifc: self,
			cursor: 0,
			steam_id: steam_id.into(),
		}
	}

	/// > Properties on a Steam Community profile item.
	/// See [`ProfileItemProperties`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn profile_item_properties(&self, steam_id: impl Into<SteamId>, item_type: ProfileItemType) -> ProfileItemProperties {
		ProfileItemProperties {
			ifc: self,
			item_type: item_type.into(),
			steam_id: steam_id.into(),
		}
	}

	/// > Gets a relationship to a specified user.
	///
	/// Gets the current user's relationship with the specified user by [`SteamId`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendRelationship)
	pub fn relationship(&self, steam_id: impl Into<SteamId>) -> Relationship {
		unsafe { sys::SteamAPI_ISteamFriends_GetFriendRelationship(*self.fip, steam_id.into().0) }.into()
	}

	/// > Requests the list of equipped Steam Community profile items for the given user from Steam.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestEquippedProfileItems)
	pub async fn request_equipped_profile_items(&self, steam_id: impl Into<SteamId>) -> Result<EquippedProfileItems, CallError<GeneralError>> {
		struct RequestEquippedProfileItems {
			steam: SteamChild,
			steam_id: SteamId,
		}

		unsafe impl Dispatch for RequestEquippedProfileItems {
			type CType = sys::EquippedProfileItems_t;
			type Output = EquippedProfileItems;
			type Error = GeneralError;

			unsafe fn dispatch(&mut self, _: Private) -> sys::SteamAPICall_t {
				sys::SteamAPI_ISteamFriends_RequestEquippedProfileItems(*self.steam.get().client_interfaces().friends.fip, self.steam_id.0 as _)
			}

			fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
				if let Some(error) = GeneralError::new(c_data.m_eResult) {
					return Err(error);
				}

				Ok(EquippedProfileItems {
					animated_avatar: c_data.m_bHasAnimatedAvatar,
					avatar_frame: c_data.m_bHasAvatarFrame,
					profile_modifier: c_data.m_bHasProfileModifier,
					profile_background: c_data.m_bHasProfileBackground,
					mini_profile_background: c_data.m_bHasMiniProfileBackground,
				})
			}
		}

		let steam = self.steam.get();
		let mut call_manager = steam.call_manager_lock();

		let future = call_manager.dispatch(RequestEquippedProfileItems {
			steam: steam.child(),
			steam_id: steam_id.into(),
		});

		//explicit drop for significant drop
		drop(call_manager);

		future.await
	}

	/// > Gets a handle to the large (128 x 128px) avatar for the specified user.
	/// This only works for users that the local user knows about.
	/// They will automatically know about their friends,
	/// people on leaderboards they've requested,
	/// or people in the same source as them (Steam group, chat room, lobby, or game server).
	/// If they don't know about them then you must call [`request_user_info`] to cache the avatar locally.
	///
	/// Medium and small avatars can be requested with [`request_user_info`]
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetLargeFriendAvatar)
	///
	/// [`request_user_info`]: Self::request_user_info
	pub async fn request_large_avatar(&self, steam_id: impl Into<SteamId>) -> Option<ImageHandle> {
		let mut guard = self.large_avatars.lock().unwrap();
		let steam_id = steam_id.into();

		//I hate sentinel values
		let future = match unsafe { sys::SteamAPI_ISteamFriends_GetLargeFriendAvatar(*self.fip, steam_id.0) } {
			-1 => guard.insert(steam_id, ()), //avatar is loading
			0 => return None,                 //no avatar is set
			handle => return Some(ImageHandle::new(handle, None)),
		};

		//explicit drop for significant drop
		drop(guard);

		future.await.ok()
	}

	/// > Requests the persona name and optionally the avatar of a specified user.
	/// It's a lot slower to download avatars and churns the local cache,
	/// so if you don't need avatars, don't request them.
	///
	/// Returns `None` if Steam says we already have the user info and a request isn't needed.
	///
	/// If `request_avatar` is `true` the small and medium size avatars will be loaded.
	///
	/// The return value is not a [`Future`] of an [`Option`] e.g.
	/// `impl Future<Output = Option<()>>`
	/// but a [`Option`] of a [`Future`] being:
	/// `Option<impl Future<Output = ()>>`
	///
	/// This means the call only needs to be `await`ed if the request was made.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation)
	pub fn request_user_info(&self, steam_id: impl Into<SteamId>, request_avatar: bool) -> Option<impl Future<Output = ()>> {
		let steam_id = steam_id.into();
		let mut guard = self.user_info_requests.lock().unwrap();

		//only make the request when we have the guard
		//to prevent race conditions
		if !unsafe { sys::SteamAPI_ISteamFriends_RequestUserInformation(*self.fip, steam_id.0, !request_avatar) } {
			//if steam says we're good
			//we don't need to do async!
			return None;
		}

		let rx = guard.insert(steam_id, request_avatar);

		drop(guard); //drop before we await!

		Some(async {
			let _ = rx.await;
		})
	}

	/// Returns an interface for modifying the current user's rich presence.
	/// Although multiple [`RichPresenceInterface`] can exist at once,
	/// inconsistent ordering.
	pub fn rich_presence(&self) -> RichPresenceInterface {
		RichPresenceInterface { ifc: self }
	}

	/// > Mark a target user as 'played with'.
	///
	/// A list of players with this mark can be iterated using [`coplay_friend_iter`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetPlayedWith)
	///
	/// [`coplay_friend_iter`]: Self::coplay_friend_iter
	pub fn set_played_with(&self, steam_id: impl Into<SteamId>) {
		unsafe { sys::SteamAPI_ISteamFriends_SetPlayedWith(*self.fip, steam_id.into().0) };
	}
}

impl Interface for FriendsInterface {
	type CInterface = sys::ISteamFriends;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, steam: SteamChild) -> Self {
		Self {
			fip,
			clan_lru: Mutex::new(LruCache::new(NonZeroUsize::new(Self::CLAN_LRU_CAP).unwrap())),
			in_game_speaking: Mutex::default(),
			large_avatars: Mutex::default(),
			members_lock: Mutex::default(),
			user_info_requests: Mutex::default(),
			steam,
		}
	}

	fn initialize(steam: &SteamInterface) {
		let mut call_manager = steam.call_manager_lock();

		call_manager.register_pub::<AvatarImageLoaded>(); //needed for request_large_avatar
		call_manager.register_pub::<PersonaStateChange>(); //needed for resuest_user_info
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		sys::SteamAPI_SteamFriends_v018()
	}
}

/// For use with [`FriendsInterface::activate_game_overlay`],
/// represents the available parameters for all possible options of the following:
/// - [ActivateGameOverlay](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlay)
/// - [ActivateGameOverlayToUser](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToUser)
/// - [ActivateGameOverlayInviteDialog](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayInviteDialog)
/// - [ActivateGameOverlayToStore](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToStore)
/// - [ActivateGameOverlayToWebPage](https://partner.steamgames.com/doc/api/ISteamFriends#ActivateGameOverlayToWebPage)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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
		/// If this contains any null characters, [`activate_game_overlay`] will panic.
		///
		/// [`activate_game_overlay`]: FriendsInterface::activate_game_overlay
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

/// Error returned by [`FriendsInterface::get_large_avatar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, thiserror::Error)]
pub enum AvatarError {
	/// The user does not have an avatar.
	#[error("user does not have an avatar")]
	NoAvatar,

	/// The avatar is not loaded, and is currently being requested.
	#[error("avatar is not loaded, and is currently being requested")]
	Downloading,
}

/// Callback.
///
/// > Called when a large avatar is loaded if you have tried requesting it when it was unavailable.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#AvatarImageLoaded_t)
#[derive(Debug)]
pub struct AvatarImageLoaded {
	steam: SteamChild,
}

unsafe impl CallbackRaw for AvatarImageLoaded {
	const CALLBACK_ID: i32 = sys::AvatarImageLoaded_t_k_iCallback as i32;
	type CType = sys::AvatarImageLoaded_t;
	type Output = (SteamId, ImageHandle, u32, u32);

	unsafe fn on_callback(&mut self, c_data: &Self::CType, _: Private) -> Self::Output {
		let width = c_data.m_iWide as u32;
		let height = c_data.m_iTall as u32;
		let image_handle = ImageHandle::new(c_data.m_iImage, Some([width, height]));
		let steam_id = SteamId::from(c_data.m_steamID);
		let tuple = (steam_id, image_handle, width, height);

		let steam = self.steam.get();
		let mut guard = steam.client_interfaces().friends.large_avatars.lock().unwrap();

		guard.fulfil_all(&steam_id, image_handle);
		drop(guard); //explicit drop for significant drop

		tuple
	}

	fn register(steam: &SteamInterface, _: Private) -> Self {
		Self { steam: steam.child() }
	}
}

impl Callback for AvatarImageLoaded {
	const KEEP_REGISTERED: bool = true;

	type Fn = dyn FnMut(SteamId, ImageHandle, u32, u32) + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: Private) {
		listener_fn(params.0, params.1, params.2, params.3)
	}
}

/// A category in the user's friends list.
/// The user can put a single friend in multiple of these.
///
/// See [`FriendsInterface::category`].
pub struct Category<'a> {
	id: c_short,
	ifc: &'a FriendsInterface,
}

impl<'a> Category<'a> {
	/// Returns a [`Vec`] containing the [`SteamId`]s of the friends in the provided category.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetFriendsGroupMembersList)
	pub fn members(&self) -> Vec<SteamId> {
		let mut members = Vec::<SteamId>::new();
		let count = unsafe { sys::SteamAPI_ISteamFriends_GetFriendsGroupMembersCount(*self.ifc.fip, self.id) };

		if count <= 0 {
			return members;
		}

		let count_usize = count as usize;

		members.reserve_exact(count_usize);

		//sanity
		debug_assert!(members.capacity() >= count_usize);

		unsafe {
			//SAFETY: `*mut SteamId as *mut sys::CSteamID`
			//SteamId is the same size as CSteamID
			//alignment is 8 (SteamId) instead of 8 (CSteamID),
			//but that's okay because we make the allocation
			sys::SteamAPI_ISteamFriends_GetFriendsGroupMembersList(*self.ifc.fip, self.id, members.as_mut_ptr() as *mut sys::CSteamID, count);

			//adjust the vec to cover what we wrote
			members.set_len(count_usize);
		}

		members
	}

	/// Returns the name if the category still exists iin the user's friends list.
	pub fn name(&self) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamFriends_GetFriendsGroupName(*self.ifc.fip, self.id)) }
	}
}

/// ID for a category in the user's friends list.
/// The user can put a single friend in multiple of these.
///
/// Yielded by [`CategoryIter`],
/// which can be obtained from [`FriendsInterface::category_iter`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CategoryId(NonZero<c_short>);

impl CategoryId {
	fn new(id: c_short) -> Option<Self> {
		NonZero::<c_short>::new(id).map(|id| Self(id))
	}

	fn get(self) -> c_short {
		self.0.get()
	}
}

/// See [`FriendsInterface::category_iter`].
#[derive(Debug)]
pub struct CategoryIter<'a> {
	cursor: c_int,
	ifc: &'a FriendsInterface,
}

unsafe impl<'a> SteamApiIterator for CategoryIter<'a> {
	type Item = CategoryId;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		CategoryId::new(sys::SteamAPI_ISteamFriends_GetFriendsGroupIDByIndex(*self.ifc.fip, index))
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetFriendsGroupCount(*self.ifc.fip);
	}
}

/// Accessor for a Steam group/clan.
///
/// Returned by [`Clans::load`] or [`Clans::get`].
#[derive(Debug)]
pub struct Clan<'a> {
	ifc: &'a FriendsInterface,
	record: &'a mut ClanRecord,
	steam_id: SteamId,
}

impl<'a> Clan<'a> {
	/// > Gets the most recent information we have about what the users in a Steam Group are doing.
	///
	/// Returns `None` if [`request_activity_counts`] failed, or the clan is inaccessible.
	/// Call [`request_activity_counts`] to update the cached counts.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanActivityCounts)
	///
	/// [`request_activity_counts`]: Self::request_activity_counts
	pub fn activity_counts(&mut self) -> Option<ClanActivityCounts> {
		self.record.activity_counts
	}

	/// > Gets the display name for the specified Steam group; if the local client knows about it.
	///
	/// May return `None` if [`request_activity_counts`] has not been called for this group.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanName)
	///
	/// [`request_activity_counts`]: Self::request_activity_counts
	pub fn name(&self) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamFriends_GetClanName(*self.ifc.fip, self.steam_id.0)) }
	}

	/// > Checks if the Steam group is an official game group/community hub.
	///
	/// May return `false` upon failure.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#IsClanOfficialGameGroup)
	pub fn official_game_group(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamFriends_IsClanOfficialGameGroup(*self.ifc.fip, self.steam_id.0) }
	}

	/// > Gets the number of officers (administrators and moderators) in a specified Steam group.
	/// This also includes the owner of the Steam group.
	///
	/// Returns `None` if [`request_officers`] has failed, or the clan is inaccessible.
	/// Call [`request_officers`] to update the cached count.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanOfficerCount)
	///
	/// [`request_officers`]: Self::request_officers
	pub fn officer_count(&mut self) -> Option<u32> {
		self.record.officer_count
	}

	/// > Gets the owner of a Steam Group.
	///
	/// May return `None` if [`request_officers`] has failed, or the clan is inaccessible.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanOwner)
	///
	/// [`request_officers`]: Self::request_officers
	pub fn owner(&self) -> Option<SteamId> {
		unsafe { SteamId::valid_from(sys::SteamAPI_ISteamFriends_GetClanOwner(*self.ifc.fip, self.steam_id.0)) }
	}

	/// > Checks if the Steam group is public.
	///
	/// May return `false` upon failure.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#IsClanPublic)
	pub fn public(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamFriends_IsClanPublic(*self.ifc.fip, self.steam_id.0) }
	}

	/// > Refresh the Steam Group activity data or get the data from groups other than one that the current user is a member.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#DownloadClanActivityCounts)
	pub async fn request_activity_counts(&mut self) -> Result<(), CallError<UnspecifiedError>> {
		let steam = self.ifc.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(DownloadClanActivityCounts {
			steam: steam.child(),
			clan_ids: vec![self.steam_id],
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		let counts = future.await?[0];
		self.record.activity_counts = counts;

		Ok(())
	}

	/// > Requests information about a Steam group officers (administrators and moderators).
	/// You can only ask about Steam groups that a user is a member of.
	/// This won't download avatars for the officers automatically.
	/// If no avatar image is available for an officer,
	/// then call [`FriendsInterface::request_user_info`] to download the avatar.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestClanOfficerList)
	pub async fn request_officers(&mut self) -> Result<(), CallError<UnspecifiedError>> {
		#[derive(Debug)]
		struct RequestClanOfficerList {
			clan_id: SteamId,
			steam: SteamChild,
		}

		unsafe impl Dispatch for RequestClanOfficerList {
			type CType = sys::ClanOfficerListResponse_t;
			type Output = u32;
			type Error = UnspecifiedError;

			unsafe fn dispatch(&mut self, _: Private) -> sys::SteamAPICall_t {
				let steam = self.steam.get();

				sys::SteamAPI_ISteamFriends_RequestClanOfficerList(*steam.client_interfaces().friends.fip, self.clan_id.0)
			}

			fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
				//for some reason, m_bSuccess success is a u8 instead of a bool
				if c_data.m_bSuccess == 0 {
					return Err(UnspecifiedError);
				}

				Ok(c_data.m_cOfficers as u32)
			}
		}

		let steam = self.ifc.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(RequestClanOfficerList {
			clan_id: self.steam_id,
			steam: steam.child(),
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		let count = future.await?;
		self.record.officer_count = Some(count);

		Ok(())
	}

	/// > Gets the unique tag (abbreviation) for the specified Steam group;
	/// If the local client knows about it.
	/// The Steam group abbreviation is a unique way for people to identify the group and is limited to 12 characters.
	/// In some games this will appear next to the name of group members.
	///
	/// May return `None` if [`request_activity_counts`] has not been called for this group.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanTag)
	///
	/// [`request_activity_counts`]: Self::request_activity_counts
	pub fn tag(&self, steam_id: impl Into<SteamId>) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamFriends_GetClanTag(*self.ifc.fip, steam_id.into().0)) }
	}
}

/// Returned by [`FriendsInterface::clans`].  
/// Maintains a cache of Steam groups.
#[derive(Debug)]
pub struct Clans<'a> {
	ifc: &'a FriendsInterface,
	guard_cache: MutexGuard<'a, LruCache<SteamId, ClanRecord>>,
}

impl<'a> Clans<'a> {
	/// Gets a previously cached Steam group.
	/// This will return `None` if the [`Clan`] has not been loaded,
	/// or has been shoved out of the cache.
	pub fn get(&mut self, steam_id: impl Into<SteamId>) -> Option<Clan> {
		let steam_id = steam_id.into();

		self.guard_cache.get_mut(&steam_id).map(|record| Clan { ifc: self.ifc, record, steam_id })
	}

	/// Gets the [`Clan`], or calls [`load`].
	///
	/// Setting `officers` to `true` will also call if the [`Clan`] does not have officers cached.
	///
	/// [`load`]: Self::load
	pub async fn get_or_load(&mut self, steam_id: impl Into<SteamId>, officers: bool) -> Result<Clan, CallError<UnspecifiedError>> {
		let steam_id = steam_id.into();

		if self.guard_cache.peek(&steam_id).is_some() {
			let record = self.guard_cache.get_mut(&steam_id).unwrap();
			let missing_officers = record.officer_count.is_none();

			let mut clan = Clan { ifc: self.ifc, record, steam_id };

			if officers && missing_officers {
				clan.request_officers().await?;
			}

			Ok(clan)
		} else {
			self.load(steam_id, officers).await
		}
	}

	/// Automatically calls [`Clan::request_activity_counts`].
	///
	/// Setting `officers` to `true` will call [`Clan::request_officers`].
	pub async fn load(&mut self, steam_id: impl Into<SteamId>, officers: bool) -> Result<Clan, CallError<UnspecifiedError>> {
		let steam_id = steam_id.into();
		let mut clan = Clan {
			ifc: self.ifc,
			record: self.guard_cache.get_or_insert_mut(steam_id, ClanRecord::new),
			steam_id,
		};

		if officers {
			//make both requests, error if either fail
			//cannot join futures because they require mutable access
			let act_result = clan.request_activity_counts().await;
			let ofc_result = clan.request_officers().await;

			act_result?;
			ofc_result?;
		} else {
			clan.request_activity_counts().await?;
		}

		Ok(clan)
	}

	/// Loads without calling [`request_activity_counts`].
	/// This will return an existing [`Clan`] if it already exists.
	///
	/// [`request_activity_counts`]: Clan::request_activity_counts
	pub fn load_empty(&mut self, steam_id: impl Into<SteamId>) -> Clan {
		let steam_id = steam_id.into();

		Clan {
			ifc: self.ifc,
			record: self.guard_cache.get_or_insert_mut(steam_id, ClanRecord::new),
			steam_id,
		}
	}
}

/// The different counts for Steam groups the Steam API provides.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClanActivityCounts {
	/// Online users, exclusing invisible users.
	pub online: u32,

	/// Online users in a game, exclusing invisible users.
	pub in_game: u32,

	/// Online users with the group's chat open, exclusing invisible users.
	pub chatting: u32,
}

/// A chat window in Steam, retrieved with [`FriendsInterface::clan_chat_window`].
#[derive(Debug, Clone, Copy)]
pub struct ClanChatWindow<'a> {
	ifc: &'a FriendsInterface,
	steam_id: SteamId,
}

impl<'a> ClanChatWindow<'a> {
	/// > Closes the specified Steam group chat room in the Steam UI.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#CloseClanChatWindowInSteam)
	pub fn close(&mut self) -> Result<(), UnspecifiedError> {
		success(unsafe { sys::SteamAPI_ISteamFriends_CloseClanChatWindowInSteam(*self.ifc.fip, self.steam_id.into()) })
	}

	/// > Checks if the Steam Group chat room is open in the Steam UI.
	///
	/// May also return `false` in case of an error.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#IsClanChatWindowOpenInSteam)
	pub fn is_open(&self) -> bool {
		unsafe { sys::SteamAPI_ISteamFriends_IsClanChatWindowOpenInSteam(*self.ifc.fip, self.steam_id.into()) }
	}

	/// > Opens the specified Steam group chat room in the Steam UI.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#OpenClanChatWindowInSteam)
	pub fn open(&mut self) -> Result<(), UnspecifiedError> {
		success(unsafe { sys::SteamAPI_ISteamFriends_OpenClanChatWindowInSteam(*self.ifc.fip, self.steam_id.into()) })
	}
}

impl<'a> Eq for ClanChatWindow<'a> {}

impl<'a> PartialEq for ClanChatWindow<'a> {
	fn eq(&self, other: &Self) -> bool {
		<SteamId as PartialEq>::eq(&self.steam_id, &other.steam_id)
	}
}

/// Internal record for clan information.
/// Use [`FriendsInterface::load_clan`] and [`FriendsInterface::clans`] to access this data.
#[derive(Debug)]
#[doc(hidden)]
struct ClanRecord {
	activity_counts: Option<ClanActivityCounts>,
	officer_count: Option<u32>,
}

impl ClanRecord {
	fn new() -> Self {
		Self {
			activity_counts: None,
			officer_count: None,
		}
	}
}

/// See [`FriendsInterface::coplay_friend_iter`].
#[derive(Debug)]
pub struct CoplayFriendIter<'a> {
	cursor: c_int,
	ifc: &'a FriendsInterface,
}

unsafe impl<'a> SteamApiIterator for CoplayFriendIter<'a> {
	type Item = SteamId;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		SteamId::valid_from(sys::SteamAPI_ISteamFriends_GetCoplayFriend(*self.ifc.fip, index))
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetCoplayFriendCount(*self.ifc.fip);
	}
}

/// Used by [`FriendsInterface::download_clan_activity_counts`] and [`FriendsInterface::request_activity_counts`].
#[derive(Debug)]
struct DownloadClanActivityCounts {
	steam: SteamChild,
	clan_ids: Vec<SteamId>,
}

unsafe impl Dispatch for DownloadClanActivityCounts {
	type CType = sys::DownloadClanActivityCountsResult_t;
	type Output = Vec<Option<ClanActivityCounts>>;
	type Error = UnspecifiedError;

	unsafe fn dispatch(&mut self, _: Private) -> sys::SteamAPICall_t {
		let steam = self.steam.get();

		sys::SteamAPI_ISteamFriends_DownloadClanActivityCounts(*steam.client_interfaces().friends.fip, self.clan_ids.as_ptr() as *mut SteamId as _, self.clan_ids.len() as _)
	}

	fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
		if !c_data.m_bSuccess {
			return Err(UnspecifiedError);
		}

		let steam = self.steam.get();
		let ifc = &steam.client_interfaces().friends;
		let fip = *ifc.fip;
		let mut vec: Self::Output = Vec::with_capacity(self.clan_ids.len());
		//let mut guard_counters = ifc.clan_counters.lock().unwrap();

		for steam_id in &self.clan_ids {
			let mut counts = ClanActivityCounts::default();

			vec.push(
				if unsafe {
					sys::SteamAPI_ISteamFriends_GetClanActivityCounts(
						fip,
						steam_id.0,
						&mut counts.online as *mut u32 as _,
						&mut counts.in_game as *mut u32 as _,
						&mut counts.chatting as *mut u32 as _,
					)
				} {
					//guard_counters.insert(*steam_id, counts);

					Some(counts)
				} else {
					None
				},
			);
		}

		//explicit drop for significant drop
		//drop(guard_counters);

		Ok(vec)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EquippedProfileItems {
	pub animated_avatar: bool,
	pub avatar_frame: bool,
	pub profile_modifier: bool,
	pub profile_background: bool,
	pub mini_profile_background: bool,
}

impl EquippedProfileItems {
	pub fn has(self, item_type: ProfileItemType) -> bool {
		use ProfileItemType::*;

		match item_type {
			AnimatedAvatar => self.animated_avatar,
			AvatarFrame => self.avatar_frame,
			ProfileModifier => self.profile_modifier,
			ProfileBackground => self.profile_background,
			MiniProfileBackground => self.mini_profile_background,
		}
	}
}

/// Callback.
///
/// > Callback for when a user's equipped Steam Community profile items have changed. This can be for the current user or their friends.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EquippedProfileItemsChanged_t)
#[derive(Debug)]
pub struct EquippedProfileItemsChanged;

unsafe impl CallbackRaw for EquippedProfileItemsChanged {
	const CALLBACK_ID: i32 = sys::EquippedProfileItemsChanged_t_k_iCallback as i32;
	type CType = sys::EquippedProfileItemsChanged_t;
	type Output = SteamId;

	unsafe fn on_callback(&mut self, c_data: &Self::CType, _: Private) -> Self::Output {
		SteamId::from(c_data.m_steamID)
	}

	fn register(_steam: &SteamInterface, _: Private) -> Self {
		Self
	}
}

impl Callback for EquippedProfileItemsChanged {
	type Fn = dyn FnMut(SteamId) + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: Private) {
		listener_fn(params)
	}
}

/// Used solely by [`FollowingListStream`].
#[derive(Debug)]
#[doc(hidden)]
struct FollowingListDispatch {
	index: c_uint,
	steam: SteamChild,
}

unsafe impl Dispatch for FollowingListDispatch {
	type CType = sys::FriendsEnumerateFollowingList_t;
	type Output = VecDeque<SteamId>;
	type Error = GeneralError;

	unsafe fn dispatch(&mut self, _: Private) -> sys::SteamAPICall_t {
		let steam = self.steam.get();

		sys::SteamAPI_ISteamFriends_EnumerateFollowingList(*steam.client_interfaces().friends.fip, self.index as _)
	}

	fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
		if let Some(general_error) = GeneralError::new(c_data.m_eResult) {
			return Err(general_error);
		}

		let mut queue: VecDeque<SteamId> = VecDeque::new();

		//only allocate if there are entries to push
		if c_data.m_nResultsReturned != 0 {
			//the vec will never grow
			//allocate once, fill it, then pass it to be popped
			queue.reserve_exact(c_data.m_nResultsReturned as usize);

			for c_steam_id in &c_data.m_rgSteamID[..c_data.m_nResultsReturned as usize] {
				let steam_id = SteamId::from(*c_steam_id);

				queue.push_back(steam_id);
			}
		}

		Ok(queue)
	}
}

/// See [`FriendsInterface::following_list`].
#[derive(Debug)]
pub struct FollowingListStream {
	/// Set to `false` when there is no need for dispatching.
	/// E.g. the last page of [`SteamId`]s has been reached.
	allow_dispatch: bool,

	/// The current in-progress dispatch.
	call_future: Option<CallFuture<FollowingListDispatch>>,

	/// The index to use in a dispatch if the stream runs out of [`SteamId`]s.
	dispatch_cursor: c_uint,

	/// A queue of fetched [`SteamId`]s waiting to be returned.
	queue: VecDeque<SteamId>,

	/// Weak reference to the [`SteamInterface`].
	/// If this gets dropped, the stream should giveup.
	steam: SteamChild,

	/// Set to `true` when the stream should stop yielding [`SteamId`]s.
	terminated: bool,
}

impl FollowingListStream {
	const MAX_QUEUE: usize = sys::k_cEnumerateFollowersMax as usize;

	/// Irreversibly terminate the stream.
	/// Performs drops and deallocs where possible.
	fn kill(&mut self) {
		self.allow_dispatch = false;
		self.call_future = None;
		self.terminated = true;

		self.steam.kill();

		if self.queue.capacity() != 0 {
			self.queue = VecDeque::new();
		}
	}
}

unsafe impl SteamApiStream for FollowingListStream {
	type Item = Result<SteamId, CallError<GeneralError>>;

	fn steam_api_poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>, _: Private) -> Poll<Option<Self::Item>> {
		if self.terminated {
			return Poll::Ready(None);
		}

		if let Some(call_future) = &mut self.call_future {
			call_future.register(cx.waker());

			match call_future.post() {
				Poll::Ready(result) => {
					//we only iterate if there is no dispatch
					self.call_future = None;

					match result {
						Ok(new_queue) => {
							match new_queue.len() {
								//its empty, the stream is finished
								0 => return Poll::Ready(None),

								//we have a full buffer, so there is likely more to fetch
								Self::MAX_QUEUE => {}

								//if we get some, but not a full buffer
								//this is the last page we needed to fetch
								//prevent dispatching so we don't make a bad decision
								_ => self.allow_dispatch = false,
							}

							//prepare the cursor for future dispatches
							self.dispatch_cursor += new_queue.len() as u32;
							self.queue = new_queue;
						}

						Err(error) => {
							self.kill();

							return Poll::Ready(Some(Err(error)));
						}
					}
				}

				Poll::Pending => return Poll::Pending,
			}
		}

		if let Some(steam_id) = self.queue.pop_front() {
			//we won't bother queuing up more steam ids unless they're needed
			Poll::Ready(Some(Ok(steam_id)))
		} else if self.allow_dispatch {
			let steam = self.steam.get();
			let mut guard_call_manager = steam.call_manager_lock();
			let call_future = guard_call_manager.dispatch(FollowingListDispatch {
				steam: steam.child(),
				index: self.dispatch_cursor,
			});

			//register with the waker for this stream
			//normally, the waker would be provoided by Future::poll
			//but we're a different kind of future
			call_future.register(cx.waker());

			//dropping *after* we register the waker
			//because of race conditions
			drop(guard_call_manager);

			//on the next poll, we will check if it's ready
			self.call_future = Some(call_future);

			//FollowingListDispatch
			Poll::Pending
		} else {
			//its empty
			//and we can't dispatch since we already got access to the last page
			//so the stream is finished
			self.kill();

			Poll::Ready(None)
		}
	}
}

impl futures::stream::FusedStream for Unreliable<FollowingListStream> {
	fn is_terminated(&self) -> bool {
		self.terminated
	}
}

/// See [`FriendsInterface::friend_iter`].
pub struct FriendIter<'a> {
	cursor: c_int,
	flags: FriendFlags,
	ifc: &'a FriendsInterface,
}

unsafe impl<'a> SteamApiIterator for FriendIter<'a> {
	type Item = SteamId;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		SteamId::valid_from(sys::SteamAPI_ISteamFriends_GetFriendByIndex(*self.ifc.fip, index, self.flags.bits() as _))
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetFriendCount(*self.ifc.fip, self.flags.bits() as Self::Index);
	}
}

/// > Information about the game a friend is playing.
/// Obtainable from: [`FriendsInterface::get_friend_game_played`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#FriendGameInfo_t)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FriendGameInfo {
	pub game_id: GameId,
	pub game_ip: SocketAddrV4,
	pub query_port: u16,
	pub lobby_id: LobbyId,
}

/// Callback.
///
/// First [`SteamId`] is the lobby, second is the user's.
///
/// > Called when the user tries to join a lobby from their friends list or from an invite.
/// The game client should attempt to connect to specified lobby when this is received.
/// If the game isn't running yet then the game will be automatically launched with the command line parameter `+connect_lobby <64-bit lobby Steam ID>` instead.
/// This callback is made when joining a lobby.
/// If the user is attempting to join a game but not a lobby,
/// then the callback [`rich_presence::GameRichPresenceJoinRequested`] will be made.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GameLobbyJoinRequested_t)
#[derive(Debug)]
pub struct GameLobbyJoinRequested;

unsafe impl CallbackRaw for GameLobbyJoinRequested {
	const CALLBACK_ID: i32 = sys::GameLobbyJoinRequested_t_k_iCallback as i32;
	type CType = sys::GameLobbyJoinRequested_t;
	type Output = (LobbyId, SteamId);

	unsafe fn on_callback(&mut self, c_data: &Self::CType, _: Private) -> Self::Output {
		(LobbyId::from(c_data.m_steamIDLobby), SteamId::from(c_data.m_steamIDFriend))
	}

	fn register(_steam: &SteamInterface, _: Private) -> Self {
		Self
	}
}

impl Callback for GameLobbyJoinRequested {
	type Fn = dyn FnMut(LobbyId, SteamId) + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: Private) {
		listener_fn(params.0, params.1);
	}
}

#[derive(Debug)]
pub struct InGameSpeaking<'a> {
	ifc: &'a FriendsInterface,
	speakers: MutexGuard<'a, HashSet<SteamId>>,
}

impl<'a> InGameSpeaking<'a> {
	pub fn clear(&mut self) {
		for steam_id in self.speakers.drain() {
			unsafe { sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(*self.ifc.fip, steam_id.into(), false) };
		}
	}

	/// > Let Steam know that the user is currently using voice chat in game.
	/// This will suppress the microphone for all voice communication in the Steam UI.
	///
	/// Make sure you do the opposite using [`remove`] or [`clear`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetInGameVoiceSpeaking)
	///
	/// [`clear`]: Self::clear
	/// [`remove`]: Self::remove
	pub fn insert(&mut self, steam_id: impl Into<SteamId>) {
		let steam_id = steam_id.into();

		if !self.speakers.insert(steam_id) {
			unsafe { sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(*self.ifc.fip, steam_id.into(), true) };
		}
	}

	/// Returns `true` if the user with the given [`SteamId`] is marked as speaking in-game.
	pub fn is_speaking(&self, steam_id: impl Into<SteamId>) -> bool {
		self.speakers.contains(&steam_id.into())
	}

	/// Iterator yielding the players currently marked as speaking in-game.  
	/// This is exactly what is set with [`insert`].
	///
	/// [`insert`]: Self::insert
	pub fn iter(&self) -> impl Iterator<Item = &SteamId> {
		self.speakers.iter()
	}

	/// Opposite of [`insert`].
	///
	/// [`insert`]: Self::insert
	pub fn remove(&mut self, steam_id: impl Into<SteamId>) {
		let steam_id = steam_id.into();

		if self.speakers.remove(&steam_id) {
			unsafe { sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(*self.ifc.fip, steam_id.into(), false) };
		}
	}
}

/// See [`FriendsInterface::members_iter`].
#[derive(Debug)]
pub struct MembersIter<'a> {
	cursor: c_int,
	ifc: &'a FriendsInterface,
	steam_id: SteamId,

	/// Prevents other `MembersIter` from being created.
	_lock: MutexGuard<'a, ()>,
}

unsafe impl<'a> SteamApiIterator for MembersIter<'a> {
	type Item = SteamId;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		SteamId::valid_from(sys::SteamAPI_ISteamFriends_GetFriendFromSourceByIndex(*self.ifc.fip, self.steam_id.0, index))
	}

	unsafe fn steam_api_setup(&self, _: Private) {
		sys::SteamAPI_ISteamFriends_GetFriendCountFromSource(*self.ifc.fip, self.steam_id.0);
	}
}

/// See [`FriendsInterface::persona_name_history_iter`].
#[derive(Debug)]
pub struct PersonaNameHistoryIter<'a> {
	ifc: &'a FriendsInterface,
	cursor: c_int,
	steam_id: SteamId,
}

unsafe impl<'a> SteamApiIterator for PersonaNameHistoryIter<'a> {
	type Item = String;
	type Index = c_int;

	fn steam_api_cursor(&mut self, _: Private) -> &mut Self::Index {
		&mut self.cursor
	}

	unsafe fn steam_api_get(&self, index: Self::Index, _: Private) -> Option<Self::Item> {
		some_string(sys::SteamAPI_ISteamFriends_GetFriendPersonaNameHistory(*self.ifc.fip, self.steam_id.0, index))
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
			k_EPersonaStateMax => unreachable!(),
		}
	}
}

/// Steam API callback.
///
/// > Called whenever a friends' status changes.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#PersonaStateChange_t)
#[derive(Debug)]
pub struct PersonaStateChange {
	steam: SteamChild,
}

unsafe impl CallbackRaw for PersonaStateChange {
	const CALLBACK_ID: i32 = sys::PersonaStateChange_t_k_iCallback as i32;
	type CType = sys::PersonaStateChange_t;
	type Output = (SteamId, PersonaChange);

	unsafe fn on_callback(&mut self, c_data: &Self::CType, _: Private) -> Self::Output {
		let change = PersonaChange::from_bits_retain(c_data.m_nChangeFlags);
		let changed_avatar = change.contains(PersonaChange::AVATAR);
		let steam_id = SteamId(c_data.m_ulSteamID);
		let steam = self.steam.get();

		//this may be triggered by a `FriendsInterface::request_info`
		if changed_avatar || change.contains(PersonaChange::NAME | PersonaChange::NICKNAME) {
			let mut guard_requests = steam.client_interfaces().friends.user_info_requests.lock().unwrap();

			if changed_avatar {
				guard_requests.fulfil_all(&steam_id, ());
			} else {
				guard_requests.fulfil_if(&steam_id, |needs_avatar| if *needs_avatar { None } else { Some(()) });
			}

			//explicit drop for significant drop
			drop(guard_requests);
		}

		(steam_id, change)
	}

	fn register(steam: &SteamInterface, _: Private) -> Self {
		Self { steam: steam.child() }
	}
}

impl Callback for PersonaStateChange {
	const KEEP_REGISTERED: bool = true;

	type Fn = dyn FnMut(SteamId, PersonaChange) + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output, _: Private) {
		listener_fn(params.0, params.1);
	}
}

/// > Properties on a Steam Community profile item.
///
/// Returned by [`FriendsInterface::profile_item_properties`].
/// Provides several functions to get at the item's properties.
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
#[derive(Debug)]
pub struct ProfileItemProperties<'a> {
	ifc: &'a FriendsInterface,
	item_type: sys::ECommunityProfileItemType,
	steam_id: SteamId,
}

impl<'a> ProfileItemProperties<'a> {
	/// > [`AppId`] of the item
	///
	/// May return an invalid [`AppId`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn app_id(&self) -> AppId {
		AppId(unsafe { self.u32(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_AppID) })
	}

	/// > Localized description of the item
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn description(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_Description) }
	}

	/// > URL to the large or static version of the image
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn image_large(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_ImageLarge) }
	}

	/// > URL to the small or animated version of the image
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn image_small(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_ImageSmall) }
	}

	/// > Internal name entered on the partner site (for debugging)
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn internal_name(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_InternalName) }
	}

	/// [`ProfileItemType`] that this `ProfileItemProperties` was created with.
	pub fn item_type(&self) -> ProfileItemType {
		self.item_type.into()
	}

	/// > URL to the mp4 video file
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn movie_mp4(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_MovieMP4) }
	}

	/// > URL to the small mp4 video file
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn movie_mp4_small(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_MovieMP4Small) }
	}

	/// > URL to the webm video file
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn movie_webm(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_MovieWebM) }
	}

	/// > URL to the small webm video file
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn movie_webm_small(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_MovieWebMSmall) }
	}

	/// [`SteamId`] of the user this `ProfileItemProperties` was created for.
	pub fn steam_id(&self) -> SteamId {
		self.steam_id
	}

	/// > Localized name of the item
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn title(&self) -> Option<String> {
		unsafe { self.string(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_Title) }
	}

	/// > Type id of the item, unique to the [`AppId`]
	///
	/// May return 0 upon failure.
	/// Not the same as [`std::any::TypeId`].
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemProperty)
	pub fn type_id(&self) -> u32 {
		unsafe { self.u32(sys::ECommunityProfileItemProperty::k_ECommunityProfileItemProperty_TypeID) }
	}

	/// # Safety
	/// Must be a uint property.
	unsafe fn u32(&self, prop: sys::ECommunityProfileItemProperty) -> u32 {
		sys::SteamAPI_ISteamFriends_GetProfileItemPropertyUint(*self.ifc.fip, self.steam_id.0 as _, self.item_type, prop)
	}

	/// # Safety
	/// Must be a string property.
	unsafe fn string(&self, prop: sys::ECommunityProfileItemProperty) -> Option<String> {
		some_string(sys::SteamAPI_ISteamFriends_GetProfileItemPropertyString(*self.ifc.fip, self.steam_id.0 as _, self.item_type, prop))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProfileItemType {
	/// > Animated avatar image (GIF)
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemType)
	AnimatedAvatar,

	/// > Avatar frame (may or may not be an animated PNG)
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemType)
	AvatarFrame,

	/// > Special profile modifier item, like Seasonal Profile or Artist Profile
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemType)
	ProfileModifier,

	/// > Profile background image or movie
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemType)
	ProfileBackground,

	/// > Background image or movie for the hover flyout for a user
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ECommunityProfileItemType)
	MiniProfileBackground,
}

impl From<sys::ECommunityProfileItemType> for ProfileItemType {
	fn from(value: sys::ECommunityProfileItemType) -> Self {
		use sys::ECommunityProfileItemType::*;

		match value {
			k_ECommunityProfileItemType_AnimatedAvatar => Self::AnimatedAvatar,
			k_ECommunityProfileItemType_AvatarFrame => Self::AvatarFrame,
			k_ECommunityProfileItemType_ProfileModifier => Self::ProfileModifier,
			k_ECommunityProfileItemType_ProfileBackground => Self::ProfileBackground,
			k_ECommunityProfileItemType_MiniProfileBackground => Self::MiniProfileBackground,
		}
	}
}

impl From<ProfileItemType> for sys::ECommunityProfileItemType {
	fn from(value: ProfileItemType) -> Self {
		use ProfileItemType::*;

		match value {
			AnimatedAvatar => Self::k_ECommunityProfileItemType_AnimatedAvatar,
			AvatarFrame => Self::k_ECommunityProfileItemType_AvatarFrame,
			ProfileModifier => Self::k_ECommunityProfileItemType_ProfileModifier,
			ProfileBackground => Self::k_ECommunityProfileItemType_ProfileBackground,
			MiniProfileBackground => Self::k_ECommunityProfileItemType_MiniProfileBackground,
		}
	}
}

/// > Declares the set of relationships that Steam users may have.
/// "How the users know each other."
///
/// See [`FriendsInterface::relationship`].
///
/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
pub enum Relationship {
	/// > The users have no relationship.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	None,

	/// > The user has just clicked Ignore on an friendship invite.
	/// This doesn't get stored.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	Blocked,

	/// > The user has requested to be friends with the current user.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	RequestRecipient,

	/// > A "regular" friend.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	Friend,

	/// > The current user has sent a friend invite.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	RequestInitiator,

	/// > The current user has explicit blocked this other user from comments/chat/etc.
	/// This is stored.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	Ignored,

	/// > The user has ignored the current user.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendRelationship)
	IgnoredFriend,
}

impl From<sys::EFriendRelationship> for Relationship {
	fn from(value: sys::EFriendRelationship) -> Self {
		use sys::EFriendRelationship::*;

		match value {
			k_EFriendRelationshipNone => Self::None,
			k_EFriendRelationshipBlocked => Self::Blocked,
			k_EFriendRelationshipRequestRecipient => Self::RequestRecipient,
			k_EFriendRelationshipFriend => Self::Friend,
			k_EFriendRelationshipRequestInitiator => Self::RequestInitiator,
			k_EFriendRelationshipIgnored => Self::Ignored,
			k_EFriendRelationshipIgnoredFriend => Self::IgnoredFriend,
			k_EFriendRelationshipSuggested_DEPRECATED | k_EFriendRelationshipMax => unreachable!(),
		}
	}
}

bitflags! {
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EFriendFlags)
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct FriendFlags: i32 {
		/// > None.
		const NONE = sys::EFriendFlags::k_EFriendFlagNone as i32;

		/// > Users that the current user has blocked from contacting.
		const BLOCKED = sys::EFriendFlags::k_EFriendFlagBlocked as i32;

		/// > Users that have sent a friend invite to the current user.
		const FRIENDSHIP_REQUESTED = sys::EFriendFlags::k_EFriendFlagFriendshipRequested as i32;

		/// > The current user's "regular" friends.
		const IMMEDIATE = sys::EFriendFlags::k_EFriendFlagImmediate as i32;

		/// > Users that are in one of the same (small) Steam groups as the current user.
		const CLAN_MEMBER = sys::EFriendFlags::k_EFriendFlagClanMember as i32;

		/// > Users that are on the same game server; as set by [`FriendsInterface::set_played_with`].
		const ON_GAME_SERVER = sys::EFriendFlags::k_EFriendFlagOnGameServer as i32;

		/// > Users that the current user has sent friend invites to.
		const REQUESTING_FRIENDSHIP = sys::EFriendFlags::k_EFriendFlagRequestingFriendship as i32;

		/// > Users that are currently sending additional info about themselves after a call to [`FriendsInterface::request_user_info`]
		const REQUESTING_INFO = sys::EFriendFlags::k_EFriendFlagRequestingInfo as i32;

		/// > Users that the current user has ignored from contacting them.
		const IGNORED = sys::EFriendFlags::k_EFriendFlagIgnored as i32;

		/// > Users that have ignored the current user; but the current user still knows about them.
		const IGNORED_FRIEND = sys::EFriendFlags::k_EFriendFlagIgnoredFriend as i32;

		/// > Users in one of the same chats.
		const CHAT_MEMBER = sys::EFriendFlags::k_EFriendFlagChatMember as i32;
	}

	/// > Provided by the [`PersonaStateChange`] callback.
	///
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

		/// "SOURCE" refers to a source-engine game.
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

#[cfg(test)]
mod test {
	use static_assertions::assert_eq_size;
	use std::ffi::c_int;

	#[test]
	fn assert_sizes() {
		assert_eq_size!(super::FriendFlags, c_int);
	}
}
