ENTRY(_start)

SECTIONS {
    . = 2M;
    .multiboot2 : {
        KEEP(*(.multiboot2))
    }

    .bss : ALIGN(8) {
        *(COMMON)
        *(.bss, .bss.*)
    }

    .data : ALIGN(4K) {
        *(.data, .data.*)
        *(.data.rel.ro .data.rel.ro.*)
    }

    .rodata : {
        *(.rodata, .rodata.*)
    }

    .text BLOCK(4K) : ALIGN(4K) {
        *(.text .text.*)
    }

    . = ALIGN(16);
    kernel_start_addr = .;
    .kernel : {
        BYTE(1)
    }
    kernel_end_addr = .;
}

