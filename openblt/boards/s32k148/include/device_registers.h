#ifndef DEVICE_REGISTERS_H
#define DEVICE_REGISTERS_H

#include <stdint.h>

/* Base addresses */
#define S32K148_FLASH_BASE      0x40020000u
#define S32K148_SCG_BASE        0x40064000u
#define S32K148_PCC_BASE        0x40065000u
#define S32K148_PORT_BASE       0x40049000u
#define S32K148_GPIO_BASE       0x400FF000u

/* Flash registers */
#define FLASH_STAT_REG          ((volatile uint32_t*)(S32K148_FLASH_BASE + 0x00u))
#define FLASH_CTRL_REG          ((volatile uint32_t*)(S32K148_FLASH_BASE + 0x04u))
#define FLASH_CMD_REG           ((volatile uint32_t*)(S32K148_FLASH_BASE + 0x08u))

/* SCG registers */
#define SCG_RCCR               ((volatile uint32_t*)(S32K148_SCG_BASE + 0x014u))
#define SCG_VCCR               ((volatile uint32_t*)(S32K148_SCG_BASE + 0x018u))
#define SCG_HCCR               ((volatile uint32_t*)(S32K148_SCG_BASE + 0x01Cu))
#define SCG_CLKOUTCNFG         ((volatile uint32_t*)(S32K148_SCG_BASE + 0x020u))

/* PCC registers */
#define PCC_PORTA              ((volatile uint32_t*)(S32K148_PCC_BASE + 0x124u))
#define PCC_PORTB              ((volatile uint32_t*)(S32K148_PCC_BASE + 0x128u))
#define PCC_PORTC              ((volatile uint32_t*)(S32K148_PCC_BASE + 0x12Cu))
#define PCC_PORTD              ((volatile uint32_t*)(S32K148_PCC_BASE + 0x130u))
#define PCC_PORTE              ((volatile uint32_t*)(S32K148_PCC_BASE + 0x134u))

/* PORT registers */
#define PORTA_PCR(n)           ((volatile uint32_t*)(S32K148_PORT_BASE + 0x000u + (n * 4u)))
#define PORTB_PCR(n)           ((volatile uint32_t*)(S32K148_PORT_BASE + 0x040u + (n * 4u)))
#define PORTC_PCR(n)           ((volatile uint32_t*)(S32K148_PORT_BASE + 0x080u + (n * 4u)))
#define PORTD_PCR(n)           ((volatile uint32_t*)(S32K148_PORT_BASE + 0x0C0u + (n * 4u)))
#define PORTE_PCR(n)           ((volatile uint32_t*)(S32K148_PORT_BASE + 0x100u + (n * 4u)))

/* GPIO registers */
#define GPIOA_PDOR             ((volatile uint32_t*)(S32K148_GPIO_BASE + 0x000u))
#define GPIOB_PDOR             ((volatile uint32_t*)(S32K148_GPIO_BASE + 0x040u))
#define GPIOC_PDOR             ((volatile uint32_t*)(S32K148_GPIO_BASE + 0x080u))
#define GPIOD_PDOR             ((volatile uint32_t*)(S32K148_GPIO_BASE + 0x0C0u))
#define GPIOE_PDOR             ((volatile uint32_t*)(S32K148_GPIO_BASE + 0x100u))

#endif /* DEVICE_REGISTERS_H */ 
