#![no_std]

use s32k148_hal::{S32K148, S32KHal};

pub struct S32K148Board {
    hal: S32K148,
}

impl S32K148Board {
    pub fn new() -> Result<Self, <S32K148 as S32KHal>::Error> {
        let hal = S32K148::init()?;
        Ok(Self { hal })
    }

    pub fn init(&mut self) -> Result<(), <S32K148 as S32KHal>::Error> {
        let new_hal = S32K148::init()?;
        self.hal = new_hal;
        Ok(())
    }

    pub fn get_can(&self) -> &<S32K148 as S32KHal>::Can {
        self.hal.get_can()
    }

    pub fn get_can_mut(&mut self) -> &mut <S32K148 as S32KHal>::Can {
        self.hal.get_can_mut()
    }

    pub fn enter_programming_mode(&mut self) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.enter_programming_mode()
    }

    pub fn exit_programming_mode(&mut self) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.exit_programming_mode()
    }

    pub fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.erase_flash(address, length)
    }

    pub fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.write_flash(address, data)
    }

    pub fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.read_flash(address, data)
    }

    pub fn is_programming_pin_active(&self) -> bool {
        self.hal.is_programming_pin_active()
    }

    pub fn jump_to_application(&self, entry_point: u32) -> Result<(), <S32K148 as S32KHal>::Error> {
        self.hal.jump_to_application(entry_point)
    }
} 
