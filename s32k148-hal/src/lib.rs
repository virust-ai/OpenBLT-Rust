#![no_std]

use cortex_m::interrupt::free;
use embedded_can::blocking::Can;
use nb::block;
use vcell::VolatileCell;

mod can;
mod flash;
mod hal;

use can::{CanRegisters, CAN0_BASE};
use flash::Flash;
use hal::S32KHal;

// GPIO Register Base Addresses
const PORTA_BASE: u32 = 0x4004_9000;
const PORTB_BASE: u32 = 0x4004_A000;
const PORTC_BASE: u32 = 0x4004_B000;
const PORTD_BASE: u32 = 0x4004_C000;
const PORTE_BASE: u32 = 0x4004_D000;

// GPIO Register Structure
#[repr(C)]
struct GpioRegisters {
    pcr: [VolatileCell<u32>; 32],  // Pin Control Registers
    gpclr: VolatileCell<u32>,      // Global Pin Control Low Register
    gpchr: VolatileCell<u32>,      // Global Pin Control High Register
    isfr: VolatileCell<u32>,       // Interrupt Status Flag Register
    dfer: VolatileCell<u32>,       // Digital Filter Enable Register
    dfcr: VolatileCell<u32>,       // Digital Filter Clock Register
    dfwr: VolatileCell<u32>,       // Digital Filter Width Register
}

// Programming Pin Configuration
const PROGRAMMING_PIN_PORT: u32 = PORTA_BASE;
const PROGRAMMING_PIN_NUM: u8 = 12;  // PA12

#[derive(Debug)]
pub enum S32K148Error {
    CanError,
    FlashError,
    InvalidOperation,
    ProgrammingModeError,
    JumpError,
    GpioError,
}

impl core::fmt::Display for S32K148Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            S32K148Error::CanError => write!(f, "CAN error"),
            S32K148Error::FlashError => write!(f, "Flash error"),
            S32K148Error::InvalidOperation => write!(f, "Invalid operation"),
            S32K148Error::ProgrammingModeError => write!(f, "Programming mode error"),
            S32K148Error::JumpError => write!(f, "Jump error"),
            S32K148Error::GpioError => write!(f, "GPIO error"),
        }
    }
}

impl core::error::Error for S32K148Error {}

impl embedded_can::Error for S32K148Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        embedded_can::ErrorKind::Other
    }
}

pub struct S32K148Hal {
    can: CanRegisters,
    flash: Flash,
    gpio: &'static GpioRegisters,
}

impl S32K148Hal {
    pub fn init() -> Result<Self, S32K148Error> {
        // Initialize CAN
        let can = unsafe { &*(CAN0_BASE as *const CanRegisters) };
        can.init_can().map_err(|_| S32K148Error::CanError)?;

        // Initialize Flash
        let flash = Flash::new();

        // Initialize GPIO
        let gpio = unsafe { &*(PROGRAMMING_PIN_PORT as *const GpioRegisters) };
        
        // Configure programming pin as input with pull-up
        unsafe {
            let pcr = &gpio.pcr[PROGRAMMING_PIN_NUM as usize];
            let mut value = pcr.get();
            value &= !(0x1FF << 0);  // Clear MUX and other fields
            value |= (1 << 8) | (1 << 1);  // Enable pull-up and pull-up enable
            pcr.set(value);
        }

        Ok(Self { can: can.clone(), flash, gpio })
    }

    pub fn init_led(&mut self) {
        // TODO: Initialize LED GPIO
        // Configure LED pin as output
        // Set initial state
    }

    pub fn set_led(&mut self, on: bool) {
        // TODO: Set LED state
        // Set LED pin high/low
    }

    pub fn toggle_led(&mut self) {
        // TODO: Toggle LED state
        // Toggle LED pin
    }

    pub fn blink_led(&mut self, on_ms: u32, off_ms: u32) {
        // Convert milliseconds to CPU cycles (assuming 80MHz clock)
        let on_cycles = on_ms * 80_000;
        let off_cycles = off_ms * 80_000;

        // Turn LED on
        self.set_led(true);
        cortex_m::asm::delay(on_cycles);

        // Turn LED off
        self.set_led(false);
        cortex_m::asm::delay(off_cycles);
    }

    pub fn is_programming_pin_active(&self) -> bool {
        unsafe {
            // Read the programming pin state
            let pcr = &self.gpio.pcr[PROGRAMMING_PIN_NUM as usize];
            let value = pcr.get();
            (value & (1 << 0)) == 0  // Active low
        }
    }
}

impl S32KHal for S32K148Hal {
    type Can = CanRegisters;
    type Error = S32K148Error;

    fn init() -> Result<Self, Self::Error> {
        Self::init()
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
        self.flash
            .erase(address, length)
            .map_err(|_| S32K148Error::FlashError)
    }

    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error> {
        self.flash
            .write(address, data)
            .map_err(|_| S32K148Error::FlashError)
    }

    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        let read_data = self.flash
            .read(address, data.len() as u32)
            .map_err(|_| S32K148Error::FlashError)?;
        
        data.copy_from_slice(read_data);
        Ok(())
    }

    fn is_programming_pin_active(&self) -> bool {
        self.is_programming_pin_active()
    }

    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error> {
        // TODO: Implement jump to application
        Ok(())
    }
}
