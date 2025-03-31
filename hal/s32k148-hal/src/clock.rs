#![no_std]

pub struct Clock {
    // Clock configuration will be added later
}

impl Clock {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_peripheral_clock(&self) -> u32 {
        // For now, return a default value
        80000000 // 80MHz
    }
} 
