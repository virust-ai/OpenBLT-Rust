mod controller;
use controller::{ControllerError, FlashController};

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
    #[error("Controller error: {0}")]
    ControllerError(#[from] ControllerError),
}

const FLASH_BASE: u32 = 0x0000_0000;
const FLASH_SIZE: u32 = 0x1000_0000; // 16MB
const FLASH_PAGE_SIZE: u32 = 4096; // 4KB
const FLASH_SECTOR_SIZE: u32 = 65536; // 64KB

pub struct Flash {
    base_address: u32,
    size: u32,
    controller: FlashController,
}

impl Flash {
    pub fn new() -> Self {
        Self {
            base_address: FLASH_BASE,
            size: FLASH_SIZE,
            controller: FlashController::new(),
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

        // Erase each sector
        let mut current_addr = address;
        while current_addr < address + length {
            self.controller.erase_sector(current_addr)?;
            current_addr += FLASH_SECTOR_SIZE;
        }

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

        // Write data in 4-byte chunks
        for (i, chunk) in data.chunks(4).enumerate() {
            let word = u32::from_le_bytes(chunk.try_into().unwrap());
            self.controller.program_word(address + (i * 4) as u32, word)?;
        }

        Ok(())
    }

    pub fn read(&self, address: u32, length: u32) -> Result<Vec<u8>, FlashError> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        let mut data = vec![0u8; length as usize];
        
        // Read data in 4-byte chunks
        for i in 0..(length / 4) {
            let word = self.controller.read_word(address + (i * 4));
            data[i as usize * 4..(i as usize + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        Ok(data)
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
} 
