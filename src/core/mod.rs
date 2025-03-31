#![no_std]

use crate::S32KHal;
use crate::Protocol;
use crate::MemoryManager;
use core::fmt::Debug;

pub struct Bootloader<H: S32KHal + Clone> {
    hal: H,
    protocol: Protocol<H::Can>,
    memory_manager: MemoryManager<H>,
}

impl<H: S32KHal + Clone> Bootloader<H> {
    pub fn new(hal: H) -> Self {
        let can = hal.clone().get_can();
        Self {
            protocol: Protocol::new(can),
            memory_manager: MemoryManager::new(hal.clone()).expect("Failed to initialize memory manager"),
            hal,
        }
    }

    pub fn init(&mut self) -> Result<(), H::Error> {
        let new_hal = H::init()?;
        self.hal = new_hal;
        Ok(())
    }

    pub fn process(&mut self) -> Result<(), H::Error> {
        Ok(())
    }

    pub fn get_memory_manager(&self) -> &MemoryManager<H> {
        &self.memory_manager
    }
} 
