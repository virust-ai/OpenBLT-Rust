use crate::clock::Clock;
use crate::gpio::{Pin, Port};
use crate::peripheral::{Peripheral, PeripheralRef};
use crate::reg::{Register, RegisterValue};

pub struct Uart<const N: u8> {
    _peripheral: PeripheralRef<UartPeripheral<N>>,
}

struct UartPeripheral<const N: u8>;

impl<const N: u8> Uart<N> {
    pub fn new(
        peripheral: Peripheral<UartPeripheral<N>>,
        tx_pin: Pin,
        rx_pin: Pin,
    ) -> Self {
        // Configure UART pins
        tx_pin.set_mux(3); // UART TX mux value
        rx_pin.set_mux(3); // UART RX mux value

        // Configure UART peripheral
        let uart = unsafe { &*UART_BASE[N as usize] };
        
        // Disable UART before configuration
        uart.c2.modify(|r, w| {
            w.set_re(0);
            w.set_te(0);
            ()
        });
        
        // Set baud rate (115200)
        // Using default clock frequency for now
        let baud = 115200;
        let sbr = 26; // This value needs to be adjusted based on actual clock frequency
        uart.bdh.write(|w| {
            w.set_sbr((sbr >> 8) as u8);
            ()
        });
        uart.bdl.write(|w| {
            w.set_sbr(sbr as u8);
            ()
        });
        
        // Configure for 8N1
        uart.c1.write(|w| {
            w.set_m(0);
            w.set_pe(0);
            ()
        });
        
        // Enable UART
        uart.c2.modify(|r, w| {
            w.set_re(1);
            w.set_te(1);
            ()
        });

        Self {
            _peripheral: peripheral.into_ref(),
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        let uart = unsafe { &*UART_BASE[N as usize] };
        while !uart.s1.read().tdre() {}
        uart.d.write(|w| {
            w.set_data(byte);
            ()
        });
    }

    pub fn read_byte(&mut self) -> Option<u8> {
        let uart = unsafe { &*UART_BASE[N as usize] };
        if uart.s1.read().rdrf() {
            Some(uart.d.read().data())
        } else {
            None
        }
    }
}

// UART base addresses
const UART_BASE: [*mut UartRegs; 5] = [
    0x4006A000 as *mut UartRegs, // UART0
    0x4006B000 as *mut UartRegs, // UART1
    0x4006C000 as *mut UartRegs, // UART2
    0x4006D000 as *mut UartRegs, // UART3
    0x400EA000 as *mut UartRegs, // UART4
];

#[repr(C)]
struct UartRegs {
    bdh: Register<u8>,
    bdl: Register<u8>,
    c1: Register<u8>,
    c2: Register<u8>,
    s1: Register<u8>,
    s2: Register<u8>,
    c3: Register<u8>,
    d: Register<u8>,
    ma1: Register<u8>,
    ma2: Register<u8>,
    c4: Register<u8>,
    c5: Register<u8>,
    ed: Register<u8>,
    modem: Register<u8>,
    ir: Register<u8>,
}

// Global UART instance for debug output
static mut DEBUG_UART: Option<Uart<0>> = None;

pub fn init_debug_uart() {
    unsafe {
        let uart = Uart::new(
            Peripheral::new(),
            Pin::new(Port::A, 14), // UART0 TX
            Pin::new(Port::A, 15), // UART0 RX
        );
        DEBUG_UART = Some(uart);
    }
}

pub fn debug_print(s: &str) {
    unsafe {
        if let Some(uart) = &mut DEBUG_UART {
            for byte in s.bytes() {
                uart.write_byte(byte);
            }
        }
    }
}

pub fn debug_println(s: &str) {
    debug_print(s);
    debug_print("\r\n");
} 
