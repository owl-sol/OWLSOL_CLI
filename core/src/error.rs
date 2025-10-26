use thiserror::Error;

pub type Result<T> = std::result::Result<T, CompressionError>;

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Compression failed: {0}")]
    CompressionFailed(String),

    #[error("Decompression failed: {0}")]
    DecompressionFailed(String),

    #[error("Unsupported algorithm: {0}")]
    UnsupportedAlgorithm(String),

    #[error("Invalid metadata: {0}")]
    InvalidMetadata(String),

    #[error("Data too large: {0} bytes (max: {1} bytes)")]
    DataTooLarge(usize, usize),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Insufficient data for decompression")]
    InsufficientData,

    #[error("Corrupted data: {0}")]
    CorruptedData(String),
}

impl CompressionError {
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    pub fn compression_failed<S: Into<String>>(msg: S) -> Self {
        Self::CompressionFailed(msg.into())
    }

    pub fn decompression_failed<S: Into<String>>(msg: S) -> Self {
        Self::DecompressionFailed(msg.into())
    }
}
