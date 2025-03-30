use core::convert::TryInto;
use s32k148_hal::flash::{Flash, FlashError};

#[derive(Debug)]
pub enum MemoryError {
    InvalidAddress,
    InvalidLength,
    WriteError,
    ReadError,
    EraseError,
    FlashError(FlashError),
}

impl core::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MemoryError::InvalidAddress => write!(f, "Invalid memory address"),
            MemoryError::InvalidLength => write!(f, "Invalid memory length"),
            MemoryError::WriteError => write!(f, "Write error"),
            MemoryError::ReadError => write!(f, "Read error"),
            MemoryError::EraseError => write!(f, "Erase error"),
            MemoryError::FlashError(e) => write!(f, "Flash error: {}", e),
        }
    }
}

impl core::error::Error for MemoryError {}

impl From<FlashError> for MemoryError {
    fn from(error: FlashError) -> Self {
        MemoryError::FlashError(error)
    }
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

    pub fn read(&mut self, address: u32, length: u32) -> Result<&[u8], MemoryError> {
        self.flash.read(address, length)
            .map_err(MemoryError::FlashError)
    }

    pub fn calculate_checksum(&mut self, address: u32, length: u32) -> Result<u32, MemoryError> {
        self.flash.calculate_checksum(address, length)
            .map_err(MemoryError::FlashError)
    }
} 
