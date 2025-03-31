use embedded_can::blocking::Can;

pub trait S32KHal {
    type Can;
    type Error;

    fn init() -> Result<Self, Self::Error>
    where
        Self: Sized;

    fn get_can(&self) -> &Self::Can;
    fn get_can_mut(&mut self) -> &mut Self::Can;

    fn enter_programming_mode(&mut self) -> Result<(), Self::Error>;
    fn exit_programming_mode(&mut self) -> Result<(), Self::Error>;

    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error>;
    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error>;
    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error>;

    fn is_programming_pin_active(&self) -> bool;
    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error>;
} 
