use cortex_m::asm;
use cortex_m_rt::{exception, ExceptionFrame};

// Default handler macro to reduce code duplication
macro_rules! default_handler {
    ($name:ident) => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn $name() {
            loop {
                asm::nop();
            }
        }
    };
}

// Default handler for unhandled interrupts
#[exception]
unsafe fn DefaultHandler(_irqn: i16) -> ! {
    loop {
        asm::nop();
    }
}

// Hard fault handler
#[exception]
unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
    loop {
        asm::nop();
    }
}

// DMA handlers
default_handler!(DMA0_IRQHandler);
default_handler!(DMA1_IRQHandler);
default_handler!(DMA2_IRQHandler);
default_handler!(DMA3_IRQHandler);
default_handler!(DMA4_IRQHandler);
default_handler!(DMA5_IRQHandler);
default_handler!(DMA6_IRQHandler);
default_handler!(DMA7_IRQHandler);
default_handler!(DMA8_IRQHandler);
default_handler!(DMA9_IRQHandler);
default_handler!(DMA10_IRQHandler);
default_handler!(DMA11_IRQHandler);
default_handler!(DMA12_IRQHandler);
default_handler!(DMA13_IRQHandler);
default_handler!(DMA14_IRQHandler);
default_handler!(DMA15_IRQHandler);
default_handler!(DMA_Error_IRQHandler);

// System handlers
default_handler!(MCM_IRQHandler);
default_handler!(FTFC_IRQHandler);
default_handler!(Read_Collision_IRQHandler);
default_handler!(LVD_LVW_IRQHandler);
default_handler!(LLWU_IRQHandler);
default_handler!(WDOG_EWM_IRQHandler);
default_handler!(RNG_IRQHandler);

// I2C handlers
default_handler!(I2C0_IRQHandler);
default_handler!(I2C1_IRQHandler);
default_handler!(I2C2_IRQHandler);
default_handler!(I2C3_IRQHandler);
default_handler!(I2C4_IRQHandler);
default_handler!(I2C5_IRQHandler);

// SPI handlers
default_handler!(SPI0_IRQHandler);
default_handler!(SPI1_IRQHandler);
default_handler!(SPI2_IRQHandler);

// CAN handlers
default_handler!(CAN0_ORed_MessageBuffer_IRQHandler);
default_handler!(CAN0_Bus_Off_IRQHandler);
default_handler!(CAN0_Error_IRQHandler);
default_handler!(CAN0_Tx_Warning_IRQHandler);
default_handler!(CAN0_Rx_Warning_IRQHandler);
default_handler!(CAN0_Wake_Up_IRQHandler);
default_handler!(CAN1_ORed_MessageBuffer_IRQHandler);
default_handler!(CAN1_Bus_Off_IRQHandler);
default_handler!(CAN1_Error_IRQHandler);
default_handler!(CAN1_Tx_Warning_IRQHandler);
default_handler!(CAN1_Rx_Warning_IRQHandler);
default_handler!(CAN1_Wake_Up_IRQHandler);
default_handler!(CAN2_ORed_MessageBuffer_IRQHandler);
default_handler!(CAN2_Bus_Off_IRQHandler);
default_handler!(CAN2_Error_IRQHandler);
default_handler!(CAN2_Tx_Warning_IRQHandler);
default_handler!(CAN2_Rx_Warning_IRQHandler);
default_handler!(CAN2_Wake_Up_IRQHandler);

// UART handlers
default_handler!(UART0_RX_TX_IRQHandler);
default_handler!(UART0_ERR_IRQHandler);
default_handler!(UART1_RX_TX_IRQHandler);
default_handler!(UART1_ERR_IRQHandler);
default_handler!(UART2_RX_TX_IRQHandler);
default_handler!(UART2_ERR_IRQHandler);
default_handler!(UART3_RX_TX_IRQHandler);
default_handler!(UART3_ERR_IRQHandler);
default_handler!(UART4_RX_TX_IRQHandler);
default_handler!(UART4_ERR_IRQHandler);
default_handler!(UART5_RX_TX_IRQHandler);
default_handler!(UART5_ERR_IRQHandler);

