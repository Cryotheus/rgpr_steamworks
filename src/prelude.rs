#![allow(unused_imports)]
use cfg_if::cfg_if;

pub use crate::dt::{AppId, SteamId};

cfg_if! {
	if #[cfg(feature = "steam")] {
		pub use crate::call::Callback;
		pub use crate::config::SteamBuilder;
		pub use crate::steam::Steam;
	}
}

#[cfg(feature = "steam_apps")]
pub use crate::interfaces::apps::AppsInterface;

#[cfg(feature = "steam_friends")]
pub use crate::interfaces::friends::FriendsInterface;

#[cfg(feature = "steam_utils")]
pub use crate::interfaces::utils::UtilsInterface;
