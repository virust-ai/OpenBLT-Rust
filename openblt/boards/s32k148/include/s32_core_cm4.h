#ifndef S32_CORE_CM4_H
#define S32_CORE_CM4_H

#include <stdint.h>

/* ARM Cortex-M4 System Control Block */
typedef struct {
    volatile uint32_t CPUID;                  /*!< CPUID Base Register */
    volatile uint32_t ICSR;                   /*!< Interrupt Control and State Register */
    volatile uint32_t VTOR;                   /*!< Vector Table Offset Register */
    volatile uint32_t AIRCR;                  /*!< Application Interrupt and Reset Control Register */
    volatile uint32_t SCR;                    /*!< System Control Register */
    volatile uint32_t CCR;                    /*!< Configuration Control Register */
    volatile uint8_t  SHP[12U];              /*!< System Handlers Priority Registers (4-7, 8-11, 12-15) */
    volatile uint32_t SHCSR;                  /*!< System Handler Control and State Register */
    volatile uint32_t CFSR;                   /*!< Configurable Fault Status Register */
    volatile uint32_t HFSR;                   /*!< HardFault Status Register */
    volatile uint32_t DFSR;                   /*!< Debug Fault Status Register */
    volatile uint32_t MMFAR;                  /*!< MemManage Fault Address Register */
    volatile uint32_t BFAR;                   /*!< BusFault Address Register */
    volatile uint32_t AFSR;                   /*!< Auxiliary Fault Status Register */
    volatile uint32_t PFR[2U];               /*!< Processor Feature Register */
    volatile uint32_t DFR;                    /*!< Debug Feature Register */
    volatile uint32_t ADR;                    /*!< Auxiliary Feature Register */
    volatile uint32_t MMFR[4U];              /*!< Memory Model Feature Register */
    volatile uint32_t ISAR[5U];              /*!< Instruction Set Attributes Register */
    uint32_t RESERVED0[5U];
    volatile uint32_t CPACR;                  /*!< Coprocessor Access Control Register */
} SCB_Type;

/* ARM Cortex-M4 NVIC */
typedef struct {
    volatile uint32_t ISER[8U];              /*!< Interrupt Set Enable Register */
    uint32_t RESERVED0[24U];
    volatile uint32_t ICER[8U];              /*!< Interrupt Clear Enable Register */
    uint32_t RESERVED1[24U];
    volatile uint32_t ISPR[8U];              /*!< Interrupt Set Pending Register */
    uint32_t RESERVED2[24U];
    volatile uint32_t ICPR[8U];              /*!< Interrupt Clear Pending Register */
    uint32_t RESERVED3[24U];
    volatile uint32_t IABR[8U];              /*!< Interrupt Active bit Register */
    uint32_t RESERVED4[56U];
    volatile uint8_t  IP[240U];              /*!< Interrupt Priority Register (8Bit wide) */
    uint32_t RESERVED5[644U];
    volatile uint32_t STIR;                   /*!< Software Trigger Interrupt Register */
} NVIC_Type;

/* ARM Cortex-M4 SysTick */
typedef struct {
    volatile uint32_t CTRL;                   /*!< SysTick Control and Status Register */
    volatile uint32_t LOAD;                   /*!< SysTick Reload Value Register */
    volatile uint32_t VAL;                    /*!< SysTick Current Value Register */
    volatile uint32_t CALIB;                  /*!< SysTick Calibration Register */
} SysTick_Type;

/* ARM Cortex-M4 MPU */
typedef struct {
    volatile uint32_t TYPE;                   /*!< MPU Type Register */
    volatile uint32_t CTRL;                   /*!< MPU Control Register */
    volatile uint32_t RNR;                    /*!< MPU Region RNRber Register */
    volatile uint32_t RBAR;                   /*!< MPU Region Base Address Register */
    volatile uint32_t RASR;                   /*!< MPU Region Attribute and Size Register */
    volatile uint32_t RBAR_A1;                /*!< MPU Alias 1 Region Base Address Register */
    volatile uint32_t RASR_A1;                /*!< MPU Alias 1 Region Attribute and Size Register */
    volatile uint32_t RBAR_A2;                /*!< MPU Alias 2 Region Base Address Register */
    volatile uint32_t RASR_A2;                /*!< MPU Alias 2 Region Attribute and Size Register */
    volatile uint32_t RBAR_A3;                /*!< MPU Alias 3 Region Base Address Register */
    volatile uint32_t RASR_A3;                /*!< MPU Alias 3 Region Attribute and Size Register */
} MPU_Type;

#endif /* S32_CORE_CM4_H */ 
