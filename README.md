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

```rust
use libpcapng_rs::{PcapNg, PcapNgOpenMode};

fn main() {
    let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Write);
    pcap_writer.open().expect("issue opening file");
    pcap_writer.write_custom("this is a test".as_bytes().to_vec()).expect("issue writing custom frame");
    pcap_writer.close();
    let mut pcap_writer = PcapNg::new("test.pcapng", PcapNgOpenMode::Read);
    pcap_writer.open().expect("issue opening file");
    pcap_writer.read_packats(Some(callback_rs)).unwrap();
    pcap_writer.close();
    fs::remove_file("test.pcapng").unwrap();
}

fn callback_rs(block_counter: u32, block_type: u32, block_total_length: u32, bytes: Vec<u8>) {
    println!("hello world");
    println!("block_counter: {}, block_type: {}, block_total_length: {} bytes {:02X?}", block_counter, block_type, block_total_length, bytes);
}
```