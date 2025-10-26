/// Bit-level writer for efficient bit packing
pub struct BitWriter {
    bytes: Vec<u8>,
    current_byte: u8,
    bit_pos: u8,
}

impl BitWriter {
    pub fn new() -> Self {
        Self {
            bytes: Vec::new(),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            bytes: Vec::with_capacity(capacity),
            current_byte: 0,
            bit_pos: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) {
        if bit {
            self.current_byte |= 1 << (7 - self.bit_pos);
        }
        self.bit_pos += 1;
        
        if self.bit_pos == 8 {
            self.bytes.push(self.current_byte);
            self.current_byte = 0;
            self.bit_pos = 0;
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.write_bit((byte >> (7 - i)) & 1 == 1);
        }
    }

    pub fn write_bits(&mut self, bits: &[bool]) {
        for &bit in bits {
            self.write_bit(bit);
        }
    }

    pub fn write_u16(&mut self, value: u16) {
        self.write_byte((value >> 8) as u8);
        self.write_byte(value as u8);
    }

    pub fn write_u32(&mut self, value: u32) {
        self.write_byte((value >> 24) as u8);
        self.write_byte((value >> 16) as u8);
        self.write_byte((value >> 8) as u8);
        self.write_byte(value as u8);
    }

    pub fn into_bytes(mut self) -> Vec<u8> {
        if self.bit_pos > 0 {
            self.bytes.push(self.current_byte);
        }
        self.bytes
    }

    pub fn len(&self) -> usize {
        self.bytes.len() + if self.bit_pos > 0 { 1 } else { 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty() && self.bit_pos == 0
    }
}

impl Default for BitWriter {
    fn default() -> Self {
        Self::new()
    }
}

/// Bit-level reader for unpacking bit-packed data
pub struct BitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit_pos: u8,
}

impl<'a> BitReader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            byte_pos: 0,
            bit_pos: 0,
        }
    }

    pub fn read_bit(&mut self) -> Option<bool> {
        if self.byte_pos >= self.bytes.len() {
            return None;
        }
        
        let bit = (self.bytes[self.byte_pos] >> (7 - self.bit_pos)) & 1 == 1;
        self.bit_pos += 1;
        
        if self.bit_pos == 8 {
            self.byte_pos += 1;
            self.bit_pos = 0;
        }
        
        Some(bit)
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        let mut byte = 0u8;
        for i in 0..8 {
            if self.read_bit()? {
                byte |= 1 << (7 - i);
            }
        }
        Some(byte)
    }

    pub fn read_bits(&mut self, count: usize) -> Option<Vec<bool>> {
        let mut bits = Vec::with_capacity(count);
        for _ in 0..count {
            bits.push(self.read_bit()?);
        }
        Some(bits)
    }

    pub fn read_u16(&mut self) -> Option<u16> {
        let high = self.read_byte()? as u16;
        let low = self.read_byte()? as u16;
        Some((high << 8) | low)
    }

    pub fn read_u32(&mut self) -> Option<u32> {
        let b1 = self.read_byte()? as u32;
        let b2 = self.read_byte()? as u32;
        let b3 = self.read_byte()? as u32;
        let b4 = self.read_byte()? as u32;
        Some((b1 << 24) | (b2 << 16) | (b3 << 8) | b4)
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes.len().saturating_sub(self.byte_pos)
    }

    pub fn has_data(&self) -> bool {
        self.byte_pos < self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_writer_reader_roundtrip() {
        let mut writer = BitWriter::new();
        writer.write_bit(true);
        writer.write_bit(false);
        writer.write_bit(true);
        writer.write_byte(0xAB);

        let bytes = writer.into_bytes();
        let mut reader = BitReader::new(&bytes);

        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_bit(), Some(false));
        assert_eq!(reader.read_bit(), Some(true));
        assert_eq!(reader.read_byte(), Some(0xAB));
    }

    #[test]
    fn test_u16_roundtrip() {
        let mut writer = BitWriter::new();
        writer.write_u16(0x1234);
        
        let bytes = writer.into_bytes();
        let mut reader = BitReader::new(&bytes);
        
        assert_eq!(reader.read_u16(), Some(0x1234));
    }
}
