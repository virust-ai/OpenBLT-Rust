#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use s32k148_board::Board;
use s32k148_hal::{S32K148, CanDevice, Flash, CanRegisters, debug_println};

#[entry]
fn main() -> ! {
    // Initialize the HAL
    let can_registers = unsafe { &mut *(0x40024000 as *mut CanRegisters) };
    let can = CanDevice::new(can_registers);
    let flash = Flash::new();
    let hal = S32K148::new(can, flash);
    
    // Initialize the board
    let mut board = Board::new(hal);
    
    // Print bootloader startup message
    debug_println("S32K148 Bootloader Starting...");
    debug_println("Waiting for programming request...");
    
    // Main bootloader loop
    loop {
        if board.check_programming_request() {
            debug_println("Programming request detected!");
            match board.enter_programming_mode() {
                Ok(_) => {
                    debug_println("Entered programming mode");
                    // TODO: Implement programming protocol
                }
                Err(e) => {
                    debug_println("Failed to enter programming mode");
                }
            }
        }
        cortex_m::asm::nop();
    }
} 
