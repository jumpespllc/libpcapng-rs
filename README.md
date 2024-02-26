# Rust Bindings for [libpcapng](https://github.com/stricaud/libpcapng)

Under Construction...

## Modules

`libpcapng-rs` The main module for interfacing with

`libpcapng-sys` The bindings to the native c library

## Features

static: toggles whether to statically compile in libpcapng c library

# Example

```rust 
use libpcapng_rs::{PcapNgWriter, PcapWriterMode};

fn main() {
    let mut pcap_writer = PcapNgWriter::new("test.pcapng", PcapWriterMode::Write);
    pcap_writer.open().expect("issue opening file");
    pcap_writer.write_custom("this is a test".as_bytes().to_vec()).expect("issue writing custom frame");
    pcap_writer.close();
}
```