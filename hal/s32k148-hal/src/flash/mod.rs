#![no_std]

use controller::{FlashError, FlashController};
use core::fmt;
use core::convert::TryInto;

pub mod controller;

const FLASH_BASE: u32 = 0x0000_0000;
const FLASH_SIZE: u32 = 0x1000_0000; // 16MB
const FLASH_PAGE_SIZE: u32 = 4096; // 4KB
const FLASH_SECTOR_SIZE: u32 = 65536; // 64KB

#[derive(Debug)]
pub enum Error {
    Controller(FlashError),
    InvalidAddress,
    InvalidLength,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Controller(e) => write!(f, "Flash controller error: {:?}", e),
            Error::InvalidAddress => write!(f, "Invalid address"),
            Error::InvalidLength => write!(f, "Invalid length"),
        }
    }
}

pub struct Flash {
    base_address: u32,
    size: u32,
    controller: FlashController,
}

impl Flash {
    pub fn new() -> Self {
        Flash {
            base_address: FLASH_BASE,
            size: FLASH_SIZE,
            controller: FlashController::new(),
        }
    }

    pub fn erase(&mut self, address: u32, length: u32) -> Result<(), Error> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(Error::InvalidAddress);
        }

        // Ensure length is a multiple of sector size
        if length % FLASH_SECTOR_SIZE != 0 {
            return Err(Error::InvalidLength);
        }

        // Erase each sector
        let mut current_addr = address;
        while current_addr < address + length {
            self.controller
                .erase_sector(current_addr)
                .map_err(Error::Controller)?;
            current_addr += FLASH_SECTOR_SIZE;
        }

        Ok(())
    }

    pub fn write(&mut self, address: u32, data: &[u8]) -> Result<(), Error> {
        // Validate address and length
        if address < self.base_address || address + data.len() as u32 > self.base_address + self.size {
            return Err(Error::InvalidAddress);
        }

        // Ensure data length is a multiple of 4 (word size)
        if data.len() % 4 != 0 {
            return Err(Error::InvalidLength);
        }

        // Write data in 4-byte chunks
        for (i, chunk) in data.chunks(4).enumerate() {
            let word = u32::from_le_bytes(chunk.try_into().unwrap());
            self.controller
                .program_word(address + (i * 4) as u32, word)
                .map_err(Error::Controller)?;
        }

        Ok(())
    }

    pub fn read(&self, address: u32, length: u32) -> Result<&[u8], Error> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(Error::InvalidAddress);
        }

        // Return a slice to the flash memory
        let start = address as usize;
        let end = (address + length) as usize;
        Ok(unsafe { core::slice::from_raw_parts(start as *const u8, end - start) })
    }

    pub fn calculate_checksum(&self, address: u32, length: u32) -> Result<u32, Error> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(Error::InvalidAddress);
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


