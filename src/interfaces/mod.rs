//! Collections of the Steam API's individual interfaces.
//!
//! Interfaces which are always available no matter the usage context are fields in [`Interfaces`].
//! Some interfaces are exclusive to clients or game servers. They can be found as fields in [`ClientInterfaces`] and [`GameServerInterfaces`] respectively.

#[cfg(feature = "steam_apps")]
#[cfg_attr(doc, doc(cfg(feature = "steam_apps")))]
pub mod apps;

#[cfg(feature = "steam_friends")]
#[cfg_attr(doc, doc(cfg(feature = "steam_friends")))]
pub mod friends;

#[cfg(feature = "steam_utils")]
#[cfg_attr(doc, doc(cfg(feature = "steam_utils")))]
pub mod utils;

#[cfg(feature = "steam_client")]
#[cfg_attr(doc, doc(cfg(feature = "steam_client")))]
pub mod client;

use crate::call::{CallManager, CallThread};
use crate::config::{OverrideAppId, SteamBuilder};
use crate::dt::AppId;
use crate::error::SteamError;
use crate::steam::Steam;
use crate::util::CStrArray;
use crate::{sys, Private};
use std::fs;
use std::io::Write;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, Weak};

/// Reference to an interface's initialization function.
type InterfaceInitFn = &'static (dyn Fn(&SteamInterface) + Send + Sync);

//we pretty much only read this
//the only time there are writes are
// - when the steam api is init'd
// - the SteamInterface is dropped
//since I plan to use this for bevy
//which can launch dozens of systems in parallel on different threads
//a rwlock can't hurt
pub(super) static STEAM_INTERFACE: RwLock<Weak<SteamInterface>> = RwLock::new(Weak::new());

/// Stores a raw mutable pointer to a thread-safe Steam API interface.
/// # Safety
/// - Must not be used after Steamworks has been shutdown
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
#[cfg(not(feature = "sys"))]
pub(crate) struct FixedInterfacePtr<T>(*mut T);

/// Stores a raw mutable pointer to a thread-safe Steam API interface.
/// # Safety
/// - Must not be used after Steamworks has been shutdown
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
#[cfg(feature = "sys")]
pub struct FixedInterfacePtr<T>(*mut T);

impl<T> FixedInterfacePtr<T> {
	pub unsafe fn new(ptr: *mut T) -> Self {
		assert!(!ptr.is_null());

		Self(ptr)
	}
}

unsafe impl<T> Send for FixedInterfacePtr<T> {}
unsafe impl<T> Sync for FixedInterfacePtr<T> {}

impl<T> Deref for FixedInterfacePtr<T> {
	type Target = *mut T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> From<FixedInterfacePtr<T>> for *mut T {
	fn from(FixedInterfacePtr(ptr): FixedInterfacePtr<T>) -> Self {
		ptr
	}
}

/// Variants contain either [`ClientInterfaces`] or [`GameServerInterfaces`].
#[derive(Debug)]
pub enum ExclusiveInterfaces {
	Client(ClientInterfaces),
	GameServer(GameServerInterfaces),
}

impl ExclusiveInterfaces {
	/// Gets a reference to the Steam API client-only interfaces.
	/// # Panics
	/// If called on the `GameServer` variant.
	pub fn client(&self) -> &ClientInterfaces {
		match self {
			Self::Client(client_interfaces) => client_interfaces,
			Self::GameServer(_) => panic!("called ExclusiveInterfaces::client on ExclusiveInterfaces::GameServer"),
		}
	}

	/// Gets a reference to the Steam API game server only interfaces.
	/// # Panics
	/// If called on the `Client` variant.
	pub fn game_server(&self) -> &GameServerInterfaces {
		match self {
			Self::Client(_) => panic!("called ExclusiveInterfaces::game_server on ExclusiveInterfaces::Client"),
			Self::GameServer(game_server_interfaces) => game_server_interfaces,
		}
	}

	/// Gets a reference to the Steam API client-only interfaces.  
	/// Returns `None` if called on the `GameServer` variant.
	pub fn get_client(&self) -> Option<&ClientInterfaces> {
		match self {
			Self::Client(client_interfaces) => Some(client_interfaces),
			Self::GameServer(_) => None,
		}
	}

	/// Gets a reference to the Steam API game server only interfaces.  
	/// Returns `None` if called on the `Client` variant.
	pub fn get_game_server(&self) -> Option<&GameServerInterfaces> {
		match self {
			Self::Client(_) => None,
			Self::GameServer(game_server_interfaces) => Some(game_server_interfaces),
		}
	}
}

/// Steam API client-only interfaces.
#[derive(Debug)]
pub struct ClientInterfaces {
	/// Prevent the struct from being created by crate users when none of its fields are enabled.
	#[doc(hidden)]
	_private: Private,

