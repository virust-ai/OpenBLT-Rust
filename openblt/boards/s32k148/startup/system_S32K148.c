/******************************************************************************
 * File: system_S32K148.c
 * Purpose: System initialization for S32K148
 ******************************************************************************/

#include <stdint.h>
#include "system_S32K148.h"
#include "S32K148.h"
#include "S32K148/scg.h"
#include "S32K148/pcc.h"

/* Default system clock frequency */
#define DEFAULT_SYSTEM_CLOCK    80000000U

/* System clock frequency (core clock) */
uint32_t SystemCoreClock = DEFAULT_SYSTEM_CLOCK;

/* System bus clock frequency */
uint32_t SystemBusClock = DEFAULT_SYSTEM_CLOCK / 2;

/* System slow clock frequency */
uint32_t SystemSlowClock = DEFAULT_SYSTEM_CLOCK / 4;

/* Function declarations */
static void SystemClockConfig(void);
static void SystemFlashConfig(void);
static void SystemBusClockConfig(void);
static void SystemMemoryConfig(void);

/* System initialization function */
void SystemInit(void)
{
    /* Disable interrupts */
    __asm volatile ("cpsid i");

    /* Configure the system clock */
    SystemClockConfig();

    /* Configure the flash wait states */
    SystemFlashConfig();

    /* Configure the system bus clock */
    SystemBusClockConfig();

    /* Configure the system memory */
    SystemMemoryConfig();

    /* Enable interrupts */
    __asm volatile ("cpsie i");
}

/* System clock configuration */
static void SystemClockConfig(void)
{
    /* Configure the system clock to 80MHz */
    /* Enable external oscillator */
    SCG->SOSCCSR |= SCG_SOSCCSR_SOSCEN_MASK;
    while (!(SCG->SOSCCSR & SCG_SOSCCSR_SOSCVLD_MASK));

    /* Configure PLL */
    SCG->SPLLCSR = SCG_SPLLCSR_SPLLEN_MASK;
    while (!(SCG->SPLLCSR & SCG_SPLLCSR_SPLLVLD_MASK));

    /* Select PLL as system clock source */
    SCG->RCCR = SCG_RCCR_SCS(3);
    while ((SCG->CSR & SCG_CSR_SCS_MASK) != 3);
}

/* Flash wait states configuration */
static void SystemFlashConfig(void)
{
    /* Configure flash wait states for 80MHz */
    FTFC->FCCOB3 = 0x0A;  /* Program flash wait states */
    FTFC->FCCOB2 = 0x00;  /* Data flash wait states */
    FTFC->FCCOB1 = 0x00;  /* Reserved */
    FTFC->FCCOB0 = 0x00;  /* Reserved */
    FTFC->FSTAT = FTFC_FSTAT_CCIF_MASK;
    while (!(FTFC->FSTAT & FTFC_FSTAT_CCIF_MASK));
}

/* System bus clock configuration */
static void SystemBusClockConfig(void)
{
    /* Configure system bus clock divider */
    PCC->PCCn[PCC_PORTE_INDEX] |= PCC_PCCn_PCS(1);
    PCC->PCCn[PCC_PORTE_INDEX] |= PCC_PCCn_CGC_MASK;
}

/* System memory configuration */
static void SystemMemoryConfig(void)
{
    /* Configure memory protection unit */
    MPU->CTRL = 0x00000000;  /* Disable MPU */
}

/* Update SystemCoreClock variable */
void SystemCoreClockUpdate(void)
{
    uint32_t scs;
    uint32_t pll_clk;
    uint32_t sys_clk;

    /* Get system clock source */
    scs = SCG->CSR & SCG_CSR_SCS_MASK;

    /* Calculate system clock frequency */
    switch (scs)
    {
        case 0:  /* Slow IRC */
            sys_clk = SCG_SIRCCSR_SIRCDIV(SCG->SIRCCSR) + 1;
            break;

        case 1:  /* Fast IRC */
            sys_clk = SCG_FIRCCSR_FIRCDIV(SCG->FIRCCSR) + 1;
            break;

        case 2:  /* System OSC */
            sys_clk = SCG_SOSCCSR_SOSCDIV(SCG->SOSCCSR) + 1;
            break;

        case 3:  /* PLL */
            pll_clk = SCG_SPLLCSR_SPLLDIV(SCG->SPLLCSR) + 1;
            sys_clk = pll_clk * SCG_SPLLCFG_MULT(SCG->SPLLCFG);
            break;

        default:
            sys_clk = DEFAULT_SYSTEM_CLOCK;
            break;
    }

    SystemCoreClock = sys_clk;
    SystemBusClock = sys_clk / 2;
    SystemSlowClock = sys_clk / 4;
} 
