#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use s32k148_board::Board;
use s32k148_hal::{S32K148, CanDevice, Flash, CanRegisters};

#[entry]
fn main() -> ! {
    // Initialize the HAL
    let can_registers = unsafe { &mut *(0x40024000 as *mut CanRegisters) };
    let can = CanDevice::new(can_registers);
    let flash = Flash::new();
    let hal = S32K148::new(can, flash);
    
    // Initialize the board
    let mut board = Board::new(hal);
    
    // Main bootloader loop
    loop {
        // TODO: Implement bootloader logic
        cortex_m::asm::nop();
    }
} 