	#[cfg(feature = "steam_friends")]
	friends: friends::FriendsInterface,

	#[cfg(feature = "steam_game_coordinator")]
	game_coordinator: game_coordinator::GameCoordinatorInterface,

	#[cfg(feature = "steam_html")]
	html: html::HtmlInterface,

	#[cfg(feature = "steam_input")]
	input: input::InputInterface,

	#[cfg(feature = "steam_inventory")]
	inventory: inventory::InventoryInterface,

	#[cfg(feature = "steam_match_making")]
	match_making: match_making::MatchMakingInterface,

	#[cfg(feature = "steam_match_making_servers")]
	match_making_servers: match_making_servers::MatchMakingServersInterface,

	#[cfg(feature = "steam_music_remote")]
	music_remote: music_remote::MusicRemoteInterface,

	#[cfg(feature = "steam_music")]
	music: music::MusicInterface,

	#[cfg(feature = "steam_parental_settings")]
	parental_settings: parental_settings::ParentalSettingsInterface,

	#[cfg(feature = "steam_parties")]
	parties: parties::PartiesInterface,

	#[cfg(feature = "steam_remote_play")]
	remote_play: remote_play::RemotePlayInterface,

	#[cfg(feature = "steam_remote_storage")]
	remote_storage: remote_storage::RemoteStorageInterface,

	#[cfg(feature = "steam_screenshots")]
	screenshots: screenshots::ScreenshotsInterface,

	#[cfg(feature = "steam_timeline")]
	timeline: timeline::TimelineInterface,

	#[cfg(feature = "steam_user")]
	user: user::UserInterface,

	#[cfg(feature = "steam_user_stats")]
	user_stats: user_stats::UserStatsInterface,

	#[cfg(feature = "steam_video")]
	video: video::VideoInterface,
}

/// Steam API game server only interfaces.
#[derive(Debug)]
pub struct GameServerInterfaces {
	/// Prevent the struct from being created by crate users when none of its fields are enabled.
	#[doc(hidden)]
	_private: Private,

	#[cfg(feature = "steam_game_server")]
	game_server: game_server::GameServerInterface,

	#[cfg(feature = "steam_game_server_stats")]
	game_server_stats: game_server_stats::GameServerStatsInterface,
}

/// Holds Steamworks API interfaces available to both Steam clients and game servers, as well as their [`ExclusiveInterfaces`].
#[derive(Debug)]
pub struct Interfaces {
	exclusive_interfaces: ExclusiveInterfaces,

	#[cfg(feature = "steam_apps")]
	apps: apps::AppsInterface,

	#[cfg(feature = "steam_client")]
	client: client::ClientInterface,

	#[cfg(feature = "steam_http")]
	http: HttpInterface,

	#[cfg(feature = "steam_networking")]
	networking: NetworkingInterface,

	#[cfg(feature = "steam_ugc")]
	ugc: UgcInterface,

