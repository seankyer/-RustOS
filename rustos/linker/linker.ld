ENTRY(_start)

MEMORY {
    FLASH (rx) : ORIGIN = 0x00000000, LENGTH = 512K    /* Adjust size as needed */
    SRAM  (rw) : ORIGIN = 0x20000000, LENGTH = 128K    /* Adjust size as needed */
}

SECTIONS {
    .text : {
        *(.text)
    } > FLASH

    .data : {
        *(.data)
    } > SRAM

    .bss : {
        *(.bss)
    } > SRAM

    .heap : {
        *(.heap)
    } > SRAM

    .stack : {
        *(.stack)
    } > SRAM
}
