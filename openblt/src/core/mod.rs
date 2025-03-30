use crate::hal::S32KHal;
use crate::protocol::{Command, Protocol, ProtocolError};
use thiserror::Error;

mod memory;
use memory::{MemoryManager, MemoryManagementError};

#[derive(Debug)]
pub enum BootloaderError {
    Hardware(&'static str),
    InvalidState,
    InvalidCommand,
    InvalidAddress,
    InvalidLength,
    VerificationFailed,
    MemoryError,
    ProtocolError,
}

impl core::fmt::Display for BootloaderError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            BootloaderError::Hardware(msg) => write!(f, "Hardware error: {}", msg),
            BootloaderError::InvalidState => write!(f, "Invalid state"),
            BootloaderError::InvalidCommand => write!(f, "Invalid command"),
            BootloaderError::InvalidAddress => write!(f, "Invalid address"),
            BootloaderError::InvalidLength => write!(f, "Invalid length"),
            BootloaderError::VerificationFailed => write!(f, "Verification failed"),
            BootloaderError::MemoryError => write!(f, "Memory error"),
            BootloaderError::ProtocolError => write!(f, "Protocol error"),
        }
    }
}

impl core::error::Error for BootloaderError {}

pub struct Bootloader<H: S32KHal> {
    hal: H,
    protocol: Protocol<H::Can>,
    memory: MemoryManager<H>,
    is_programming_enabled: bool,
}

impl<H: S32KHal> Bootloader<H> {
    pub fn new(hal: H) -> Result<Self, BootloaderError> {
        let can = hal.get_can();
        let memory = MemoryManager::new(hal.clone())?;
        
        Ok(Self {
            hal,
            protocol: Protocol::new(can),
            memory,
            is_programming_enabled: false,
        })
    }

    pub fn run(&mut self) -> Result<(), BootloaderError> {
        match self.state {
            BootloaderState::Idle => {
                if self.hal.is_programming_pin_active() {
                    self.enter_programming_mode()?;
                } else {
                    self.jump_to_application()?;
                }
            }
            BootloaderState::Programming => {
                self.handle_can_messages()?;
            }
            BootloaderState::Verifying => {
                let (address, data) = self.protocol.get_firmware_data()
                    .map_err(|_| BootloaderError::ProtocolError)?;
                self.verify_firmware(address, data)?;
            }
            BootloaderState::Rebooting => {
                self.exit_programming_mode()?;
            }
        }
        Ok(())
    }

    fn should_enter_programming_mode(&self) -> bool {
        // Basic entry conditions:
        // 1. Check if programming request pin is active
        // 2. Check if application is valid
        // 3. Check if programming mode is requested via CAN

        // TODO: Implement actual pin check
        let programming_pin_active = false;

        // Check if application is valid
        let application_valid = self.is_application_valid();

        // Check if programming mode is requested via CAN
        let programming_requested = self.protocol.is_programming_requested();

        programming_pin_active || !application_valid || programming_requested
    }

    pub fn is_application_valid(&self) -> bool {
        // Check if application exists in flash
        let app_start = self.memory.get_app_start();
        let app_end = self.memory.get_app_end();

        // Read first word of application
        let mut first_word = [0u8; 4];
        if self.memory.read_memory(app_start, &mut first_word).is_err() {
            return false;
        }

        // Check if first word is a valid stack pointer (should be in RAM)
        let sp = u32::from_le_bytes(first_word);
        if sp < 0x2000_0000 || sp > 0x2004_0000 {
            return false;
        }

        // Check if application has valid vector table
        let mut reset_vector = [0u8; 4];
        if self.memory.read_memory(app_start + 4, &mut reset_vector).is_err() {
            return false;
        }

        // Reset vector should point to application code
        let reset_addr = u32::from_le_bytes(reset_vector);
        if reset_addr < app_start || reset_addr > app_end {
            return false;
        }

        true
    }

    fn jump_to_application(&self) -> Result<(), BootloaderError> {
        self.hal
            .jump_to_application(self.memory.get_application_start())
            .map_err(|_| BootloaderError::Hardware("Failed to jump to application"))
    }

    fn enter_programming_mode(&mut self) -> Result<(), BootloaderError> {
        self.hal
            .enter_programming_mode()
            .map_err(|_| BootloaderError::Hardware("Failed to enter programming mode"))?;
        self.is_programming_enabled = true;
        Ok(())
    }

    fn exit_programming_mode(&mut self) -> Result<(), BootloaderError> {
        self.hal
            .exit_programming_mode()
            .map_err(|_| BootloaderError::Hardware("Failed to exit programming mode"))?;
        self.is_programming_enabled = false;
        Ok(())
    }

    fn handle_can_messages(&mut self) -> Result<(), BootloaderError> {
        self.protocol
            .run()
            .map_err(|_| BootloaderError::ProtocolError)
    }

    fn process_commands(&mut self) -> Result<(), BootloaderError> {
        self.protocol
            .run()
            .map_err(|_| BootloaderError::ProtocolError)
    }

    fn program_firmware(&mut self) -> Result<(), BootloaderError> {
        self.state = BootloaderState::Programming;
        Ok(())
    }

    pub fn verify_firmware(&mut self, address: u32, data: &[u8]) -> Result<(), BootloaderError> {
        let mut verify_data = [0u8; 1024]; // Fixed-size buffer
        let mut remaining = data.len();
        let mut offset = 0;

        while remaining > 0 {
            let chunk_size = remaining.min(verify_data.len());
            self.memory
                .read(address + offset as u32, &mut verify_data[..chunk_size])
                .map_err(|_| BootloaderError::MemoryError)?;

            if &verify_data[..chunk_size] != &data[offset..offset + chunk_size] {
                return Err(BootloaderError::VerificationFailed);
            }

            remaining -= chunk_size;
            offset += chunk_size;
        }

        Ok(())
    }

    pub fn calculate_firmware_checksum(&mut self) -> Result<u32, BootloaderError> {
        let mut data = [0u8; 1024]; // Fixed-size buffer
        let mut checksum = 0u32;
        let mut remaining = self.memory.get_application_size() as usize;
        let mut offset = 0;

        while remaining > 0 {
            let chunk_size = remaining.min(data.len());
            self.memory
                .read(offset as u32, &mut data[..chunk_size])
                .map_err(|_| BootloaderError::MemoryError)?;

            for &byte in &data[..chunk_size] {
                checksum = checksum.wrapping_add(byte as u32);
            }

            remaining -= chunk_size;
            offset += chunk_size;
        }

        Ok(checksum)
    }

    pub fn get_hal(&self) -> &H {
        &self.hal
    }

    pub fn get_hal_mut(&mut self) -> &mut H {
        &mut self.hal
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
}
