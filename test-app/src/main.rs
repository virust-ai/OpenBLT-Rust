#![no_std]
#![no_main]

use cortex_m_rt::entry;
use s32k148_hal::{S32K148Hal, CanRegisters};

#[entry]
fn main() -> ! {
    // Initialize hardware
    let hal = S32K148Hal::init().expect("Failed to initialize hardware");
    
    // Initialize LED for visual feedback
    hal.init_led();
    
    // Blink LED to indicate application is running
    loop {
        hal.set_led(true);
        cortex_m::asm::delay(8_000_000); // ~1 second delay
        hal.set_led(false);
        cortex_m::asm::delay(8_000_000); // ~1 second delay
    }
} 
