use std::fmt::{Display, Formatter};
use std::os::raw::{c_uchar, c_void};
use std::path::PathBuf;

use libc::{c_char, fclose, fflush, FILE, fopen, fwrite, malloc, size_t};
use thiserror::Error;

use libpcapng_sys::{libpcapng_custom_data_block_size, libpcapng_custom_data_block_write, libpcapng_write_enhanced_packet_to_file, libpcapng_write_enhanced_packet_with_time_to_file, libpcapng_write_header_to_file, PCAPNG_PEN};

use crate::PcapNgError::{FileNotOpen, FileOpenError};

#[derive(Debug, Error)]
pub enum PcapNgError {
    #[error("error opening file")]
    FileOpenError,
    #[error("error closing file")]
    FileCloseError,
    #[error("file has not been opened")]
    FileNotOpen,
    #[error("unknown error {0}")]
    UnknownError(String),
}

pub type Error = PcapNgError;
pub type Result<T> = std::result::Result<T, Error>;


pub struct PcapNgWriter {
    file_path: PathBuf,
    file_handle: Option<*mut FILE>,
    mode: PcapWriterMode,
}

impl PcapNgWriter {
    pub fn new<P: AsRef<PathBuf>>(path: P, mode: PcapWriterMode) -> Self {
        PcapNgWriter {
            file_path: path.to_owned(),
            file_handle: None,
            mode,
        }
    }

    pub fn open(&mut self) -> Result<()> {
        unsafe {
            let path = format!("{}\0", self.file_path.to_str().unwrap()).as_ptr() as *const c_char;
            let mode = format!("{}\0", self.mode).as_ptr() as *const c_char;
            let fh = fopen(path, mode);
            if fh.is_null() {
                Err(FileOpenError)
            } else {
                match self.mode {
                    PcapWriterMode::Write => libpcapng_write_header_to_file(fh),
                    PcapWriterMode::Append => (),
                }
                self.file_handle = Some(fh);
                Ok(())
            }
        }
    }

    pub fn write_custom(&mut self, data: Vec<u8>) -> Result<()> {
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
    pub fn write_packet_with_time(&mut self, data: Vec<u8>, timestamp: u32) -> Result<()> {
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

pub enum PcapWriterMode {
    Write,
    Append,
}

impl Display for PcapWriterMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PcapWriterMode::Write => write!(f, "{}", "wb".to_string()),
            PcapWriterMode::Append => write!(f, "{}", "a".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("hello world");
    }
}
