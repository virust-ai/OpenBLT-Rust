#![no_std]

use cortex_m::interrupt::free;
use embedded_can::blocking::Can;
use embedded_can::{Error as CanError, ExtendedId, Frame, Id, StandardId};
use nb;
use openblt::hal::HalError;

mod can;
mod hal;
use can::{BitTiming, CanError as S32KCanError, CanRegisters, Ctrl1, MessageBufferControl, Stat1};
use hal::S32KHal;

pub mod flash;
use flash::{Flash, FlashError};

#[derive(Debug, Clone)]
pub struct S32K148Can {
    registers: CanRegisters,
}

impl S32K148Can {
    fn new(registers: CanRegisters) -> Self {
        Self { registers }
    }
}

impl Can for S32K148Can {
    type Frame = S32K148Frame;
    type Error = S32K148Error;

    fn transmit(&mut self, frame: &Self::Frame) -> Result<(), Self::Error> {
        unsafe {
            // Check if any transmit buffer is available
            let status = Stat1::from_bits_truncate(self.registers.stat1.get());
            if !status.contains(Stat1::TX) {
                return Err(S32K148Error::Can(S32KCanError::TransmitError));
            }

            // Find an available transmit buffer
            for i in 0..16 {
                let mb_cs = self.registers.mb[i].cs.get();
                if (mb_cs & MessageBufferControl::CODE.bits()) == 0 {
                    // Configure message buffer for transmission
                    let id = match frame.id() {
                        Id::Standard(id) => id.as_raw() as u32,
                        Id::Extended(id) => id.as_raw(),
                    };

                    self.registers.mb[i].id.set(id);
                    self.registers.mb[i]
                        .word0
                        .set(u32::from_le_bytes(frame.data[0..4].try_into().unwrap()));
                    self.registers.mb[i]
                        .word1
                        .set(u32::from_le_bytes(frame.data[4..8].try_into().unwrap()));

                    // Set up control word for transmission
                    let cs = MessageBufferControl::CODE.bits() | // Active for transmission
                            (frame.dlc() as u32) << 16; // Data length code
                    self.registers.mb[i].cs.set(cs);

                    return Ok(());
                }
            }

            Err(S32K148Error::Can(S32KCanError::TransmitError))
        }
    }

    fn receive(&mut self) -> Result<Self::Frame, Self::Error> {
        unsafe {
            // Check if any message is available
            let status = Stat1::from_bits_truncate(self.registers.stat1.get());
            if !status.contains(Stat1::RX) {
                return Err(S32K148Error::Can(S32KCanError::ReceiveError));
            }

            // Find a received message
            for i in 0..16 {
                let mb_cs = self.registers.mb[i].cs.get();
                if (mb_cs & MessageBufferControl::CODE.bits()) == 0x4 {
                    // RX_FULL
                    let id = self.registers.mb[i].id.get();
                    let dlc = ((mb_cs >> 16) & 0xF) as usize;

                    let word0 = self.registers.mb[i].word0.get();
                    let word1 = self.registers.mb[i].word1.get();

                    let mut data = [0u8; 8];
                    data[0..4].copy_from_slice(&word0.to_le_bytes());
                    data[4..8].copy_from_slice(&word1.to_le_bytes());

                    // Clear the RX_FULL flag
                    self.registers.mb[i].cs.set(0);

                    // Create frame with appropriate ID type
                    let id = if (id & (1 << 30)) != 0 {
                        Id::Extended(ExtendedId::new(id & 0x1FFFFFFF).unwrap())
                    } else {
                        Id::Standard(StandardId::new((id & 0x7FF) as u16).unwrap())
                    };

                    return S32K148Frame::new(id, &data[..dlc])
                        .ok_or(S32K148Error::Can(S32KCanError::ReceiveError));
                }
            }

            Err(S32K148Error::Can(S32KCanError::ReceiveError))
        }
    }
}

pub struct S32K148Hal {
    can: S32K148Can,
    flash: Flash,
    programming_mode: bool,
}

#[derive(Debug)]
pub enum S32K148Error {
    Can(S32KCanError),
    Flash(FlashError),
    InvalidOperation,
    ProgrammingModeError,
    JumpError,
}

impl core::fmt::Display for S32K148Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            S32K148Error::Can(e) => write!(f, "CAN error: {:?}", e),
            S32K148Error::Flash(e) => write!(f, "Flash error: {:?}", e),
            S32K148Error::InvalidOperation => write!(f, "Invalid operation"),
            S32K148Error::ProgrammingModeError => write!(f, "Programming mode error"),
            S32K148Error::JumpError => write!(f, "Jump error"),
        }
    }
}

impl core::error::Error for S32K148Error {}

impl From<S32KCanError> for S32K148Error {
    fn from(err: S32KCanError) -> Self {
        S32K148Error::Can(err)
    }
}

impl From<FlashError> for S32K148Error {
    fn from(err: FlashError) -> Self {
        S32K148Error::Flash(err)
    }
}

impl From<S32K148Error> for HalError {
    fn from(err: S32K148Error) -> Self {
        match err {
            S32K148Error::Can(_) => HalError::CanError,
            S32K148Error::Flash(_) => HalError::FlashError,
            S32K148Error::InvalidOperation => HalError::InvalidOperation,
            S32K148Error::ProgrammingModeError => HalError::ProgrammingModeError,
            S32K148Error::JumpError => HalError::JumpError,
        }
    }
}

impl CanError for S32K148Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        embedded_can::ErrorKind::Other
    }
}

