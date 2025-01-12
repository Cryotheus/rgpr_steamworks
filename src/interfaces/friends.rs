use crate::call::{CallFuture, Callback, CallbackRaw, Dispatch};
use crate::dt::{AppId, SteamId};
use crate::error::{CallError, GeneralError, SilentFailure};
use crate::interfaces::{FixedInterfacePtr, Interface, SteamChild, SteamInterface};
use crate::util::{some_string, success};
use crate::{sys, Private};
use bitflags::bitflags;
use futures::channel::oneshot::{channel, Sender};
use lru::LruCache;
use rgpr_steamworks_sys::SteamAPICall_t;
use std::collections::{HashMap, HashSet, VecDeque};
use std::default::Default;
use std::ffi::CString;
use std::future::Future;
use std::num::NonZeroUsize;
use std::ops::Deref;
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};
use std::task::{Context, Poll};
use std::usize;
//const k_cchPersonaNameMax

#[derive(Debug)]
pub struct FriendsInterface {
	fip: FixedInterfacePtr<sys::ISteamFriends>,
	clan_lru: Mutex<LruCache<SteamId, ClanRecord>>,
	clan_counters: Mutex<HashMap<SteamId, ClanActivityCounts>>,
	in_game_speaking: Mutex<HashSet<SteamId>>,
	user_info_requests: Mutex<HashMap<SteamId, VecDeque<(Sender<()>, bool)>>>,
	steam: SteamChild,
}

impl FriendsInterface {
	const CLAN_LRU_CAP: usize = 64;