	#[cfg(feature = "steam_utils")]
	utils: utils::UtilsInterface,
}

impl Interfaces {
	pub(crate) fn new(steam: SteamChild, init_functions: &mut Vec<InterfaceInitFn>) -> Self {
		//rename it to something shorter - for less typing below
		let pair = &mut (init_functions, &steam);

		//less typing
		fn setup<I: Interface>((init_fns, steam): &mut (&mut Vec<InterfaceInitFn>, &SteamChild)) -> I {
			init_fns.push(&I::initialize);

			I::create(unsafe { FixedInterfacePtr::new(I::raw_interface()) }, steam.clone())
		}

		Self {
			exclusive_interfaces: ExclusiveInterfaces::Client(ClientInterfaces {
				_private: Private,

				#[cfg(feature = "steam_friends")]
				friends: setup(pair),

				#[cfg(feature = "steam_game_coordinator")]
				game_coordinator: setup(pair),

				#[cfg(feature = "steam_html")]
				html: setup(pair),

				#[cfg(feature = "steam_input")]
				input: setup(pair),

				#[cfg(feature = "steam_inventory")]
				inventory: setup(pair),

				#[cfg(feature = "steam_match_making")]
				match_making: setup(pair),

				#[cfg(feature = "steam_match_making_servers")]
				match_making_servers: setup(pair),

				#[cfg(feature = "steam_music_remote")]
				music_remote: setup(pair),

				#[cfg(feature = "steam_music")]
				music: setup(pair),

				#[cfg(feature = "steam_parental_settings")]
				parental_settings: setup(pair),

				#[cfg(feature = "steam_parties")]
				parties: setup(pair),

				#[cfg(feature = "steam_remote_play")]
				remote_play: setup(pair),

				#[cfg(feature = "steam_remote_storage")]
				remote_storage: setup(pair),

				#[cfg(feature = "steam_screenshots")]
				screenshots: setup(pair),

				#[cfg(feature = "steam_timeline")]
				timeline: setup(pair),

				#[cfg(feature = "steam_user")]
				user: setup(pair),

				#[cfg(feature = "steam_user_stats")]
				user_stats: setup(pair),

				#[cfg(feature = "steam_video")]
				video: setup(pair),
			}),

			#[cfg(feature = "steam_apps")]
			apps: setup(pair),

			#[cfg(feature = "steam_client")]
			client: setup(pair),

			#[cfg(feature = "steam_http")]
			http: setup(pair),

			#[cfg(feature = "steam_networking")]
			networking: setup(pair),

			#[cfg(feature = "steam_ugc")]
			ugc: setup(pair),

			#[cfg(feature = "steam_utils")]
			utils: setup(pair),
		}
	}
}

/// Weak version of [`Steam`], for data types that are "children" of [`SteamInterface`].
/// It is only ever appropriate to use this in a context where an [`Arc<SteamInterface>`] is sure to be maintained.
#[derive(Clone, Debug)]
#[doc(hidden)]
pub struct SteamChild(pub(crate) Weak<SteamInterface>);

impl SteamChild {
	/// Gets a strong reference to the [`SteamInterface`].
	/// # Panics
	/// If the [`SteamInterface`] has been dropped.
	pub(crate) fn get(&self) -> Steam {
		Steam(self.0.upgrade().unwrap())
	}

	pub(crate) fn try_get(&self) -> Option<Steam> {
		self.0.upgrade().map(|arc| Steam(arc))
	}

	/// Sets the internal reference to an invalid one,
	/// allowing the reference counter to fully drop.
	pub(crate) fn kill(&mut self) {
		self.0 = Weak::new();
	}
}

/// Interface into the Steam API.
///
/// This is a singleton: only one instance exists during the lifetime of the program.
/// Use [`Steam::get`] to get a reference to the current instance,
/// or use [`SteamBuilder::build`] to initialize the Steam API and create a [`Steam`] reference.
#[derive(Debug)]
pub struct SteamInterface {
	app_id: AppId,
	arc: Weak<SteamInterface>,
	call_manager: Mutex<CallManager>,
	call_thread: Mutex<Option<CallThread>>,
	interfaces: Interfaces,

	/// Gets deleted when dropped.
	steam_appid_file: Option<PathBuf>,
}

impl SteamInterface {
	pub unsafe fn new(config: &SteamBuilder) -> Result<Arc<SteamInterface>, SteamError> {
		let mut global_writer = STEAM_INTERFACE.write().unwrap();

		//dont initialize if we're already attached
		if global_writer.upgrade().is_some() {
			return Err(SteamError::AlreadyExists);
		}

		if config.restart_through_steam {
			//launches the app id through steam if the exe was not launched through steam
			if sys::SteamAPI_RestartAppIfNecessary(config.app_id.0) {
				return Err(SteamError::RestartingThroughSteam);
			}
		}

		let steam_appid_file = match config.override_app_id {
			OverrideAppId::Env => {
				use std::env::set_var;

				let id_str = config.app_id.to_string();

				set_var("SteamAppId", &id_str);
				set_var("SteamGameId", id_str);

				None
			}

			OverrideAppId::File => {
				//overriding the app ID is for development builds only, so unwrapping is fine here
				let file_path = std::env::current_exe().unwrap().with_file_name("steam_appid.txt");
				let mut file_handle = fs::OpenOptions::new().write(true).create(true).truncate(true).open(&file_path).unwrap();

				file_handle.write(config.app_id.to_string().as_bytes()).unwrap();
				file_handle.flush().unwrap();

				Some(file_path)
			}

			OverrideAppId::Inherit => None,
		};

		let mut err_msg: CStrArray<1024> = CStrArray::new();
		let result_cenum = sys::SteamAPI_InitFlat(err_msg.as_mut());

		if let Some(error) = SteamError::steam_init(result_cenum, err_msg) {
			return Err(error);
		}

		//required as per steamworks docs if we are using the steam api
		//from a programming language other than C++
		sys::SteamAPI_ManualDispatch_Init();

		//steamworks successfully initialized,
		//time for us to do our init
		let mut init_functions = Vec::new();

		//convenience
		fn mutex<T>(t: T) -> Mutex<T> {
			Mutex::new(t)
		}

		let steam_interface = Arc::<SteamInterface>::new_cyclic(|weak| {
			*global_writer = Weak::clone(weak);

			let steam = SteamInterface {
				app_id: config.app_id,
				arc: Weak::clone(weak),
				call_thread: mutex(config.call_thread_config.as_ref().map(|config| CallThread::new(config.interval, SteamChild(Weak::clone(weak))))),
				call_manager: mutex(CallManager::new(SteamChild(Weak::clone(weak)))),
				interfaces: Interfaces::new(SteamChild(Weak::clone(weak)), &mut init_functions),
				steam_appid_file,
			};

			steam
		});

		//must be called after the SteamInterface is arc'd
		for init_function in init_functions {
			init_function(steam_interface.as_ref());
		}

		//we only drop the writer lock once everything is ready to be used
		//since `Steam::get` will block the thread until it can get a reader lock
		//we are able to make sure no instances of `Steam` are available until this drop
		//explicit drop for significant drop
		drop(global_writer);

		if let Some(call_thread_config) = &config.call_thread_config {
			if call_thread_config.auto_start {
				//start the CallThread to automatically run the CallManager
				steam_interface.call_thread.lock().unwrap().as_mut().unwrap().start();
			}
		}

		Ok(steam_interface)
	}

