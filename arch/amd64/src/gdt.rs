use crate::descriptors::{CodeDescriptor, DataDescriptor, Descriptor};

#[derive(Debug)]
#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null: u64,
    code: Descriptor,
    data: Descriptor,
}

impl GlobalDescriptorTable {
    pub const EARLY_GDT_CODE: u32 = 0x08;
    pub const EARLY_GDT_DATA: u32 = 0x10;

    pub const fn new(code: CodeDescriptor, data: DataDescriptor) -> Self {
        Self {
            null: 0,
            code: code.descriptor(),
            data: data.descriptor(),
        }
    }

    pub fn load_gdt(&'static self) {
        #[derive(Debug)]
        #[repr(C, packed)]
        struct Register(u16, u32);

        let register = Register(
            core::mem::size_of::<Self>() as u16,
            self as *const _ as usize as _,
        );

        unsafe {
            core::arch::asm!(
                "lgdt [{}]",
                "mov ds, eax",
                "mov es, eax",
                "mov gs, eax",
                "mov fs, eax",
                "mov ss, eax",
                in(reg) &register,
                in("eax") Self::EARLY_GDT_DATA
            );

            core::arch::asm!("ljmp $0x08,$3f", "3:", options(att_syntax));
        };
    }
}
