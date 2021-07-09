fn main() {
    println!("cargo:rustc-flags=-l calc_ffi -L ./../calc-ffi/target/debug");
}
