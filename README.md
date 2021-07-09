# Calc Around The World

This is a WIP exercise for Ferrous Teaching Material, demonstrating:

* Making a native Rust library (the `calc/` crate)
* Exposing this library with a C FFI interface (the `calc-ffi` crate), to create a static library
* Using `cbindgen` to generate a C Header file from the public interface
* Using C to call the Rust static library (see `calc-ffi/demo.c`)
* Using `bindgen` to generate a Rust interface
* Calling the rust-pretending-to-be-C interfaces using FFI (see the `reverse-calc-ffi/` crate)

## License

Code included in this repo is licensed under the MIT License.

Prose/text/documentation included in this repo is licensed under the CC-BY-SA license.
