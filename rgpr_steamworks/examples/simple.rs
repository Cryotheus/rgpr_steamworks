use pollster::FutureExt;
use rgpr_steamworks::{steam_api::friends::PersonaStateChange, steam_api::user_stats::GetNumberOfCurrentPlayers, Steamworks, SteamworksInitConfig};
use std::time::Duration;
use std::thread::sleep;

fn dur(secs: impl Into<f32>) -> Duration {
	Duration::from_secs_f32(secs.into())
}

struct YourSignature;

fn main() {
	let steamworks = Steamworks::init(SteamworksInitConfig::development(480)).unwrap();
	let mut call_manager = steamworks.call_manager();
	let call_future = call_manager.call(GetNumberOfCurrentPlayers);

	println!("dispatched async call");
	sleep(dur(1.));
	println!("running callbacks");
	call_manager.run_callbacks();

	let result = call_future.block_on();

	println!("{result:?}");
	println!("next test: callback listener");

	call_manager.listen::<PersonaStateChange, YourSignature>(Box::from(|steam_id, persona| {
		println!("{steam_id:?} -> {persona:?}");
	}));
	
	loop {
		sleep(dur(1. / 60.));
		call_manager.run_callbacks();
	}
}
