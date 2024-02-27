use std::fmt::{Display};
use std::mem::transmute;
use std::os::raw::{c_int, c_uchar, c_void};
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;
use std::ptr::null_mut;

use libc::{c_char, fclose, fflush, FILE, fopen, fwrite, malloc, size_t};
use thiserror::Error;

use libpcapng_sys::{libpcapng_custom_data_block_size, libpcapng_custom_data_block_write, libpcapng_fp_read, libpcapng_write_enhanced_packet_to_file, libpcapng_write_enhanced_packet_with_time_to_file, libpcapng_write_header_to_file, PCAPNG_PEN};

use crate::PcapNgError::{FileNotOpen, FileOpenError, OperationOnlySupportedInReadMode, OperationOnlySupportedInWriteMode};

#[derive(Debug, Error)]
pub enum PcapNgError {
    #[error("error opening file")]
    FileOpenError,
    #[error("error closing file")]
    FileCloseError,
    #[error("file has not been opened")]
    FileNotOpen,
    #[error("this operation is only supported in read mode")]
    OperationOnlySupportedInReadMode,
    #[error("this operation is only supported in write or append mode")]
    OperationOnlySupportedInWriteMode,
    #[error("unknown error {0}")]
    UnknownError(String),
}

pub type VoidPtr = *mut c_void;
pub type CbFn = fn(u32, u32, u32, Vec<u8>);
pub type Error = PcapNgError;
pub type Result<T> = std::result::Result<T, Error>;


pub struct PcapNg {
    file_path: PathBuf,
    file_handle: Option<*mut FILE>,
    mode: PcapNgOpenMode,
}

impl PcapNg {
    pub fn new<P: Into<PathBuf>>(path: P, mode: PcapNgOpenMode) -> Self {
        PcapNg {
            file_path: path.into(),
            file_handle: None,
            mode,
        }
    }

    pub fn open(&mut self) -> Result<()> {
        unsafe {
            let mut path_bytes = self.file_path.as_os_str().as_bytes().to_vec();
            path_bytes.push(0);
            let fh = match self.mode {
                PcapNgOpenMode::Write => fopen(path_bytes.as_ptr() as *const i8, "wb\0".as_ptr() as *const c_char),
                PcapNgOpenMode::Append => fopen(path_bytes.as_ptr() as *const i8, "a\0".as_ptr() as *const c_char),
                PcapNgOpenMode::Read => fopen(path_bytes.as_ptr() as *const i8, "r\0".as_ptr() as *const c_char),
            };

            if fh.is_null() {
                Err(FileOpenError)
            } else {
                match self.mode {
                    PcapNgOpenMode::Write => { libpcapng_write_header_to_file(fh); }
                    _ => (),
                }
                self.file_handle = Some(fh);
                Ok(())
            }
        }
    }


    pub fn write_custom(&mut self, data: Vec<u8>) -> Result<()> {
        if self.mode == PcapNgOpenMode::Read {
            return Err(OperationOnlySupportedInWriteMode);
        }
        let data_len = data.len() as size_t;
        unsafe {
            let buffer_size = libpcapng_custom_data_block_size(data_len);
            let buffer = malloc(buffer_size) as *mut c_uchar;
            let data_bytes = data.as_ptr();
            libpcapng_custom_data_block_write(PCAPNG_PEN, data_bytes, data_len, buffer);
            if let Some(fh) = self.file_handle {
                fwrite(buffer as *const c_void, buffer_size, 1, fh);
                Ok(())
            } else {
                Err(FileNotOpen)
            }
        }
    }
    pub fn write_packet(&mut self, data: Vec<u8>) -> Result<()> {
        if self.mode == PcapNgOpenMode::Read {
            return Err(OperationOnlySupportedInWriteMode);
        }
        let data_len = data.len() as size_t;
        let data_bytes = data.as_ptr() as *mut c_uchar;
        unsafe {
            if let Some(fh) = self.file_handle {
                libpcapng_write_enhanced_packet_to_file(fh, data_bytes, data_len);
                Ok(())
            } else {
                Err(FileNotOpen)
            }
        }
    }

    pub fn read_packets(&mut self, callback_fn: Option<CbFn>) -> Result<()> {
        if self.mode != PcapNgOpenMode::Read {
            return Err(OperationOnlySupportedInReadMode);
        }
        unsafe {
            if let Some(fh) = self.file_handle {
                //let buffer = malloc(10) as *mut c_void;
                let callback_function = callback_fn.map(|callback_function| { transmute::<CbFn, VoidPtr>(callback_function) }).unwrap_or(null_mut());
                libpcapng_fp_read(fh, Some(callback), callback_function);
                Ok(())
            } else {
                Err(FileNotOpen)
            }
        }
    }
    pub fn write_packet_with_time(&mut self, data: Vec<u8>, timestamp: u32) -> Result<()> {
        if self.mode == PcapNgOpenMode::Read {
            return Err(OperationOnlySupportedInWriteMode);
        }
        let data_len = data.len() as size_t;
        let data_bytes = data.as_ptr() as *mut c_uchar;
        unsafe {
            if let Some(fh) = self.file_handle {
                libpcapng_write_enhanced_packet_with_time_to_file(fh, data_bytes, data_len, timestamp);
                Ok(())
            } else {
                Err(FileNotOpen)
            }
        }
    }

    pub fn close(&mut self) {
        unsafe {
            if let Some(fh) = self.file_handle {
                fflush(fh);
                fclose(fh);
            }
        }
    }
}


#[no_mangle]
unsafe extern "C" fn callback(block_counter: u32, block_type: u32, block_total_length: u32, data: *mut c_uchar, userdata: *mut c_void) -> c_int {
    let bytes: Vec<u8> = std::slice::from_raw_parts(data, block_total_length as usize - 8).iter().map(|x| { x.to_owned() as u8 }).collect();
    if let Some(fn_ptr) = userdata.as_mut() {
        let cb = transmute::<VoidPtr, CbFn>(fn_ptr);
        cb(block_counter, block_type, block_total_length, bytes.to_owned());
    }
    0
}

#[derive(Debug, Eq, PartialEq)]
pub enum PcapNgOpenMode {
    Write,
    Append,
    Read,
}

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