// Timer handlers
default_handler!(FTM0_Ch0_Ch1_IRQHandler);
default_handler!(FTM0_Ch2_Ch3_IRQHandler);
default_handler!(FTM0_Ch4_Ch5_IRQHandler);
default_handler!(FTM0_Ch6_Ch7_IRQHandler);
default_handler!(FTM0_Fault_IRQHandler);
default_handler!(FTM0_Ovf_Reload_IRQHandler);
default_handler!(FTM1_Ch0_Ch1_IRQHandler);
default_handler!(FTM1_Ch2_Ch3_IRQHandler);
default_handler!(FTM1_Ch4_Ch5_IRQHandler);
default_handler!(FTM1_Ch6_Ch7_IRQHandler);
default_handler!(FTM1_Fault_IRQHandler);
default_handler!(FTM1_Ovf_Reload_IRQHandler);
default_handler!(FTM2_Ch0_Ch1_IRQHandler);
default_handler!(FTM2_Ch2_Ch3_IRQHandler);
default_handler!(FTM2_Ch4_Ch5_IRQHandler);
default_handler!(FTM2_Ch6_Ch7_IRQHandler);
default_handler!(FTM2_Fault_IRQHandler);
default_handler!(FTM2_Ovf_Reload_IRQHandler);
default_handler!(FTM3_Ch0_Ch1_IRQHandler);
default_handler!(FTM3_Ch2_Ch3_IRQHandler);
default_handler!(FTM3_Ch4_Ch5_IRQHandler);
default_handler!(FTM3_Ch6_Ch7_IRQHandler);
default_handler!(FTM3_Fault_IRQHandler);
default_handler!(FTM3_Ovf_Reload_IRQHandler);

// ADC handlers
default_handler!(ADC0_IRQHandler);
default_handler!(ADC1_IRQHandler);

// Comparator handlers
default_handler!(CMP0_IRQHandler);
default_handler!(CMP1_IRQHandler);
default_handler!(CMP2_IRQHandler);

// Other handlers
default_handler!(PDB0_IRQHandler);
default_handler!(PDB1_IRQHandler);
default_handler!(PORTA_IRQHandler);
default_handler!(PORTB_IRQHandler);
default_handler!(PORTC_IRQHandler);
default_handler!(PORTD_IRQHandler);
default_handler!(PORTE_IRQHandler);
default_handler!(LPTMR0_IRQHandler);
default_handler!(LPIT0_Ch0_IRQHandler);
default_handler!(LPIT0_Ch1_IRQHandler);
default_handler!(LPIT0_Ch2_IRQHandler);
default_handler!(LPIT0_Ch3_IRQHandler);
default_handler!(PWT_IRQHandler);
default_handler!(FLEXIO_IRQHandler);
default_handler!(SAI0_IRQHandler);
default_handler!(SAI1_IRQHandler);
default_handler!(ENET_1588_Timer_IRQHandler);
default_handler!(ENET_Transmit_IRQHandler);
default_handler!(ENET_Receive_IRQHandler);
default_handler!(ENET_Error_IRQHandler);
default_handler!(LPUART0_RX_TX_IRQHandler);
default_handler!(LPUART0_ERR_IRQHandler);
default_handler!(LPUART1_RX_TX_IRQHandler);
default_handler!(LPUART1_ERR_IRQHandler);
default_handler!(LPUART2_RX_TX_IRQHandler);
default_handler!(LPUART2_ERR_IRQHandler);
default_handler!(LPUART3_RX_TX_IRQHandler);
default_handler!(LPUART3_ERR_IRQHandler);
default_handler!(LPUART4_RX_TX_IRQHandler);
default_handler!(LPUART4_ERR_IRQHandler);
default_handler!(LPUART5_RX_TX_IRQHandler);
default_handler!(LPUART5_ERR_IRQHandler);
default_handler!(QuadSPI0_IRQHandler);
default_handler!(Reserved88_IRQHandler);
default_handler!(Reserved89_IRQHandler);
default_handler!(Reserved90_IRQHandler);
default_handler!(Reserved91_IRQHandler);
default_handler!(Reserved92_IRQHandler);
default_handler!(Reserved93_IRQHandler);
default_handler!(Reserved94_IRQHandler);
default_handler!(Reserved95_IRQHandler);
default_handler!(Reserved96_IRQHandler);
default_handler!(Reserved97_IRQHandler);
default_handler!(Reserved98_IRQHandler);
default_handler!(Reserved99_IRQHandler);
default_handler!(Reserved100_IRQHandler);
default_handler!(Reserved101_IRQHandler); 
