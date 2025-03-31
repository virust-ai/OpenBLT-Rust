#![no_std]

use crate::hal::S32KHal;
use crate::protocol::Protocol;
use core::fmt;

pub mod memory;
use memory::{MemoryManager, MemoryManagementError};

#[derive(Debug)]
pub enum BootloaderError {
    InvalidState,
    ProtocolError,
    MemoryError(MemoryManagementError),
    HalError,
}

impl fmt::Display for BootloaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootloaderError::InvalidState => write!(f, "Invalid bootloader state"),
            BootloaderError::ProtocolError => write!(f, "Protocol error"),
            BootloaderError::MemoryError(e) => write!(f, "Memory error: {}", e),
            BootloaderError::HalError => write!(f, "Hardware abstraction layer error"),
        }
    }
}

pub struct Bootloader<H: S32KHal + Clone> {
    hal: H,
    protocol: Protocol<H::Can>,
    memory_manager: MemoryManager<H>,
}

impl<H: S32KHal + Clone> Bootloader<H> {
    pub fn new(hal: H) -> Result<Self, BootloaderError> {
        let can = hal.clone().get_can();
        Ok(Self {
            hal: hal.clone(),
            protocol: Protocol::new(can),
            memory_manager: MemoryManager::new(hal)
                .map_err(BootloaderError::MemoryError)?,
        })
    }

    pub fn init(&mut self) -> Result<(), BootloaderError> {
        // Initialize protocol
        self.protocol.init().map_err(|_| BootloaderError::ProtocolError)?;
        
        Ok(())
    }

    pub fn get_memory_manager(&self) -> &MemoryManager<H> {
        &self.memory_manager
    }

    pub fn process(&mut self) -> Result<(), BootloaderError> {
        // TODO: Implement main bootloader loop
        Ok(())
    }
}
