//! library for Rust based on [libpcapng].
//!
//! ## The library
//!
//! `libpcapng-rs` provides a Rust interface to [libpcapng](https://github.com/stricaud/libpcapng).
//!
//!
//! ### Features
//!
//! The main features provided at the moment are:
//!
//! - Create new PCAP file
//! - Append to existing PCAP file
//! - Write network packet frames with and without a timestamp
//! - Write custom frames
//! - Read frames from pcap
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! libpcapng_rs = { version = "0.1", features = ["static"] }
//! ```
//!
//! This crate will compile libpcapng from sources and link it statically to
//! your executable. To compile libpcapng you'll need:
//!
//! * the GNU toolchain
//! * GNU `make`
//! * `wandio`
//! * `pybind11`
//! * `cmake`
//!
//!
//! By default a submodule with the libpcapng sources will be used to compile
//! and statically link the library. The disabling default features allows
//! to instead dynamically link libpcapng-rs to the system's libpcapng shared object Example:
//!
//! ```toml
//! [dependencies]
//! libpcapng_rs = { version = "0.1", default-features = false }
//! ```
//! ## Features
//! `static` this feature statically compiles libpcapng c library in to the crate
//! `macos` this feature enables building on macos as opposed to linux as the native libc interfaces are a bit different
//!
//! ## Examples
//!
//! ### Example Code
//! ```rust
//! use libpcapng_rs::{PcapNg, PcapNgOpenMode};
//! use std::fs;
//!
//! fn main() {
//!     let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Write);
//!     pcap_writer.open().expect("issue opening file");
//!     pcap_writer.write_custom("this is a test".as_bytes().to_vec()).expect("issue writing custom frame");
//!     pcap_writer.close();
//!     let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Read);
//!     pcap_writer.open().expect("issue opening file");
//!     pcap_writer.read_packets(Some(callback_rs)).unwrap();
//!     pcap_writer.close();
//!     fs::remove_file("test.pcapng").unwrap();
//! }
//!
//! fn callback_rs(block_counter: u32, block_type: u32, block_total_length: u32, bytes: Vec<u8>) {
//!     println!("hello world");
//!     println!("block_counter: {}, block_type: {}, block_total_length: {} bytes {:02X?}", block_counter, block_type, block_total_length, bytes);
//! }
//! ```
//!
//! ### Example Output
//! ```text
//! hello world
//! block_counter: 1, block_type: 168627466, block_total_length: 28 bytes [4D, 3C, 2B, 1A, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 1C, 00, 00, 00]
//! hello world
//! block_counter: 2, block_type: 1, block_total_length: 20 bytes [65, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00]
//! hello world
//! block_counter: 3, block_type: 2989, block_total_length: 32 bytes [69, 7A, 00, 00, 74, 68, 69, 73, 20, 69, 73, 20, 61, 20, 74, 65, 73, 74, 00, 00, 20, 00, 00, 00]
//! ```
//!

#![forbid(missing_docs)]
#![deny(rust_2018_idioms)]
#![allow(clippy::type_complexity)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod pcapng;

pub use pcapng::*;

pub use error::*;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{PcapNg, PcapNgOpenMode};

    fn callback_rs(block_counter: u32, block_type: u32, block_total_length: u32, bytes: Vec<u8>) {
        println!("hello world");
        println!("block_counter: {}, block_type: {}, block_total_length: {} bytes {:02X?}", block_counter, block_type, block_total_length, bytes);
    }

    #[test]
    fn write_and_read_test() {
        let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Write);
        pcap_writer.open().expect("issue opening file");
        pcap_writer.write_custom("this is a test".as_bytes().to_vec()).expect("issue writing custom frame");
        pcap_writer.close();
        let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Read);
        pcap_writer.open().expect("issue opening file");
        pcap_writer.read_packets(Some(callback_rs)).unwrap();
        pcap_writer.close();
        fs::remove_file("test.pcapng").unwrap();
    }
}
