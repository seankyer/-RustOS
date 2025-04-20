ENTRY(reset_handler)

MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 512K
    SRAM  (rw) : ORIGIN = 0x20000000, LENGTH = 128K
}

SECTIONS {
    /* Vector table goes at the start of FLASH (0x00000000) */
    .vector_table : {
        KEEP(*(.vector_table)) /* Keep so it's not stripped by linker GC */
    } > FLASH

    /* Code (text) */
    .text : {
        *(.text*)
        *(.rodata*)
    } > FLASH

    /* Initialized data: loaded from FLASH to SRAM */
    .data : {
        __data_start__ = .;
        *(.data*)
        __data_end__ = .;
    } > SRAM AT > FLASH
    __data_load__ = LOADADDR(.data);

    /* Zero-initialized data */
    .bss : {
        __bss_start__ = .;
        *(.bss*)
        *(COMMON)
        __bss_end__ = .;
    } > SRAM

    /* Heap */
    .heap : {
        *(.heap*)
    } > SRAM

    /* Stack */
    .stack (NOLOAD) : {
        . = ALIGN(8);
        KEEP(*(.stack*))
    } > SRAM
}
