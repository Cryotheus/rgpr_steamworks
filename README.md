# RgPr Steamworks
High-level Rust bindings to [Steamworks](https://partner.steamgames.com/), the Steam API[¹](#notes).

This is purely for Steam apps that require integration with the Steam client and not the Steam Web API.

### Progress to Alpha Release
Currently in pre-alpha.  
When this table is all of the features in this table are near 100%,
an alpha release will become available on [crates.io](https://crates.io).

| Feature                      | Estimate |
|------------------------------|----------|
| `steam_apps`                 | 100%     |
| `steam_client`               |          |
| `steam_friends`              | 50%      |
| `steam_game_coordinator`     |          |
| `steam_game_server`          |          |
| `steam_game_server_stats`    |          |
| `steam_html`                 |          |
| `steam_http`                 |          |
| `steam_input`                |          |
| `steam_inventory`            |          |
| `steam_networking`           |          |
| `steam_match_making`         |          |
| `steam_match_making_servers` |          |
| `steam_music_remote`         |          |
| `steam_music`                |          |
| `steam_parental_settings`    |          |
| `steam_parties`              |          |
| `steam_remote_play`          |          |
| `steam_remote_storage`       |          |
| `steam_screenshots`          |          |
| `steam_timeline`             |          |
| `steam_ugc`                  |          |
| `steam_user`                 |          |
| `steam_user_stats`           |          |
| `steam_utils`                |          |
| `steam_video`                |          |

## Goals
This crate follows several standards to ensure quality, all of which are listed below.

### Full Coverage
All of the Steam API's provided features should be made available.  
Support will always be offered for, and only for, the latest release of the Steam API.  

The game server features of the Steam API should also be accessible.
The interfaces they share with the client version of the Steam API should not be accessed any differently.
Game server exclusive interfaces are accessed in a similar way to client exclusive interfaces.

### Simple
New Rust users should not struggle to use the crate.

### Modular
Manifest features allow conditional compilation to reduce compile times, and binary sizes.

### Safety
Anything that can be safe, is safe.  
Same for `Send + Sync`.
Naturally unsafe features will be given lite/pass-through wrappers.  

### C-phobic
The crate should not naturally expose `repr(C)` types, or `extern "C"` functions.
All exported data types are their native representations.

### Async
Asynchronous Steam API calls are provided as futures.
This includes the Steam API's "call results" and functions that are dependent on asynchronous behavior.

### Platform Agnostic
Behavior should be consistent across all platforms supported by the Steam API.


- Support the API for game servers ⁴

² Use the `sys` feature for a re-export of `rgpr_steamworks_sys` as `rgpr_steamworks::sys`.  
³ Mac requires the bindings in [rgpr_steamworks_sys](rgpr_steamworks_sys) to be generated. Both Steam Deck and MacOs are untested.  
⁴ A major version bump will likely be necessary.  

## Notes
1. This crate uses the names "_Steam API_" and "_Steamworks_" interchangeably, "_Steam API_" is most often used.  
The publicly released headers refer to the Steam API under the following names:
	- Steam API ([`steam_api.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/steam/steam_api.h))
    - Steamworks ([`isteamnetworkingutils.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/steam/isteamnetworkingutils.h))
    - Steamworks API ([`isteamclient.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/steam/isteamclient.h))
    - Steamworks SDK ([`isteamclient.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/steam/isteamclient.h))

# License
This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `rgpr_steamworks` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
