use core::ptr::{read_volatile, write_volatile};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Busy error")]
    BusyError,
    #[error("Command sequence error")]
    CommandSequenceError,
    #[error("Write error")]
    WriteError,
    #[error("Erase error")]
    EraseError,
}

const FLASH_BASE: *mut u32 = 0x0000_0000 as *mut u32;
const FLASH_CMD_REG: *mut u32 = 0x4002_0000 as *mut u32; // Flash command register
const FLASH_STAT_REG: *mut u32 = 0x4002_0004 as *mut u32; // Flash status register

const CMD_ERASE_SECTOR: u32 = 0x09;
const CMD_PROGRAM: u32 = 0x20;
const CMD_READ: u32 = 0x00;

const STAT_BUSY: u32 = 0x01;
const STAT_ERROR: u32 = 0x02;
const STAT_READY: u32 = 0x04;

pub struct FlashController {
    base: *mut u32,
}

impl FlashController {
    pub fn new() -> Self {
        Self {
            base: FLASH_BASE,
        }
    }

    pub fn erase_sector(&mut self, sector_addr: u32) -> Result<(), ControllerError> {
        // Wait for flash to be ready
        self.wait_for_ready()?;

        // Send erase command
        unsafe {
            write_volatile(FLASH_CMD_REG, CMD_ERASE_SECTOR);
        }

        // Write to any address in the sector to trigger erase
        unsafe {
            write_volatile(self.base.add((sector_addr / 4) as usize), 0xFFFFFFFF);
        }

        // Wait for completion
        self.wait_for_ready()?;

        Ok(())
    }

    pub fn program_word(&mut self, addr: u32, data: u32) -> Result<(), ControllerError> {
        // Wait for flash to be ready
        self.wait_for_ready()?;

        // Send program command
        unsafe {
            write_volatile(FLASH_CMD_REG, CMD_PROGRAM);
        }

        // Write data
        unsafe {
            write_volatile(self.base.add((addr / 4) as usize), data);
        }

        // Wait for completion
        self.wait_for_ready()?;

        Ok(())
    }

    pub fn read_word(&self, addr: u32) -> u32 {
        unsafe {
            read_volatile(self.base.add((addr / 4) as usize))
        }
    }

    fn wait_for_ready(&self) -> Result<(), ControllerError> {
        loop {
            let status = unsafe { read_volatile(FLASH_STAT_REG) };
            
            if (status & STAT_ERROR) != 0 {
                return Err(ControllerError::CommandSequenceError);
            }
            
            if (status & STAT_BUSY) == 0 {
                return Ok(());
            }
        }
    }
} 
