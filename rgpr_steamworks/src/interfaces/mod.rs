#[cfg(feature = "steam_apps")]
pub mod apps;

#[cfg(feature = "steam_friends")]
pub mod friends;

use std::ffi::c_char;
use crate::call::{CallManager, CallThread};
use crate::config::{CallThreadBuilder, SteamBuilder};
use crate::dt::AppId;
use crate::error::Error;
use crate::{sys, Private};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard, RwLock, Weak};
use crate::util::CStrArray;

/// Reference to an interface's initialization function.
type InterfaceInitFn = &'static (dyn Fn(&SteamInterface) + Send + Sync);

//we pretty much only read this
//the only time there are writes are
// - when the steam api is init'd
// - the SteamInterface is dropped
//since I plan to use this for bevy
//which can launch dozens of systems in parallel on different threads
//a rwlock can't hurt
static STEAM_INTERFACE: RwLock<Weak<SteamInterface>> = RwLock::new(Weak::new());

/// Stores a raw mutable pointer to a thread-safe Steam API interface.
/// # Safety
/// - Must not be used after Steamworks has been shutdown
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct FixedInterfacePtr<T>(*mut T);

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
	#[doc(hidden)]
	_private: Private,

	#[cfg(feature = "steam_friends")]
	friends: Arc<friends::FriendsInterface>,

	#[cfg(feature = "steam_game_coordinator")]
	game_coordinator: Arc<game_coordinator::GameCoordinatorInterface>,

	#[cfg(feature = "steam_html")]
	html: Arc<html::HtmlInterface>,

	#[cfg(feature = "steam_input")]
	input: Arc<input::InputInterface>,

	#[cfg(feature = "steam_inventory")]
	inventory: Arc<inventory::InventoryInterface>,

	#[cfg(feature = "steam_match_making")]
	match_making: Arc<match_making::MatchMakingInterface>,

	#[cfg(feature = "steam_match_making_servers")]
	match_making_servers: Arc<match_making_servers::MatchMakingServersInterface>,

	#[cfg(feature = "steam_music_remote")]
	music_remote: Arc<music_remote::MusicRemoteInterface>,

	#[cfg(feature = "steam_music")]
	music: Arc<music::MusicInterface>,

	#[cfg(feature = "steam_parental_settings")]
	parental_settings: Arc<parental_settings::ParentalSettingsInterface>,

	#[cfg(feature = "steam_parties")]
	parties: Arc<parties::PartiesInterface>,

	#[cfg(feature = "steam_remote_play")]
	remote_play: Arc<remote_play::RemotePlayInterface>,

	#[cfg(feature = "steam_remote_storage")]
	remote_storage: Arc<remote_storage::RemoteStorageInterface>,

	#[cfg(feature = "steam_screenshots")]
	screenshots: Arc<screenshots::ScreenshotsInterface>,

	#[cfg(feature = "steam_timeline")]
	timeline: Arc<timeline::TimelineInterface>,

	#[cfg(feature = "steam_user")]
	user: Arc<user::UserInterface>,

	#[cfg(feature = "steam_user_stats")]
	user_stats: Arc<user_stats::UserStatsInterface>,

	#[cfg(feature = "steam_video")]
	video: Arc<video::VideoInterface>,
}

#[cfg(feature = "steam_friends")]
impl AsRef<friends::FriendsInterface> for ClientInterfaces {
	fn as_ref(&self) -> &friends::FriendsInterface {
		&*self.friends
	}
}

#[cfg(feature = "steam_game_coordinator")]
impl AsRef<game_coordinator::GameCoordinatorInterface> for ClientInterfaces {
	fn as_ref(&self) -> &game_coordinator::GameCoordinatorInterface {
		&*self.game_coordinator
	}
}

#[cfg(feature = "steam_html")]
impl AsRef<html::HtmlInterface> for ClientInterfaces {
	fn as_ref(&self) -> &html::HtmlInterface {
		&*self.html
	}
}

#[cfg(feature = "steam_input")]
impl AsRef<input::InputInterface> for ClientInterfaces {
	fn as_ref(&self) -> &input::InputInterface {
		&*self.input
	}
}

#[cfg(feature = "steam_inventory")]
impl AsRef<inventory::InventoryInterface> for ClientInterfaces {
	fn as_ref(&self) -> &inventory::InventoryInterface {
		&*self.inventory
	}
}

#[cfg(feature = "steam_match_making")]
impl AsRef<match_making::MatchMakingInterface> for ClientInterfaces {
	fn as_ref(&self) -> &match_making::MatchMakingInterface {
		&*self.match_making
	}
}

