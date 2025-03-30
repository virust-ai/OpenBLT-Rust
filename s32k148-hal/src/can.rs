use bitflags::bitflags;
use vcell::VolatileCell;
use volatile_register::{RO, RW, WO};

// CAN Peripheral Base Addresses
pub const CAN0_BASE: u32 = 0x4002_4000;
pub const CAN1_BASE: u32 = 0x4002_5000;

// CAN Register Structure
#[repr(C)]
pub struct CanRegisters {
    // Module Configuration Register
    pub mcr: RW<u32>,
    // Control Register
    pub ctrl1: RW<u32>,
    // Status Register
    pub stat1: RO<u32>,
    // Error Counter Register
    pub err_cnt: RO<u32>,
    // Bit Timing Register
    pub btr: RW<u32>,
    // Rx FIFO Global Mask Register
    pub rxmgmask: RW<u32>,
    // Rx FIFO Individual Mask Registers
    pub rx14mask: RW<u32>,
    pub rx15mask: RW<u32>,
    // Rx FIFO Information Register
    pub rx_fifo_info: RO<u32>,
    // Rx FIFO Data Register
    pub rx_fifo_data: RO<u32>,
    // Tx Buffer Information Register
    pub tx_buffer_info: RO<u32>,
    // Tx Buffer Data Register
    pub tx_buffer_data: WO<u32>,
    // Message Buffer Registers (16 buffers)
    pub mb: [MessageBuffer; 16],
}

// Message Buffer Structure
#[repr(C)]
pub struct MessageBuffer {
    // Control and Status
    pub cs: RW<u32>,
    // ID
    pub id: RW<u32>,
    // Word 0
    pub word0: RW<u32>,
    // Word 1
    pub word1: RW<u32>,
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