	/// Gets the [`AppId`] set by the [`SteamBuilder`].
	pub fn app_id(&self) -> AppId {
		self.app_id
	}

	/// Blocks the thread until a lock on the [`CallManager`] can be made.
	/// Make sure to drop this as early as possible.
	pub fn call_manager_lock(&self) -> MutexGuard<CallManager> {
		self.call_manager.lock().unwrap()
	}

	/// Only for use by data types that get dropped along-side the `SteamInterface`.
	#[doc(hidden)]
	pub(crate) fn child(&self) -> SteamChild {
		SteamChild(Weak::clone(&self.arc))
	}

	/// Calls [`ExclusiveInterfaces::client`].
	/// # Panics
	/// If called on the `GameServer` variant.
	pub fn client_interfaces(&self) -> &ClientInterfaces {
		self.interfaces.exclusive_interfaces.client()
	}

	/// Calls [`ExclusiveInterfaces::game_server`].
	/// # Panics
	/// If called on the `Client` variant.
	pub fn game_server_interfaces(&self) -> &GameServerInterfaces {
		self.interfaces.exclusive_interfaces.game_server()
	}

	/// Calls [`ExclusiveInterfaces::get_client`].
	pub fn get_client_interfaces(&self) -> Option<&ClientInterfaces> {
		self.interfaces.exclusive_interfaces.get_client()
	}

	/// Calls [`ExclusiveInterfaces::get_game_server`].
	pub fn get_game_server_interfaces(&self) -> Option<&GameServerInterfaces> {
		self.interfaces.exclusive_interfaces.get_game_server()
	}

	/// Returns a reference to the full collection of enabled interfaces for the Steam API.
	pub fn interfaces(&self) -> &Interfaces {
		&self.interfaces
	}
}

impl Drop for SteamInterface {
	fn drop(&mut self) {
		//lock this first so no refs are created during the shutdown
		let writer_result = STEAM_INTERFACE.write();

		unsafe {
			sys::SteamAPI_Shutdown();
		}

		//it was a temporary file, so we should delete it now
		if let Some(ref path) = self.steam_appid_file {
			let _ = fs::remove_file(path);
		}

		//we don't want to keep the Arc counters alloc'd by holding onto a weak with them
		if let Ok(mut writer) = writer_result {
			*writer = Weak::new();
		}
	}
}

impl<T> AsRef<T> for SteamInterface
where
	Interfaces: AsRef<T>,
{
	fn as_ref(&self) -> &T {
		self.interfaces.as_ref()
	}
}

/// Implemented by Steam API interfaces.
/// Primarily used for initialization of the interface on both the Rust and C sides.
pub(crate) trait Interface: Send + Sync + 'static {
	type CInterface;

	/// Called to create an instance of the implementer, an interface for the Steam API.
	/// The `SteamChild` provided here will become valid _after_ all enabled interfaces are created.
	fn create(fip: FixedInterfacePtr<Self::CInterface>, steam: SteamChild) -> Self;

	/// Called once all the interfaces are created.
	///
	/// [register_callbacks]: Self::register_callbacks
	fn initialize(_steam: &SteamInterface) {}

	/// Return a `*mut T` to wrap inside a [`FixedInterface<T>`].
	///
	/// [`FixedInterface<T>`]: FixedInterfacePtr
	unsafe fn raw_interface() -> *mut Self::CInterface;
}
