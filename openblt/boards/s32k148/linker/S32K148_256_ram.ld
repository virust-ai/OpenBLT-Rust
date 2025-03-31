/* Memory regions for S32K148 RAM */
MEMORY
{
  /* RAM region */
  RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}

/* Entry point */
ENTRY(main)

/* Stack size */
STACK_SIZE = 0x4000;

SECTIONS
{
  /* Code and data */
  .text :
  {
    . = ALIGN(4);
    *(.text*)
    *(.rodata*)
    *(.data*)
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
  } > RAM

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
