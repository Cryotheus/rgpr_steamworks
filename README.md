# RgPr Steamworks

High-level Rust bindings to [Steamworks](https://partner.steamgames.com/), the Steam API[¹](#notes).  
*Using Steamworks SDK v1.62*

> The Steamworks SDK provides a range of features which are designed to help ship your application or game on Steam in an efficient manner.  
> \- [Steamworks Documentation](https://partner.steamgames.com/doc/sdk)

Headers and binaries are in the [rgpr_steamworks_sys](rgpr_steamworks_sys) crate.
This is purely for Steam apps that require integration with the Steam client and not the Steam Web API.

### Progress to Alpha Release

Currently in pre-alpha.  
When all of the features in this table are near 100%,
an alpha release will become available on [crates.io](https://crates.io).

_
- Full coverage for non-game-server Steam API

| Feature                      | Estimate |
|------------------------------|:--------:|
| `steam_apps`                 |   100%   |
| `steam_encrypted_app_ticket` |   100%   |
| `steam_friends`              |   75%    |
| `steam_game_coordinator`     |    -     |
| `steam_html`                 |    -     |
| `steam_http`                 |    -     |
| `steam_input`                |    -     |
| `steam_inventory`            |    -     |
| `steam_networking`           |    -     |
| `steam_match_making`         |    -     |
| `steam_match_making_servers` |    -     |
| `steam_music_remote`         |    -     |
| `steam_music`                |    -     |
| `steam_parental_settings`    |    -     |
| `steam_parties`              |    -     |
| `steam_remote_play`          |    -     |
| `steam_remote_storage`       |    -     |
| `steam_screenshots`          |    -     |
| `steam_timeline`             |    -     |
| `steam_ugc`                  |    -     |
| `steam_user`                 |    -     |
| `steam_user_stats`           |    -     |
| `steam_utils`                |   90%    |
| `steam_video`                |    -     |
| `sys`                        |   100%   |

### After Alpha
- Full coverage for game server Steam API
- Multiplexed gameservers (via `isteamclient.h`)

| Feature                      | Estimate |
|------------------------------|:--------:|
| `steam_client`               |    -     |
| `steam_game_server`          |    -     |
| `steam_game_server_stats`    |    -     |

## Why

I'd like to one day make a game worth putting on the Steam store.
Over christmas of 2024, when I had the time to spare, I started working on integrating an existing project with Steamworks.
The previous crate I used, despite being popular, had many stability and memory issues.
I ran into memory leaks, crashes, a lack of API coverage, call results treated like callbacks, and many anti-patterns.
After modifying the code several times over - I decided it would be best to just start from scratch.

## Goals

This crate follows several standards to ensure quality, all of which are listed below.

### Full Coverage

All of the Steam API's provided features should be made available.  
Support will always be offered for, and only for, the latest release of the Steam API.

The game server features of the Steam API should also be accessible.
The interfaces they share with the client version of the Steam API should not be accessed any differently.
Game server exclusive interfaces are accessed in a similar way to client exclusive interfaces.

### Modular

Manifest features allow conditional compilation to reduce compile times, and binary sizes.

### Safety

Anything that can be safe, is safe.  
Same for `Send + Sync`.
Naturally unsafe features will be given lite/pass-through wrappers.

### C-phobic

The crate should not naturally expose `repr(C)` types, or `extern "C"` functions.
All exported data types are their native (Rust) representations.

### Async

Asynchronous Steam API calls are provided as futures.
This includes the Steam API's "call results" and functions that are dependent on asynchronous behavior.

### Platform Agnostic

The behavior of safe functionality should be consistent across all platforms supported by the Steam API.

## Notes

1. This crate uses the names "_Steam API_" and "_Steamworks_" interchangeably, "_Steam API_" is most often used.  
   The publicly released headers refer to the Steam API under the following names:
	- Steam API ([`steam_api.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/steam_api.h))
	- Steamworks ([`isteamnetworkingutils.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/isteamnetworkingutils.h))
	- Steamworks API ([`isteamclient.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/isteamclient.h))
	- Steamworks SDK ([`isteamclient.h`](rgpr_steamworks_sys/lib/steamworks_sdk/headers/isteamclient.h))

Both Steam Deck and MacOs are untested.
Mac requires the bindings in [rgpr_steamworks_sys](rgpr_steamworks_sys) to be generated.

# Building

Use `cargo run` and `cargo build` as you normally would.

When shipping your binaries, make sure to include the binaries in [steamworks_sdk/\<target os\>/\<target arch\>](rgpr_steamworks_sys/lib/steamworks_sdk).

For example,

- On 64bit Windows: [`rgpr_steamworks_sys/lib/steamworks_sdk/windows/64/steam_api64.dll`](rgpr_steamworks_sys/lib/steamworks_sdk/windows/64)
- On 32bit Linux: [`rgpr_steamworks_sys/lib/steamworks_sdk/windows/32/libsteam_api.dll`](rgpr_steamworks_sys/lib/steamworks_sdk/windows/32)

You will always need `libsteam_api` / `steam_api` / `steam_api64`.

If the `steam_encrypted_app_ticket` feature is enabled, make sure to include the `libsdkencryptedappticket` / `sdkencryptedappticket` / `sdkencryptedappticket64` binaries as well.

# Executable won't run?

See [Building](#building).

# License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Disclaimer

This project, `rgpr_steamworks`, is not affiliated with, endorsed by, or associated with Valve Corporation.
All trademarks, service marks, and company names are the property of their respective owners.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `rgpr_steamworks` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
