/******************************************************************************
 * File: system_S32K148.h
 * Purpose: System initialization header for S32K148
 ******************************************************************************/

#ifndef SYSTEM_S32K148_H
#define SYSTEM_S32K148_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

/* Default system clock frequency */
#define DEFAULT_SYSTEM_CLOCK    80000000U

/* External function declarations */
void SystemInit(void);
void SystemCoreClockUpdate(void);

/* External variable declarations */
extern uint32_t SystemCoreClock;

#ifdef __cplusplus
}
#endif

#endif /* SYSTEM_S32K148_H */ 
