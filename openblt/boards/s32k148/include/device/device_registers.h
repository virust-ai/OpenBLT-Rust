#ifndef DEVICE_REGISTERS_H
#define DEVICE_REGISTERS_H

#include <stdint.h>
#include <stdbool.h>
#include "S32K148.h"
#include "S32K148_features.h"

/* Interrupt module features */
#define FEATURE_INTERRUPT_IRQ_MAX         (122U)
#define FEATURE_INTERRUPT_PRIORITY_BITS   (4U)

/* Core modules features */
#define FEATURE_SCB_VECTKEY              (0x05FAU)

/* System Control Block Base Address */
#define SCB_BASE                         (0xE000E000UL)
#define SCB                              ((SCB_Type *)SCB_BASE)

/* System Control Space Base Address */
#define SCS_BASE                         (0xE000E000UL)

/* Memory Protection Unit Base Address */
#define MPU_BASE                         (0xE000ED90UL)
#define MPU                              ((MPU_Type *)MPU_BASE)

/* Nested Vectored Interrupt Controller Base Address */
#define NVIC_BASE                        (0xE000E100UL)
#define NVIC                             ((NVIC_Type *)NVIC_BASE)

/* System Tick Timer Base Address */
#define SysTick_BASE                     (0xE000E010UL)
#define SysTick                          ((SysTick_Type *)SysTick_BASE)

/* Device specific interrupts */
typedef enum {
    DMA0_IRQn                     = 0,
    DMA1_IRQn                     = 1,
    DMA2_IRQn                     = 2,
    DMA3_IRQn                     = 3,
    DMA4_IRQn                     = 4,
    DMA5_IRQn                     = 5,
    DMA6_IRQn                     = 6,
    DMA7_IRQn                     = 7,
    DMA8_IRQn                     = 8,
    DMA9_IRQn                     = 9,
    DMA10_IRQn                    = 10,
    DMA11_IRQn                    = 11,
    DMA12_IRQn                    = 12,
    DMA13_IRQn                    = 13,
    DMA14_IRQn                    = 14,
    DMA15_IRQn                    = 15,
    DMA_Error_IRQn                = 16,
    MCM_IRQn                      = 17,
    FTFC_IRQn                     = 18,
    Read_Collision_IRQn           = 19,
    LVD_LVW_IRQn                 = 20,
    FTFC_Fault_IRQn              = 21,
    WDOG_EWM_IRQn                = 22,
    RCM_IRQn                     = 23,
    LPI2C0_Master_IRQn           = 24,
    LPI2C0_Slave_IRQn            = 25,
    LPSPI0_IRQn                  = 26,
    LPSPI1_IRQn                  = 27,
    LPSPI2_IRQn                  = 28,
    LPUART0_RxTx_IRQn            = 31,
    LPUART1_RxTx_IRQn            = 33,
    LPUART2_RxTx_IRQn            = 35,
    ADC0_IRQn                    = 39,
    ADC1_IRQn                    = 40,
    CMP0_IRQn                    = 41,
    ERM_single_fault_IRQn        = 44,
    ERM_double_fault_IRQn        = 45,
    RTC_IRQn                     = 46,
    RTC_Seconds_IRQn             = 47,
    LPIT0_Ch0_IRQn               = 48,
    LPIT0_Ch1_IRQn               = 49,
    LPIT0_Ch2_IRQn               = 50,
    LPIT0_Ch3_IRQn               = 51,
    PDB0_IRQn                    = 52,
    SCG_IRQn                     = 57,
    LPTMR0_IRQn                  = 58,
    PORTA_IRQn                   = 59,
    PORTB_IRQn                   = 60,
    PORTC_IRQn                   = 61,
    PORTD_IRQn                   = 62,
    PORTE_IRQn                   = 63,
    SWI_IRQn                     = 64,
    PDB1_IRQn                    = 68,
    FLEXIO_IRQn                  = 69,
    CAN0_ORed_IRQn               = 78,
    CAN0_Error_IRQn              = 79,
    CAN0_Wake_Up_IRQn            = 80,
    CAN0_ORed_0_15_MB_IRQn       = 81,
    CAN0_ORed_16_31_MB_IRQn      = 82,
    CAN1_ORed_IRQn               = 85,
    CAN1_Error_IRQn              = 86,
    CAN2_ORed_IRQn               = 92,
    CAN2_Error_IRQn              = 93,
    FTM0_Ch0_Ch1_IRQn            = 99,
    FTM0_Ch2_Ch3_IRQn            = 100,
    FTM0_Ch4_Ch5_IRQn            = 101,
    FTM0_Ch6_Ch7_IRQn            = 102,
    FTM0_Fault_IRQn              = 103,
    FTM0_Ovf_Reload_IRQn         = 104,
    FTM1_Ch0_Ch1_IRQn            = 105,
    FTM1_Ch2_Ch3_IRQn            = 106,
    FTM1_Ch4_Ch5_IRQn            = 107,
    FTM1_Ch6_Ch7_IRQn            = 108,
    FTM1_Fault_IRQn              = 109,
    FTM1_Ovf_Reload_IRQn         = 110,
    FTM2_Ch0_Ch1_IRQn            = 111,
    FTM2_Ch2_Ch3_IRQn            = 112,
    FTM2_Ch4_Ch5_IRQn            = 113,
    FTM2_Ch6_Ch7_IRQn            = 114,
    FTM2_Fault_IRQn              = 115,
    FTM2_Ovf_Reload_IRQn         = 116,
    FTM3_Ch0_Ch1_IRQn            = 117,
    FTM3_Ch2_Ch3_IRQn            = 118,
    FTM3_Ch4_Ch5_IRQn            = 119,
    FTM3_Ch6_Ch7_IRQn            = 120,
    FTM3_Fault_IRQn              = 121,
    FTM3_Ovf_Reload_IRQn         = 122
} DeviceVectors;

#endif /* DEVICE_REGISTERS_H */ 