	/// See [`ActivateGameOverlay`].
	pub fn activate_game_overlay(&self, instruction: ActivateGameOverlay) {
		use ActivateGameOverlay::*;

		let interface = *self.fip;

		unsafe {
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

	/// Clears the list of currently speaking users.  
	/// This is the same as calling [`set_in_game_speaking`] with `false` for each user that last had `true` set,
	/// but does so uninterrupted as the lock on the speaking users cache is held until it is cleared.
	///
	/// Returns the [`SteamId`] of the users who were marked as speaking.
	///
	/// This does nothing if [`set_in_game_speaking`] is never used.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetInGameVoiceSpeaking)
	pub fn clear_in_game_speaking(&self) -> Vec<SteamId> {
		let mut guard = self.in_game_speaking.lock().unwrap();
		let interface = *self.fip;
		let mut vec = Vec::with_capacity(guard.len());

		for steam_id in guard.drain() {
			unsafe {
				sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(interface, steam_id.0, false);
			}

			vec.push(steam_id);
		}

		//explicit drop for
		drop(guard);

		vec
	}

	/// > Clears all of the current user's Rich Presence key/values.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#ClearRichPresence)
	pub fn clear_rich_presence(&self) {
		unsafe {
			sys::SteamAPI_ISteamFriends_ClearRichPresence(*self.fip);
		}
	}

	/// > Closes the specified Steam group chat room in the Steam UI.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#CloseClanChatWindowInSteam)
	pub fn close_clan_chat_window_in_steam(&self, group_chat_id: SteamId) -> Result<(), SilentFailure> {
		unsafe { success(sys::SteamAPI_ISteamFriends_CloseClanChatWindowInSteam(*self.fip, group_chat_id.0)) }
	}

	/// Downloads and caches [`ClanActivityCounts`] for the provided clans' [`SteamId`]s.  
	/// Use [`load_clan`] alongside [`Clan::activity_counts`] if you just want one.  
	/// If you don't want to use an existing cached count, use [`get_clan`] instead.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#DownloadClanActivityCounts)
	///
	/// [`get_clan`]: Self::get_clan
	/// [`load_clan`]: Self::load_clan
	/// [zip]: Iterator::zip
	pub async fn download_clan_activity_counts(&self, clan_ids: impl Into<Vec<SteamId>>) -> Result<Vec<Option<ClanActivityCounts>>, CallError<SilentFailure>> {
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

	/// Returns an asynchronous stream for querying the current user's followed users.
	///
	/// > Gets the list of users that the current user is following.
	/// You can be following people that are not your friends.
	/// Following allows you to receive updates when the person does things like post a new piece of content to the Steam Workshop.
	///
	/// Dispatches will be created as the stream is polled.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#EnumerateFollowingList)
	pub fn following_list(&self) -> FollowingListStream {
		FollowingListStream {
			allow_dispatch: true,
			call_future: None,
			dedupe: Some(HashSet::new()),
			dispatch_cursor: 0,
			terminated: false,
			reported_total: 0,
			steam: self.steam.clone(),
			queue: VecDeque::new(),
		}
	}

	/// Gets a cached [`Clan`].
	/// Also see [`load_clan`].
	///
	/// [`load_clan`]: Self::load_clan
	pub fn get_clan(&self, steam_id: SteamId) -> Option<Clan> {
		let mut guard_cache = self.clan_lru.lock().unwrap();

		if guard_cache.get(&steam_id).is_none() {
			return None;
		}

		Some(Clan {
			clan_id: steam_id,
			friends_interface: &self,
			guard_cache,
		})
	}

	/// > Gets the number of Steam groups that the current user is a member of.
	///
	/// The function to iterate with is deprecated according to the docs.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanCount)
	pub fn get_clan_count(&self) -> u32 {
		unsafe { sys::SteamAPI_ISteamFriends_GetClanCount(*self.fip) as u32 }
	}

	/// Returns a single cached [`ClanActivityCounts`], or `None` if there is no cached value.
	/// If [`download_clan_activity_counts`] has not yet been called for the [`SteamId`],
	/// the returned `Option<ClanActivityCounts>` will always be `None`.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#DownloadClanActivityCounts)
	///
	/// [`download_clan_activity_counts`]: Self::download_clan_activity_counts
	pub fn get_single_clan_activity_counts(&self, clan_id: &SteamId) -> Option<ClanActivityCounts> {
		self.clan_counters.lock().unwrap().get(clan_id).cloned()
	}

	/// Gets a clan from the cache, or loads one.
	/// The [`Clan`] will be accessible with [`get_clan`],
	/// until it is pushed out of the cache by more recently used [`Clan`]s.
	///
	/// [`get_clan`]: Self::get_clan
	pub async fn load_clan(&self, steam_id: SteamId) -> Clan {
		let mut guard_cache = self.clan_lru.lock().unwrap();
		let clan_record = guard_cache.get_or_insert_mut(steam_id, || ClanRecord {
			activity_counts: None,
			officer_count: None,
		});

		let needs_activity_counts = clan_record.activity_counts.is_none();
		let needs_officer_count = clan_record.officer_count.is_none();

		let mut clan = Clan {
			clan_id: steam_id,
			friends_interface: &self,
			guard_cache,
		};

		if needs_activity_counts {
			let _ = clan.request_activity_counts().await;
		}

		if needs_officer_count {
			let _ = clan.request_officers().await;
		}

		clan
	}

	/// > Requests the persona name and optionally the avatar of a specified user.
	/// It's a lot slower to download avatars and churns the local cache,
	/// so if you don't need avatars, don't request them.
	///
	/// Returns `None` if Steam says we already have the user info and a request isn't needed.
	///
	/// The return value is not a [`Future`] of an [`Option`] e.g.  
	/// `impl Future<Output = Option<()>>`  
	/// but a [`Option`] of a [`Future`] being:  
	/// `Option<impl Future<Output = ()>>`
	///
	/// This means the call only needs to be `await`ed if the request was made.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestUserInformation)
	pub fn request_user_info(&self, steam_id: SteamId, request_avatar: bool) -> Option<impl Future<Output = ()>> {
		let mut guard = self.user_info_requests.lock().unwrap();

		//only make the request when we have the guard
		//to prevent race conditions
		if !unsafe { sys::SteamAPI_ISteamFriends_RequestUserInformation(*self.fip, steam_id.0, !request_avatar) } {
			//if steam says we're good
			//we don't need to do async!
			return None;
		}

		let mut existing = guard.get_mut(&steam_id);

		if existing.is_none() {
			guard.insert(steam_id, VecDeque::new());

			existing = guard.get_mut(&steam_id);
		}

		let queue = existing.unwrap();
		let (tx, rx) = channel::<()>();

		if request_avatar {
			queue.push_front((tx, true));
		} else {
			queue.push_back((tx, false));
		}

		drop(guard); //drop before we await!

		Some(async {
			let _ = rx.await;
		})
	}

	/// > Let Steam know that the user is currently using voice chat in game.
	/// This will suppress the microphone for all voice communication in the Steam UI.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#SetInGameVoiceSpeaking)
	pub fn set_in_game_speaking(&self, steam_id: SteamId, speaking: bool) {
		let mut guard = self.in_game_speaking.lock().unwrap();

		if speaking {
			guard.insert(steam_id);
		} else {
			guard.remove(&steam_id);
		}

		unsafe {
			sys::SteamAPI_ISteamFriends_SetInGameVoiceSpeaking(*self.fip, steam_id.0, speaking);
		}

		//sanity check: make sure we are showing how
		drop(guard);
	}
}

impl Interface for FriendsInterface {
	type CInterface = sys::ISteamFriends;

	fn create(fip: FixedInterfacePtr<Self::CInterface>, steam: SteamChild) -> Self {
		Self {
			fip,
			clan_lru: Mutex::new(LruCache::new(NonZeroUsize::new(Self::CLAN_LRU_CAP).unwrap())),
			steam,

			//simple collections
			clan_counters: Default::default(),
			in_game_speaking: Default::default(),
			user_info_requests: Default::default(),
		}
	}

	fn initialize(steam: &SteamInterface) {
		let mut call_manager = steam.call_manager_lock();

		//needed for resuest_user_info
		call_manager.register_pub::<PersonaStateChange>();
	}

	unsafe fn raw_interface() -> *mut Self::CInterface {
		sys::SteamAPI_SteamFriends_v017()
	}
}

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

#[derive(Debug)]
pub struct Clan<'a> {
	clan_id: SteamId,
	friends_interface: &'a FriendsInterface,
	guard_cache: MutexGuard<'a, LruCache<SteamId, ClanRecord>>,
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
		self.record().activity_counts
	}

	/// > Gets the display name for the specified Steam group; if the local client knows about it.
	///
	/// Returns `None` if the clan is inaccessible.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanName)
	pub fn name(&self) -> Option<String> {
		unsafe { some_string(sys::SteamAPI_ISteamFriends_GetClanName(*self.friends_interface.fip, self.clan_id.0)) }
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
		self.record().officer_count
	}

	/// > Gets the owner of a Steam Group.
	///
	/// May return `None` if [`request_officers`] has failed, or the clan is inaccessible.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#GetClanOwner)
	///
	/// [`request_officers`]: Self::request_officers
	pub fn owner(&self) -> Option<SteamId> {
		unsafe { SteamId::non_zero_from(sys::SteamAPI_ISteamFriends_GetClanOwner(*self.friends_interface.fip, self.clan_id.0)) }
	}

	/// > Refresh the Steam Group activity data or get the data from groups other than one that the current user is a member.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#DownloadClanActivityCounts)
	pub async fn request_activity_counts(&mut self) -> Result<(), CallError<SilentFailure>> {
		let steam = self.friends_interface.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(DownloadClanActivityCounts {
			steam: steam.child(),
			clan_ids: vec![self.clan_id],
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		self.record().activity_counts = future.await?[0];

		Ok(())
	}

	/// > Requests information about a Steam group officers (administrators and moderators).  
	/// You can only ask about Steam groups that a user is a member of.
	/// This won't download avatars for the officers automatically.
	/// If no avatar image is available for an officer,
	/// then call [`FriendsInterface::request_user_info`] to download the avatar.
	///
	/// [Steamworks Docs](https://partner.steamgames.com/doc/api/ISteamFriends#RequestClanOfficerList)
	pub async fn request_officers(&mut self) -> Result<(), CallError<SilentFailure>> {
		#[derive(Debug)]
		struct RequestClanOfficerList {
			clan_id: SteamId,
			steam: SteamChild,
		}

		unsafe impl Dispatch for RequestClanOfficerList {
			type CType = sys::ClanOfficerListResponse_t;
			type Output = u32;
			type Error = SilentFailure;

			unsafe fn dispatch(&mut self, _: Private) -> SteamAPICall_t {
				sys::SteamAPI_ISteamFriends_RequestClanOfficerList(*self.steam.get().client_interfaces().friends.fip, self.clan_id.0)
			}

			fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
				//for some reason, m_bSuccess success is a u8 instead of a bool
				if c_data.m_bSuccess == 0 {
					return Err(SilentFailure);
				}

				Ok(c_data.m_cOfficers as u32)
			}
		}

		let steam = self.friends_interface.steam.get();
		let mut guard_call_manager = steam.call_manager_lock();

		let future = guard_call_manager.dispatch(RequestClanOfficerList {
			clan_id: self.clan_id,
			steam: steam.child(),
		});

		//explicit drop for significant drop
		drop(guard_call_manager);

		self.record().officer_count = Some(future.await?);

		Ok(())
	}

	/// Gets a lock on the [`ClanRecord`] this `Clan` reference is associated with.
	#[doc(hidden)]
	fn record(&mut self) -> &mut ClanRecord {
		self.guard_cache.get_mut(&self.clan_id).unwrap()
	}
}

/// Internal record for clan information.
/// Use [`FriendsInterface::load_clan`] and [`Clan`] to access this data.
#[derive(Debug)]
#[doc(hidden)]
struct ClanRecord {
	activity_counts: Option<ClanActivityCounts>,
	officer_count: Option<u32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ClanActivityCounts {
	pub online: u32,
	pub in_game: u32,
	pub chatting: u32,
}

#[derive(Debug)]
struct DownloadClanActivityCounts {
	steam: SteamChild,
	clan_ids: Vec<SteamId>,
}

impl DownloadClanActivityCounts {
	fn fip(&self) -> FixedInterfacePtr<sys::ISteamFriends> {
		self.steam.get().client_interfaces().friends.fip
	}
}

unsafe impl Dispatch for DownloadClanActivityCounts {
	type CType = sys::DownloadClanActivityCountsResult_t;
	type Output = Vec<Option<ClanActivityCounts>>;
	type Error = SilentFailure;

	unsafe fn dispatch(&mut self, _: Private) -> SteamAPICall_t {
		sys::SteamAPI_ISteamFriends_DownloadClanActivityCounts(*self.fip(), self.clan_ids.as_ptr() as *mut SteamId as _, self.clan_ids.len() as _)
	}

	fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
		if !c_data.m_bSuccess {
			return Err(SilentFailure);
		}

		let steam = self.steam.get();
		let friends_interface = steam.client_interfaces().friends.deref();
		let fip = *friends_interface.fip;
		let mut vec: Self::Output = Vec::with_capacity(self.clan_ids.len());
		let mut guard_counters = friends_interface.clan_counters.lock().unwrap();

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
					guard_counters.insert(*steam_id, counts);

					Some(counts)
				} else {
					None
				},
			);
		}

