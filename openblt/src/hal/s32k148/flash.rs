#![no_std]

use core::ptr::NonNull;
use core::cell::UnsafeCell;
use crate::hal::{FlashError, HalError};

const FLASH_BASE: u32 = 0x00000000;
const FLASH_PAGE_SIZE: u32 = 4096; // 4KB

#[repr(C)]
pub struct FlashRegisters {
    fstat: UnsafeCell<u32>,
    fcmd: UnsafeCell<u32>,
    faddr: UnsafeCell<u32>,
    fdata: UnsafeCell<u32>,
}

pub struct Flash {
    registers: NonNull<FlashRegisters>,
}

impl Flash {
    pub unsafe fn new() -> Self {
        Self {
            registers: NonNull::new_unchecked(FLASH_BASE as *mut FlashRegisters),
        }
    }

    unsafe fn registers(&mut self) -> &mut FlashRegisters {
        self.registers.as_mut()
    }

    pub fn erase(&mut self, address: u32, length: u32) -> Result<(), HalError> {
        // TODO: Implement flash erase
        Ok(())
    }

    pub fn write(&mut self, address: u32, data: &[u8]) -> Result<(), HalError> {
        // TODO: Implement flash write
        Ok(())
    }

    pub fn read(&self, address: u32, data: &mut [u8]) -> Result<(), HalError> {
        // TODO: Implement flash read
        Ok(())
    }
} 