#[cfg(feature = "steam_match_making_servers")]
impl AsRef<match_making_servers::MatchMakingServersInterface> for ClientInterfaces {
	fn as_ref(&self) -> &match_making_servers::MatchMakingServersInterface {
		&*self.match_making_servers
	}
}

#[cfg(feature = "steam_music_remote")]
impl AsRef<music_remote::MusicRemoteInterface> for ClientInterfaces {
	fn as_ref(&self) -> &music_remote::MusicRemoteInterface {
		&*self.music_remote
	}
}

#[cfg(feature = "steam_music")]
impl AsRef<music::MusicInterface> for ClientInterfaces {
	fn as_ref(&self) -> &music::MusicInterface {
		&*self.music
	}
}

#[cfg(feature = "steam_parental_settings")]
impl AsRef<parental_settings::ParentalSettingsInterface> for ClientInterfaces {
	fn as_ref(&self) -> &parental_settings::ParentalSettingsInterface {
		&*self.parental_settings
	}
}

#[cfg(feature = "steam_parties")]
impl AsRef<parties::PartiesInterface> for ClientInterfaces {
	fn as_ref(&self) -> &parties::PartiesInterface {
		&*self.parties
	}
}

#[cfg(feature = "steam_remote_play")]
impl AsRef<remote_play::RemotePlayInterface> for ClientInterfaces {
	fn as_ref(&self) -> &remote_play::RemotePlayInterface {
		&*self.remote_play
	}
}

#[cfg(feature = "steam_remote_storage")]
impl AsRef<remote_storage::RemoteStorageInterface> for ClientInterfaces {
	fn as_ref(&self) -> &remote_storage::RemoteStorageInterface {
		&*self.remote_storage
	}
}

#[cfg(feature = "steam_screenshots")]
impl AsRef<screenshots::ScreenshotsInterface> for ClientInterfaces {
	fn as_ref(&self) -> &screenshots::ScreenshotsInterface {
		&*self.screenshots
	}
}

#[cfg(feature = "steam_timeline")]
impl AsRef<timeline::TimelineInterface> for ClientInterfaces {
	fn as_ref(&self) -> &timeline::TimelineInterface {
		&*self.timeline
	}
}

#[cfg(feature = "steam_user")]
impl AsRef<user::UserInterface> for ClientInterfaces {
	fn as_ref(&self) -> &user::UserInterface {
		&*self.user
	}
}

#[cfg(feature = "steam_user_stats")]
impl AsRef<user_stats::UserStatsInterface> for ClientInterfaces {
	fn as_ref(&self) -> &user_stats::UserStatsInterface {
		&*self.user_stats
	}
}

#[cfg(feature = "steam_video")]
impl AsRef<video::VideoInterface> for ClientInterfaces {
	fn as_ref(&self) -> &video::VideoInterface {
		&*self.video
	}
}

/// Steam API game server only interfaces.
#[derive(Debug)]
pub struct GameServerInterfaces {
	#[doc(hidden)]
	_private: Private,

	#[cfg(feature = "steam_game_server")]
	game_server: Arc<game_server::GameServerInterface>,

	#[cfg(feature = "steam_game_server_stats")]
	game_server_stats: Arc<game_server_stats::GameServerStatsInterface>,
}

#[cfg(feature = "steam_game_server")]
impl AsRef<game_server::GameServerInterface> for GameServerInterfaces {
	fn as_ref(&self) -> &game_server::GameServerInterface {
		&self.game_server
	}
}

#[cfg(feature = "steam_game_server_stats")]
impl AsRef<game_server_stats::GameServerStatsInterface> for GameServerInterfaces {
	fn as_ref(&self) -> &game_server_stats::GameServerStatsInterface {
		&self.game_server_stats
	}
}

#[derive(Debug)]
pub struct Interfaces {
	#[doc(hidden)]
	_private: Private,

	exclusive_interfaces: ExclusiveInterfaces,

	#[cfg(feature = "steam_apps")]
	apps: Arc<apps::AppsInterface>,

	#[cfg(feature = "steam_client")]
	client: Arc<ClientInterface>,

	#[cfg(feature = "steam_http")]
	http: Arc<HttpInterface>,

	#[cfg(feature = "steam_networking")]
	networking: Arc<NetworkingInterface>,

	#[cfg(feature = "steam_ugc")]
	ugc: Arc<UgcInterface>,

	#[cfg(feature = "steam_utils")]
	utils: Arc<UtilsInterface>,
}

