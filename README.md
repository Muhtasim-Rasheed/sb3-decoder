# `sb3-decoder`

A rust crate for decoding Scratch 3.0 project files (`.sb3`).
It provides functionality to read and parse the contents of `.sb3` files, which are ZIP archives containing JSON files and assets used in Scratch projects.

## Todo

- [ ] Add examples
- [x] Nicely support C-blocks
- [ ] Simplify parts of the API
- [ ] Add more documentation to the code

## Feature flags
The features `costumes` and `sounds` are enabled by default.

- `costumes`: costume decoding support, using image and resvg.
- `sounds`: sound decoding support, using hound and nanomp3.
