use core::mem::size_of;

#[repr(C, align(8))]
struct TagHeader {
    typ: u16,
    flags: u16,
    size: u32,
}

#[repr(C)]
struct InformationRequest {
    header: TagHeader,
    requests: [u32; 1],
}

#[repr(C)]
struct EndTag(TagHeader);

/// Header for a multiboot v2.0 compliant binary
#[repr(C, align(8))]
struct MultibootHeader {
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: i32,
    info_req: InformationRequest,
    end_tag: EndTag,
}

impl MultibootHeader {
    const MAGIC: u32 = 0xE85250D6;
    const HEADER_SIZE: usize = core::mem::size_of::<Self>();

    const fn new() -> MultibootHeader {
        let requests = [6]; // Memory map

        let mut header = MultibootHeader {
            magic: Self::MAGIC,
            architecture: 0,
            header_length: Self::HEADER_SIZE as u32,
            checksum: 0,
            info_req: InformationRequest {
                header: TagHeader {
                    typ: 1,
                    flags: 0,
                    size: (size_of::<TagHeader>() + size_of::<u32>() * requests.len()) as u32,
                },
                requests: requests,
            },
            end_tag: EndTag(TagHeader {
                typ: 0,
                flags: 0,
                size: size_of::<TagHeader>() as u32,
            }),
        };

        header.checksum -= (header.magic + header.architecture + header.header_length) as i32;

        header
    }
}

#[no_mangle]
#[link_section = ".multiboot2"]
static HEADER: MultibootHeader = MultibootHeader::new();

core::arch::global_asm!(r"
    .global _lambix_early_stack
    .comm _lambix_early_stack, 16384, 16

    .text
    .global _start
    _start:
        cld
        cli
        xor ebp, ebp
        mov esp, _lambix_early_stack
        add esp, 16384

        push ebx
        push eax
        push ebp
        jmp {start}
    ",
    start = sym crate::boot_start,
);
