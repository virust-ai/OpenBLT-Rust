// Hardware Abstraction Layer for S32K series

pub mod s32k118;
pub mod s32k148;

use embedded_can::Can;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HalError {
    #[error("CAN communication error")]
    CanError,
    #[error("Hardware initialization error")]
    InitError,
    #[error("Invalid configuration")]
    ConfigError,
}

pub trait S32KHal {
    type Can: Can;
    
    fn init() -> Result<Self, HalError> where Self: Sized;
    fn get_can(&self) -> &Self::Can;
    fn get_can_mut(&mut self) -> &mut Self::Can;
} 
