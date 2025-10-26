use serde::{Deserialize, Serialize};

pub const COMPRESSION_VERSION: u8 = 1;
pub const MAX_DATA_SIZE: usize = 10 * 1024 * 1024; // 10MB
pub const MIN_COMPRESSION_THRESHOLD: f64 = 0.95; // Only compress if saves 5%+

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum CompressionAlgorithm {
    None = 0,
    Huffman = 1,
    Dictionary = 2,
    RunLength = 3,
    Hybrid = 4,
    Lz4 = 5,
    Zstd = 6,
}

impl CompressionAlgorithm {
    pub fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Huffman),
            2 => Some(Self::Dictionary),
            3 => Some(Self::RunLength),
            4 => Some(Self::Hybrid),
            5 => Some(Self::Lz4),
            6 => Some(Self::Zstd),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::Huffman => "Huffman",
            Self::Dictionary => "Dictionary",
            Self::RunLength => "RLE",
            Self::Hybrid => "Hybrid",
            Self::Lz4 => "LZ4",
            Self::Zstd => "Zstd",
        }
    }

    pub fn to_u8(&self) -> u8 {
        *self as u8
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionMetadata {
    pub version: u8,
    pub algorithm: CompressionAlgorithm,
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub timestamp: i64,
    pub checksum: u32, // Simple checksum for data integrity
}

impl CompressionMetadata {
    pub fn new(algo: CompressionAlgorithm, orig: u64, comp: u64) -> Self {
        Self {
            version: COMPRESSION_VERSION,
            algorithm: algo,
            original_size: orig,
            compressed_size: comp,
            compression_ratio: if orig > 0 {
                comp as f64 / orig as f64
            } else {
                0.0
            },
            timestamp: chrono::Utc::now().timestamp(),
            checksum: 0, // Will be calculated separately
        }
    }

    pub fn with_checksum(mut self, checksum: u32) -> Self {
        self.checksum = checksum;
        self
    }

    pub fn compression_percentage(&self) -> f64 {
        (1.0 - self.compression_ratio) * 100.0
    }

    pub fn space_saved(&self) -> u64 {
        self.original_size.saturating_sub(self.compressed_size)
    }

    pub fn is_worthwhile(&self) -> bool {
        self.compression_ratio < MIN_COMPRESSION_THRESHOLD
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(self)
    }

    pub fn from_bytes(b: &[u8]) -> Result<Self, serde_json::Error> {
        serde_json::from_slice(b)
    }

    pub fn validate(&self) -> bool {
        self.version == COMPRESSION_VERSION
            && self.original_size > 0
            && self.compressed_size > 0
            && self.compression_ratio >= 0.0
    }
}

#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub data: Vec<u8>,
    pub metadata: CompressionMetadata,
}

impl CompressionResult {
    pub fn new(data: Vec<u8>, metadata: CompressionMetadata) -> Self {
        Self { data, metadata }
    }

    pub fn total_size(&self) -> usize {
        self.data.len() + std::mem::size_of::<CompressionMetadata>()
    }
}
