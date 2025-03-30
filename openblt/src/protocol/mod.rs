use embedded_can::{Can, Frame, Id, StandardId};
use nb::block;
use s32k148_hal::S32K148Frame;
use thiserror::Error;

mod memory;
use memory::{Memory, MemoryError};

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Timeout error")]
    TimeoutError,
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid length")]
    InvalidLength,
    #[error("Invalid data")]
    InvalidData,
    #[error("Programming not enabled")]
    ProgrammingNotEnabled,
    ChecksumError,
    CommunicationError,
    MemoryError(MemoryError),
}

impl core::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProtocolError::InvalidCommand => write!(f, "Invalid command"),
            ProtocolError::InvalidLength => write!(f, "Invalid length"),
            ProtocolError::ChecksumError => write!(f, "Checksum error"),
            ProtocolError::CommunicationError => write!(f, "Communication error"),
            ProtocolError::TimeoutError => write!(f, "Timeout error"),
            ProtocolError::ProgrammingNotEnabled => write!(f, "Programming not enabled"),
            ProtocolError::MemoryError(e) => write!(f, "Memory error: {}", e),
            ProtocolError::InvalidData => write!(f, "Invalid data"),
        }
    }
}

impl core::error::Error for ProtocolError {}

impl From<MemoryError> for ProtocolError {
    fn from(error: MemoryError) -> Self {
        ProtocolError::MemoryError(error)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
    GetProtocolVersion,
    SetProgrammingEnabled,
    GetProgrammingEnabled,
    EraseMemory,
    WriteData,
    ReadData,
    GetChecksum,
    Reboot,
}

impl Command {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x01 => Some(Command::GetProtocolVersion),
            0x02 => Some(Command::SetProgrammingEnabled),
            0x03 => Some(Command::GetProgrammingEnabled),
            0x04 => Some(Command::EraseMemory),
            0x05 => Some(Command::WriteData),
            0x06 => Some(Command::ReadData),
            0x07 => Some(Command::GetChecksum),
            0x08 => Some(Command::Reboot),
            _ => None,
        }
    }
}

pub struct Protocol<C: Can> {
    can: C,
    is_programming_enabled: bool,
    command_id: StandardId,
    memory: Memory,
    read_buffer: [u8; 1024], // Buffer for read operations
}

impl<C: Can> Protocol<C> {
    pub fn new(can: C) -> Self {
        Self {
            can,
            is_programming_enabled: false,
            command_id: StandardId::new(0x123).unwrap(), // Default command ID
            memory: Memory::new(0x0000_0000, 0x1000_0000), // 16MB flash memory
            read_buffer: [0; 1024],
        }
    }

    pub fn run(&mut self) -> Result<(), ProtocolError> {
        loop {
            // Wait for a command frame
            let frame = block!(self.can.receive()).map_err(|_| ProtocolError::TimeoutError)?;
            
            // Parse command from frame data
            if frame.dlc() < 1 {
                return Err(ProtocolError::InvalidLength);
            }

            let cmd = Command::from_byte(frame.data()[0])
                .ok_or(ProtocolError::InvalidCommand)?;

            // Handle the command
            self.handle_command(cmd, frame.data())?;
        }
    }

    pub fn handle_command(&mut self, cmd: Command, data: &[u8]) -> Result<(), ProtocolError> {
        match cmd {
            Command::GetProtocolVersion => self.handle_get_protocol_version(),
            Command::SetProgrammingEnabled => self.handle_set_programming_enabled(),
            Command::GetProgrammingEnabled => self.handle_get_programming_enabled(),
            Command::EraseMemory => self.handle_erase_memory(data),
            Command::WriteData => self.handle_write_data(data),
            Command::ReadData => self.handle_read_data(data),
            Command::GetChecksum => self.handle_get_checksum(data),
            Command::Reboot => self.handle_reboot(),
        }
    }

    fn handle_get_protocol_version(&mut self) -> Result<(), ProtocolError> {
        // OpenBLT protocol version 1.0
        let response = [0x01, 0x00];
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_set_programming_enabled(&mut self) -> Result<(), ProtocolError> {
        self.is_programming_enabled = true;
        let response = [0x00]; // Success
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_get_programming_enabled(&mut self) -> Result<(), ProtocolError> {
        let response = [if self.is_programming_enabled { 0x01 } else { 0x00 }];
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_erase_memory(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if !self.is_programming_enabled {
            return Err(ProtocolError::ProgrammingNotEnabled);
        }

        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        self.erase_memory(address, length)
    }

    fn handle_write_data(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if !self.is_programming_enabled {
            return Err(ProtocolError::ProgrammingNotEnabled);
        }

        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        self.write_data(address, &data[8..])
    }

    fn handle_read_data(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        self.read_data(address, length)
    }

    fn handle_get_checksum(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        // TODO: Implement checksum calculation
        let response = [0x00, 0x00, 0x00, 0x00]; // Checksum
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_reboot(&mut self) -> Result<(), ProtocolError> {
        // TODO: Implement reboot
        let response = [0x00]; // Success
        self.send_response(&response)?;
        Ok(())
    }

    fn send_response(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        // Create response frame with command ID
        let id = StandardId::new(self.command_id)
            .ok_or(ProtocolError::InvalidCommand)?;
        
        // Create frame with data
        let frame = S32K148Frame::new(id, data)
            .ok_or(ProtocolError::InvalidData)?;

        // Transmit frame
        self.can.transmit(&frame)
            .map_err(|_| ProtocolError::TimeoutError)?;

        Ok(())
    }

    pub fn is_programming_requested(&self) -> bool {
        self.is_programming_enabled
    }

    fn erase_memory(&mut self, address: u32, length: u32) -> Result<(), ProtocolError> {
        // Validate address and length
        if !self.memory.is_valid_address_range(address, length) {
            return Err(ProtocolError::InvalidData);
        }

        // Erase the memory region
        self.memory.erase_region(address, length)
            .map_err(|e| ProtocolError::MemoryError(e))?;

        Ok(())
    }

    fn write_data(&mut self, address: u32, data: &[u8]) -> Result<(), ProtocolError> {
        // Validate address and length
        if !self.memory.is_valid_address_range(address, data.len() as u32) {
            return Err(ProtocolError::InvalidData);
        }

        // Write the data to memory
        self.memory.write_memory(address, data)
            .map_err(|e| ProtocolError::MemoryError(e))?;

        Ok(())
    }

    fn read_data(&mut self, address: u32, length: u32) -> Result<&[u8], ProtocolError> {
        // Validate address and length
        if !self.memory.is_valid_address_range(address, length) {
            return Err(ProtocolError::InvalidData);
        }

        // Ensure read buffer is large enough
        if length as usize > self.read_buffer.len() {
            return Err(ProtocolError::InvalidLength);
        }

        // Read the data into buffer
        self.memory.read_memory(address, &mut self.read_buffer[..length as usize])
            .map_err(|e| ProtocolError::MemoryError(e))?;

        Ok(&self.read_buffer[..length as usize])
    }
}

pub struct Memory {
    base_address: u32,
    size: u32,
}

impl Memory {
    pub fn new(base_address: u32, size: u32) -> Self {
        Self {
            base_address,
            size,
        }
    }

    pub fn read(&self, address: u32, length: u32) -> Result<&[u8], ()> {
        // Validate address and length
        if address < self.base_address || address + length > self.base_address + self.size {
            return Err(());
        }

        // Return a slice to the memory
        let start = address as usize;
        let end = (address + length) as usize;
        Ok(unsafe { core::slice::from_raw_parts(start as *const u8, end - start) })
    }
}
