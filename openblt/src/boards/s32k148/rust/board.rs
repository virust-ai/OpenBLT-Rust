#![no_std]

use s32k148_hal::{S32K148, S32KHal};

#[derive(Debug)]
pub enum BoardError {
    HalError(<S32K148 as S32KHal>::Error),
}

pub struct Board {
    hal: S32K148,
}

impl Board {
    pub fn new(hal: S32K148) -> Self {
        Self { hal }
    }

    pub fn init(&mut self) -> Result<(), BoardError> {
        let new_hal = S32K148::init().map_err(BoardError::HalError)?;
        self.hal = new_hal;
        Ok(())
    }

    pub fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), BoardError> {
        self.hal.erase_flash(address, length).map_err(BoardError::HalError)
    }

    pub fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), BoardError> {
        self.hal.write_flash(address, data).map_err(BoardError::HalError)
    }

    pub fn read_flash(&mut self, address: u32, data: &mut [u8]) -> Result<(), BoardError> {
        self.hal.read_flash(address, data).map_err(BoardError::HalError)
    }

    pub fn jump_to_application(&self, entry_point: u32) -> Result<(), BoardError> {
        self.hal.jump_to_application(entry_point).map_err(BoardError::HalError)
    }

    pub fn enter_programming_mode(&mut self) -> Result<(), BoardError> {
        self.hal.enter_programming_mode().map_err(BoardError::HalError)
    }

    pub fn exit_programming_mode(&mut self) -> Result<(), BoardError> {
        self.hal.exit_programming_mode().map_err(BoardError::HalError)
    }

    pub fn is_programming_pin_active(&self) -> bool {
        self.hal.is_programming_pin_active()
    }
} 
