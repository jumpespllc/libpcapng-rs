use thiserror::Error;

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

pub type Error = PcapNgError;
pub type Result<T> = std::result::Result<T, crate::Error>;