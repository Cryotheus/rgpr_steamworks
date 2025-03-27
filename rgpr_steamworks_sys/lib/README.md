# Foreign Libraries
External source code and binaries used by `rgpr_steamworks`.

As of right now, only the headers and redistributable binaries from the Steamworks SDK are here.
This will likely remain the case even into the future.

When updating, make sure to re-target the `#include` pre-processors in `steam_api_flat.h` to use the parent folder `headers` instead of `steam`.
