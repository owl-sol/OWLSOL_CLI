use crate::error::{CompressionError, Result};
use crate::utils::bitstream::{BitReader, BitWriter};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone)]
struct HuffmanNode {
    freq: u64,
    value: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn leaf(v: u8, f: u64) -> Self {
        Self {
            freq: f,
            value: Some(v),
            left: None,
            right: None,
        }
    }

    fn internal(l: Box<HuffmanNode>, r: Box<HuffmanNode>) -> Self {
        Self {
            freq: l.freq + r.freq,
            value: None,
            left: Some(l),
            right: Some(r),
        }
    }

    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.freq == other.freq
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap behavior
        other.freq.cmp(&self.freq)
    }
}

pub struct HuffmanCodec {
    codes: HashMap<u8, Vec<bool>>,
    tree: Option<Box<HuffmanNode>>,
}

impl HuffmanCodec {
    pub fn new() -> Self {
        Self {
            codes: HashMap::new(),
            tree: None,
        }
    }

    pub fn build_from_data(&mut self, data: &[u8]) -> Result<()> {
        if data.is_empty() {
            return Err(CompressionError::invalid_input("Empty data"));
        }

        // Calculate frequency distribution
        let mut freqs = HashMap::new();
        for &byte in data {
            *freqs.entry(byte).or_insert(0u64) += 1;
        }

        // Build Huffman tree
        self.tree = Some(Self::build_tree(freqs)?);
        
        // Generate codes from tree
        self.codes.clear();
        if let Some(ref tree) = self.tree {
            Self::generate_codes(tree, Vec::new(), &mut self.codes);
        }

        Ok(())
    }

    fn build_tree(freqs: HashMap<u8, u64>) -> Result<Box<HuffmanNode>> {
        let mut heap = BinaryHeap::new();
        
        for (byte, freq) in freqs {
            heap.push(HuffmanNode::leaf(byte, freq));
        }

        // Handle single-symbol case
        if heap.len() == 1 {
            let node = heap.pop().unwrap();
            return Ok(Box::new(HuffmanNode::internal(
                Box::new(node),
                Box::new(HuffmanNode::leaf(0, 0)),
            )));
        }

        // Build tree bottom-up
        while heap.len() > 1 {
            let left = Box::new(heap.pop().unwrap());
            let right = Box::new(heap.pop().unwrap());
            heap.push(HuffmanNode::internal(left, right));
        }

        Ok(Box::new(heap.pop().unwrap()))
    }

    fn generate_codes(
        node: &HuffmanNode,
        prefix: Vec<bool>,
        codes: &mut HashMap<u8, Vec<bool>>,
    ) {
        if node.is_leaf() {
            if let Some(value) = node.value {
                codes.insert(value, prefix);
            }
        } else {
            if let Some(ref left) = node.left {
                let mut left_prefix = prefix.clone();
                left_prefix.push(false);
                Self::generate_codes(left, left_prefix, codes);
            }
            if let Some(ref right) = node.right {
                let mut right_prefix = prefix.clone();
                right_prefix.push(true);
                Self::generate_codes(right, right_prefix, codes);
            }
        }
    }

    pub fn encode(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut writer = BitWriter::with_capacity(data.len() / 2);
        
        for &byte in data {
            let code = self.codes.get(&byte).ok_or_else(|| {
                CompressionError::compression_failed(format!("No code for byte {}", byte))
            })?;
            
            writer.write_bits(code);
        }

        Ok(writer.into_bytes())
    }

