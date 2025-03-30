#![no_std]

use cortex_m::interrupt::free;
use embedded_can::{Can, ExtendedId, Frame, Id, StandardId};
use nb;
use thiserror::Error;

mod can;
use can::{BitTiming, CanError, CanRegisters, Ctrl1, MessageBufferControl, Stat1};

pub mod flash;

#[derive(Error, Debug)]
pub enum Error {
    #[error("CAN communication error: {0}")]
    CanError(#[from] CanError),
    #[error("Hardware initialization error")]
    InitError,
    #[error("Invalid configuration")]
    ConfigError,
}

impl embedded_can::Error for Error {
    fn kind(&self) -> embedded_can::ErrorKind {
        match self {
            Error::CanError(_) => embedded_can::ErrorKind::Other,
            Error::InitError => embedded_can::ErrorKind::Other,
            Error::ConfigError => embedded_can::ErrorKind::Other,
        }
    }
}

pub struct S32K148Hal {
    can: S32K148Can,
}

struct S32K148Can {
    registers: &'static mut CanRegisters,
}

// Custom CAN frame implementation for S32K148
#[derive(Debug, Clone, Copy)]
pub struct S32K148Frame {
    id: Id,
    data: [u8; 8],
    dlc: u8,
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
            dlc: len as u8,
            is_remote: false,
        })
    }

    fn new_remote(id: impl Into<Id>, dlc: u8) -> Option<Self> {
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

    fn dlc(&self) -> u8 {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.dlc as usize]
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        self.is_remote
    }
}

impl S32K148Hal {
    pub fn new() -> Result<Self, Error> {
        // TODO: Initialize S32K148 hardware
        // 1. Configure clock system
        // 2. Initialize CAN peripheral
        // 3. Configure GPIO for CAN pins

        // Initialize CAN peripheral
        let can = unsafe {
            S32K148Can {
                registers: &mut *(can::CAN0_BASE as *mut CanRegisters),
            }
        };

        // Configure CAN
        can.setup_can()?;

        Ok(S32K148Hal { can })
    }

    pub fn get_can(&self) -> &S32K148Can {
        &self.can
    }

    pub fn get_can_mut(&mut self) -> &mut S32K148Can {
        &mut self.can
    }
}

impl S32K148Can {
    fn setup_can(&self) -> Result<(), Error> {
        unsafe {
            // Enter freeze mode
            self.registers.mcr.write(1 << 2); // FRZ bit

            // Configure bit timing for 500kbps
            // Assuming 80MHz CAN clock
            // Tq = (PRESDIV + 1) * (1/80MHz)
            // Bit time = (1 + PROPSEG + PSEG1 + PSEG2) * Tq
            // 500kbps = 2µs bit time
            // With 80MHz clock, we need 160 Tq per bit
            let btr = BitTiming::PRESDIV.bits() | // Prescaler = 0
                     (6 << 0) | // PROPSEG = 6
                     (7 << 3) | // PSEG1 = 7
                     (6 << 6) | // PSEG2 = 6
                     (2 << 9); // RJW = 2
            self.registers.btr.write(btr);

            // Configure message buffers
            for i in 0..16 {
                self.registers.mb[i].cs.write(0);
                self.registers.mb[i].id.write(0);
                self.registers.mb[i].word0.write(0);
                self.registers.mb[i].word1.write(0);
            }

            // Exit freeze mode and enable CAN
            self.registers.mcr.write(0);
            self.registers.ctrl1.write(Ctrl1::CAN_EN.bits());
        }

        Ok(())
    }
}

impl Can for S32K148Can {
    type Frame = S32K148Frame;
    type Error = Error;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        unsafe {
            // Check if any transmit buffer is available
            let status = Stat1::from_bits_truncate(self.registers.stat1.read());
            if !status.contains(Stat1::TX) {
                return Err(nb::Error::WouldBlock);
            }

            // Find an available transmit buffer
            for i in 0..16 {
                let mb_cs = self.registers.mb[i].cs.read();
                if (mb_cs & MessageBufferControl::CODE.bits()) == 0 {
                    // Configure message buffer for transmission
                    let id = match frame.id() {
                        Id::Standard(id) => id.as_raw() as u32,
                        Id::Extended(id) => id.as_raw(),
                    };

                    self.registers.mb[i].id.write(id);
                    self.registers.mb[i]
                        .word0
                        .write(u32::from_le_bytes(frame.data[0..4].try_into().unwrap()));
                    self.registers.mb[i]
                        .word1
                        .write(u32::from_le_bytes(frame.data[4..8].try_into().unwrap()));

                    // Set up control word for transmission
                    let cs = MessageBufferControl::CODE.bits() | // Active for transmission
                            (frame.dlc() as u32) << 16; // Data length code
                    self.registers.mb[i].cs.write(cs);

                    return Ok(None);
                }
            }

            Err(nb::Error::WouldBlock)
        }
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        unsafe {
            // Check if any message is available
            let status = Stat1::from_bits_truncate(self.registers.stat1.read());
            if !status.contains(Stat1::RX) {
                return Err(nb::Error::WouldBlock);
            }

            // Find a received message
            for i in 0..16 {
                let mb_cs = self.registers.mb[i].cs.read();
                if (mb_cs & MessageBufferControl::CODE.bits()) == 0x4 {
                    // RX_FULL
                    let id = self.registers.mb[i].id.read();
                    let dlc = ((mb_cs >> 16) & 0xF) as u8;

                    let word0 = self.registers.mb[i].word0.read();
                    let word1 = self.registers.mb[i].word1.read();

                    let mut data = [0u8; 8];
                    data[0..4].copy_from_slice(&word0.to_le_bytes());
                    data[4..8].copy_from_slice(&word1.to_le_bytes());

                    // Clear the RX_FULL flag
                    self.registers.mb[i].cs.write(0);

                    // Create frame with appropriate ID type
                    let frame = if (id & (1 << 30)) != 0 {
                        S32K148Frame::new(ExtendedId::new(id & 0x1FFFFFFF), &data[..dlc as usize])
                    } else {
                        S32K148Frame::new(
                            StandardId::new((id & 0x7FF) as u16),
                            &data[..dlc as usize],
                        )
                    };

                    return Ok(frame);
                }
            }

            Err(nb::Error::WouldBlock)
        }
    }
}
