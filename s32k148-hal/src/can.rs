use bitflags::bitflags;
use vcell::VolatileCell;
use volatile_register::{RO, RW, WO};
use core::fmt;
use core::array;

// CAN Peripheral Base Addresses
pub const CAN0_BASE: u32 = 0x4002_4000;
pub const CAN1_BASE: u32 = 0x4002_5000;

// CAN Register Structure
#[repr(C)]
pub struct CanRegisters {
    // Module Configuration Register
    pub mcr: VolatileCell<u32>,
    // Control Register
    pub ctrl1: VolatileCell<u32>,
    // Status Register
    pub stat1: VolatileCell<u32>,
    // Error Counter Register
    pub err_cnt: VolatileCell<u32>,
    // Bit Timing Register
    pub btr: VolatileCell<u32>,
    // Rx FIFO Global Mask Register
    pub rxmgmask: VolatileCell<u32>,
    // Rx FIFO Individual Mask Registers
    pub rx14mask: VolatileCell<u32>,
    pub rx15mask: VolatileCell<u32>,
    // Rx FIFO Information Register
    pub rx_fifo_info: VolatileCell<u32>,
    // Rx FIFO Data Register
    pub rx_fifo_data: VolatileCell<u32>,
    // Tx Buffer Information Register
    pub tx_buffer_info: VolatileCell<u32>,
    // Tx Buffer Data Register
    pub tx_buffer_data: VolatileCell<u32>,
    // Message Buffer Registers (16 buffers)
    pub mb: [MessageBuffer; 16],
}

impl core::fmt::Debug for CanRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("CanRegisters")
            .field("mcr", &self.mcr.get())
            .field("ctrl1", &self.ctrl1.get())
            .field("stat1", &self.stat1.get())
            .field("err_cnt", &self.err_cnt.get())
            .field("btr", &self.btr.get())
            .field("rxmgmask", &self.rxmgmask.get())
            .field("rx14mask", &self.rx14mask.get())
            .field("rx15mask", &self.rx15mask.get())
            .field("rx_fifo_info", &self.rx_fifo_info.get())
            .field("rx_fifo_data", &self.rx_fifo_data.get())
            .field("tx_buffer_info", &self.tx_buffer_info.get())
            .field("tx_buffer_data", &self.tx_buffer_data.get())
            .field("mb", &self.mb)
            .finish()
    }
}

impl Clone for CanRegisters {
    fn clone(&self) -> Self {
        Self {
            mcr: VolatileCell::new(self.mcr.get()),
            ctrl1: VolatileCell::new(self.ctrl1.get()),
            stat1: VolatileCell::new(self.stat1.get()),
            err_cnt: VolatileCell::new(self.err_cnt.get()),
            btr: VolatileCell::new(self.btr.get()),
            rxmgmask: VolatileCell::new(self.rxmgmask.get()),
            rx14mask: VolatileCell::new(self.rx14mask.get()),
            rx15mask: VolatileCell::new(self.rx15mask.get()),
            rx_fifo_info: VolatileCell::new(self.rx_fifo_info.get()),
            rx_fifo_data: VolatileCell::new(self.rx_fifo_data.get()),
            tx_buffer_info: VolatileCell::new(self.tx_buffer_info.get()),
            tx_buffer_data: VolatileCell::new(self.tx_buffer_data.get()),
            mb: self.mb.clone(),
        }
    }
}

// Message Buffer Structure
#[repr(C)]
pub struct MessageBuffer {
    // Control and Status
    pub cs: VolatileCell<u32>,
    // ID
    pub id: VolatileCell<u32>,
    // Word 0
    pub word0: VolatileCell<u32>,
    // Word 1
    pub word1: VolatileCell<u32>,
}

impl core::fmt::Debug for MessageBuffer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MessageBuffer")
            .field("cs", &self.cs.get())
            .field("id", &self.id.get())
            .field("word0", &self.word0.get())
            .field("word1", &self.word1.get())
            .finish()
    }
}

impl Clone for MessageBuffer {
    fn clone(&self) -> Self {
        Self {
            cs: VolatileCell::new(self.cs.get()),
            id: VolatileCell::new(self.id.get()),
            word0: VolatileCell::new(self.word0.get()),
            word1: VolatileCell::new(self.word1.get()),
        }
    }
}

// CAN Control Register 1 Bit Definitions
bitflags! {
    pub struct Ctrl1: u32 {
        const CAN_EN = 1 << 0;
        const HALT = 1 << 1;
        const FRZ = 1 << 2;
        const MDIS = 1 << 3;
        const WAK_MSK = 1 << 4;
        const LOM = 1 << 5;
        const ABORT = 1 << 6;
        const IDAM = 1 << 8;
        const MAXMB = 0x1F << 16;
    }
}

// CAN Status Register 1 Bit Definitions
bitflags! {
    pub struct Stat1: u32 {
        const LACK = 1 << 0;
        const LACK_INT = 1 << 1;
        const TWRN_INT = 1 << 2;
        const RWRN_INT = 1 << 3;
        const BOFF_INT = 1 << 4;
        const ERR_INT = 1 << 5;
        const SYNC_OK = 1 << 6;
        const ERR_FAST = 1 << 7;
        const ERR_PASSIVE = 1 << 8;
        const CAN_ACTIVE = 1 << 9;
        const LAST_ERROR_CODE = 0x7 << 10;
        const FLT_CONF = 1 << 13;
        const TX = 1 << 14;
        const RX = 1 << 15;
        const IDLE = 1 << 16;
        const AERR = 1 << 17;
        const BERR = 1 << 18;
        const STER = 1 << 19;
        const FERR = 1 << 20;
        const CERR = 1 << 21;
    }
}

