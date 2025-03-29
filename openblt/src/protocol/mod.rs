use embedded_can::Can;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid data length")]
    InvalidLength,
    #[error("Checksum error")]
    ChecksumError,
    #[error("Communication error")]
    CommunicationError,
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

pub struct Protocol<C: Can> {
    can: C,
}

impl<C: Can> Protocol<C> {
    pub fn new(can: C) -> Self {
        Self { can }
    }

    pub fn handle_command(&mut self, cmd: Command) -> Result<(), ProtocolError> {
        match cmd {
            Command::GetProtocolVersion => self.handle_get_protocol_version(),
            Command::SetProgrammingEnabled => self.handle_set_programming_enabled(),
            Command::GetProgrammingEnabled => self.handle_get_programming_enabled(),
            Command::EraseMemory => self.handle_erase_memory(),
            Command::WriteData => self.handle_write_data(),
            Command::ReadData => self.handle_read_data(),
            Command::GetChecksum => self.handle_get_checksum(),
            Command::Reboot => self.handle_reboot(),
        }
    }

    // TODO: Implement command handlers
    fn handle_get_protocol_version(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_set_programming_enabled(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_get_programming_enabled(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_erase_memory(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_write_data(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_read_data(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_get_checksum(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }

    fn handle_reboot(&mut self) -> Result<(), ProtocolError> {
        Ok(())
    }
} 
