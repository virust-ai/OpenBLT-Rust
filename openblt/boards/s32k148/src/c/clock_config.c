/******************************************************************************
 * File: clock_config.c
 * Purpose: Clock configuration for S32K148
 ******************************************************************************/

#include <stdint.h>
#include <stdbool.h>
#include "S32K148.h"
#include "clock_config.h"

/* SIRC configuration */
static const scg_sirc_config_t scgSircConfig = {
    .enableInStop = true,                      /* SIRC is enabled in stop mode */
    .enableInLowPower = true,                  /* SIRC is enabled in low power mode */
    .range = SCG_SIRC_RANGE_HIGH,             /* High range (8 MHz) */
    .div1 = SCG_ASYNC_CLOCK_DIV_BY_1,         /* SIRCDIV1: divided by 1 */
    .div2 = SCG_ASYNC_CLOCK_DIV_BY_2          /* SIRCDIV2: divided by 2 */
};

/* FIRC configuration */
static const scg_firc_config_t scgFircConfig = {
    .enableInStop = true,                      /* FIRC is enabled in stop mode */
    .enableInLowPower = true,                  /* FIRC is enabled in low power mode */
    .range = SCG_FIRC_RANGE_48M,              /* 48 MHz */
    .div1 = SCG_ASYNC_CLOCK_DIV_BY_1,         /* FIRCDIV1: divided by 1 */
    .div2 = SCG_ASYNC_CLOCK_DIV_BY_1          /* FIRCDIV2: divided by 1 */
};

/* System OSC configuration */
static const scg_sosc_config_t scgSoscConfig = {
    .range = SCG_SOSC_RANGE_HIGH,             /* High frequency range */
    .gain = SCG_SOSC_GAIN_HIGH,               /* High gain */
    .extRef = SCG_SOSC_REF_OSC,               /* Use external oscillator */
    .div1 = SCG_ASYNC_CLOCK_DIV_BY_1,         /* SOSCDIV1: divided by 1 */
    .div2 = SCG_ASYNC_CLOCK_DIV_BY_1          /* SOSCDIV2: divided by 1 */
};

/* System PLL configuration */
static const scg_spll_config_t scgSpllConfig = {
    .source = SCG_SPLL_CLOCK_SOURCE_SOSC,     /* Source: System OSC */
    .prediv = SCG_SPLL_CLOCK_PREDIV_BY_1,     /* Predivided by 1 */
    .mult = SCG_SPLL_CLOCK_MULTIPLY_BY_20,    /* Multiply by 20 */
    .div1 = SCG_ASYNC_CLOCK_DIV_BY_2,         /* SPLLDIV1: divided by 2 */
    .div2 = SCG_ASYNC_CLOCK_DIV_BY_4          /* SPLLDIV2: divided by 4 */
};

/* Run mode configuration */
static const scg_rccr_config_t scgRccrConfig = {
    .src = SYSTEM_CLOCK_SRC_SIRC,             /* SIRC */
    .divCore = SYSTEM_CLOCK_DIV_BY_1,         /* Core clock divided by 1 */
    .divBus = SYSTEM_CLOCK_DIV_BY_1,          /* Bus clock divided by 1 */
    .divSlow = SYSTEM_CLOCK_DIV_BY_2          /* Slow clock divided by 2 */
};

/* VLPR mode configuration */
static const scg_rccr_config_t scgVccrConfig = {
    .src = SYSTEM_CLOCK_SRC_SIRC,             /* SIRC */
    .divCore = SYSTEM_CLOCK_DIV_BY_2,         /* Core clock divided by 2 */
    .divBus = SYSTEM_CLOCK_DIV_BY_1,          /* Bus clock divided by 1 */
    .divSlow = SYSTEM_CLOCK_DIV_BY_4          /* Slow clock divided by 4 */
};

/* HSRUN mode configuration */
static const scg_rccr_config_t scgHccrConfig = {
    .src = SYSTEM_CLOCK_SRC_SYS_PLL,          /* System PLL */
    .divCore = SYSTEM_CLOCK_DIV_BY_1,         /* Core clock divided by 1 */
    .divBus = SYSTEM_CLOCK_DIV_BY_2,          /* Bus clock divided by 2 */
    .divSlow = SYSTEM_CLOCK_DIV_BY_4          /* Slow clock divided by 4 */
};

/* Clock configuration */
static const scg_config_t scgConfig = {
    .sirc = &scgSircConfig,                   /* SIRC configuration */
    .firc = &scgFircConfig,                   /* FIRC configuration */
    .sosc = &scgSoscConfig,                   /* System OSC configuration */
    .spll = &scgSpllConfig,                   /* System PLL configuration */
    .rccr = &scgRccrConfig,                   /* RUN mode configuration */
    .vccr = &scgVccrConfig,                   /* VLPR mode configuration */
    .hccr = &scgHccrConfig                    /* HSRUN mode configuration */
};

/* Peripheral clock configuration */
static const pcc_config_t pccConfig = {
    .peripheralClocks = {
        {
            .clockName = PCC_PORTA_CLOCK,
            .clkGate = true,
            .clkSrc = 0U,
            .divider = 0U
        },
        {
            .clockName = PCC_PORTB_CLOCK,
            .clkGate = true,
            .clkSrc = 0U,
            .divider = 0U
        },
        {
            .clockName = PCC_PORTC_CLOCK,
            .clkGate = true,
            .clkSrc = 0U,
            .divider = 0U
        },
        {
            .clockName = PCC_PORTD_CLOCK,
            .clkGate = true,
            .clkSrc = 0U,
            .divider = 0U
        },
        {
            .clockName = PCC_PORTE_CLOCK,
            .clkGate = true,
            .clkSrc = 0U,
            .divider = 0U
        },
        {
            .clockName = PCC_FlexCAN0_CLOCK,
            .clkGate = true,
            .clkSrc = 1U,
            .divider = 0U
        },
        {
            .clockName = PCC_FlexCAN1_CLOCK,
            .clkGate = true,
            .clkSrc = 1U,
            .divider = 0U
        },
        {
            .clockName = PCC_FlexCAN2_CLOCK,
            .clkGate = true,
            .clkSrc = 1U,
            .divider = 0U
        }
    }
};

void CLOCK_CONFIG_Init(void)
{
    /* Initialize SCG */
    SCG_Init(&scgConfig);

    /* Initialize PCC */
    PCC_Init(&pccConfig);
} 
