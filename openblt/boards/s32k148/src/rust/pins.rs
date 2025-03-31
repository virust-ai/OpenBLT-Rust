#![no_std]

use s32k148_hal::{S32K148, S32KHal};

pub struct Pins {
    hal: S32K148,
}

impl Pins {
    pub fn new(hal: S32K148) -> Self {
        Self { hal }
    }

    pub fn init(&mut self) -> Result<(), <S32K148 as S32KHal>::Error> {
        // TODO: Initialize pin configuration
        Ok(())
    }
} 
