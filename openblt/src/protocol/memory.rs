use core::convert::TryInto;
use thiserror::Error;
use s32k148_hal::flash::{Flash, FlashError};

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Invalid memory address")]
    InvalidAddress,
    #[error("Invalid memory length")]
    InvalidLength,
    #[error("Write error")]
    WriteError,
    #[error("Read error")]
    ReadError,
    #[error("Erase error")]
    EraseError,
    #[error("Flash error: {0}")]
    FlashError(#[from] FlashError),
}

pub struct Memory {
    flash: Flash,
}

impl Memory {
    pub fn new(start_address: u32, size: u32) -> Self {
        Self {
            flash: Flash::new(),
        }
    }

    pub fn erase(&mut self, address: u32, length: u32) -> Result<(), MemoryError> {
        self.flash.erase(address, length)
            .map_err(MemoryError::FlashError)
    }

    pub fn write(&mut self, address: u32, data: &[u8]) -> Result<(), MemoryError> {
        self.flash.write(address, data)
            .map_err(MemoryError::FlashError)
    }

    pub fn read(&mut self, address: u32, length: u32) -> Result<Vec<u8>, MemoryError> {
        self.flash.read(address, length)
            .map_err(MemoryError::FlashError)
    }

    pub fn calculate_checksum(&mut self, address: u32, length: u32) -> Result<u32, MemoryError> {
        self.flash.calculate_checksum(address, length)
            .map_err(MemoryError::FlashError)
    }
} 
