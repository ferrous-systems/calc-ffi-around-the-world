# Calc Around The World

This is a WIP exercise for Ferrous Teaching Material, demonstrating:

* Making a native Rust library (the `calc/` crate)
* Exposing this library with a C FFI interface (the `calc-ffi` crate), to create a static library
* Using `cbindgen` to generate a C Header file from the public interface
* Using C to call the Rust static library (see `calc-ffi/demo.c`)
* Using `bindgen` to generate a Rust interface
* Calling the rust-pretending-to-be-C interfaces using FFI (see the `reverse-calc-ffi/` crate)

## Instructions (mostly for the trainer)

### Step Zero: Prerequisites
make sure everyone knows:
- what is an ABI?
- why do we need a stable ABI when we interop with other languages?
- what does FFI mean?

intro could go something like this:
- an ABI describes how a program is organized *at the binary level* (e.g. when I call a function, which register holds which parameter, how are they ordered, how do I construct my call stack)
- Rust's ABI isn't stable yet *on purpose* because that might lock us out of performance benefits in the future
- opinions vary in C++ ABI's stability (e.g. Android considers its unstable!)
- C ABI is "set in stone" because it's so old -> the de-facto "lingua franca"
    - we use that when interoperating with other languages so everyone agrees on which register holds which function paramters, etc
    - *but* it is really limited!
- FFI = "Foreign Function Interface"
    - can be between any language!
    - but the C ABI is the glue, usually (e.g. you might translate Rust -> C ABI -> Go)

### Step One: Start with the calc crate

Here, there are a couple of important things to point out:

* Make sure we have the relevant items made "public"
* Talk about the inputs and outputs of each function signature

### Step Two: Making an `ffi` crate

A common practice I like to have is to have a separate crate, which ONLY exposes the C function interface (and exports relevant types), because this keeps the "just Rust" crate cleaner.

This step is discussing that stage.

The important pieces of information to hit are:

* Making a new crate, e.g. `calc-ffi`
* Configuring the `Cargo.toml` to build a static library
    * Talk about the different kinds of crates Rust can build, including:
        * Static libraries (e.g. `.a` files) - `crate-type = ["staticlib"]`
        * Dynamic libs (e.g. `.dylib` or `.so` or `.dll`) - `crate-type = ["cdylib"]`
    * Note that you can actually build both!
* Start with a "do everything at once" approach, where you pass a string in, and get a status code + result i64 out
    * NOTE: We do this first to avoid "opaque structs" and allocation concerns!
    * Talk about what types are available to you for FFI, particularly comparisons with `stdint.h` types (e.g. `intptr_t` vs `isize` vs `int`, etc.)
    * Talk about items like `CStr`/`CString`, and how they map to (and are different from!) the main string types
    * Talk about not having `Result` types, and instead using `outptr`s and status codes
    * Talk about panicking across FFI boundaries, and how you should catch panics.
    * Talk to students about `extern "C"` and `#[no_mangle]`, and why they are necessary for FFI with C
    * Talk about why FFI functions need to be `unsafe`, wrt pointers
    * Have the students try and make a function signature manually for the "all in one parse+eval" function, and putting it in a header file
    * Have them try to compile and run this with a C file (see [demo.c](./calc-ffi/demo.c) for GCC commands and sample code)

### Step Three: Using `cbindgen`

* Have the students use `cbindgen` to generate the header file
* Explain the output they see
* See [build.sh](./calc-ffi/build.sh) for relevant syntax

### Step Four: More complex FFI

* Once that is successful, then try moving on to exposing separate `parse` and `eval` steps
    * Talk about "opaque types" that can be used as pointers in C, but can not be instantiated (they are basically `!Sized`, which C can't handle)
    * Talk about allocation and ownership, and why Boxing (and leaking) is necessary (at least in this case)
    * Show that Rust sometimes needs to "hand back" things that C can't understand, to use later
    * Talk about why you should have a `free_struct()` function that is used to re-box and free the memory, rather than just calling `free()` directly from C
    * Have the students write a public set of functions, like:
        * parse
        * eval
        * free expr
    * Have the students test their code from C

### Step Five: Going the other way

Instead of writing a C library to link with a Rust-calling-C sort of workflow, we immediately turn around and use our "Rust with C FFI interfaces" library + header file, and call it from Rust.

* Have the students write an `extern "C"` method declaration (manually) from Rust
* Have the students configure Cargo to also link against the static library they generated. There are multiple ways to do this:
    * RUSTFLAGS env var
    * In a Build script (See [this file](./reverse-calc-ffi/__build.rs))
    * In a `.cargo/config` (See [this file](./reverse-calc-ffi/.cargo/config.toml)])
* Make sure they can call the library from their Rust code

### Step Six: Bindgen

* Introduce the `bindgen` tool and what it is used for
* Use `bindgen` to generate Rust declarations for the C header we generated with `cbindgen`
    * Note how verbose this is
    * Describe typical practices of putting this into a `lib.rs` or a `bindings.rs`
    * Describe "bindgen as a lib" vs "bindgen as an application", and the tradeoffs around this
    * Note the generated tests, which are useful for checking portability, recommend running them
    * See [this file](./reverse-calc-ffi/generate_bindings.sh) for relevant syntax

### Step Seven: Closing the loop

* Have the students use their bindgen-generated prototypes to call the `parse` and `eval` steps separately
* Confirm the logic still works
* Make sure you instill the fear of "C can't understand Rust's guarantees, and your unsafe code should expect this!"

## License

Code included in this repo is licensed under the MIT License.

Prose/text/documentation included in this repo is licensed under the CC-BY-SA license.
