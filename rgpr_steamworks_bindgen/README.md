# RgPr Steamworks Bindgen
Binding generator specifically for the Steam API.
Utilizes header files and `steam_api.json`.

## Why?
`bindgen` fails to parse some of the Steam API due to the use C++ macros.
This crate was made as an extension of `bindgen` tailored specifically for the Steam API.

## Goals
- Pre-process headers.
- Separate generated items into modules.
- Link exported bindings to original header locations.


