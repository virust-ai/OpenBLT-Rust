use crate::hal::{S32KHal, EmbeddedCan, FlashError};
use embedded_can::blocking::Can as EmbeddedCanTrait;
use embedded_can::ErrorKind;
use embedded_can::{Frame, Id, StandardId};

#[derive(Clone)]
pub struct S32K118Frame {
    pub id: Id,
    pub data: [u8; 8],
    pub dlc: usize,
}

impl Frame for S32K118Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if data.len() > 8 {
            return None;
        }
        let mut frame_data = [0u8; 8];
        frame_data[..data.len()].copy_from_slice(data);
        Some(Self {
            id: id.into(),
            data: frame_data,
            dlc: data.len(),
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        if dlc > 8 {
            return None;
        }
        Some(Self {
            id: id.into(),
            data: [0u8; 8],
            dlc,
        })
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn is_extended(&self) -> bool {
        match self.id {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.dlc]
    }
}

pub struct S32K118Can {
    // TODO: Add CAN registers
}

impl EmbeddedCanTrait for S32K118Can {
    type Error = ErrorKind;
    type Frame = S32K118Frame;

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        // TODO: Implement CAN transmission
        Ok(())
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        // TODO: Implement CAN reception
        Ok(S32K118Frame::new(Id::Standard(StandardId::new(0).unwrap()), &[]).unwrap())
    }
}

pub struct S32K118Hal {
    can: S32K118Can,
}

impl S32KHal for S32K118Hal {
    type Can = S32K118Can;
    type Error = FlashError;

    fn init() -> Result<Self, Self::Error> {
        // TODO: Initialize S32K118 hardware
        Ok(Self { can: S32K118Can {} })
    }

    fn get_can(self) -> Self::Can {
        self.can
    }

    fn get_can_mut(&mut self) -> &mut Self::Can {
        &mut self.can
    }

    fn is_programming_pin_active(&self) -> bool {
        // TODO: Implement programming pin check
        false
    }

    fn enter_programming_mode(&mut self) -> Result<(), Self::Error> {
        // TODO: Implement programming mode entry
        Ok(())
    }

    fn exit_programming_mode(&mut self) -> Result<(), Self::Error> {
        // TODO: Implement programming mode exit
        Ok(())
    }

    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error> {
        // TODO: Implement flash erasure
        Ok(())
    }

    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error> {
        // TODO: Implement flash writing
        Ok(())
    }

    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        // TODO: Implement flash reading
        Ok(())
    }

    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error> {
        // TODO: Implement application jump
        Ok(())
    }
}
