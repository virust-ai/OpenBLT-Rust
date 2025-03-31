#![no_std]

use core::marker::PhantomData;

pub mod can;
pub mod flash;
pub mod hal;
pub mod uart;
pub mod clock;
pub mod gpio;
pub mod peripheral;
pub mod reg;

pub use can::{CanDevice, CanError, CanRegisters};
pub use flash::{Flash, Error as FlashError};
pub use hal::S32KHal;
pub use uart::{debug_println, init_debug_uart};
pub use clock::Clock;
pub use gpio::{Pin, Port};
pub use peripheral::{Peripheral, PeripheralRef};
pub use reg::Register;

pub struct S32K148 {
    can: CanDevice,
    flash: Flash,
    _phantom: PhantomData<()>,
}

impl S32K148 {
    pub fn new(can: CanDevice, flash: Flash) -> Self {
        Self {
            can,
            flash,
            _phantom: PhantomData,
        }
    }
}

impl S32KHal for S32K148 {
    type Can = CanDevice;
    type Error = FlashError;

    fn init() -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        let registers = unsafe { &mut *(0x4002_4000 as *mut CanRegisters) };
        let can = CanDevice::new(registers);
        let flash = Flash::new();
        
        // Initialize debug UART
        init_debug_uart();
        
        Ok(Self::new(can, flash))
    }

    fn get_can(&self) -> &Self::Can {
        &self.can
    }

    fn get_can_mut(&mut self) -> &mut Self::Can {
        &mut self.can
    }

    fn enter_programming_mode(&mut self) -> Result<(), Self::Error> {
        // TODO: Implement programming mode entry
        Ok(())
    }

    fn exit_programming_mode(&mut self) -> Result<(), Self::Error> {
        // TODO: Implement programming mode exit
        Ok(())
    }

    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error> {
        self.flash.erase(address, length)
    }

    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error> {
        self.flash.write(address, data)
    }

    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        let flash_data = self.flash.read(address, data.len() as u32)?;
        data.copy_from_slice(flash_data);
        Ok(())
    }

    fn is_programming_pin_active(&self) -> bool {
        // TODO: Implement programming pin check
        false
    }

    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error> {
        // TODO: Implement application jump
        Ok(())
    }
}
