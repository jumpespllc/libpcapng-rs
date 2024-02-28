# API Wrapper for [libpcapng](https://github.com/stricaud/libpcapng)

## The library
`libpcapng-rs` provides a Rust interface to [libpcapng](https://github.com/stricaud/libpcapng).

## Features

The main features provided at the moment are:

- Create new PCAP file
- Append to existing PCAP file
- Write network packet frames with and without a timestamp
- Write custom frames
- Read frames from pcap

## Building

Add this to your `Cargo.toml`:

```toml
[dependencies]
libpcapng_rs = { version = "0.1", features = ["static"] }
```

This crate will compile libpcapng from sources and link it statically to
your executable. To compile libpcapng you'll need:

* the GNU toolchain
* GNU `make`
* `wandio`
* `pybind11`
* `cmake`

### MacOS
```
brew install wandio cmake pybind11
```

### Debian
```bash
sudo apt-get install build-essential cmake libwandio1 libwandio1-dev pybind11-dev python3-pybind11
```


# Example

## Cargo.toml
```toml
# ...
[dependencies]
libpcapng-rs = { version="0.1.3", features = ["static"] }
```
## main.rs
```rust
use libpcapng_rs::{PcapNg, PcapNgOpenMode};
use std::fs;

fn main() {
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

fn callback_rs(block_counter: u32, block_type: u32, block_total_length: u32, bytes: Vec<u8>) {
    println!("hello world");
    println!("block_counter: {}, block_type: {}, block_total_length: {} bytes {:02X?}", block_counter, block_type, block_total_length, bytes);
}
```

## Output 
```text
hello world
block_counter: 1, block_type: 168627466, block_total_length: 28 bytes [4D, 3C, 2B, 1A, 01, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 00, 1C, 00, 00, 00]
hello world
block_counter: 2, block_type: 1, block_total_length: 20 bytes [65, 00, 00, 00, 00, 00, 00, 00, 14, 00, 00, 00]
hello world
block_counter: 3, block_type: 2989, block_total_length: 32 bytes [69, 7A, 00, 00, 74, 68, 69, 73, 20, 69, 73, 20, 61, 20, 74, 65, 73, 74, 00, 00, 20, 00, 00, 00]
```