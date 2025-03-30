// Hardware Abstraction Layer for S32K series

pub mod s32k118;
pub mod s32k148;

use embedded_can::Can;

#[derive(Debug)]
pub enum HalError {
    FlashError,
    CanError,
    InvalidOperation,
    ProgrammingModeError,
    JumpError,
}

impl core::fmt::Display for HalError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            HalError::FlashError => write!(f, "Flash error"),
            HalError::CanError => write!(f, "CAN error"),
            HalError::InvalidOperation => write!(f, "Invalid operation"),
            HalError::ProgrammingModeError => write!(f, "Programming mode error"),
            HalError::JumpError => write!(f, "Jump error"),
        }
    }
}

impl core::error::Error for HalError {}

pub trait S32KHal {
    type Can: Can;
    type Error: Into<HalError>;

    fn init() -> Result<Self, HalError>
    where
        Self: Sized;
    fn get_can(&self) -> Self::Can;
    fn get_can_mut(&mut self) -> &mut Self::Can;
    fn enter_programming_mode(&mut self) -> Result<(), Self::Error>;
    fn exit_programming_mode(&mut self) -> Result<(), Self::Error>;
    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error>;
    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error>;
    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error>;
    fn is_programming_pin_active(&self) -> bool;
    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error>;
}
