#![no_std]

// Core bootloader functionality
pub mod core;
pub mod protocol;
pub mod hal;
pub mod boards;

// Re-export commonly used types
pub use core::Bootloader;
pub use hal::S32KHal;
pub use protocol::Protocol;

pub use boards::s32k148::Board as S32K148Board; 
