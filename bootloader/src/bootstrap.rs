core::arch::global_asm!(r"
    .global _lambix_early_stack
    .comm _lambix_early_stack, 65536, 16

    .text
    .global _start
    _start:
        cld
        cli
        xor ebp, ebp
        mov esp, _lambix_early_stack
        add esp, 4096

        push ebx
        push eax
        call boot_start
");
