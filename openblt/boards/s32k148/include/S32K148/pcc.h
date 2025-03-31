#ifndef PCC_H
#define PCC_H

#include <stdint.h>

/* PCC Clock names */
#define PCC_PORTA_CLOCK                   0x124U
#define PCC_PORTB_CLOCK                   0x128U
#define PCC_PORTC_CLOCK                   0x12CU
#define PCC_PORTD_CLOCK                   0x130U
#define PCC_PORTE_CLOCK                   0x134U

/* FlexCAN clock names */
#define PCC_FlexCAN0_CLOCK               0x090U
#define PCC_FlexCAN1_CLOCK               0x094U
#define PCC_FlexCAN2_CLOCK               0x098U

/* Function prototypes */
void PCC_Init(const void *config);

#endif /* PCC_H */ 
