use std::fmt::{Display, Formatter};
use std::os::raw::{c_uchar, c_void};
use std::os::unix::prelude::OsStrExt;
use std::path::PathBuf;

use libc::{fclose, fflush, FILE, fopen, fwrite, malloc, size_t};
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
    pub fn new<P: Into<PathBuf>>(path: P, mode: PcapWriterMode) -> Self {
        PcapNgWriter {
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
                PcapWriterMode::Write => fopen(path_bytes.as_ptr(), "wb\0".as_ptr()),
                PcapWriterMode::Append => fopen(path_bytes.as_ptr(), "a\0".as_ptr()),
            };

            if fh.is_null() {
                Err(FileOpenError)
            } else {
                match self.mode {
                    PcapWriterMode::Write => { libpcapng_write_header_to_file(fh); }
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
    use crate::{PcapNgWriter, PcapWriterMode};

    #[test]
    fn it_works() {
        let mut pcap_writer = PcapNgWriter::new("test.pcapng", PcapWriterMode::Write);
        pcap_writer.open().expect("issue opening file");
        pcap_writer.write_custom("this is a test".as_bytes().to_vec()).expect("issue writing custom frame");
        pcap_writer.close();
    }
}
