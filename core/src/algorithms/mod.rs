pub mod huffman;
pub mod dictionary;
pub mod rle;

pub use huffman::HuffmanCodec;
pub use dictionary::{compress as dict_compress, decompress as dict_decompress};
pub use rle::{compress as rle_compress, decompress as rle_decompress};
