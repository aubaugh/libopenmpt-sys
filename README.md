libopenmpt-sys
==============

Rust raw FFI bindings for libopenmpt.

To avoid long build times, the finished bindings are committed to this repository. If you would prefer to generate the bindings at build time, there is a 'generate-bindings' feature to do so.

## Dependencies
The libopenmpt shared library

## Regenerating bindings
To regenerate the bindings: ensure you have the development headers for libopenmpt installed and run `./regenerate_bindings.sh`.

This will generate the bindings with the build script, and run the regenerate_bindings binary, which copies the generated bindings into src/.

## Example
To play a module with the use of the 'cpal' crate. Run `cargo run --example play <mod file path>`

## License
BSD-3-Clause