use std::intrinsics::transmute;
use std::os::raw::{c_int, c_uchar, c_void};
use std::path::PathBuf;
use std::ptr::null_mut;
use std::os::unix::prelude::OsStrExt;
#[cfg(not(feature = "debian"))]
use libc::c_char;
use libc::{fclose, fflush, FILE, fopen, fwrite, malloc, size_t};
use libpcapng_sys::{libpcapng_custom_data_block_size, libpcapng_custom_data_block_write, libpcapng_fp_read, libpcapng_write_enhanced_packet_to_file, libpcapng_write_enhanced_packet_with_time_to_file, libpcapng_write_header_to_file, PCAPNG_PEN};
use crate::PcapNgError::{FileOpenError, FileNotOpen, OperationOnlySupportedInReadMode, OperationOnlySupportedInWriteMode};

/// Type for casting callback function a mutable void pointer
pub type VoidPtr = *mut c_void;

/// Type for casting a void pointer to the callback function signature
pub type CbFn = fn(u32, u32, u32, Vec<u8>);


/// A struct which provides an interface to interact with the libpcapng functions in a cohesive way
pub struct PcapNg {
    file_path: PathBuf,
    file_handle: Option<*mut FILE>,
    mode: PcapNgOpenMode,
}

impl PcapNg {

    /// The constructor
    pub fn new<P: Into<PathBuf>>(path: P, mode: PcapNgOpenMode) -> Self {
        PcapNg {
            file_path: path.into(),
            file_handle: None,
            mode,
        }
    }

    /// Opens the pcap file
    pub fn open(&mut self) -> crate::Result<()> {
        unsafe {
            let mut path_bytes = self.file_path.as_os_str().as_bytes().to_vec();
            path_bytes.push(0);
            let fh = match self.mode {
                #[cfg(feature="debian")]
                PcapNgOpenMode::Write => fopen(path_bytes.as_ptr(), "wb\0".as_ptr()),
                #[cfg(not(feature="debian"))]
                PcapNgOpenMode::Write => fopen(path_bytes.as_ptr() as *const i8, "wb\0".as_ptr() as *const c_char),
                #[cfg(feature="debian")]
                PcapNgOpenMode::Append => fopen(path_bytes.as_ptr(), "a\0".as_ptr()),
                #[cfg(not(feature="debian"))]
                PcapNgOpenMode::Append => fopen(path_bytes.as_ptr() as *const i8, "a\0".as_ptr() as *const c_char),
                #[cfg(feature="debian")]
                PcapNgOpenMode::Read => fopen(path_bytes.as_ptr(), "r\0".as_ptr()),
                #[cfg(not(feature="debian"))]
                PcapNgOpenMode::Read => fopen(path_bytes.as_ptr() as *const i8, "r\0".as_ptr() as *const c_char),
            };

            if fh.is_null() {
                Err(FileOpenError)
            } else {
                if self.mode == PcapNgOpenMode::Write { libpcapng_write_header_to_file(fh); }
                self.file_handle = Some(fh);
                Ok(())
            }
        }
    }

    /// Write a custom frame to the pcap
    pub fn write_custom(&mut self, data: Vec<u8>) -> crate::Result<()> {
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

    /// Writes a packet frame to the pcap
    pub fn write_packet(&mut self, data: Vec<u8>) -> crate::Result<()> {
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

    /// Reads all the frames from a pcap passing them to the callback function provided
    pub fn read_packets(&mut self, callback_fn: Option<CbFn>) -> crate::Result<()> {
        if self.mode != PcapNgOpenMode::Read {
            return Err(OperationOnlySupportedInReadMode);
        }
        unsafe {
            if let Some(fh) = self.file_handle {
                let callback_function = callback_fn.map(|callback_function| { callback_function as *mut libc::c_void }).unwrap_or(null_mut());
                libpcapng_fp_read(fh, Some(callback), callback_function);
                Ok(())
            } else {
                Err(FileNotOpen)
            }
        }
    }

    /// Writes a packet to the pcap including the timestamp
    pub fn write_packet_with_time(&mut self, data: Vec<u8>, timestamp: u32) -> crate::Result<()> {
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

    /// Close the open file handle
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
    let bytes: Vec<u8> = std::slice::from_raw_parts(data, block_total_length as usize - 8).iter().map(|x| { x.to_owned() }).collect();
    if let Some(fn_ptr) = userdata.as_mut() {
        let cb = transmute::<VoidPtr, CbFn>(fn_ptr);
        cb(block_counter, block_type, block_total_length, bytes.to_owned());
    }
    0
}

/// The mode for opening the pcap file
#[derive(Debug, Eq, PartialEq)]
pub enum PcapNgOpenMode {
    /// This mode opens in write mode and will write the header to the beginning since it will be a new pcap file
    Write,
    /// Opens in write mode to append to an existing pcap file
    Append,
    /// Opens in read mode to read from pcap
    Read,
}