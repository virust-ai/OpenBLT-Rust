#![no_std]
#![no_main]

use cortex_m_rt::entry;
use log::info;

mod core;
mod hal;
mod protocol;
mod utils;

#[entry]
fn main() -> ! {
    // Initialize logging
    info!("OpenBLT Rust Bootloader Starting...");

    // TODO: Initialize hardware
    // TODO: Initialize CAN communication
    // TODO: Enter bootloader main loop

    loop {
        // Main bootloader loop will go here
        cortex_m::asm::nop();
    }
}
