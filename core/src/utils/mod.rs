pub mod bitstream;

use std::num::Wrapping;

/// Calculate a simple checksum for data integrity
pub fn calculate_checksum(data: &[u8]) -> u32 {
    let mut sum = Wrapping(0u32);
    for &byte in data {
        sum += Wrapping(byte as u32);
        sum = Wrapping(sum.0.rotate_left(1));
    }
    sum.0
}

/// Verify checksum matches data
pub fn verify_checksum(data: &[u8], expected: u32) -> bool {
    calculate_checksum(data) == expected
}