impl Interfaces {
	pub(crate) fn new(steam: SteamChild, init_functions: &mut Vec<InterfaceInitFn>) -> Self {
		//rename it to something shorter - for less typing below
		let pair = &mut (init_functions, &steam);

		//less typing
		fn setup<I: Interface>((init_fns, steam): &mut (&mut Vec<InterfaceInitFn>, &SteamChild)) -> I {
			init_fns.push(&I::initialize);

			I::create(FixedInterfacePtr(unsafe { I::raw_interface() }), steam.clone())
		}

		Self {
			_private: Private,

			exclusive_interfaces: ExclusiveInterfaces::Client(ClientInterfaces {
				_private: Private,

				#[cfg(feature = "steam_friends")]
				friends: Arc::new(setup(pair)),

				#[cfg(feature = "steam_game_coordinator")]
				game_coordinator: Arc::new(setup(pair)),

				#[cfg(feature = "steam_html")]
				html: Arc::new(setup(pair)),

				#[cfg(feature = "steam_input")]
				input: Arc::new(setup(pair)),

				#[cfg(feature = "steam_inventory")]
				inventory: Arc::new(setup(pair)),

				#[cfg(feature = "steam_match_making")]
				match_making: Arc::new(setup(pair)),

				#[cfg(feature = "steam_match_making_servers")]
				match_making_servers: Arc::new(setup(pair)),

				#[cfg(feature = "steam_music_remote")]
				music_remote: Arc::new(setup(pair)),

				#[cfg(feature = "steam_music")]
				music: Arc::new(setup(pair)),

				#[cfg(feature = "steam_parental_settings")]
				parental_settings: Arc::new(setup(pair)),

				#[cfg(feature = "steam_parties")]
				parties: Arc::new(setup(pair)),

				#[cfg(feature = "steam_remote_play")]
				remote_play: Arc::new(setup(pair)),

				#[cfg(feature = "steam_remote_storage")]
				remote_storage: Arc::new(setup(pair)),

				#[cfg(feature = "steam_screenshots")]
				screenshots: Arc::new(setup(pair)),

				#[cfg(feature = "steam_timeline")]
				timeline: Arc::new(setup(pair)),

				#[cfg(feature = "steam_user")]
				user: Arc::new(setup(pair)),

				#[cfg(feature = "steam_user_stats")]
				user_stats: Arc::new(setup(pair)),

				#[cfg(feature = "steam_video")]
				video: Arc::new(setup(pair)),
			}),

			#[cfg(feature = "steam_apps")]
			apps: Arc::new(setup(pair)),

			#[cfg(feature = "steam_client")]
			client: Arc::new(setup(pair)),

			#[cfg(feature = "steam_http")]
			http: Arc::new(setup(pair)),

			#[cfg(feature = "steam_networking")]
			networking: Arc::new(setup(pair)),

			#[cfg(feature = "steam_ugc")]
			ugc: Arc::new(setup(pair)),

			#[cfg(feature = "steam_utils")]
			utils: Arc::new(setup(pair)),
		}
	}
}

#[cfg(feature = "steam_apps")]
impl AsRef<apps::AppsInterface> for Interfaces {
	fn as_ref(&self) -> &apps::AppsInterface {
		&*self.apps
	}
}

#[cfg(feature = "steam_client")]
impl AsRef<client::ClientInterface> for Interfaces {
	fn as_ref(&self) -> &client::ClientInterface {
		&*self.client
	}
}

#[cfg(feature = "steam_http")]
impl AsRef<http::HttpInterface> for Interfaces {
	fn as_ref(&self) -> &http::HttpInterface {
		&*self.http
	}
}

#[cfg(feature = "steam_networking")]
impl AsRef<networking::NetworkingInterface> for Interfaces {
	fn as_ref(&self) -> &networking::NetworkingInterface {
		&*self.networking
	}
}

#[cfg(feature = "steam_ugc")]
impl AsRef<ugc::UgcInterface> for Interfaces {
	fn as_ref(&self) -> &ugc::UgcInterface {
		&*self.ugc
	}
}

#[cfg(feature = "steam_utils")]
impl AsRef<utils::UtilsInterface> for Interfaces {
	fn as_ref(&self) -> &utils::UtilsInterface {
		&*self.utils
	}
}

/// Reference to the current [SteamInterface].
#[derive(Clone, Debug)]
pub struct Steam(Arc<SteamInterface>);

