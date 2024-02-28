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

    . = ALIGN(4K);
    .kernel : {
        lambix_kernel_header = .;
        LONG(0) # Magic
        LONG(0) # Kernel size
        lambix_kernel_start = .;
        LONG(0)
    }
    kernel_end_addr = .;
}

