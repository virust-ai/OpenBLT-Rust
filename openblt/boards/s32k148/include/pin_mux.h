/******************************************************************************
 * File: pin_mux.h
 * Purpose: Pin multiplexing header for S32K148
 ******************************************************************************/

#ifndef PIN_MUX_H
#define PIN_MUX_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>
#include <stdbool.h>

/* Pin direction options */
typedef enum {
    PIN_INPUT = 0U,
    PIN_OUTPUT = 1U,
} pin_direction_t;

/* Pin pull configuration options */
typedef enum {
    PORT_PULL_DISABLE = 0U,
    PORT_PULL_DOWN = 1U,
    PORT_PULL_UP = 2U,
} port_pull_config_t;

/* Pin drive strength options */
typedef enum {
    PORT_LOW_DRIVE = 0U,
    PORT_HIGH_DRIVE = 1U,
} port_drive_strength_t;

/* Pin interrupt configuration options */
typedef enum {
    PORT_INT_DISABLE = 0U,
    PORT_DMA_RISING = 1U,
    PORT_DMA_FALLING = 2U,
    PORT_DMA_EITHER = 3U,
    PORT_INT_LOGIC_0 = 8U,
    PORT_INT_RISING = 9U,
    PORT_INT_FALLING = 10U,
    PORT_INT_EITHER = 11U,
    PORT_INT_LOGIC_1 = 12U,
} port_interrupt_config_t;

/* Pin mux options */
typedef enum {
    PORT_MUX_DISABLED = 0U,
    PORT_MUX_GPIO = 1U,
    PORT_MUX_ALT2 = 2U,
    PORT_MUX_ALT3 = 3U,
    PORT_MUX_ALT4 = 4U,
    PORT_MUX_ALT5 = 5U,
    PORT_MUX_ALT6 = 6U,
    PORT_MUX_ALT7 = 7U,
} port_mux_t;

/* Pin configuration structure */
typedef struct {
    void* port;                      /* Port base address */
    uint32_t pin;                    /* Pin number */
    port_mux_t mux;                  /* Pin mux control */
    pin_direction_t direction;       /* Pin direction */
    port_pull_config_t pullConfig;   /* Pull resistor configuration */
    port_drive_strength_t driveSelect; /* Drive strength */
    bool passiveFilter;              /* Passive filter */
    bool lockRegister;               /* Lock register */
    port_interrupt_config_t interrupt; /* Interrupt configuration */
} pin_config_t;

/* Function declarations */
void PIN_MUX_Init(void);

/* External declarations */
extern const pin_config_t pinConfig[];

#ifdef __cplusplus
}
#endif

#endif /* PIN_MUX_H */ 
