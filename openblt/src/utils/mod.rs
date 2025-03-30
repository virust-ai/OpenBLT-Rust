// Utility functions for the bootloader

/// Calculate CRC32 of data
pub fn calculate_crc32(data: &[u8]) -> u32 {
    // TODO: Implement CRC32 calculation
    0
}

/// Verify firmware checksum
pub fn verify_checksum(data: &[u8], expected_checksum: u32) -> bool {
    let calculated = calculate_crc32(data);
    calculated == expected_checksum
}

/// Convert bytes to u32 (little-endian)
pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut value = 0u32;
    for (i, &byte) in bytes.iter().enumerate() {
        value |= (byte as u32) << (i * 8);
    }
    value
}

/// Convert u32 to bytes (little-endian)
pub fn u32_to_bytes(value: u32) -> [u8; 4] {
    [
        (value & 0xFF) as u8,
        ((value >> 8) & 0xFF) as u8,
        ((value >> 16) & 0xFF) as u8,
        ((value >> 24) & 0xFF) as u8,
    ]
}
