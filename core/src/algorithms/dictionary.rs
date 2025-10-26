use crate::error::Result;
use std::collections::{hash_map::Entry, HashMap};

const DICT_MARKER: u8 = 0xFF;
const MIN_MATCH_LEN: usize = 3; // Only match 3+ bytes
const MAX_MATCH_LEN: usize = 8; // Longer matches
const MAX_DICT_SIZE: u16 = 256; // Smaller dictionary

pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut dictionary: HashMap<Vec<u8>, u16> = HashMap::new();
    let mut result = Vec::new();
    let mut next_id = 0u16;
    let mut i = 0;

    // Build dictionary as we scan
    while i < data.len() {
        let mut best_len = 0;
        let mut best_id = 0u16;

        // Try to find match in dictionary (only for MIN_MATCH_LEN..=MAX_MATCH_LEN)
        for len in (MIN_MATCH_LEN..=MAX_MATCH_LEN.min(data.len() - i)).rev() {
            let sequence = &data[i..i + len];
            if let Some(&id) = dictionary.get(sequence) {
                best_len = len;
                best_id = id;
                break;
            }
        }

        if best_len >= MIN_MATCH_LEN {
            // Use dictionary reference
            result.push(DICT_MARKER);
            result.extend_from_slice(&best_id.to_le_bytes());
            i += best_len;
        } else {
            // Literal byte
            let byte = data[i];
            // Escape marker byte
            if byte == DICT_MARKER {
                result.push(DICT_MARKER);
                result.push(0x00); // Escape sequence
            } else {
                result.push(byte);
            }
            // Add all possible sequences for future matches
            for len in MIN_MATCH_LEN..=MAX_MATCH_LEN.min(data.len() - i) {
                if next_id >= MAX_DICT_SIZE {
                    break;
                }
                let sequence = data[i..i + len].to_vec();
                if let Entry::Vacant(e) = dictionary.entry(sequence) {
                    e.insert(next_id);
                    next_id += 1;
                }
            }
            i += 1;
        }
    }

    // Prepend dictionary for decompression
    let mut compressed = Vec::new();
    // Write dictionary size
    let dict_size = dictionary.len() as u32;
    compressed.extend_from_slice(&dict_size.to_le_bytes());
    // Write dictionary entries
    for (sequence, id) in dictionary.iter() {
        compressed.extend_from_slice(&id.to_le_bytes());
        compressed.push(sequence.len() as u8);
        compressed.extend_from_slice(sequence);
    }
    // Append compressed data
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
        if data[pos] == DICT_MARKER {
            if pos + 1 < data.len() && data[pos + 1] == 0x00 {
                // Escaped marker
                result.push(DICT_MARKER);
                pos += 2;
            } else if pos + 2 < data.len() {
                // Dictionary reference
                let id = u16::from_le_bytes([data[pos + 1], data[pos + 2]]);
                match dictionary.get(&id) {
                    Some(sequence) => result.extend_from_slice(sequence),
                    None => {
                        println!(
                            "[DICT DECOMPRESS] Missing dictionary entry for id {} at pos {}",
                            id, pos
                        );
                        return Err(crate::error::CompressionError::CorruptedData(format!(
                            "Missing dictionary entry for id {} at pos {}",
                            id, pos
                        )));
                    }
                }
                pos += 3;
            } else {
                // Incomplete marker, treat as literal
                result.push(data[pos]);
                pos += 1;
            }
        } else {
            // Literal byte
            result.push(data[pos]);
            pos += 1;
        }
    }
    println!("[DICT DECOMPRESS] Output length: {}", result.len());
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
        println!("Expected: {:?}", data);
        println!("Actual:   {:?}", decompressed);
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_no_repeats() {
        let data = b"abcdefghijk";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        println!("Expected: {:?}", data);
        println!("Actual:   {:?}", decompressed);
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_many_repeats() {
        let data = b"repeatrepeatrepeatrepeat";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        println!("Expected: {:?}", data);
        println!("Actual:   {:?}", decompressed);
        assert_eq!(data.to_vec(), decompressed);
        // Compression may not always be smaller for small or edge cases; only check roundtrip correctness
    }

    #[test]
    fn test_empty() {
        let data = b"";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_marker_escape() {
        let data = b"\xFF\xFF\xFF";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data.to_vec(), decompressed);
    }

    #[test]
    fn test_long_sequences() {
        let sequence = b"ABCDEFGH";
        let data: Vec<u8> = sequence.iter().cycle().take(1000).copied().collect();
        let compressed = compress(&data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed);
        assert!(compressed.len() < data.len());
    }
}
