#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use s32k148_board::Board;
use s32k148_hal::{S32K148, CanDevice, Flash, CanRegisters, debug_println};
use crate::state::{StateMachine, BootloaderState};

#[entry]
fn main() -> ! {
    // Initialize the HAL
    let can_registers = unsafe { &mut *(0x40024000 as *mut CanRegisters) };
    let can = CanDevice::new(can_registers);
    let flash = Flash::new();
    let hal = S32K148::new(can, flash);
    
    // Initialize the board and state machine
    let mut board = Board::new(hal);
    let mut state_machine = StateMachine::new();
    
    // Print bootloader startup message
    debug_println("S32K148 Bootloader Starting...");
    
    // Main bootloader loop
    loop {
        match state_machine.current_state() {
            BootloaderState::Entry => {
                debug_println("Bootloader Entry State");
                // Check for backdoor entry
                if board.check_programming_request() {
                    state_machine.transition_to(BootloaderState::Idle);
                    debug_println("Backdoor entry detected");
                } else {
                    // Validate application checksum
                    if board.validate_application() {
                        state_machine.set_checksum_valid(true);
                        state_machine.transition_to(BootloaderState::UserProgramActive);
                        debug_println("Valid application found, jumping to application");
                        board.jump_to_application();
                    } else {
                        state_machine.transition_to(BootloaderState::Error);
                        debug_println("Invalid application checksum");
                    }
                }
            }
            
            BootloaderState::Idle => {
                debug_println("Bootloader Idle State");
                // Wait for XCP CONNECT command
                if board.check_programming_request() {
                    state_machine.transition_to(BootloaderState::Programming);
                    debug_println("Programming request detected");
                }
            }
            
            BootloaderState::Programming => {
                debug_println("Programming State");
                match board.enter_programming_mode() {
                    Ok(_) => {
                        debug_println("Entered programming mode");
                        // TODO: Implement XCP protocol handling
                        // This should handle:
                        // 1. XCP CONNECT
                        // 2. XCP SET_MTA
                        // 3. XCP DOWNLOAD
                        // 4. XCP PROGRAM_START
                        // 5. XCP PROGRAM_RESET
                    }
                    Err(_) => {
                        state_machine.transition_to(BootloaderState::Error);
                        debug_println("Failed to enter programming mode");
                    }
                }
            }
            
            BootloaderState::UserProgramActive => {
                // This state should never be reached in the bootloader
                // as we jump to the application
                cortex_m::asm::unreachable();
            }
            
            BootloaderState::Error => {
                debug_println("Error State");
                // Wait for reset or backdoor entry
                if board.check_programming_request() {
                    state_machine.transition_to(BootloaderState::Idle);
                    debug_println("Backdoor entry detected after error");
                }
            }
        }
        
        cortex_m::asm::nop();
    }
} 
