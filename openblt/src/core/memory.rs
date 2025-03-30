use core::convert::TryInto;
use thiserror::Error;
use crate::hal::S32KHal;

#[derive(Error, Debug)]
pub enum MemoryManagementError {
    #[error("Invalid memory region")]
    InvalidRegion,
    #[error("Region overlap")]
    RegionOverlap,
    #[error("Region not aligned")]
    RegionNotAligned,
    #[error("Hardware error: {0}")]
    Hardware(String),
    #[error("Protected region access")]
    ProtectedRegionAccess,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    start: u32,
    size: u32,
    is_protected: bool,
}

impl MemoryRegion {
    pub fn new(start: u32, size: u32, is_protected: bool) -> Result<Self, MemoryManagementError> {
        // Validate alignment (must be 4KB aligned)
        if start % 4096 != 0 || size % 4096 != 0 {
            return Err(MemoryManagementError::RegionNotAligned);
        }

        Ok(Self {
            start,
            size,
            is_protected,
        })
    }

    pub fn contains(&self, address: u32) -> bool {
        address >= self.start && address < self.start + self.size
    }

    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        (self.start < other.start + other.size) && (other.start < self.start + self.size)
    }

    pub fn is_protected(&self) -> bool {
        self.is_protected
    }
}

pub struct MemoryManager<H: S32KHal> {
    hal: H,
    bootloader_region: MemoryRegion,
    application_region: MemoryRegion,
    config_region: MemoryRegion,
}

impl<H: S32KHal> MemoryManager<H> {
    pub fn new(hal: H) -> Result<Self, MemoryManagementError> {
        // Define memory regions
        let bootloader_region = MemoryRegion::new(0x0000_0000, 0x10000, true)?; // 64KB bootloader
        let application_region = MemoryRegion::new(0x0001_0000, 0x0F0000, false)?; // 960KB application
        let config_region = MemoryRegion::new(0x0100_0000, 0x1000, true)?; // 4KB config

        // Verify regions don't overlap
        if bootloader_region.overlaps(&application_region) ||
           bootloader_region.overlaps(&config_region) ||
           application_region.overlaps(&config_region) {
            return Err(MemoryManagementError::RegionOverlap);
        }

        Ok(Self {
            hal,
            bootloader_region,
            application_region,
            config_region,
        })
    }

    pub fn erase_region(&mut self, address: u32, length: u32) -> Result<(), MemoryManagementError> {
        // Check if operation is within application region
        if !self.application_region.contains(address) ||
           !self.application_region.contains(address + length - 1) {
            return Err(MemoryManagementError::InvalidRegion);
        }

        // Ensure length is a multiple of 4KB (flash page size)
        if length % 4096 != 0 {
            return Err(MemoryManagementError::RegionNotAligned);
        }

        self.hal.erase_flash(address, length)
            .map_err(|e| MemoryManagementError::Hardware(e.into().to_string()))
    }

    pub fn write_region(&mut self, address: u32, data: &[u8]) -> Result<(), MemoryManagementError> {
        // Check if operation is within application region
        if !self.application_region.contains(address) ||
           !self.application_region.contains(address + data.len() as u32 - 1) {
            return Err(MemoryManagementError::InvalidRegion);
        }

        // Ensure data length is a multiple of 4 (word size)
        if data.len() % 4 != 0 {
            return Err(MemoryManagementError::RegionNotAligned);
        }

        self.hal.write_flash(address, data)
            .map_err(|e| MemoryManagementError::Hardware(e.into().to_string()))
    }

    pub fn read_region(&mut self, address: u32, data: &mut [u8]) -> Result<(), MemoryManagementError> {
        // Check if operation is within any region
        if !self.bootloader_region.contains(address) &&
           !self.application_region.contains(address) &&
           !self.config_region.contains(address) {
            return Err(MemoryManagementError::InvalidRegion);
        }

        self.hal.read_flash(address, data)
            .map_err(|e| MemoryManagementError::Hardware(e.into().to_string()))
    }

    pub fn is_protected_region(&self, address: u32) -> bool {
        self.bootloader_region.contains(address) || self.config_region.contains(address)
    }

    pub fn get_application_start(&self) -> u32 {
        self.application_region.start
    }

    pub fn get_application_size(&self) -> u32 {
        self.application_region.size
    }

    pub fn get_bootloader_start(&self) -> u32 {
        self.bootloader_region.start
    }

    pub fn get_bootloader_size(&self) -> u32 {
        self.bootloader_region.size
    }

    pub fn get_config_start(&self) -> u32 {
        self.config_region.start
    }

    pub fn get_config_size(&self) -> u32 {
        self.config_region.size
    }
} 
