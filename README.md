# Rust Bindings for [libpcapng](https://github.com/stricaud/libpcapng)

Under Construction...

## Modules

`libpcapng-rs` The main module for interfacing with

`libpcapng-sys` The bindings to the native c library

## Features

static: toggles whether to statically compile in libpcapng c library

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