mod bindings;

use bindings::parse_and_eval;
use std::ffi::CStr;

// extern "C" {
//     pub fn parse_and_eval(maybe_cstr: *const ::std::os::raw::c_char, output: *mut i64) -> isize;
// }

fn main() {
    let text = b"4 3 +\0";
    let cstr = CStr::from_bytes_with_nul(text).unwrap();
    let mut output: i64 = 0;
    let ptr = cstr.as_ptr();

    let result = unsafe {
        parse_and_eval(ptr, &mut output)
    };

    if result == 0 {
        println!("Result: {}", output);
    }
}