		//explicit drop for significant drop
		drop(guard_counters);

		Ok(vec)
	}
}

/// Used solely by [`FollowingListStream`].
#[derive(Debug)]
#[doc(hidden)]
struct FollowingListDispatch {
	dedupe: Option<HashSet<SteamId>>,
	index: u32,
	steam: SteamChild,
}

unsafe impl Dispatch for FollowingListDispatch {
	type CType = sys::FriendsEnumerateFollowingList_t;
	type Output = (u32, VecDeque<SteamId>, HashSet<SteamId>);
	type Error = GeneralError;

	unsafe fn dispatch(&mut self, _: Private) -> SteamAPICall_t {
		sys::SteamAPI_ISteamFriends_EnumerateFollowingList(*self.steam.get().client_interfaces().friends.fip, self.index as _)
	}

	fn post(&mut self, c_data: Box<Self::CType>, _: Private) -> Result<Self::Output, Self::Error> {
		if let Some(general_error) = GeneralError::new(c_data.m_eResult) {
			return Err(general_error);
		}

		let mut dedupe = self.dedupe.take().unwrap();
		let mut queue: VecDeque<SteamId> = VecDeque::new();

		//only allocate if there are entries to push
		if c_data.m_nResultsReturned != 0 {
			//the vec will never grow
			//allocate once, fill it, then pass it to be popped
			queue.reserve_exact(c_data.m_nResultsReturned as usize);

			for c_steam_id in &c_data.m_rgSteamID[..c_data.m_nResultsReturned as usize] {
				let steam_id = SteamId::from(*c_steam_id);

				if dedupe.insert(steam_id) {
					queue.push_back(steam_id);
				}
			}
		}

		Ok((c_data.m_nTotalResultCount as u32, queue, dedupe))
	}
}

