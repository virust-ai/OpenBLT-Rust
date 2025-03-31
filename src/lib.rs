#![no_std]

// Re-export core functionality from openblt crate
pub use openblt::{
    hal::{S32KHal, EmbeddedCan, FlashError, HalError},
    protocol::Protocol,
    core::{Bootloader, memory::{MemoryManager, MemoryManagementError}},
    boards::s32k148::Board as S32K148Board,
};

// Only declare modules that are specific to this crate
pub mod utils;
