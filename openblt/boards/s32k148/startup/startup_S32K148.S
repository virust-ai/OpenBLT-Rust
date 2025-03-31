/******************************************************************************
 * File: startup_S32K148.S
 * Purpose: Startup file for S32K148
 ******************************************************************************/

    .syntax unified
    .arch armv7-m
    .thumb

    .section .isr_vector, "a"
    .align 2
    .globl __isr_vector
__isr_vector:
    .long   __StackTop            /* Top of Stack */
    .long   Reset_Handler         /* Reset Handler */
    .long   NMI_Handler           /* NMI Handler */
    .long   HardFault_Handler     /* Hard Fault Handler */
    .long   MemManage_Handler     /* MPU Fault Handler */
    .long   BusFault_Handler      /* Bus Fault Handler */
    .long   UsageFault_Handler    /* Usage Fault Handler */
    .long   0                     /* Reserved */
    .long   0                     /* Reserved */
    .long   0                     /* Reserved */
    .long   0                     /* Reserved */
    .long   SVC_Handler           /* SVCall Handler */
    .long   DebugMon_Handler      /* Debug Monitor Handler */
    .long   0                     /* Reserved */
    .long   PendSV_Handler        /* PendSV Handler */
    .long   SysTick_Handler       /* SysTick Handler */

    /* External Interrupts */
    .long   DMA0_IRQHandler       /* DMA channel 0 transfer complete/error interrupt */
    .long   DMA1_IRQHandler       /* DMA channel 1 transfer complete/error interrupt */
    .long   DMA2_IRQHandler       /* DMA channel 2 transfer complete/error interrupt */
    .long   DMA3_IRQHandler       /* DMA channel 3 transfer complete/error interrupt */
    .long   DMA4_IRQHandler       /* DMA channel 4 transfer complete/error interrupt */
    .long   DMA5_IRQHandler       /* DMA channel 5 transfer complete/error interrupt */
    .long   DMA6_IRQHandler       /* DMA channel 6 transfer complete/error interrupt */
    .long   DMA7_IRQHandler       /* DMA channel 7 transfer complete/error interrupt */
    .long   DMA8_IRQHandler       /* DMA channel 8 transfer complete/error interrupt */
    .long   DMA9_IRQHandler       /* DMA channel 9 transfer complete/error interrupt */
    .long   DMA10_IRQHandler      /* DMA channel 10 transfer complete/error interrupt */
    .long   DMA11_IRQHandler      /* DMA channel 11 transfer complete/error interrupt */
    .long   DMA12_IRQHandler      /* DMA channel 12 transfer complete/error interrupt */
    .long   DMA13_IRQHandler      /* DMA channel 13 transfer complete/error interrupt */
    .long   DMA14_IRQHandler      /* DMA channel 14 transfer complete/error interrupt */
    .long   DMA15_IRQHandler      /* DMA channel 15 transfer complete/error interrupt */
    .long   DMA_Error_IRQHandler  /* DMA error interrupt channels 0-15 */
    .long   MCM_IRQHandler        /* MCM interrupt */
    .long   FTFC_IRQHandler       /* Command complete and read collision */
    .long   Read_Collision_IRQHandler /* Read collision */
    .long   LVD_LVW_IRQHandler    /* Low-voltage detect, low-voltage warning */
    .long   LLWU_IRQHandler       /* Low leakage wakeup */
    .long   WDOG_EWM_IRQHandler   /* WDOG and EWM interrupt */
    .long   RNG_IRQHandler        /* RNG interrupt */
    .long   I2C0_IRQHandler       /* I2C0 interrupt */
    .long   I2C1_IRQHandler       /* I2C1 interrupt */
    .long   I2C2_IRQHandler       /* I2C2 interrupt */
    .long   I2C3_IRQHandler       /* I2C3 interrupt */
    .long   I2C4_IRQHandler       /* I2C4 interrupt */
    .long   I2C5_IRQHandler       /* I2C5 interrupt */
    .long   SPI0_IRQHandler       /* SPI0 interrupt */
    .long   SPI1_IRQHandler       /* SPI1 interrupt */
    .long   SPI2_IRQHandler       /* SPI2 interrupt */
    .long   CAN0_ORed_MessageBuffer_IRQHandler /* CAN0 OR'd message buffers interrupt */
    .long   CAN0_Bus_Off_IRQHandler            /* CAN0 bus off interrupt */
    .long   CAN0_Error_IRQHandler              /* CAN0 error interrupt */
    .long   CAN0_Tx_Warning_IRQHandler         /* CAN0 Tx warning interrupt */
    .long   CAN0_Rx_Warning_IRQHandler         /* CAN0 Rx warning interrupt */
    .long   CAN0_Wake_Up_IRQHandler            /* CAN0 wake up interrupt */
    .long   CAN1_ORed_MessageBuffer_IRQHandler /* CAN1 OR'd message buffers interrupt */
    .long   CAN1_Bus_Off_IRQHandler            /* CAN1 bus off interrupt */
    .long   CAN1_Error_IRQHandler              /* CAN1 error interrupt */
    .long   CAN1_Tx_Warning_IRQHandler         /* CAN1 Tx warning interrupt */
    .long   CAN1_Rx_Warning_IRQHandler         /* CAN1 Rx warning interrupt */
    .long   CAN1_Wake_Up_IRQHandler            /* CAN1 wake up interrupt */
    .long   CAN2_ORed_MessageBuffer_IRQHandler /* CAN2 OR'd message buffers interrupt */
    .long   CAN2_Bus_Off_IRQHandler            /* CAN2 bus off interrupt */
    .long   CAN2_Error_IRQHandler              /* CAN2 error interrupt */
    .long   CAN2_Tx_Warning_IRQHandler         /* CAN2 Tx warning interrupt */
    .long   CAN2_Rx_Warning_IRQHandler         /* CAN2 Rx warning interrupt */
    .long   CAN2_Wake_Up_IRQHandler            /* CAN2 wake up interrupt */
    .long   DefaultISR                         /* 83 */
    .long   DefaultISR                         /* 84 */
    .long   DefaultISR                         /* 85 */
    .long   DefaultISR                         /* 86 */
    .long   DefaultISR                         /* 87 */
    .long   DefaultISR                         /* 88 */
    .long   DefaultISR                         /* 89 */
    .long   DefaultISR                         /* 90 */
    .long   DefaultISR                         /* 91 */
    .long   DefaultISR                         /* 92 */
    .long   DefaultISR                         /* 93 */
    .long   DefaultISR                         /* 94 */
    .long   DefaultISR                         /* 95 */
    .long   DefaultISR                         /* 96 */
    .long   DefaultISR                         /* 97 */
    .long   DefaultISR                         /* 98 */
    .long   DefaultISR                         /* 99 */
    .long   DefaultISR                         /* 100 */
    .long   DefaultISR                         /* 101 */
    .long   DefaultISR                         /* 102 */
    .long   DefaultISR                         /* 103 */
    .long   DefaultISR                         /* 104 */
    .long   DefaultISR                         /* 105 */
    .long   DefaultISR                         /* 106 */
    .long   DefaultISR                         /* 107 */
    .long   DefaultISR                         /* 108 */
    .long   DefaultISR                         /* 109 */
    .long   DefaultISR                         /* 110 */
    .long   DefaultISR                         /* 111 */
    .long   DefaultISR                         /* 112 */
    .long   DefaultISR                         /* 113 */
    .long   DefaultISR                         /* 114 */
    .long   DefaultISR                         /* 115 */
    .long   DefaultISR                         /* 116 */
    .long   DefaultISR                         /* 117 */
    .long   DefaultISR                         /* 118 */
    .long   DefaultISR                         /* 119 */
    .long   DefaultISR                         /* 120 */
    .long   DefaultISR                         /* 121 */
    .long   DefaultISR                         /* 122 */
    .long   DefaultISR                         /* 123 */
    .long   DefaultISR                         /* 124 */
    .long   DefaultISR                         /* 125 */
    .long   DefaultISR                         /* 126 */
    .long   DefaultISR                         /* 127 */
    .long   DefaultISR                         /* 128 */
    .long   DefaultISR                         /* 129 */
    .long   DefaultISR                         /* 130 */
    .long   DefaultISR                         /* 131 */
    .long   DefaultISR                         /* 132 */
    .long   DefaultISR                         /* 133 */
    .long   DefaultISR                         /* 134 */
    .long   DefaultISR                         /* 135 */
    .long   DefaultISR                         /* 136 */
    .long   DefaultISR                         /* 137 */
    .long   DefaultISR                         /* 138 */
    .long   DefaultISR                         /* 139 */
    .long   DefaultISR                         /* 140 */
    .long   DefaultISR                         /* 141 */
    .long   DefaultISR                         /* 142 */
    .long   DefaultISR                         /* 143 */
    .long   DefaultISR                         /* 144 */
    .long   DefaultISR                         /* 145 */
    .long   DefaultISR                         /* 146 */
    .long   DefaultISR                         /* 147 */
    .long   DefaultISR                         /* 148 */
    .long   DefaultISR                         /* 149 */
    .long   DefaultISR                         /* 150 */
    .long   DefaultISR                         /* 151 */
    .long   DefaultISR                         /* 152 */
    .long   DefaultISR                         /* 153 */
    .long   DefaultISR                         /* 154 */
    .long   DefaultISR                         /* 155 */
    .long   DefaultISR                         /* 156 */
    .long   DefaultISR                         /* 157 */
    .long   DefaultISR                         /* 158 */
    .long   DefaultISR                         /* 159 */
    .long   DefaultISR                         /* 160 */
    .long   DefaultISR                         /* 161 */
    .long   DefaultISR                         /* 162 */
    .long   DefaultISR                         /* 163 */
    .long   DefaultISR                         /* 164 */
    .long   DefaultISR                         /* 165 */
    .long   DefaultISR                         /* 166 */
    .long   DefaultISR                         /* 167 */
    .long   DefaultISR                         /* 168 */
    .long   DefaultISR                         /* 169 */
    .long   DefaultISR                         /* 170 */
    .long   DefaultISR                         /* 171 */
    .long   DefaultISR                         /* 172 */
    .long   DefaultISR                         /* 173 */
    .long   DefaultISR                         /* 174 */
    .long   DefaultISR                         /* 175 */
    .long   DefaultISR                         /* 176 */
    .long   DefaultISR                         /* 177 */
    .long   DefaultISR                         /* 178 */
    .long   DefaultISR                         /* 179 */
    .long   DefaultISR                         /* 180 */
    .long   DefaultISR                         /* 181 */
    .long   DefaultISR                         /* 182 */
    .long   DefaultISR                         /* 183 */
    .long   DefaultISR                         /* 184 */
    .long   DefaultISR                         /* 185 */
    .long   DefaultISR                         /* 186 */
    .long   DefaultISR                         /* 187 */
    .long   DefaultISR                         /* 188 */
    .long   DefaultISR                         /* 189 */
    .long   DefaultISR                         /* 190 */
    .long   DefaultISR                         /* 191 */
    .long   DefaultISR                         /* 192 */
    .long   DefaultISR                         /* 193 */
    .long   DefaultISR                         /* 194 */
    .long   DefaultISR                         /* 195 */
    .long   DefaultISR                         /* 196 */
    .long   DefaultISR                         /* 197 */
    .long   DefaultISR                         /* 198 */
    .long   DefaultISR                         /* 199 */
    .long   DefaultISR                         /* 200 */
    .long   DefaultISR                         /* 201 */
    .long   DefaultISR                         /* 202 */
    .long   DefaultISR                         /* 203 */
    .long   DefaultISR                         /* 204 */
    .long   DefaultISR                         /* 205 */
    .long   DefaultISR                         /* 206 */
    .long   DefaultISR                         /* 207 */
    .long   DefaultISR                         /* 208 */
    .long   DefaultISR                         /* 209 */
    .long   DefaultISR                         /* 210 */
    .long   DefaultISR                         /* 211 */
    .long   DefaultISR                         /* 212 */
    .long   DefaultISR                         /* 213 */
    .long   DefaultISR                         /* 214 */
    .long   DefaultISR                         /* 215 */
    .long   DefaultISR                         /* 216 */
    .long   DefaultISR                         /* 217 */
    .long   DefaultISR                         /* 218 */
    .long   DefaultISR                         /* 219 */
    .long   DefaultISR                         /* 220 */
    .long   DefaultISR                         /* 221 */
    .long   DefaultISR                         /* 222 */
    .long   DefaultISR                         /* 223 */
    .long   DefaultISR                         /* 224 */
    .long   DefaultISR                         /* 225 */
    .long   DefaultISR                         /* 226 */
    .long   DefaultISR                         /* 227 */
    .long   DefaultISR                         /* 228 */
    .long   DefaultISR                         /* 229 */
    .long   DefaultISR                         /* 230 */
    .long   DefaultISR                         /* 231 */
    .long   DefaultISR                         /* 232 */
    .long   DefaultISR                         /* 233 */
    .long   DefaultISR                         /* 234 */
    .long   DefaultISR                         /* 235 */
    .long   DefaultISR                         /* 236 */
    .long   DefaultISR                         /* 237 */
    .long   DefaultISR                         /* 238 */
    .long   DefaultISR                         /* 239 */
    .long   DefaultISR                         /* 240 */
    .long   DefaultISR                         /* 241 */
    .long   DefaultISR                         /* 242 */
    .long   DefaultISR                         /* 243 */
    .long   DefaultISR                         /* 244 */
    .long   DefaultISR                         /* 245 */
    .long   DefaultISR                         /* 246 */
    .long   DefaultISR                         /* 247 */
    .long   DefaultISR                         /* 248 */
    .long   DefaultISR                         /* 249 */
    .long   DefaultISR                         /* 250 */
    .long   DefaultISR                         /* 251 */
    .long   DefaultISR                         /* 252 */
    .long   DefaultISR                         /* 253 */
    .long   DefaultISR                         /* 254 */
    .long   0xFFFFFFFF                         /* Reserved for user TRIM value */

    .size   __isr_vector, . - __isr_vector

    .section .text.Reset_Handler
    .thumb_func
    .align 2
    .globl   Reset_Handler
    .type    Reset_Handler, %function
