use rgpr_steamworks::interfaces::{apps::AppsInterface, Steam};
use rgpr_steamworks::{config::SteamBuilder, error::SteamError};
use std::thread::{self, JoinHandle};

fn main() {
	let steam = match SteamBuilder::new(480).with_dev().build() {
		Ok(steam) => steam,

		// Gracefully handle this error, it's not something to panic over.
		Err(SteamError::RestartingThroughSteam) => return,

		// You can deal with all other errors however you like.
		Err(error) => panic!("{error:?}"),
	};

	// Shared interfaces can be accessed with `as_ref`.
	let apps_ifc: &AppsInterface = steam.as_ref();
	let owner_steam_id = apps_ifc.app_owner();
	let build_id = apps_ifc.build_id(); // Will be 0 if not launched from Steam.
	let dlc_count = apps_ifc.dlc_count();
	let dlcs_str = apps_ifc.dlc_iter().map(|dlc| dlc.name).collect::<Vec<_>>().join(", ");

	// Steam interfaces can be accessed from any thread.
	let task: JoinHandle<bool> = thread::spawn(check_if_vac_banned);

	println!("Your Steam ID 64 is {owner_steam_id}! Thanks for playing build #{build_id} of my game! Valid build: {}.", build_id.valid());
	println!("There are currently {dlc_count} DLC available: {dlcs_str}");

	for beta in apps_ifc.beta_iter() {
		println!("Beta '{}' has build ID {}", beta.name(), beta.build_id());
	}

	// React to the result of `check_if_vac_banned`.
	if task.join().unwrap() {
		println!("Naughty.");
	} else {
		println!("What a respectful gentleman!");
	}
}

fn check_if_vac_banned() -> bool {
	// The Steam interface will shutdown if nothing else is using it,
	// so make sure you store it somewhere.
	let steam = Steam::get().expect("the Steam interface shutdown");
	let apps_ifc: &AppsInterface = steam.as_ref();

	apps_ifc.vac_banned()
}
