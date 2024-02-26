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
    use libc::fopen;
    use std::path::Path;
    use crate::{libpcapng_fp_read, libpcapng_write_header_to_file};

    #[test]
    fn read_write_header() {
        unsafe {
            let outfh = fopen("pcap.pcapng\0".as_ptr(), "wb\0".as_ptr());
            libpcapng_write_header_to_file(outfh);
            let path = Path::new("pcap.pcapng");
            assert!(path.exists());
            fs::remove_file(path).expect("Unable to cleanuup afte test");
        }
    }
}
