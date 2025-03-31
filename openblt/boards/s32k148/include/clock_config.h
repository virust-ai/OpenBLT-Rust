/******************************************************************************
 * File: clock_config.h
 * Purpose: Clock configuration header for S32K148
 ******************************************************************************/

#ifndef CLOCK_CONFIG_H
#define CLOCK_CONFIG_H

#include <stdint.h>
#include <stdbool.h>

/* Clock source options */
typedef enum {
    CLK_SRC_OFF = 0U,    /* Clock source disabled */
    CLK_SRC_SOSC = 1U,   /* System oscillator */
    CLK_SRC_SIRC = 2U,   /* Slow IRC */
    CLK_SRC_FIRC = 3U,   /* Fast IRC */
    CLK_SRC_SPLL = 6U,   /* System PLL */
} clock_source_t;

/* Clock divider options */
typedef enum {
    CLK_DIVIDER_OFF = 0U,
    CLK_DIVIDER_BY_1 = 1U,
    CLK_DIVIDER_BY_2 = 2U,
    CLK_DIVIDER_BY_4 = 3U,
    CLK_DIVIDER_BY_8 = 4U,
    CLK_DIVIDER_BY_16 = 5U,
    CLK_DIVIDER_BY_32 = 6U,
    CLK_DIVIDER_BY_64 = 7U,
} clock_divider_t;

/* Clock fractional divider options */
typedef enum {
    CLK_FRAC_OFF = 0U,
    CLK_FRAC_BY_1 = 1U,
    CLK_FRAC_BY_2 = 2U,
    CLK_FRAC_BY_3 = 3U,
    CLK_FRAC_BY_4 = 4U,
    CLK_FRAC_BY_5 = 5U,
    CLK_FRAC_BY_6 = 6U,
    CLK_FRAC_BY_7 = 7U,
} clock_frac_t;

/* SCG configuration structures */
typedef struct {
    bool enableInStop;
    bool enableInLowPower;
    uint32_t range;
    uint32_t div1;
    uint32_t div2;
} scg_sirc_config_t;

typedef struct {
    bool enableInStop;
    bool enableInLowPower;
    uint32_t range;
    uint32_t div1;
    uint32_t div2;
} scg_firc_config_t;

typedef struct {
    uint32_t range;
    uint32_t gain;
    uint32_t extRef;
    uint32_t div1;
    uint32_t div2;
} scg_sosc_config_t;

typedef struct {
    uint32_t source;
    uint32_t prediv;
    uint32_t mult;
    uint32_t div1;
    uint32_t div2;
} scg_spll_config_t;

typedef struct {
    uint32_t src;
    uint32_t divCore;
    uint32_t divBus;
    uint32_t divSlow;
} scg_rccr_config_t;

typedef struct {
    const scg_sirc_config_t *sirc;
    const scg_firc_config_t *firc;
    const scg_sosc_config_t *sosc;
    const scg_spll_config_t *spll;
    const scg_rccr_config_t *rccr;
    const scg_rccr_config_t *vccr;
    const scg_rccr_config_t *hccr;
} scg_config_t;

/* PCC configuration structures */
typedef struct {
    uint32_t clockName;
    bool clkGate;
    uint32_t clkSrc;
    uint32_t divider;
} pcc_peripheral_clock_config_t;

typedef struct {
    pcc_peripheral_clock_config_t peripheralClocks[8];
} pcc_config_t;

/* Function prototypes */
void CLOCK_CONFIG_Init(void);

#endif /* CLOCK_CONFIG_H */ 
