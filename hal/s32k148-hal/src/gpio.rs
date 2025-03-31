#![no_std]

#[derive(Copy, Clone)]
pub enum Port {
    A,
    B,
    C,
    D,
    E,
}

pub struct Pin {
    port: Port,
    pin: u8,
}

impl Pin {
    pub fn new(port: Port, pin: u8) -> Self {
        Self { port, pin }
    }

    pub fn set_mux(&self, mux: u8) {
        // TODO: Implement pin multiplexing
        // This will be implemented when we add proper GPIO support
    }
} 
