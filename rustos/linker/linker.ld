/* Linker script for lm3s6965evb running an ARM Cortex-M3 */

ENTRY(reset_handler)

MEMORY
{
    /* Define the memory regions available on the board */
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 0x00040000   /* 256KB flash */
    SRAM  (rw) : ORIGIN = 0x20000000, LENGTH = 0x00010000   /* 64KB SRAM */
}

/* Define the stack size */
STACK_SIZE = 0x1000;  /* 4KB stack */

SECTIONS
{
    /* Vector table and interrupt handlers */
    .vector_table :
    {
        . = ALIGN(4);
        _vector_table_start = .;
        KEEP(*(.vector_table)) /* Keep the vector table, no garbage collection */
        _vector_table_end = .;
    } > FLASH

    /* Text section for code */
    .text :
    {
        . = ALIGN(4);
        _text_start = .;
        *(.text*) /* Collect all text/code sections */
        *(.rodata*) /* Collect all read-only data */
        . = ALIGN(4);
        _text_end = .;
    } > FLASH

    /* Read-Only Data */
    .data : AT (_text_end)
    {
        _data_start = .;
        *(.data*)
        . = ALIGN(4);
        _data_end = .;
    } > SRAM

    /* Uninitialized data (.bss) */
    .bss (NOLOAD) :
    {
        _bss_start = .;
        *(.bss*)
        . = ALIGN(4);
        _bss_end = .;
    } > SRAM

    /* Stack */
    .stack (NOLOAD) :
    {
        . = ALIGN(8);
        _stack_start = .;
        . += STACK_SIZE;
        _stack_end = .;
    } > SRAM

    .heap (NOLOAD) :
    {
        _heap_start = .;
        . += 0x1000; /* 4KB heap */
        _heap_end = .;
    } > SRAM

    /* Special sections for the linker */
    .ARM.exidx : { *(.ARM.exidx*) } > SRAM
}
