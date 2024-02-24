/// Header for a multiboot v2.0 compliant binary
#[repr(align(8))]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: i32,
}

impl MultibootHeader {
    const MAGIC: u32 = 0xE85250D6;
    const HEADER_SIZE: usize = core::mem::size_of::<Self>();

    const fn new() -> MultibootHeader {
        let mut header = MultibootHeader {
            magic: Self::MAGIC,
            architecture: 0,
            header_length: Self::HEADER_SIZE as u32,
            checksum: 0,
        };

        header.checksum -= (header.magic + header.architecture + header.header_length) as i32;

        header
    }
}

#[no_mangle]
#[link_section = ".multiboot2"]
static HEADER: MultibootHeader = MultibootHeader::new();

core::arch::global_asm!(
    r"
    .global _lambix_early_stack
    .comm _lambix_early_stack, 4096, 16

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
        push ebp
        jmp boot_start
"
);
