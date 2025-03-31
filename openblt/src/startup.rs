#![no_std]
#![no_main]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
use s32k148_board::S32K148Board;

#[entry]
fn main() -> ! {
    // Initialize hardware
    let board = S32K148Board::init().unwrap();
    
    // Initialize bootloader
    let mut bootloader = openblt::Bootloader::new(board);
    
    // Main bootloader loop
    loop {
        // Check entry conditions
        if bootloader.is_programming_pin_active() {
            // Programming mode - LED should blink rapidly (100ms on/off)
            bootloader.get_hal_mut().blink_led(100, 100);
        } else if bootloader.is_application_valid() {
            // Normal boot - LED should blink slowly (500ms on/off)
            bootloader.get_hal_mut().blink_led(500, 500);
        } else {
            // Error state - LED should pulse long (1000ms on/off)
            bootloader.get_hal_mut().blink_led(1000, 1000);
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Turn on LED to indicate panic
    if let Ok(mut board) = S32K148Board::init() {
        board.set_led(true);
    }
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn DefaultHandler() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn HardFault() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn MemManage() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn BusFault() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn UsageFault() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn SVCall() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn PendSV() -> ! {
    loop {
        cortex_m::asm::nop();
    }
}

#[no_mangle]
pub extern "C" fn SysTick() -> ! {
    loop {
        cortex_m::asm::nop();
    }
} 