impl Steam {
	/// Get a reference to the currently initialized Steam API interface.
	/// Returns `None` if the Steam API is not initialized.
	pub fn get() -> Option<Steam> {
		STEAM_INTERFACE.read().unwrap().upgrade().map(|steam_interface| Steam(steam_interface))
	}

	/// Attach to the Steam API and initialize interfaces.
	///
	/// Called by [`SteamBuilder::build`]
	///
	/// [`SteamBuilder::build`]: crate::config::SteamBuilder::build
	pub(crate) unsafe fn init(config: &SteamBuilder) -> Result<Steam, Error> {
		let mut global_writer = STEAM_INTERFACE.write().unwrap();

		//dont initialize if we're already attached
		if global_writer.upgrade().is_some() {
			return Err(Error::AlreadyExists);
		}

		if config.restart_through_steam {
			//launches the app id through steam if the exe was not launched through steam
			if sys::SteamAPI_RestartAppIfNecessary(config.app_id.0) {
				return Err(Error::RestartingThroughSteam);
			}
		}

		if config.override_app_id {
			use std::env::set_var;

			let id_str = config.app_id.to_string();

			set_var("SteamAppId", &id_str);
			set_var("SteamGameId", id_str);
		}

		let mut err_msg: CStrArray<1024> = CStrArray::new();
		let result_cenum = sys::SteamAPI_InitFlat(err_msg.as_mut());

		if let Some(error) = Error::steam_init(result_cenum, err_msg) {
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
				call_thread: mutex(config.call_thread_config.as_ref().map(|config| CallThread::new(config.interval, SteamChild::from(weak)))),
				call_manager: mutex(CallManager::new()),
				interfaces: Interfaces::new(weak.into(), &mut init_functions),
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

		Ok(Steam(steam_interface))
	}
}

impl<T> AsRef<T> for Steam
where
	<Self as Deref>::Target: AsRef<T>,
{
	fn as_ref(&self) -> &T {
		self.deref().as_ref()
	}
}

impl Deref for Steam {
	type Target = SteamInterface;

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}

/// Weak version of [`Steam`], for data types that are "children" of [`SteamInterface`].
/// It is only ever appropriate to use this in a context where an [`Arc<SteamInterface>`] is sure to be maintained.
#[derive(Clone, Debug)]
pub(crate) struct SteamChild(pub(crate) Weak<SteamInterface>);

impl SteamChild {
	/// Gets a strong reference to the [`SteamInterface`].
	/// # Panics
	/// If the [`SteamInterface`] has been dropped.
	pub(crate) fn get(&self) -> Steam {
		Steam(self.0.upgrade().unwrap())
	}

	/// Sets the internal reference to an invalid one,
	/// allowing the reference counter to fully drop.
	pub(crate) fn kill(&mut self) {
		self.0 = Weak::new();
	}
}

impl From<Steam> for SteamChild {
	fn from(Steam(ref arc): Steam) -> Self {
		SteamChild(Arc::downgrade(arc))
	}
}

impl From<Arc<SteamInterface>> for SteamChild {
	fn from(ref arc: Arc<SteamInterface>) -> Self {
		SteamChild(Arc::downgrade(arc))
	}
}

impl From<Weak<SteamInterface>> for SteamChild {
	fn from(weak: Weak<SteamInterface>) -> Self {
		SteamChild(weak)
	}
}

impl From<&Weak<SteamInterface>> for SteamChild {
	fn from(weak_ref: &Weak<SteamInterface>) -> Self {
		SteamChild(Weak::clone(weak_ref))
	}
}

/// Interface into the Steam API.
/// This is a singleton,
#[derive(Debug)]
pub struct SteamInterface {
	app_id: AppId,
	arc: Weak<SteamInterface>,
	call_manager: Mutex<CallManager>,
	call_thread: Mutex<Option<CallThread>>,
	interfaces: Interfaces,
}

impl SteamInterface {
	/// Blocks the thread until a lock on the [`CallManager`] can be made.
	/// Make sure to drop this as early as possible.
	/// Do not hold the guard across awaits.
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

	pub fn interfaces(&self) -> &Interfaces {
		&self.interfaces
	}
}

impl Drop for SteamInterface {
	fn drop(&mut self) {
		unsafe {
			sys::SteamAPI_Shutdown();
		}

		//we don't want to keep the Arc counters alloc'd by holding onto a weak with them
		if let Ok(mut writer) = STEAM_INTERFACE.write() {
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

impl AsRef<Interfaces> for SteamInterface {
	fn as_ref(&self) -> &Interfaces {
		&self.interfaces
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
