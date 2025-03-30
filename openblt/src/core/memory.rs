use core::convert::TryInto;
use crate::hal::S32KHal;

#[derive(Debug)]
pub enum MemoryManagementError {
    Hardware(&'static str),
    InvalidAddress,
    InvalidLength,
    WriteError,
    ReadError,
    EraseError,
    RegionNotAligned,
    RegionOverlap,
}

impl core::fmt::Display for MemoryManagementError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MemoryManagementError::Hardware(msg) => write!(f, "Hardware error: {}", msg),
            MemoryManagementError::InvalidAddress => write!(f, "Invalid address"),
            MemoryManagementError::InvalidLength => write!(f, "Invalid length"),
            MemoryManagementError::WriteError => write!(f, "Write error"),
            MemoryManagementError::ReadError => write!(f, "Read error"),
            MemoryManagementError::EraseError => write!(f, "Erase error"),
            MemoryManagementError::RegionNotAligned => write!(f, "Region not aligned"),
            MemoryManagementError::RegionOverlap => write!(f, "Region overlap"),
        }
    }
}

impl core::error::Error for MemoryManagementError {}

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
        self.hal
            .erase_flash(address, length)
            .map_err(|e| MemoryManagementError::Hardware("Failed to erase flash"))
    }

    pub fn write_region(&mut self, address: u32, data: &[u8]) -> Result<(), MemoryManagementError> {
        self.hal
            .write_flash(address, data)
            .map_err(|e| MemoryManagementError::Hardware("Failed to write flash"))
    }

    pub fn read_region(&mut self, address: u32, data: &mut [u8]) -> Result<(), MemoryManagementError> {
        self.hal
            .read_flash(address, data)
            .map_err(|e| MemoryManagementError::Hardware("Failed to read flash"))
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
