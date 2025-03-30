// Hardware Abstraction Layer for S32K series

pub mod s32k118;
pub mod s32k148;

use embedded_can::Can;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HalError {
    #[error("Flash error")]
    FlashError,
    #[error("CAN error")]
    CanError,
    #[error("Invalid operation")]
    InvalidOperation,
}

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
}
