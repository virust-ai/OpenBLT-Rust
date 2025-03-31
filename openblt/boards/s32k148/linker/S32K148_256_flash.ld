/* Entry Point */
ENTRY(Reset_Handler)

/* Memory regions for S32K148 */
MEMORY
{
  m_interrupts          (RX)  : ORIGIN = 0x00000000, LENGTH = 0x00000400
  m_flash_config       (RX)  : ORIGIN = 0x00000400, LENGTH = 0x00000010
  m_text              (RX)  : ORIGIN = 0x00000410, LENGTH = 0x0003FBF0
  m_data_2            (RW)  : ORIGIN = 0x20000000, LENGTH = 0x00010000
}

/* Stack size */
_Min_Stack_Size = 0x400;
_Min_Heap_Size = 0x200;

/* Define stack top */
_estack = ORIGIN(m_data_2) + LENGTH(m_data_2);
__StackTop = _estack;
__StackLimit = __StackTop - _Min_Stack_Size;

/* Define output sections */
SECTIONS
{
  /* The startup code goes first into internal flash */
  .interrupts :
  {
    __VECTOR_TABLE = .;
    . = ALIGN(4);
    KEEP(*(.isr_vector))     /* Startup code */
    . = ALIGN(4);
  } > m_interrupts

  .flash_config :
  {
    . = ALIGN(4);
    KEEP(*(.FlashConfig))    /* Flash Configuration Field (FCF) */
    . = ALIGN(4);
  } > m_flash_config

  /* The program code and other data goes into internal flash */
  .text :
  {
    . = ALIGN(4);
    *(.text)                 /* .text sections (code) */
    *(.text*)                /* .text* sections (code) */
    *(.rodata)              /* .rodata sections (constants, strings, etc.) */
    *(.rodata*)             /* .rodata* sections (constants, strings, etc.) */
    *(.glue_7)              /* glue arm to thumb code */
    *(.glue_7t)             /* glue thumb to arm code */
    *(.eh_frame)
    KEEP (*(.init))
    KEEP (*(.fini))
    . = ALIGN(4);
  } > m_text

  .ARM.extab :
  {
    *(.ARM.extab* .gnu.linkonce.armextab.*)
  } > m_text

  .ARM.exidx :
  {
    __exidx_start = .;
    *(.ARM.exidx* .gnu.linkonce.armexidx.*)
    __exidx_end = .;
  } > m_text

  .bootloader :
  {
    . = ALIGN(4);
    *(.bootloader*)         /* bootloader code */
    . = ALIGN(4);
  } > m_text

  /* Initialized data sections goes into RAM, load LMA copy after code */
  .data :
  {
    . = ALIGN(4);
    _sdata = .;        /* create a global symbol at data start */
    *(.data)           /* .data sections */
    *(.data*)          /* .data* sections */
    . = ALIGN(4);
    _edata = .;        /* define a global symbol at data end */
  } > m_data_2 AT > m_text
  _sidata = LOADADDR(.data);

  /* Uninitialized data section */
  . = ALIGN(4);
  .bss :
  {
    /* This is used by the startup in order to initialize the .bss section */
    _sbss = .;         /* define a global symbol at bss start */
    __bss_start__ = _sbss;
    *(.bss)
    *(.bss*)
    *(COMMON)
    . = ALIGN(4);
    _ebss = .;         /* define a global symbol at bss end */
    __bss_end__ = _ebss;
  } > m_data_2

  /* User_heap_stack section, used to check that there is enough RAM left */
  ._user_heap_stack :
  {
    . = ALIGN(8);
    PROVIDE ( end = . );
    PROVIDE ( _end = . );
    . = . + _Min_Heap_Size;
    . = . + _Min_Stack_Size;
    . = ALIGN(8);
  } > m_data_2

  /* Remove information from the compiler libraries */
  /DISCARD/ :
  {
    libc.a ( * )
    libm.a ( * )
    libgcc.a ( * )
  }
} 
