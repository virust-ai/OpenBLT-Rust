#![no_std]

use vcell::VolatileCell;
use core::marker::PhantomData;
use embedded_can::blocking::Can;

#[derive(Debug)]
pub enum CanError {
    TxFifoFull,
    RxFifoEmpty,
    InvalidId,
    InvalidLength,
    BusOff,
    Error,
}

pub struct CanRegisters {
    mcr: VolatileCell<u32>,
    ctrl1: VolatileCell<u32>,
    timer: VolatileCell<u32>,
    rxmgmask: VolatileCell<u32>,
    rx14mask: VolatileCell<u32>,
    rx15mask: VolatileCell<u32>,
    ecr: VolatileCell<u32>,
    esr1: VolatileCell<u32>,
    imask1: VolatileCell<u32>,
    iflag1: VolatileCell<u32>,
    ctrl2: VolatileCell<u32>,
    esr2: VolatileCell<u32>,
    crcr: VolatileCell<u32>,
    rxfgmask: VolatileCell<u32>,
    rxfir: VolatileCell<u32>,
    _reserved: [u32; 1],
    mb: [VolatileCell<u32>; 16],
    rximr: [VolatileCell<u32>; 16],
}

pub struct CanDevice {
    registers: &'static mut CanRegisters,
    _phantom: PhantomData<()>,
}

impl CanDevice {
    pub fn new(registers: &'static mut CanRegisters) -> Self {
        Self {
            registers,
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self) {
        // Reset MCR register
        self.registers.mcr.set(0x5000_0000);

        // Configure CTRL1 register
        self.registers.ctrl1.set(0x0000_0000);

        // Configure CTRL2 register
        self.registers.ctrl2.set(0x0000_0000);

        // Clear all interrupt flags
        self.registers.iflag1.set(0xFFFF_FFFF);

        // Disable all interrupts
        self.registers.imask1.set(0x0000_0000);
    }

    pub fn send_frame(&mut self, id: u32, data: &[u8], len: u8) -> Result<(), CanError> {
        if len > 8 {
            return Err(CanError::InvalidLength);
        }

        if id > 0x7FF {
            return Err(CanError::InvalidId);
        }

        // Check if TX FIFO is full
        if self.registers.esr1.get() & (1 << 4) != 0 {
            return Err(CanError::TxFifoFull);
        }

        // Write message buffer
        let mb = &self.registers.mb[0];
        mb.set(id << 18 | (len as u32) << 16);

        // Write data
        let mut data_words = [0u32; 2];
        for (i, byte) in data.iter().enumerate() {
            let word_idx = i / 4;
            let byte_idx = i % 4;
            data_words[word_idx] |= (*byte as u32) << (24 - (byte_idx * 8));
        }

        self.registers.mb[1].set(data_words[0]);
        self.registers.mb[2].set(data_words[1]);

        Ok(())
    }

    pub fn receive_frame(&mut self) -> Result<(u32, [u8; 8], u8), CanError> {
        // Check if RX FIFO is empty
        if self.registers.esr1.get() & (1 << 5) != 0 {
            return Err(CanError::RxFifoEmpty);
        }

        // Read message buffer
        let mb = self.registers.mb[0].get();
        let id = (mb >> 18) & 0x7FF;
        let len = ((mb >> 16) & 0x0F) as u8;

        // Read data
        let data_word0 = self.registers.mb[1].get();
        let data_word1 = self.registers.mb[2].get();

        let mut data = [0u8; 8];
        for i in 0..len as usize {
            let word_idx = i / 4;
            let byte_idx = i % 4;
            let word = if word_idx == 0 { data_word0 } else { data_word1 };
            data[i] = ((word >> (24 - (byte_idx * 8))) & 0xFF) as u8;
        }

        Ok((id, data, len))
    }
}
