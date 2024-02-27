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
