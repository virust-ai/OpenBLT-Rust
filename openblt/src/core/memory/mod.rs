#![no_std]

use core::fmt;
use crate::hal::S32KHal;

#[derive(Debug)]
pub enum MemoryManagementError {
    InvalidAddress,
    InvalidLength,
    WriteError,
    EraseError,
    ReadError,
    AlignmentError,
    OutOfBounds,
}

impl fmt::Display for MemoryManagementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryManagementError::InvalidAddress => write!(f, "Invalid memory address"),
            MemoryManagementError::InvalidLength => write!(f, "Invalid memory length"),
            MemoryManagementError::WriteError => write!(f, "Memory write error"),
            MemoryManagementError::EraseError => write!(f, "Memory erase error"),
            MemoryManagementError::ReadError => write!(f, "Memory read error"),
            MemoryManagementError::AlignmentError => write!(f, "Memory alignment error"),
            MemoryManagementError::OutOfBounds => write!(f, "Memory access out of bounds"),
        }
    }
}

pub struct MemoryManager<H: S32KHal> {
    hal: H,
    app_start: u32,
    app_end: u32,
}

impl<H: S32KHal> MemoryManager<H> {
    pub fn new(hal: H) -> Result<Self, MemoryManagementError> {
        Ok(Self {
            hal,
            app_start: 0x00004000, // Application start address
            app_end: 0x0007FFFF,   // Application end address
        })
    }

    pub fn erase(&mut self, address: u32, length: u32) -> Result<(), MemoryManagementError> {
        // Verify address and length
        if address < self.app_start || address + length > self.app_end {
            return Err(MemoryManagementError::OutOfBounds);
        }

        // Verify alignment
        if address % 4096 != 0 || length % 4096 != 0 {
            return Err(MemoryManagementError::AlignmentError);
        }

        // Erase flash
        self.hal.erase_flash(address, length)
            .map_err(|_| MemoryManagementError::EraseError)
    }

    pub fn write(&mut self, address: u32, data: &[u8]) -> Result<(), MemoryManagementError> {
        // Verify address and length
        if address < self.app_start || address + data.len() as u32 > self.app_end {
            return Err(MemoryManagementError::OutOfBounds);
        }

        // Verify alignment
        if address % 4 != 0 {
            return Err(MemoryManagementError::AlignmentError);
        }

        // Write data
        self.hal.write_flash(address, data)
            .map_err(|_| MemoryManagementError::WriteError)
    }

    pub fn read(&self, address: u32, data: &mut [u8]) -> Result<(), MemoryManagementError> {
        // Verify address and length
        if address < self.app_start || address + data.len() as u32 > self.app_end {
            return Err(MemoryManagementError::OutOfBounds);
        }

        // Verify alignment
        if address % 4 != 0 {
            return Err(MemoryManagementError::AlignmentError);
        }

        // Read data
        self.hal.read_flash(address, data)
            .map_err(|_| MemoryManagementError::ReadError)
    }

    pub fn get_app_start(&self) -> u32 {
        self.app_start
    }

    pub fn get_app_end(&self) -> u32 {
        self.app_end
    }
} 
