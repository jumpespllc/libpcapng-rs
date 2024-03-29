//! native bindings to  [libpcapng].
//!
//! ## The C library
//!
//! `libpcapng-sys` provides native bindings to [libpcapng](https://github.com/stricaud/libpcapng).
//!
//! ## Installation
//!
//! This crate is meant to be used by `libpcapng-rs` module and is just bindings to the c library
//!
//! ## Build Requirements
//!
//! * the GNU toolchain
//! * GNU `make`
//! * `wandio`
//! * `pybind11`
//! * `cmake`

#![forbid(missing_docs)]
#![allow(clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Create new bindings with bindgen
// bindgen \
// --allowlist-function=".*pcapng_.*" \
// --allowlist-var="PCAPNG_.*" wrapper.h  > src/bindings.rs
include!("bindings.rs");




#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use libc::{c_char, fopen};

    use crate::libpcapng_write_header_to_file;

    #[test]
    fn read_write_header() {
        unsafe {
            let x = "pcap.pcapng\0".as_ptr() as *const c_char;
            let outfh = fopen("pcap.pcapng\0".as_ptr(), "wb\0".as_ptr());
            libpcapng_write_header_to_file(outfh);
            let path = Path::new("pcap.pcapng");
            assert!(path.exists());
            fs::remove_file(path).expect("Unable to cleanuup afte test");
        }
    }
}