Reset_Handler:
    /* Disable interrupts */
    cpsid   i

    /* Initialize the stack pointer */
    ldr     r0, =__StackTop
    mov     sp, r0

    /* Initialize data section */
    ldr     r0, =_sdata
    ldr     r1, =_edata
    ldr     r2, =_sidata
    b       copy_data_loop

copy_data_loop:
    cmp     r0, r1
    bcs     copy_data_done
    ldr     r3, [r2], #4
    str     r3, [r0], #4
    b       copy_data_loop

copy_data_done:
    /* Zero initialize bss section */
    ldr     r0, =_sbss
    ldr     r1, =_ebss
    mov     r2, #0
    b       zero_bss_loop

zero_bss_loop:
    cmp     r0, r1
    bcs     zero_bss_done
    str     r2, [r0], #4
    b       zero_bss_loop

zero_bss_done:
    /* Enable interrupts */
    cpsie   i

    /* Call system initialization */
    bl      SystemInit

    /* Call main */
    bl      main

    /* Should never get here */
    b       .

    .pool
    .size Reset_Handler, . - Reset_Handler

    .section .text.Default_Handler
    .thumb_func
    .align 2
DefaultISR:
    b       .
    .size DefaultISR, . - DefaultISR

    .section .text.HardFault_Handler
    .thumb_func
    .align 2
    .weak   HardFault_Handler
    .type   HardFault_Handler, %function
