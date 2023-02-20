//! Utilities for handling file byte ordering at runtime

/// Representation of endianness
///
/// Endiannes is the order of bytes in a word of digital data
#[derive(Debug, Clone, Copy)]
pub enum ByteOrder {
    /// Least significant byte of a word at the smallest memory address
    LittleEndian,

    /// Most significant byte of a word at smallest memory address
    BigEndian,
}

impl ByteOrder {
    pub fn i32_from_bytes(&self, bytes: &[u8]) -> i32 {
        let mut buf = [0; std::mem::size_of::<i32>()];
        buf.copy_from_slice(&bytes[0..std::mem::size_of::<i32>()]);
        return match self {
            ByteOrder::LittleEndian => i32::from_le_bytes(buf),
            ByteOrder::BigEndian => i32::from_be_bytes(buf),
        };
    }
}
