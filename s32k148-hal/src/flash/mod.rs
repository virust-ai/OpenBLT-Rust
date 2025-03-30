mod controller;
use controller::{ControllerError, FlashController};

use core::convert::TryInto;
use core::fmt;

#[derive(Debug)]
pub enum FlashError {
    InvalidAddress,
    InvalidLength,
    WriteError,
    EraseError,
    BusyError,
    CommandSequenceError,
    ControllerError(ControllerError),
}

impl core::fmt::Display for FlashError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FlashError::InvalidAddress => write!(f, "Invalid address"),
            FlashError::InvalidLength => write!(f, "Invalid length"),
            FlashError::WriteError => write!(f, "Write error"),
            FlashError::EraseError => write!(f, "Erase error"),
            FlashError::BusyError => write!(f, "Busy error"),
            FlashError::CommandSequenceError => write!(f, "Command sequence error"),
            FlashError::ControllerError(e) => write!(f, "Controller error: {:?}", e),
        }
    }
}

impl core::error::Error for FlashError {}

impl From<ControllerError> for FlashError {
    fn from(err: ControllerError) -> Self {
        FlashError::ControllerError(err)
    }
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

    pub fn read(&self, address: u32, length: u32) -> Result<&[u8], FlashError> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(FlashError::InvalidAddress);
        }

        // Return a slice to the flash memory
        let start = address as usize;
        let end = (address + length) as usize;
        Ok(unsafe { core::slice::from_raw_parts(start as *const u8, end - start) })
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