HardFault_Handler:
    b       .
    .size HardFault_Handler, . - HardFault_Handler

    .section .text.MemManage_Handler
    .thumb_func
    .align 2
    .weak   MemManage_Handler
    .type   MemManage_Handler, %function
MemManage_Handler:
    b       .
    .size MemManage_Handler, . - MemManage_Handler

    .section .text.BusFault_Handler
    .thumb_func
    .align 2
    .weak   BusFault_Handler
    .type   BusFault_Handler, %function
BusFault_Handler:
    b       .
    .size BusFault_Handler, . - BusFault_Handler

    .section .text.UsageFault_Handler
    .thumb_func
    .align 2
    .weak   UsageFault_Handler
    .type   UsageFault_Handler, %function
UsageFault_Handler:
    b       .
    .size UsageFault_Handler, . - UsageFault_Handler

    .section .text.SVC_Handler
    .thumb_func
    .align 2
    .weak   SVC_Handler
    .type   SVC_Handler, %function
SVC_Handler:
    b       .
    .size SVC_Handler, . - SVC_Handler

    .section .text.DebugMon_Handler
    .thumb_func
    .align 2
    .weak   DebugMon_Handler
    .type   DebugMon_Handler, %function
DebugMon_Handler:
    b       .
    .size DebugMon_Handler, . - DebugMon_Handler

    .section .text.PendSV_Handler
    .thumb_func
    .align 2
    .weak   PendSV_Handler
    .type   PendSV_Handler, %function
PendSV_Handler:
    b       .
    .size PendSV_Handler, . - PendSV_Handler

    .section .text.SysTick_Handler
    .thumb_func
    .align 2
    .weak   SysTick_Handler
    .type   SysTick_Handler, %function
SysTick_Handler:
    b       .
    .size SysTick_Handler, . - SysTick_Handler

    .section .text.NMI_Handler
    .thumb_func
    .align 2
    .weak   NMI_Handler
    .type   NMI_Handler, %function
NMI_Handler:
    b       .
    .size NMI_Handler, . - NMI_Handler

    /* External Interrupt Handlers */
    .section .text.DMA0_IRQHandler
    .thumb_func
    .align 2
    .weak   DMA0_IRQHandler
    .type   DMA0_IRQHandler, %function
DMA0_IRQHandler:
    b       .
    .size DMA0_IRQHandler, . - DMA0_IRQHandler

    /* Add all other interrupt handlers similarly */
    /* ... */

    .end 
