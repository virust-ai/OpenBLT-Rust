#![no_std]
#![no_main]

use cortex_m_rt::entry;
use log::info;
use s32k148_hal::{S32K148Hal, CanRegisters, Flash};
use openblt::core::Bootloader;

mod core;
mod hal;
mod protocol;
mod utils;

#[entry]
fn main() -> ! {
    // Initialize hardware
    let mut hal = S32K148Hal::init().expect("Failed to initialize hardware");
    let mut bootloader = Bootloader::new(hal).expect("Failed to create bootloader");

    // Test entry conditions
    let is_programming_pin_active = bootloader.get_hal().is_programming_pin_active();
    let application_valid = check_application_validity(&bootloader);

    // Send debug information via CAN
    let can = bootloader.get_hal_mut().get_can_mut();
    send_debug_info(can, is_programming_pin_active, application_valid);

    // Enter bootloader main loop with LED feedback
    loop {
        if is_programming_pin_active {
            // Blink LED rapidly (programming mode)
            bootloader.get_hal_mut().blink_led(100, 100);
        } else if application_valid {
            // Blink LED slowly (normal boot)
            bootloader.get_hal_mut().blink_led(500, 500);
        } else {
            // Blink LED with long pulses (error)
            bootloader.get_hal_mut().blink_led(1000, 100);
        }
    }
}

fn check_application_validity(bootloader: &Bootloader<S32K148Hal>) -> bool {
    // Read application signature
    let mut signature = [0u8; 4];
    if bootloader.get_hal().read_flash(bootloader.get_memory().get_application_start(), &mut signature).is_err() {
        return false;
    }

    // Check if signature matches expected value (0xAA55AA55)
    signature == [0x55, 0xAA, 0x55, 0xAA]
}

fn send_debug_info(can: &mut CanRegisters, is_programming: bool, app_valid: bool) {
    // Create debug frame
    let mut data = [0u8; 8];
    data[0] = if is_programming { 0x01 } else { 0x00 };
    data[1] = if app_valid { 0x01 } else { 0x00 };
    
    // TODO: Send debug frame via CAN
    // This will be implemented in the HAL
}
