/* Linker script for a bare-metal RISC-V program */

OUTPUT_FORMAT("elf32-littleriscv", "elf32-littleriscv",
              "elf32-littleriscv")
OUTPUT_ARCH(riscv)
ENTRY(_start)

/* Define the memory layout */
MEMORY
{
    FLASH (rx)  : ORIGIN = 0x80000000, LENGTH = 512K    /* Flash memory */
    RAM   (rwx) : ORIGIN = 0x80080000, LENGTH = 512K    /* RAM for stack and data */
}

/* Sections definition */
SECTIONS
{
    /* Code section */
    .text :
    {
        /*_start = .;                    Define the program entry point */
        *(.text.init)
    } > FLASH

    .text :
    {
        *(.text)                       /* All .text sections (code) */
        *(.text.*)                     /* All other text sections */
    } > FLASH

    /* Read-only data section */
    .rodata :
    {
        *(.rodata)                     /* Read-only data section */
        *(.rodata.*)
    } > FLASH

    /* Initialized data section */
    .data : 
    {
        _data_start = .;               /* Start of data */
        *(.data)                       /* Initialized data */
        _data_end = .;                 /* End of data */
    } > RAM AT> FLASH

    /* Uninitialized data section (BSS) */
    .bss :
    {
        _bss_start = .;                /* Start of BSS */
        *(.bss)                        /* Uninitialized data */
        *(COMMON)
        _bss_end = .;                  /* End of BSS */
    } > RAM

    /* Stack section */
    .stack (NOLOAD) :
    {
        _stack_top = ORIGIN(RAM) + LENGTH(RAM);  /* Stack top address */
    } > RAM
}

/* Provide symbols to be used in the assembly code */
PROVIDE(_stack_top = ORIGIN(RAM) + LENGTH(RAM));  /* Stack pointer initialized to the top of RAM */
