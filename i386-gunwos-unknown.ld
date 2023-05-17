ENTRY(_start)

MEMORY {
    RAM : ORIGIN = 0x00300000, LENGTH = 1M
}

SECTIONS {
    .text : {
        *(.text)
    } > RAM

    .data : {
        *(.data)
    } > RAM

    .bss : {
        *(.bss)
    } > RAM

    .got.plt : {
        *(.got.plt)
    } > RAM
}
