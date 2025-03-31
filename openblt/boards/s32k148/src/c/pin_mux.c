/******************************************************************************
 * File: pin_mux.c
 * Purpose: Pin multiplexing configuration for S32K148
 ******************************************************************************/

#include "pin_mux.h"
#include "S32K148.h"

/* Pin configuration array */
const pin_config_t pinConfig[] = {
    /* Configure CAN0 pins */
    {
        .port = PORTA,
        .pin = 12U,
        .mux = PORT_MUX_ALT2,    /* CAN0_TX */
        .direction = PIN_OUTPUT,
        .pullConfig = PORT_PULL_DISABLE,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },
    {
        .port = PORTA,
        .pin = 13U,
        .mux = PORT_MUX_ALT2,    /* CAN0_RX */
        .direction = PIN_INPUT,
        .pullConfig = PORT_PULL_DISABLE,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },

    /* Configure LED pins */
    {
        .port = PORTD,
        .pin = 0U,
        .mux = PORT_MUX_GPIO,    /* LED_RED */
        .direction = PIN_OUTPUT,
        .pullConfig = PORT_PULL_DISABLE,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },
    {
        .port = PORTD,
        .pin = 15U,
        .mux = PORT_MUX_GPIO,    /* LED_GREEN */
        .direction = PIN_OUTPUT,
        .pullConfig = PORT_PULL_DISABLE,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },
    {
        .port = PORTD,
        .pin = 16U,
        .mux = PORT_MUX_GPIO,    /* LED_BLUE */
        .direction = PIN_OUTPUT,
        .pullConfig = PORT_PULL_DISABLE,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },

    /* Configure programming pin */
    {
        .port = PORTC,
        .pin = 13U,
        .mux = PORT_MUX_GPIO,    /* PROG_PIN */
        .direction = PIN_INPUT,
        .pullConfig = PORT_PULL_UP,
        .driveSelect = PORT_LOW_DRIVE,
        .passiveFilter = false,
        .lockRegister = false,
        .interrupt = PORT_INT_DISABLE,
    },
};

/* Initialize pin configuration */
void PIN_MUX_Init(void)
{
    const uint32_t num_pins = sizeof(pinConfig) / sizeof(pin_config_t);

    /* Enable clock for all ports */
    PCC->PCCn[PCC_PORTA_INDEX] |= PCC_PCCn_CGC_MASK;
    PCC->PCCn[PCC_PORTB_INDEX] |= PCC_PCCn_CGC_MASK;
    PCC->PCCn[PCC_PORTC_INDEX] |= PCC_PCCn_CGC_MASK;
    PCC->PCCn[PCC_PORTD_INDEX] |= PCC_PCCn_CGC_MASK;
    PCC->PCCn[PCC_PORTE_INDEX] |= PCC_PCCn_CGC_MASK;

    /* Configure each pin */
    for (uint32_t i = 0; i < num_pins; i++)
    {
        /* Configure pin multiplexing */
        pinConfig[i].port->PCR[pinConfig[i].pin] = PORT_PCR_MUX(pinConfig[i].mux) |
                                                  PORT_PCR_PE(pinConfig[i].pullConfig != PORT_PULL_DISABLE) |
                                                  PORT_PCR_PS(pinConfig[i].pullConfig == PORT_PULL_UP) |
                                                  PORT_PCR_DSE(pinConfig[i].driveSelect) |
                                                  PORT_PCR_PFE(pinConfig[i].passiveFilter) |
                                                  PORT_PCR_LK(pinConfig[i].lockRegister) |
                                                  PORT_PCR_IRQC(pinConfig[i].interrupt);

        /* Configure pin direction */
        if (pinConfig[i].direction == PIN_OUTPUT)
        {
            pinConfig[i].port->PDDR |= (1U << pinConfig[i].pin);
        }
        else
        {
            pinConfig[i].port->PDDR &= ~(1U << pinConfig[i].pin);
        }
    }
} 
