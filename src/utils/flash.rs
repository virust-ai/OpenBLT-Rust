#![no_std]

pub fn is_flash_erased(data: &[u8]) -> bool {
    data.iter().all(|&byte| byte == 0xFF)
}

pub fn is_address_aligned(address: u32, alignment: u32) -> bool {
    address % alignment == 0
} 
