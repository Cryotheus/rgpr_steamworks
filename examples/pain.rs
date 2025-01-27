use rgpr_steamworks::{config::SteamBuilder, error::SteamError, interfaces::friends::FriendsInterface};
use std::io::{stdin, BufRead};
use std::thread;
use std::time::Duration;

fn sleep_lf() {
	let mut string = String::new();

	stdin().lock().read_line(&mut string).unwrap();
}

/// # Debug
fn main() {
	let steam = match SteamBuilder::new(480).with_dev().build() {
		Ok(steam) => steam,
		Err(SteamError::RestartingThroughSteam) => {
			println!("Steam is launching");

			return;
		}
		Err(error) => panic!("{error} - {error:?}"),
	};

	thread::sleep(Duration::from_secs(1));

	let friends_ifc: &FriendsInterface = steam.client_interfaces().as_ref();

	//hold
	sleep_lf();
	drop(steam); //explicit drop for significant drop - not needed
}
