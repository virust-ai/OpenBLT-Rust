/* Memory regions for S32K148 */
MEMORY
{
  /* Bootloader region */
  FLASH_BL (rx) : ORIGIN = 0x00000000, LENGTH = 64K
  /* Application region */
  FLASH_APP (rx) : ORIGIN = 0x00010000, LENGTH = 960K
  /* RAM region */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}

/* Entry point */
ENTRY(main)

/* Stack size */
STACK_SIZE = 0x4000;

SECTIONS
{
  /* Bootloader code and data */
  .bootloader :
  {
    . = ALIGN(4);
    KEEP(*(.vector_table))
    *(.text*)
    *(.rodata*)
    *(.data*)
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
  } > FLASH_BL

  /* Application code and data */
  .application :
  {
    . = ALIGN(4);
    KEEP(*(.app_vector_table))
    *(.app_text*)
    *(.app_rodata*)
    *(.app_data*)
    *(.app_bss*)
    . = ALIGN(4);
  } > FLASH_APP

  /* Stack */
  .stack (NOLOAD) :
  {
    . = ALIGN(8);
    *(.stack)
    . = . + STACK_SIZE;
    . = ALIGN(8);
  } > RAM

  /* Heap */
  .heap (NOLOAD) :
  {
    . = ALIGN(8);
    *(.heap)
    . = . + 0x1000;
    . = ALIGN(8);
  } > RAM
} 
