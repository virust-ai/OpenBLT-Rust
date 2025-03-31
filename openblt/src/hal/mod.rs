// Hardware Abstraction Layer for S32K series

#![no_std]

use core::fmt;
pub use embedded_can::blocking::Can as EmbeddedCan;
use embedded_can::ErrorKind;

// Common error types
#[derive(Debug)]
pub enum HalError {
    InvalidState,
    HardwareError,
    FlashError,
    CanError,
}

impl embedded_can::Error for HalError {
    fn kind(&self) -> ErrorKind {
        match self {
            HalError::InvalidState => ErrorKind::Other,
            HalError::HardwareError => ErrorKind::Other,
            HalError::FlashError => ErrorKind::Other,
            HalError::CanError => ErrorKind::Other,
        }
    }
}

#[derive(Debug)]
pub enum FlashError {
    InvalidAddress,
    InvalidLength,
    WriteError,
    EraseError,
    ReadError,
    Busy,
    Timeout,
}

impl fmt::Display for FlashError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FlashError::InvalidAddress => write!(f, "Invalid flash address"),
            FlashError::InvalidLength => write!(f, "Invalid flash length"),
            FlashError::WriteError => write!(f, "Flash write error"),
            FlashError::EraseError => write!(f, "Flash erase error"),
            FlashError::ReadError => write!(f, "Flash read error"),
            FlashError::Busy => write!(f, "Flash controller busy"),
            FlashError::Timeout => write!(f, "Flash operation timeout"),
        }
    }
}

// Hardware Abstraction Layer trait
pub trait S32KHal {
    type Can: EmbeddedCan;
    type Error: core::fmt::Debug;

    fn init() -> Result<Self, Self::Error> where Self: Sized;
    fn get_can(self) -> Self::Can;
    fn get_can_mut(&mut self) -> &mut Self::Can;
    fn is_programming_pin_active(&self) -> bool;
    fn enter_programming_mode(&mut self) -> Result<(), Self::Error>;
    fn exit_programming_mode(&mut self) -> Result<(), Self::Error>;
    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error>;
    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error>;
    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error>;
    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error>;
}

// Platform-specific implementations
pub mod s32k118;
pub mod s32k148;
