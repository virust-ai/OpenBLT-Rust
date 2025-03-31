#![no_std]

use s32k148_hal::{S32K148, S32KHal};

pub struct Clock {
    hal: S32K148,
}

impl Clock {
    pub fn new(hal: S32K148) -> Self {
        Self { hal }
    }

    pub fn init(&mut self) -> Result<(), <S32K148 as S32KHal>::Error> {
        // TODO: Initialize clock configuration
        Ok(())
    }
} 
