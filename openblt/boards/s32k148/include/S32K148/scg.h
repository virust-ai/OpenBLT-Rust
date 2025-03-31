#ifndef SCG_H
#define SCG_H

#include <stdint.h>

/* SCG Clock ranges */
#define SCG_SIRC_RANGE_HIGH              1U
#define SCG_FIRC_RANGE_48M               0U
#define SCG_SOSC_RANGE_HIGH              2U
#define SCG_SOSC_GAIN_HIGH               2U
#define SCG_SOSC_REF_OSC                 0U

/* SCG Clock dividers */
#define SCG_ASYNC_CLOCK_DIV_BY_1         1U
#define SCG_ASYNC_CLOCK_DIV_BY_2         2U
#define SCG_ASYNC_CLOCK_DIV_BY_4         4U

/* SCG System PLL settings */
#define SCG_SPLL_CLOCK_SOURCE_SOSC       1U
#define SCG_SPLL_CLOCK_PREDIV_BY_1       1U
#define SCG_SPLL_CLOCK_MULTIPLY_BY_20    20U

/* SCG System clock sources */
#define SCG_SYSTEM_CLOCK_SRC_SPLL        6U

/* SCG System clock dividers */
#define SCG_SYSTEM_CLOCK_DIV_BY_1        1U
#define SCG_SYSTEM_CLOCK_DIV_BY_2        2U
#define SCG_SYSTEM_CLOCK_DIV_BY_4        4U

/* Function prototypes */
void SCG_Init(const void *config);

#endif /* SCG_H */ 
