#ifndef SYSTEM_S32K148_H
#define SYSTEM_S32K148_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* System clock frequency (core clock) */
extern uint32_t SystemCoreClock;

/* Bus clock frequency */
extern uint32_t SystemBusClock;

/* Slow clock frequency */
extern uint32_t SystemSlowClock;

/**
 * @brief Initialize the system
 *
 * @param  none
 * @return none
 *
 * @brief  Setup the microcontroller system.
 *         Initialize the System and update the SystemCoreClock variable.
 */
void SystemInit(void);

/**
 * @brief Update SystemCoreClock variable
 *
 * @param  none
 * @return none
 *
 * @brief  Updates the SystemCoreClock with current core Clock
 *         retrieved from cpu registers.
 */
void SystemCoreClockUpdate(void);

#ifdef __cplusplus
}
#endif

#endif /* SYSTEM_S32K148_H */ 
