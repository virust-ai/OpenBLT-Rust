#ifndef S32K148_H
#define S32K148_H

#include <stdint.h>
#include <stdbool.h>
#include "device_registers.h"
#include "s32_core_cm4.h"
#include "S32K148_features.h"
#include "S32K148/scg.h"
#include "S32K148/pcc.h"

/* ----------------------------------------------------------------------------
   -- Interrupt vector numbers
   ---------------------------------------------------------------------------- */
typedef enum {
  /* Auxiliary constants */
  NotAvail_IRQn                = -128,             /**< Not available device specific interrupt */

  /* Core interrupts */
  NonMaskableInt_IRQn         = -14,              /**< Non Maskable Interrupt */
  HardFault_IRQn             = -13,              /**< Cortex-M4 SV Hard Fault Interrupt */
  MemoryManagement_IRQn      = -12,              /**< Cortex-M4 Memory Management Interrupt */
  BusFault_IRQn              = -11,              /**< Cortex-M4 Bus Fault Interrupt */
  UsageFault_IRQn            = -10,              /**< Cortex-M4 Usage Fault Interrupt */
  SVCall_IRQn                = -5,               /**< Cortex-M4 SV Call Interrupt */
  DebugMonitor_IRQn          = -4,               /**< Cortex-M4 Debug Monitor Interrupt */
  PendSV_IRQn                = -2,               /**< Cortex-M4 Pend SV Interrupt */
  SysTick_IRQn               = -1,               /**< Cortex-M4 System Tick Interrupt */
} IRQn_Type;

/* Device Peripheral Access Layer */
#include "system_S32K148.h"

/* Register access macros */
#define S32_REG8(address)        (*((volatile uint8_t*)(address)))
#define S32_REG16(address)       (*((volatile uint16_t*)(address)))
#define S32_REG32(address)       (*((volatile uint32_t*)(address)))
#define S32_REG(type, address)   (*((volatile type*)(address)))

/* Base addresses */
#define SCG_BASE                             0x40064000u
#define PORTA_BASE                           0x40049000u
#define PORTB_BASE                           0x4004A000u
#define PORTC_BASE                           0x4004B000u
#define PORTD_BASE                           0x4004C000u
#define PORTE_BASE                           0x4004D000u
#define PTA_BASE                             0x400FF000u
#define PTB_BASE                             0x400FF040u
#define PTC_BASE                             0x400FF080u
#define PTD_BASE                             0x400FF0C0u
#define PTE_BASE                             0x400FF100u
#define FTFC_BASE                            0x40020000u
#define PCC_BASE                             0x40065000u
#define MPU_BASE                             0xE000ED90u

/* Register structs */
typedef struct {
    volatile uint32_t VERID;
    volatile uint32_t PARAM;
    volatile uint32_t CSR;
    volatile uint32_t RCCR;
    volatile uint32_t VCCR;
    volatile uint32_t HCCR;
    volatile uint32_t CLKOUTCNFG;
    volatile uint32_t SOSCCSR;
    volatile uint32_t SOSCDIV;
    volatile uint32_t SOSCCFG;
    volatile uint32_t SIRCCSR;
    volatile uint32_t SIRCDIV;
    volatile uint32_t SIRCCFG;
    volatile uint32_t FIRCCSR;
    volatile uint32_t FIRCDIV;
    volatile uint32_t FIRCCFG;
    volatile uint32_t SPLLCSR;
    volatile uint32_t SPLLDIV;
    volatile uint32_t SPLLCFG;
} SCG_Type;

typedef struct {
    volatile uint32_t PCR[32];
    volatile uint32_t GPCLR;
    volatile uint32_t GPCHR;
    volatile uint32_t GICLR;
    volatile uint32_t GICHR;
    volatile uint32_t ISFR;
    uint8_t RESERVED[12];
    volatile uint32_t DFER;
    volatile uint32_t DFCR;
    volatile uint32_t DFWR;
} PORT_Type;

typedef struct {
    volatile uint32_t PDOR;
    volatile uint32_t PSOR;
    volatile uint32_t PCOR;
    volatile uint32_t PTOR;
    volatile uint32_t PDIR;
    volatile uint32_t PDDR;
    volatile uint32_t PIDR;
} GPIO_Type;

typedef struct {
    volatile uint32_t FSTAT;
    volatile uint32_t FCNFG;
    volatile uint32_t FSEC;
    volatile uint32_t FOPT;
    volatile uint32_t FCCOB3;
    volatile uint32_t FCCOB2;
    volatile uint32_t FCCOB1;
    volatile uint32_t FCCOB0;
    volatile uint32_t FPROT3;
    volatile uint32_t FPROT2;
    volatile uint32_t FPROT1;
    volatile uint32_t FPROT0;
    volatile uint32_t FEPROT;
    volatile uint32_t FDPROT;
    volatile uint32_t FERSTAT;
    volatile uint32_t FERCNFG;
} FTFC_Type;

typedef struct {
    volatile uint32_t PCCn[128];
} PCC_Type;

/* Device instances */
#define SCG                                  ((SCG_Type *)SCG_BASE)
#define PORTA                                ((PORT_Type *)PORTA_BASE)
#define PORTB                                ((PORT_Type *)PORTB_BASE)
#define PORTC                                ((PORT_Type *)PORTC_BASE)
#define PORTD                                ((PORT_Type *)PORTD_BASE)
#define PORTE                                ((PORT_Type *)PORTE_BASE)
#define PTA                                  ((GPIO_Type *)PTA_BASE)
#define PTB                                  ((GPIO_Type *)PTB_BASE)
#define PTC                                  ((GPIO_Type *)PTC_BASE)
#define PTD                                  ((GPIO_Type *)PTD_BASE)
#define PTE                                  ((GPIO_Type *)PTE_BASE)
#define FTFC                                 ((FTFC_Type *)FTFC_BASE)
#define PCC                                  ((PCC_Type *)PCC_BASE)
#define MPU                                  ((MPU_Type *)MPU_BASE)

/* Register bit definitions */
#define SCG_SOSCCSR_SOSCEN_MASK            0x00000001u
#define SCG_SOSCCSR_SOSCVLD_MASK           0x00000002u
#define SCG_SPLLCSR_SPLLEN_MASK            0x00000001u
#define SCG_SPLLCSR_SPLLVLD_MASK           0x00000002u
#define SCG_CSR_SCS_MASK                   0x00000007u
#define SCG_RCCR_SCS(x)                    (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define SCG_SIRCCSR_SIRCDIV(x)             (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define SCG_FIRCCSR_FIRCDIV(x)             (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define SCG_SOSCCSR_SOSCDIV(x)             (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define SCG_SPLLCSR_SPLLDIV(x)             (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define SCG_SPLLCFG_MULT(x)                (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x0000001Fu)
#define FTFC_FSTAT_CCIF_MASK               0x00000001u
#define PCC_PCCn_PCS(x)                    (((uint32_t)(((uint32_t)(x)) << 0u)) & 0x00000007u)
#define PCC_PCCn_CGC_MASK                  0x00000040u

/* PCC indices */
#define PCC_PORTE_INDEX                    0x4Du

#endif /* S32K148_H */ 
