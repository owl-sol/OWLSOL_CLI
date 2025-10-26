use crate::error::Result;
use std::collections::HashMap;

const DICT_MARKER: u8 = 0xFF;
const WINDOW_SIZE: usize = 4;
const MAX_DICT_SIZE: u16 = 4096;

pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut dictionary: HashMap<Vec<u8>, u16> = HashMap::new();
    let mut result = Vec::new();
    let mut next_id = 0u16;
    let mut i = 0;

    while i < data.len() {
        let mut best_len = 0;
        let mut best_id = 0u16;

        // Find longest matching sequence
        for len in (2..=WINDOW_SIZE.min(data.len() - i)).rev() {
            let sequence = &data[i..i + len];
            if let Some(&id) = dictionary.get(sequence) {
                if len > best_len {
                    best_len = len;
                    best_id = id;
                }
            }
        }

        if best_len >= 2 {
            // Use dictionary reference
            result.push(DICT_MARKER);
            result.extend_from_slice(&best_id.to_le_bytes());
            i += best_len;
        } else {
            // Literal byte
            result.push(data[i]);
            
            // Add new sequences to dictionary
            if i + WINDOW_SIZE <= data.len() && next_id < MAX_DICT_SIZE {
                let sequence = data[i..i + WINDOW_SIZE].to_vec();
                dictionary.entry(sequence).or_insert_with(|| {
                    let id = next_id;
                    next_id += 1;
                    id
                });
            }
            i += 1;
        }
    }

    // Prepend dictionary
    let mut compressed = Vec::new();
    let dict_size = dictionary.len() as u32;
    compressed.extend_from_slice(&dict_size.to_le_bytes());

    for (sequence, id) in dictionary.iter() {
        compressed.extend_from_slice(&id.to_le_bytes());
        compressed.push(sequence.len() as u8);
        compressed.extend_from_slice(sequence);
    }

    compressed.extend_from_slice(&result);
    Ok(compressed)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    if data.len() < 4 {
        return Ok(Vec::new());
    }

    // Read dictionary size
    let dict_size = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
    let mut dictionary: HashMap<u16, Vec<u8>> = HashMap::new();
    let mut pos = 4;

    // Rebuild dictionary
    for _ in 0..dict_size {
        if pos + 3 > data.len() {
            break;
        }
        let id = u16::from_le_bytes([data[pos], data[pos + 1]]);
        let len = data[pos + 2] as usize;
        pos += 3;

        if pos + len > data.len() {
            break;
        }
        dictionary.insert(id, data[pos..pos + len].to_vec());
        pos += len;
    }

    // Decompress data
    let mut result = Vec::new();
    while pos < data.len() {
        if data[pos] == DICT_MARKER && pos + 2 < data.len() {
            let id = u16::from_le_bytes([data[pos + 1], data[pos + 2]]);
            if let Some(sequence) = dictionary.get(&id) {
                result.extend_from_slice(sequence);
            }
            pos += 3;
        } else {
            result.push(data[pos]);
            pos += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_compression() {
        let data = b"test data test data test";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_no_repeats() {
        let data = b"abcdefghijk";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_many_repeats() {
        let data = b"repeatrepeatrepeatrepeat";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
        assert!(compressed.len() < data.len());
    }
}