impl S32K148Hal {
    fn init_can(&mut self) -> Result<(), S32K148Error> {
        // Configure CAN peripheral
        unsafe {
            // Enable CAN clock and configure pins (TODO: Add proper clock and pin config)
            
            // Enter freeze mode
            let ctrl1 = self.can.registers.ctrl1.get();
            self.can.registers.ctrl1.set(ctrl1 & !Ctrl1::CAN_EN.bits());
            self.can.registers.ctrl1.set(ctrl1 | (Ctrl1::HALT.bits() | Ctrl1::FRZ.bits()));
            
            // Configure bit timing for 500kbps at 80MHz clock:
            // Prescaler = 4, Prop_Seg = 7, Phase_Seg1 = 4, Phase_Seg2 = 4, RJW = 4
            let timing = BitTiming::PRESDIV.bits() & (3 << 16) |
                        BitTiming::PROPSEG.bits() & (6 << 0) |
                        BitTiming::PSEG1.bits() & (3 << 3) |
                        BitTiming::PSEG2.bits() & (3 << 6) |
                        BitTiming::RJW.bits() & (3 << 9);
            self.can.registers.btr.set(timing);
            
            // Configure message buffers
            for i in 0..16 {
                self.can.registers.mb[i].cs.set(0); // Inactive
            }
            
            // Exit freeze mode and enable CAN
            let ctrl1 = self.can.registers.ctrl1.get();
            self.can.registers.ctrl1.set(ctrl1 | Ctrl1::CAN_EN.bits());
            self.can.registers.ctrl1.set(ctrl1 & !(Ctrl1::HALT.bits() | Ctrl1::FRZ.bits()));
        }
        
        Ok(())
    }
}

impl S32KHal for S32K148Hal {
    type Can = S32K148Can;
    type Error = S32K148Error;

    fn init() -> Result<Self, HalError> {
        let mut hal = Self {
            can: S32K148Can::new(CanRegisters::default()),
            flash: Flash::new(),
            programming_mode: false,
        };
        
        hal.init_can().map_err(Into::into)?;
        Ok(hal)
    }

    fn get_can(&self) -> Self::Can {
        self.can.clone()
    }

    fn get_can_mut(&mut self) -> &mut Self::Can {
        &mut self.can
    }

    fn enter_programming_mode(&mut self) -> Result<(), Self::Error> {
        // Disable interrupts
        unsafe {
            cortex_m::interrupt::disable();
        }

        // Configure system for programming mode
        // TODO: Add specific S32K148 programming mode setup

        self.programming_mode = true;
        Ok(())
    }

    fn exit_programming_mode(&mut self) -> Result<(), Self::Error> {
        // Re-enable interrupts
        unsafe {
            cortex_m::interrupt::enable();
        }

        // Restore system configuration
        // TODO: Add specific S32K148 programming mode cleanup

        self.programming_mode = false;
        Ok(())
    }

    fn erase_flash(&mut self, address: u32, length: u32) -> Result<(), Self::Error> {
        self.flash.erase(address, length)
            .map_err(S32K148Error::Flash)
    }

    fn write_flash(&mut self, address: u32, data: &[u8]) -> Result<(), Self::Error> {
        self.flash.write(address, data)
            .map_err(S32K148Error::Flash)
    }

    fn read_flash(&self, address: u32, data: &mut [u8]) -> Result<(), Self::Error> {
        let read_data = self.flash.read(address, data.len() as u32)
            .map_err(S32K148Error::Flash)?;
        
        data.copy_from_slice(read_data);
        Ok(())
    }

    fn is_programming_pin_active(&self) -> bool {
        // TODO: Implement actual pin check
        // For now, return false to indicate no programming request
        false
    }

    fn jump_to_application(&self, entry_point: u32) -> Result<(), Self::Error> {
        // Validate entry point
        if entry_point < 0x0001_0000 || entry_point > 0x00FF_FFFF {
            return Err(S32K148Error::JumpError);
        }

        // Disable interrupts
        unsafe {
            cortex_m::interrupt::disable();
        }

        // Set up jump
        unsafe {
            let jump: fn() -> ! = core::mem::transmute(entry_point as *const ());
            jump();
        }

        // Note: This point should never be reached
        Err(S32K148Error::JumpError)
    }
}

// Custom CAN frame implementation for S32K148
#[derive(Debug, Clone, Copy)]
pub struct S32K148Frame {
    id: Id,
    data: [u8; 8],
    dlc: usize,
    is_remote: bool,
}

impl Frame for S32K148Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        let id = id.into();
        let mut frame_data = [0u8; 8];
        let len = data.len().min(8);
        frame_data[..len].copy_from_slice(&data[..len]);

        Some(S32K148Frame {
            id,
            data: frame_data,
            dlc: len,
            is_remote: false,
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: usize) -> Option<Self> {
        if dlc > 8 {
            return None;
        }

        Some(S32K148Frame {
            id: id.into(),
            data: [0; 8],
            dlc,
            is_remote: true,
        })
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

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        self.is_remote
    }
}

impl From<S32K148Error> for HalError {
    fn from(err: S32K148Error) -> Self {
        match err {
            S32K148Error::Can(_) => HalError::CanError,
            S32K148Error::Flash(_) => HalError::FlashError,
            S32K148Error::InvalidOperation => HalError::InvalidOperation,
            S32K148Error::ProgrammingModeError => HalError::ProgrammingModeError,
            S32K148Error::JumpError => HalError::JumpError,
        }
    }
}