/// May miss a [`SteamId`] if the following list changes while streaming.
/// Will never return a duplicate [`SteamId`].
#[derive(Debug)]
pub struct FollowingListStream {
	/// Set to `false` when there is no need for dispatching.
	/// E.g. the last page of [`SteamId`]s has been reached.
	allow_dispatch: bool,

	/// The current in-progress dispatch.
	call_future: Option<CallFuture<FollowingListDispatch>>,

	/// Holds a list of [`SteamId`]s that have already been seen.
	dedupe: Option<HashSet<SteamId>>,

	/// The index to use in a dispatch if the stream runs out of [`SteamId`]s.
	dispatch_cursor: u32,

	/// Set to `true` when the stream should stop yielding [`SteamId`]s.
	terminated: bool,

	/// What the function returned as the total.
	/// Unrealiable because the list can change while we stream it.
	reported_total: u32,

	/// Weak reference to the [`SteamInterface`].  
	/// If this gets dropped, the stream should giveup.
	steam: SteamChild,

	/// A queue of fetched [`SteamId`]s waiting to be returned.
	queue: VecDeque<SteamId>,
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

impl futures::Stream for FollowingListStream {
	type Item = Result<SteamId, CallError<GeneralError>>;

	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
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
						Ok((new_total, new_queue, borrowed_dedupe)) => {
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
							self.dedupe = Some(borrowed_dedupe);
							self.dispatch_cursor += new_queue.len() as u32;
							self.queue = new_queue;
							self.reported_total = self.reported_total.max(new_total);
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
				dedupe: self.dedupe.take(),
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

	fn size_hint(&self) -> (usize, Option<usize>) {
		if self.terminated {
			return (0, Some(0));
		}

		let queue_len = self.queue.len();
		let lower_bound: usize = queue_len;
		let upper_bound: Option<usize>;

		if !self.allow_dispatch {
			//if dispatching isn't allowed
			//what we currently have queued is all that is left for this stream
			//if it changes while the stream is still polling these queued values - too bad!
			//we won't be dispatching again to get the updated list of users
			upper_bound = Some(queue_len);
		} else {
			let polled = queue_len + self.dispatch_cursor as usize;
			let estimated_total = Self::MAX_QUEUE.max(self.reported_total as usize - polled);

			upper_bound = Some(estimated_total);
		}

		(lower_bound, upper_bound)
	}
}

impl futures::stream::FusedStream for FollowingListStream {
	fn is_terminated(&self) -> bool {
		self.terminated
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

		if changed_avatar || change.contains(PersonaChange::NAME) {
			let mut guard_requests = steam.client_interfaces().friends.user_info_requests.lock().unwrap();

			if changed_avatar {
				//we will send the update for all since persona changes with
				if let Some(requests) = guard_requests.remove(&steam_id) {
					for (tx, _) in requests {
						tx.send(()).unwrap();
					}
				}
			} else if let Some(requests) = guard_requests.get_mut(&steam_id) {
				if let Some(position) = requests.iter().position(|(_, requested_avatar)| !requested_avatar) {
					for (tx, _) in requests.split_off(position) {
						tx.send(()).unwrap();
					}
				}
			}

			//explicit drop for significant drop
			drop(guard_requests);
		}

		(steam_id, change)
	}

	fn register(steam: &SteamInterface) -> Self {
		Self { steam: steam.child() }
	}
}

impl Callback for PersonaStateChange {
	const KEEP_REGISTERED: bool = true;

	type Fn = dyn FnMut(SteamId, PersonaChange) + Send + Sync;

	fn call_listener(&mut self, listener_fn: &mut Self::Fn, params: Self::Output) {
		listener_fn(params.0, params.1);
	}
}