// CAN Bit Timing Register Bit Definitions
bitflags! {
    pub struct BitTiming: u32 {
        const PROPSEG = 0x7 << 0;
        const PSEG1 = 0x7 << 3;
        const PSEG2 = 0x7 << 6;
        const RJW = 0x3 << 9;
        const PRESDIV = 0xFF << 16;
    }
}

// CAN Message Buffer Control and Status Bit Definitions
bitflags! {
    pub struct MessageBufferControl: u32 {
        const CODE = 0xF << 0;
        const SRR = 1 << 4;
        const IDE = 1 << 5;
        const RTR = 1 << 6;
        const BRS = 1 << 7;
        const ESI = 1 << 8;
        const DLC = 0xF << 16;
        const TIMESTAMP = 0xFFFF << 20;
    }
}

// CAN Error Types
#[derive(Debug, Clone, Copy)]
pub enum CanError {
    BitError,
    StuffError,
    FormError,
    AcknowledgmentError,
    BusOff,
    Wakeup,
    TransmitError,
    ReceiveError,
    Unknown,
}

impl CanError {
    pub fn from_status(status: Stat1) -> Option<Self> {
        if status.contains(Stat1::BERR) {
            Some(CanError::BitError)
        } else if status.contains(Stat1::STER) {
            Some(CanError::StuffError)
        } else if status.contains(Stat1::FERR) {
            Some(CanError::FormError)
        } else if status.contains(Stat1::AERR) {
            Some(CanError::AcknowledgmentError)
        } else if status.contains(Stat1::BOFF_INT) {
            Some(CanError::BusOff)
        } else if status.contains(Stat1::LACK_INT) {
            Some(CanError::Wakeup)
        } else if status.contains(Stat1::TX) {
            Some(CanError::TransmitError)
        } else if status.contains(Stat1::RX) {
            Some(CanError::ReceiveError)
        } else {
            Some(CanError::Unknown)
        }
    }
}

impl Default for CanRegisters {
    fn default() -> Self {
        Self {
            mcr: VolatileCell::new(0),
            ctrl1: VolatileCell::new(0),
            stat1: VolatileCell::new(0),
            err_cnt: VolatileCell::new(0),
            btr: VolatileCell::new(0),
            rxmgmask: VolatileCell::new(0),
            rx14mask: VolatileCell::new(0),
            rx15mask: VolatileCell::new(0),
            rx_fifo_info: VolatileCell::new(0),
            rx_fifo_data: VolatileCell::new(0),
            tx_buffer_info: VolatileCell::new(0),
            tx_buffer_data: VolatileCell::new(0),
            mb: array::from_fn(|_| MessageBuffer::default()),
        }
    }
}

impl CanRegisters {
    pub fn read(&self) -> u32 {
        0 // TODO: Implement actual register read
    }

    pub fn write(&mut self, value: u32) {
        // TODO: Implement actual register write
    }

    pub fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let value = self.read();
        let new_value = f(value);
        self.write(new_value);
    }

    pub fn init_can(&self) -> Result<(), ()> {
        unsafe {
            // Enter freeze mode
            let ctrl1 = self.ctrl1.get();
            self.ctrl1.set(ctrl1 & !Ctrl1::CAN_EN.bits());
            self.ctrl1.set(ctrl1 | (Ctrl1::HALT.bits() | Ctrl1::FRZ.bits()));
            
            // Configure bit timing for 500kbps at 80MHz clock:
            // Prescaler = 4, Prop_Seg = 7, Phase_Seg1 = 4, Phase_Seg2 = 4, RJW = 4
            let timing = BitTiming::PRESDIV.bits() & (3 << 16) |
                        BitTiming::PROPSEG.bits() & (6 << 0) |
                        BitTiming::PSEG1.bits() & (3 << 3) |
                        BitTiming::PSEG2.bits() & (3 << 6) |
                        BitTiming::RJW.bits() & (3 << 9);
            self.btr.set(timing);

            // Configure message buffers
            for i in 0..16 {
                self.mb[i].cs.set(0); // Inactive
            }
            
            // Exit freeze mode and enable CAN
            let ctrl1 = self.ctrl1.get();
            self.ctrl1.set(ctrl1 | Ctrl1::CAN_EN.bits());
            self.ctrl1.set(ctrl1 & !(Ctrl1::HALT.bits() | Ctrl1::FRZ.bits()));
        }
        
        Ok(())
    }
}

impl Default for MessageBuffer {
    fn default() -> Self {
        Self {
            cs: VolatileCell::new(0),
            id: VolatileCell::new(0),
            word0: VolatileCell::new(0),
            word1: VolatileCell::new(0),
        }
    }
}

impl MessageBuffer {
    pub fn read(&self) -> u32 {
        0 // TODO: Implement actual register read
    }

    pub fn write(&mut self, value: u32) {
        // TODO: Implement actual register write
    }

    pub fn modify<F>(&mut self, f: F)
    where
        F: FnOnce(u32) -> u32,
    {
        let value = self.read();
        let new_value = f(value);
        self.write(new_value);
    }
}
