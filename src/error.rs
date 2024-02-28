use thiserror::Error;

/// Errors For this Crate
#[derive(Debug, Error)]
pub enum PcapNgError {
    /// Indicates there was an error opening the pcap file
    #[error("error opening file")]
    FileOpenError,
    /// Indicates there was an error closing the pcap file
    #[error("error closing file")]
    FileCloseError,
    /// Indicates that the file was never opened
    #[error("file has not been opened")]
    FileNotOpen,
    /// This error is raised if using a read operation on a pcap opened in write mode
    #[error("this operation is only supported in read mode")]
    OperationOnlySupportedInReadMode,
    /// This error is raised if using a write operation on a pcap opened in read mode
    #[error("this operation is only supported in write or append mode")]
    OperationOnlySupportedInWriteMode,
    /// A catch all for other unknown errors
    #[error("unknown error {0}")]
    UnknownError(String),
}

/// Pcapng Errors
pub type Error = PcapNgError;
/// Result which wraps a pcapng error
pub type Result<T> = std::result::Result<T, crate::Error>;