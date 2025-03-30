use core::convert::TryInto;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlashError {
    #[error("Invalid address")]
    InvalidAddress,
    #[error("Invalid length")]
    InvalidLength,
    #[error("Write error")]
    WriteError,
    #[error("Erase error")]
    EraseError,
    #[error("Busy error")]
    BusyError,
    #[error("Command sequence error")]
    CommandSequenceError,
}

const FLASH_BASE: u32 = 0x0000_0000;
const FLASH_SIZE: u32 = 0x1000_0000; // 16MB
const FLASH_PAGE_SIZE: u32 = 4096; // 4KB
const FLASH_SECTOR_SIZE: u32 = 65536; // 64KB

pub struct Flash {
    base_address: u32,
    size: u32,
}

impl Flash {
    pub fn new() -> Self {
        Self {
            base_address: FLASH_BASE,
            size: FLASH_SIZE,
        }
    }

    pub fn erase(&mut self, address: u32, length: u32) -> Result<(), FlashError> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        // Ensure length is a multiple of sector size
        if length % FLASH_SECTOR_SIZE != 0 {
            return Err(FlashError::InvalidLength);
        }

        // TODO: Implement actual flash erase sequence
        // 1. Wait for flash to be ready
        // 2. Set sector erase command
        // 3. Write to any address in the sector to trigger erase
        // 4. Wait for completion
        // 5. Verify erase

        Ok(())
    }

    pub fn write(&mut self, address: u32, data: &[u8]) -> Result<(), FlashError> {
        // Validate address and length
        if address < self.base_address || address + data.len() as u32 > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        // Ensure data length is a multiple of 4 (word size)
        if data.len() % 4 != 0 {
            return Err(FlashError::InvalidLength);
        }

        // TODO: Implement actual flash write sequence
        // 1. Wait for flash to be ready
        // 2. Set program command
        // 3. Write data in 4-byte chunks
        // 4. Wait for completion
        // 5. Verify write

        Ok(())
    }

    pub fn read(&self, address: u32, length: u32) -> Result<Vec<u8>, FlashError> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        // TODO: Implement actual flash read
        // For now, return a zero-filled buffer
        Ok(vec![0; length as usize])
    }

    pub fn calculate_checksum(&self, address: u32, length: u32) -> Result<u32, FlashError> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        // Read memory and calculate checksum
        let data = self.read(address, length)?;
        
        // Simple XOR checksum
        let mut checksum = 0u32;
        for chunk in data.chunks(4) {
            let word = u32::from_le_bytes(chunk.try_into().unwrap());
            checksum ^= word;
        }
        
        Ok(checksum)
    }

    fn wait_for_ready(&self) -> Result<(), FlashError> {
        // TODO: Implement flash ready check
        // 1. Read flash status register
        // 2. Check busy bit
        // 3. Check error bits
        Ok(())
    }

    fn send_command(&self, command: u8) -> Result<(), FlashError> {
        // TODO: Implement flash command sequence
        // 1. Write command to flash command register
        // 2. Write confirmation sequence
        Ok(())
    }
} 
