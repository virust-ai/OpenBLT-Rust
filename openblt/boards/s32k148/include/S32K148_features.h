#ifndef S32K148_FEATURES_H
#define S32K148_FEATURES_H

#include <stdint.h>

/* SOC module features */
/* @brief PORT availability on the SoC. */
#define FEATURE_SOC_PORT_COUNT               (5)
#define FEATURE_SOC_SCG_COUNT               (1)

/* PORT module features */
/* @brief Width of PCR registers. */
#define FEATURE_PORT_PCR_WIDTH              (32u)
/* @brief Has GPIO feature. */
#define FEATURE_PORT_HAS_GPIO               (1)
/* @brief Has digital filter feature. */
#define FEATURE_PORT_HAS_DIGITAL_FILTER     (1)

/* SCG module features */
/* @brief Has SIRC fine trim. */
#define FEATURE_SCG_HAS_SIRC_FINE_TRIM     (1)
/* @brief Has FIRC trim. */
#define FEATURE_SCG_HAS_FIRC_TRIM          (1)
/* @brief Has SPLL trim. */
#define FEATURE_SCG_HAS_SPLL_TRIM          (1)

/* FLASH module features */
/* @brief Has flash cache. */
#define FEATURE_FLASH_HAS_CACHE            (1)
/* @brief Has flash prefetch buffer. */
#define FEATURE_FLASH_HAS_PREFETCH_BUFFER  (1)
/* @brief Has flash access control. */
#define FEATURE_FLASH_HAS_ACCESS_CONTROL   (1)

/* CAN module features */
/* @brief Has flexible data rate. */
#define FEATURE_CAN_HAS_FD                 (1)
/* @brief Has enhanced bit timing. */
#define FEATURE_CAN_HAS_PE_CLKSRC_SELECT   (1)

/* Clock names */
typedef enum {
    CORE_CLK,           /* Core clock */
    BUS_CLK,            /* Bus clock */
    SLOW_CLK,           /* Slow clock */
    CLKOUT_CLK,         /* CLKOUT clock */
    SIRC_CLK,           /* Slow IRC clock */
    FIRC_CLK,           /* Fast IRC clock */
    SOSC_CLK,           /* System OSC clock */
    SPLL_CLK,           /* System PLL clock */
    RTC_CLKIN_CLK,      /* RTC_CLKIN clock */
    SCG_END_OF_CLOCKS   /* End of SCG clocks */
} clock_names_t;

/* System clock divider enumeration */
typedef enum {
    SYSTEM_CLOCK_DIV_BY_1     = 0U,    /* Divided by 1 */
    SYSTEM_CLOCK_DIV_BY_2     = 1U,    /* Divided by 2 */
    SYSTEM_CLOCK_DIV_BY_3     = 2U,    /* Divided by 3 */
    SYSTEM_CLOCK_DIV_BY_4     = 3U,    /* Divided by 4 */
    SYSTEM_CLOCK_DIV_BY_5     = 4U,    /* Divided by 5 */
    SYSTEM_CLOCK_DIV_BY_6     = 5U,    /* Divided by 6 */
    SYSTEM_CLOCK_DIV_BY_7     = 6U,    /* Divided by 7 */
    SYSTEM_CLOCK_DIV_BY_8     = 7U     /* Divided by 8 */
} system_clock_divider_t;

/* System clock source enumeration */
typedef enum {
    SYSTEM_CLOCK_SRC_SOSC     = 1U,    /* System OSC */
    SYSTEM_CLOCK_SRC_SIRC     = 2U,    /* Slow IRC */
    SYSTEM_CLOCK_SRC_FIRC     = 3U,    /* Fast IRC */
    SYSTEM_CLOCK_SRC_SYS_PLL  = 6U     /* System PLL */
} system_clock_source_t;

#endif /* S32K148_FEATURES_H */ 
