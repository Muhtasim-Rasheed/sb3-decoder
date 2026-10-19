# `sb3-decoder`

A rust crate for decoding Scratch 3.0 project files (`.sb3`).
It provides functionality to read and parse the contents of `.sb3` files, which are ZIP archives containing JSON files and assets used in Scratch projects.

## Todo

- [ ] Add examples
- [ ] Nicely support C-blocks
- [ ] Simplify parts of the API
- [ ] Add more documentation to the code

## Feature flags

- `costume_png`: Rasterizes SVG costumes to PNG using `resvg`. Re-exports the `image` crate.
- `costume_svg`: Parses SVG costumes using `usvg`. Re-exports the `usvg` crate.

**Note**: Enabling both `costume_png` and `costume_svg` will cause a compile-time error. You must enable only one of them.\
**Note**: Enabling none of these features will also cause a compile-time error. You must enable one of them.
