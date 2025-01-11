# RgPr Steamworks

High-level Rust bindings to [Steamworks](https://partner.steamgames.com/), the Steam API.

## Goals

- Full Steam API coverage (deprecated API features are not included)
- Simple enough for new Rust users to pickup
- Feature-gated modularity to save compile times and binary sizes
- Make anything that can be safe, safe ¹
- No export of C types or functions ²
- Futures for asynchronous API calls (when awaiting the Steam API's "call results")
- Always support the latest release of the Steam API, ASAP
- Support all of the Steam API's targets ³ ⁴
- Support the API for game servers ⁴

¹ Wrappers or lite bindings to unsafe functionality will always be provided if safety is not possible.  
² Use the `sys` feature for a re-export of `rgpr_steamworks_sys` as `rgpr_steamworks::sys`.  
³ Mac requires the bindings in [rgpr_steamworks_sys](rgpr_steamworks_sys) to be generated. Both Steam Deck and MacOs are untested.  
⁴ A major version bump may be necessary.  


# License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or  https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `rgpr_steamworks` by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
