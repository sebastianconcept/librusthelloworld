#![deny(unsafe_code)]
use std::ffi::CString;

use safer_ffi::{ffi_export, prelude::char_p};

#[ffi_export]
pub fn get_hello_world() -> char_p::Box {
    let content = "Hello World from the Rust side!";
    let value = CString::new(content).ok().unwrap();
    char_p::Box::try_from(value).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[::safer_ffi::cfg_headers]
#[test]
fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("target/debug/librusthashmap.h")?
        .generate()
}
