#![no_std]

use embedded_can::{Frame, Id};

pub struct CanProtocol {
    // TODO: Implement CAN protocol
}

impl CanProtocol {
    pub fn new() -> Self {
        CanProtocol {}
    }

    pub fn process_frame<F: Frame>(&mut self, frame: F) -> Option<F> {
        // TODO: Implement frame processing
        None
    }
} 
