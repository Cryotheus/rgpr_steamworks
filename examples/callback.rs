use rgpr_steamworks::interfaces::friends::{PersonaChange, PersonaStateChange};
use rgpr_steamworks::prelude::*;

/// Solely used to uniquely identify callbacks.
/// The type used could be anything -
struct Id<const I: usize>;

fn main() {
	let steam = SteamBuilder::new(480).with_dev().build().unwrap();
	let mut call_manager = steam.call_manager_lock();

	// Register a closure or function - both work
	call_manager.listen::<PersonaStateChange, Id<0>>(|steam_id, change| println!("this will get replaced anyways"));
	call_manager.listen::<PersonaStateChange, Id<0>>(persona_listener);
	
	//drop it before we sleep - otherwise the call manager won't be run!
	drop(call_manager);

	//wait for keyboard input (line feed from stdin)
	timeout();
	drop(steam); //explicit drop for significant drop - not needed in your code
}

fn persona_listener(steam_id: SteamId, change: PersonaChange) {
	println!("hello from function {steam_id} {change:?}");
}

// Wait for an input of ENTER.
fn timeout() {
	let mut buffer = Vec::new();

	std::io::BufRead::read_until(&mut std::io::stdin().lock(), b'\n', &mut buffer).unwrap();
}