    pub fn decode(&self, encoded: &[u8], original_len: usize) -> Result<Vec<u8>> {
        let tree = self.tree.as_ref().ok_or_else(|| {
            CompressionError::decompression_failed("No Huffman tree available")
        })?;

        let mut reader = BitReader::new(encoded);
        let mut decoded = Vec::with_capacity(original_len);

        while decoded.len() < original_len {
            let mut current = tree.as_ref();
            
            loop {
                if current.is_leaf() {
                    if let Some(value) = current.value {
                        decoded.push(value);
                    }
                    break;
                }

                let bit = reader.read_bit().ok_or_else(|| {
                    CompressionError::decompression_failed("Unexpected end of data")
                })?;

                current = if bit {
                    current.right.as_ref()
                } else {
                    current.left.as_ref()
                }.ok_or_else(|| {
                    CompressionError::decompression_failed("Invalid tree structure")
                })?;
            }
        }

        Ok(decoded)
    }

    pub fn serialize_tree(&self) -> Result<Vec<u8>> {
        let tree = self.tree.as_ref().ok_or_else(|| {
            CompressionError::compression_failed("No tree to serialize")
        })?;

        let mut writer = BitWriter::new();
        Self::serialize_node(tree, &mut writer);
        Ok(writer.into_bytes())
    }

    fn serialize_node(node: &HuffmanNode, writer: &mut BitWriter) {
        if node.is_leaf() {
            writer.write_bit(true); // Leaf marker
            if let Some(value) = node.value {
                writer.write_byte(value);
            }
        } else {
            writer.write_bit(false); // Internal node marker
            if let Some(ref left) = node.left {
                Self::serialize_node(left, writer);
            }
            if let Some(ref right) = node.right {
                Self::serialize_node(right, writer);
            }
        }
    }

    pub fn deserialize_tree(&mut self, data: &[u8]) -> Result<()> {
        let mut reader = BitReader::new(data);
        self.tree = Some(Box::new(Self::deserialize_node(&mut reader)?));
        
        // Regenerate codes
        self.codes.clear();
        if let Some(ref tree) = self.tree {
            Self::generate_codes(tree, Vec::new(), &mut self.codes);
        }

        Ok(())
    }

    fn deserialize_node(reader: &mut BitReader) -> Result<HuffmanNode> {
        let is_leaf = reader.read_bit().ok_or_else(|| {
            CompressionError::decompression_failed("Incomplete tree data")
        })?;

        if is_leaf {
            let value = reader.read_byte().ok_or_else(|| {
                CompressionError::decompression_failed("Incomplete leaf node")
            })?;
            Ok(HuffmanNode::leaf(value, 0))
        } else {
            let left = Box::new(Self::deserialize_node(reader)?);
            let right = Box::new(Self::deserialize_node(reader)?);
            Ok(HuffmanNode::internal(left, right))
        }
    }

    pub fn get_code_size(&self) -> usize {
        self.codes.len()
    }

    pub fn get_avg_code_length(&self) -> f64 {
        if self.codes.is_empty() {
            return 0.0;
        }
        let total: usize = self.codes.values().map(|c| c.len()).sum();
        total as f64 / self.codes.len() as f64
    }
}

impl Default for HuffmanCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_huffman_roundtrip() {
        let data = b"hello world hello world hello";
        let mut codec = HuffmanCodec::new();
        codec.build_from_data(data).unwrap();

        let encoded = codec.encode(data).unwrap();
        let decoded = codec.decode(&encoded, data.len()).unwrap();

        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_single_symbol() {
        let data = b"aaaaa";
        let mut codec = HuffmanCodec::new();
        codec.build_from_data(data).unwrap();

        let encoded = codec.encode(data).unwrap();
        let decoded = codec.decode(&encoded, data.len()).unwrap();

        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_tree_serialization() {
        let data = b"test data";
        let mut codec = HuffmanCodec::new();
        codec.build_from_data(data).unwrap();

        let tree_data = codec.serialize_tree().unwrap();
        
        let mut codec2 = HuffmanCodec::new();
        codec2.deserialize_tree(&tree_data).unwrap();

        let encoded = codec.encode(data).unwrap();
        let decoded = codec2.decode(&encoded, data.len()).unwrap();

        assert_eq!(data, decoded.as_slice());
    }
}
