use s32k148_hal::{
    S32K148,
    S32KHal,
    CanDevice,
    Flash,
    can::CanError,
    flash::Error as FlashError,
};

pub struct Board {
    hal: S32K148,
}

impl Board {
    pub fn new(hal: S32K148) -> Self {
        Self { hal }
    }

    pub fn init_can(&mut self) {
        // Initialize CAN
        self.hal.get_can_mut().init();
    }

    pub fn init_flash(&mut self) {
        // Flash is already initialized in S32K148::new()
    }

    pub fn check_programming_request(&mut self) -> bool {
        // Check if programming pin is active
        if self.hal.is_programming_pin_active() {
            return true;
        }

        // Check for CAN programming request
        if let Ok((id, _, _)) = self.hal.get_can_mut().receive_frame() {
            if id == 0x7E0 {
                return true;
            }
        }
        false
    }

    pub fn enter_programming_mode(&mut self) -> Result<(), FlashError> {
        self.hal.enter_programming_mode()
    }

    pub fn exit_programming_mode(&mut self) -> Result<(), FlashError> {
        self.hal.exit_programming_mode()
    }

    pub fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), FlashError> {
        self.hal.erase_flash(address, length)
    }

    pub fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), FlashError> {
        self.hal.write_flash(address, data)
    }

    pub fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), FlashError> {
        self.hal.read_flash(address, data)
    }

    pub fn jump_to_application(&self, entry_point: u32) -> Result<(), FlashError> {
        self.hal.jump_to_application(entry_point)
    }
} 
