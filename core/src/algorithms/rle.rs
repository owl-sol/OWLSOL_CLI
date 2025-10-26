use crate::error::Result;

const RLE_MARKER: u8 = 0xFF;
const RLE_ESCAPE: u8 = 0x00;
const MIN_RUN_LENGTH: usize = 3;

pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::with_capacity(data.len());
    let mut i = 0;

    while i < data.len() {
        let current = data[i];
        let mut count = 1;

        // Count consecutive identical bytes
        while i + count < data.len() && data[i + count] == current && count < 255 {
            count += 1;
        }

        // Use RLE encoding if run is long enough
        if count >= MIN_RUN_LENGTH {
            result.push(RLE_MARKER);
            result.push(current);
            result.push(count as u8);
            i += count;
        } else {
            // Handle marker byte specially
            if current == RLE_MARKER {
                result.push(RLE_MARKER);
                result.push(RLE_ESCAPE);
            } else {
                result.push(current);
            }
            i += 1;
        }
    }

    Ok(result)
}

pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut result = Vec::with_capacity(data.len() * 2);
    let mut i = 0;

    while i < data.len() {
        if data[i] == RLE_MARKER {
            if i + 1 < data.len() && data[i + 1] == RLE_ESCAPE {
                // Escaped marker
                result.push(RLE_MARKER);
                i += 2;
            } else if i + 2 < data.len() {
                // RLE sequence
                let byte = data[i + 1];
                let count = data[i + 2] as usize;
                // More concise and efficient than repeat().take()
                let new_len = result.len() + count;
                result.resize(new_len, byte);
                i += 3;
            } else {
                // Incomplete sequence, treat as literal
                result.push(data[i]);
                i += 1;
            }
        } else {
            result.push(data[i]);
            i += 1;
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_compression() {
        let data = b"aaaaaabbbbbcccccc";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
        assert!(compressed.len() < data.len());
    }

    #[test]
    fn test_no_runs() {
        let data = b"abcdef";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_marker_escape() {
        let data = b"\xFF\xFF";
        let compressed = compress(data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed.as_slice());
    }

    #[test]
    fn test_long_run() {
        let data = vec![b'A'; 200];
        let compressed = compress(&data).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(data, decompressed);
        assert!(compressed.len() < 10);
    }
}
