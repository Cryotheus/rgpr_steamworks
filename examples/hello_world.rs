use rgpr_steamworks::{config::SteamBuilder, error::Error};
use rgpr_steamworks::interfaces::{apps::AppsInterface, Steam};
use std::thread::{self, JoinHandle};

fn main() {
	let steam = match SteamBuilder::new(480).with_restart_through_steam(true).build() {
		Ok(steam) => steam,
		Err(Error::RestartingThroughSteam) => return,
		Err(error) => panic!("{error:?}"),
	};

	let apps_interface: &AppsInterface = steam.as_ref();
	let owner_steam_id = apps_interface.app_owner();
	let build_id = apps_interface.build_id();
	let task: JoinHandle<bool> = thread::spawn(check_if_vac_banned);

	println!("Your Steam ID 64 is {owner_steam_id}! Thanks for playing build #{build_id} of my game!");
	println!("Here's the DLC that is avilable right now:");

	for dlc in apps_interface.dlc_iter() {
		if dlc.available {
			println!("{} (App ID: {})", dlc.name, dlc.app_id);
		}
	}

	if task.join().unwrap() {
		println!("Naughty.");
	} else {
		println!("What a respectful gentleman!");
	}
}

fn check_if_vac_banned() -> bool {
	//the Steam interface will shutdown if nothing else is using it
	//so make sure you store it somewhere
	let steam = Steam::get().expect("the Steam interface shutdown");
	let apps_interface: &AppsInterface = steam.as_ref();

	apps_interface.vac_banned()
}
