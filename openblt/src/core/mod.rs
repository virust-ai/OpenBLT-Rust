use crate::hal::S32KHal;
use crate::protocol::{Command, Protocol, ProtocolError};
use thiserror::Error;

mod memory;
use memory::{MemoryManager, MemoryManagementError};

#[derive(Error, Debug)]
pub enum BootloaderError {
    #[error("Protocol error: {0}")]
    Protocol(#[from] ProtocolError),
    #[error("Hardware error: {0}")]
    Hardware(String),
    #[error("Invalid firmware")]
    InvalidFirmware,
    #[error("Verification failed")]
    VerificationFailed,
    #[error("Programming failed")]
    ProgrammingFailed,
    #[error("Memory error: {0}")]
    Memory(#[from] MemoryManagementError),
    #[error("Invalid entry condition")]
    InvalidEntryCondition,
}

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
        
        Self {
            hal,
            protocol: Protocol::new(can),
            memory,
            is_programming_enabled: false,
        }
    }

    pub fn run(&mut self) -> Result<(), BootloaderError> {
        // Check entry conditions
        if self.should_enter_programming_mode() {
            self.enter_programming_mode()?;
        } else {
            // Jump to application if valid
            self.jump_to_application()?;
            return Ok(());
        }

        // Main bootloader loop
        loop {
            // Handle incoming CAN messages
            self.handle_can_messages()?;

            // Process commands
            self.process_commands()?;

            // Handle firmware programming
            if self.is_programming_enabled {
                self.program_firmware()?;
            }

            cortex_m::asm::nop();
        }
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

    fn is_application_valid(&self) -> bool {
        // Basic application validation:
        // 1. Check if application region is not empty (not all 0xFF)
        // 2. Check if application has valid entry point
        // 3. Check if application checksum is valid

        // Read first word of application
        let mut data = [0u8; 4];
        if let Ok(()) = self.memory.read_region(
            self.memory.get_application_start(),
            &mut data
        ) {
            let first_word = u32::from_le_bytes(data);
            
            // Check if not erased (0xFFFFFFFF)
            if first_word == 0xFFFFFFFF {
                return false;
            }

            // TODO: Add more validation (entry point, checksum)
            true
        } else {
            false
        }
    }

    fn jump_to_application(&self) -> Result<(), BootloaderError> {
        // Get application entry point
        let mut data = [0u8; 4];
        self.memory.read_region(
            self.memory.get_application_start(),
            &mut data
        )?;

        let entry_point = u32::from_le_bytes(data);

        // TODO: Implement actual jump to application
        // This will be implemented in the HAL
        Ok(())
    }

    fn enter_programming_mode(&mut self) -> Result<(), BootloaderError> {
        // Initialize programming mode
        self.is_programming_enabled = true;
        self.hal.enter_programming_mode()?;
        Ok(())
    }

    fn exit_programming_mode(&mut self) -> Result<(), BootloaderError> {
        // Clean up programming mode
        self.is_programming_enabled = false;
        self.hal.exit_programming_mode()?;
        Ok(())
    }

    fn handle_can_messages(&mut self) -> Result<(), BootloaderError> {
        // Handle incoming CAN messages
        // This is handled by the protocol module
        Ok(())
    }

    fn process_commands(&mut self) -> Result<(), BootloaderError> {
        // Process any pending commands
        // This is handled by the protocol module
        Ok(())
    }

    fn program_firmware(&mut self) -> Result<(), BootloaderError> {
        // Erase application region
        self.memory.erase_region(
            self.memory.get_application_start(),
            self.memory.get_application_size()
        )?;

        // Program firmware
        // This will be handled by the protocol module's write commands
        // The protocol module will call verify_firmware after each write

        Ok(())
    }

    pub fn verify_firmware(&mut self, address: u32, data: &[u8]) -> Result<(), BootloaderError> {
        // Verify firmware data
        let mut verify_data = vec![0u8; data.len()];
        self.memory.read_region(address, &mut verify_data)?;

        // Compare data
        if verify_data != data {
            return Err(BootloaderError::VerificationFailed);
        }

        Ok(())
    }

    pub fn calculate_firmware_checksum(&mut self) -> Result<u32, BootloaderError> {
        // Calculate checksum of entire firmware region
        let mut data = vec![0u8; self.memory.get_application_size() as usize];
        self.memory.read_region(self.memory.get_application_start(), &mut data)?;

        let mut checksum = 0u32;
        for chunk in data.chunks(4) {
            let word = u32::from_le_bytes(chunk.try_into().unwrap());
            checksum ^= word;
        }

        Ok(checksum)
    }
}
