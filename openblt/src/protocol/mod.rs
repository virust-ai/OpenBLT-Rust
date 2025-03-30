use embedded_can::{Can, Frame, Id, StandardId};
use nb::block;
use s32k148_hal::S32K148Frame;

mod memory;
use memory::{Memory, MemoryError};

#[derive(Debug)]
pub enum ProtocolError {
    InvalidCommand,
    InvalidLength,
    ChecksumError,
    CommunicationError,
    TimeoutError,
    ProgrammingNotEnabled,
    MemoryError(MemoryError),
}

impl core::fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ProtocolError::InvalidCommand => write!(f, "Invalid command"),
            ProtocolError::InvalidLength => write!(f, "Invalid data length"),
            ProtocolError::ChecksumError => write!(f, "Checksum error"),
            ProtocolError::CommunicationError => write!(f, "Communication error"),
            ProtocolError::TimeoutError => write!(f, "Timeout error"),
            ProtocolError::ProgrammingNotEnabled => write!(f, "Programming not enabled"),
            ProtocolError::MemoryError(e) => write!(f, "Memory error: {}", e),
        }
    }
}

impl core::error::Error for ProtocolError {}

impl From<MemoryError> for ProtocolError {
    fn from(error: MemoryError) -> Self {
        ProtocolError::MemoryError(error)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Command {
    GetProtocolVersion = 0x01,
    SetProgrammingEnabled = 0x02,
    GetProgrammingEnabled = 0x03,
    EraseMemory = 0x04,
    WriteData = 0x05,
    ReadData = 0x06,
    GetChecksum = 0x07,
    Reboot = 0x08,
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

        // Parse address and length from data
        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        // Erase memory
        self.memory.erase(address, length)?;

        let response = [0x00]; // Success
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_write_data(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if !self.is_programming_enabled {
            return Err(ProtocolError::ProgrammingNotEnabled);
        }

        // Parse address and data
        if data.len() < 4 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let write_data = &data[4..];

        // Write data to memory
        self.memory.write(address, write_data)?;

        let response = [0x00]; // Success
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_read_data(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if !self.is_programming_enabled {
            return Err(ProtocolError::ProgrammingNotEnabled);
        }

        // Parse address and length
        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        // Read data from memory
        let read_data = self.memory.read(address, length)?;

        // Send response with data
        self.send_response(read_data)?;
        Ok(())
    }

    fn handle_get_checksum(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        if !self.is_programming_enabled {
            return Err(ProtocolError::ProgrammingNotEnabled);
        }

        // Parse address and length
        if data.len() < 8 {
            return Err(ProtocolError::InvalidLength);
        }

        let address = u32::from_le_bytes(data[0..4].try_into().unwrap());
        let length = u32::from_le_bytes(data[4..8].try_into().unwrap());

        // Calculate checksum
        let checksum = self.memory.calculate_checksum(address, length)?;

        // Send response with checksum
        let response = checksum.to_le_bytes();
        self.send_response(&response)?;
        Ok(())
    }

    fn handle_reboot(&mut self) -> Result<(), ProtocolError> {
        // Send success response before rebooting
        let response = [0x00];
        self.send_response(&response)?;
        Ok(())
    }

    fn send_response(&mut self, data: &[u8]) -> Result<(), ProtocolError> {
        // Create response frame
        let frame = C::Frame::new(self.command_id, data)
            .ok_or(ProtocolError::InvalidLength)?;

        // Send frame
        self.can.transmit(&frame)
            .map_err(|_| ProtocolError::CommunicationError)
    }
}
