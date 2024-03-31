
use crate::descriptors::{Code64Descriptor, CodeDescriptor, Data64Descriptor, DataDescriptor};

#[macro_export]
macro_rules! io_write_port {
    (u8,  $port:expr, $value:expr) => { core::arch::asm!("out dx, al", in("dx") $port, in("al") $value as u8) };
    (u16, $port:expr, $value:expr) => { core::arch::asm!("out dx, ax", in("dx") $port, in("ax") $value as u16) };
    (u32, $port:expr, $value:expr) => { core::arch::asm!("out dx, eax", in("dx") $port, in("eax") $value as u32) };
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    null: u64,
    code: CodeDescriptor,
    data: DataDescriptor,
    code64: Code64Descriptor,
    data64: Data64Descriptor,
}

#[derive(Debug)]
#[repr(C, packed)]
struct Register(u16, u32);

impl GlobalDescriptorTable {
    pub const NULL: u32 = 0;

    pub const GDT_CODE: u32 = 0x08;
    pub const GDT_DATA: u32 = 0x10;

    pub const GDT_CODE64: u32 = 0x18;
    pub const GDT_DATA64: u32 = 0x20;

    pub const fn new(code: CodeDescriptor, data: DataDescriptor) -> Self {
        Self {
            null: 0,
            code: code,
            data: data,
            code64: Code64Descriptor::new(0, 0),
            data64: Data64Descriptor::new(0, 0),
        }
    }

    pub fn load_gdt(&'static self) {
        let register = Register(
            core::mem::size_of::<Self>() as u16,
            self as *const _ as usize as _,
        );

        unsafe {
            core::arch::asm!(
                "lgdt [{}]",
                in(reg) &register
            );
        };
    }

    pub fn set_protected_mode(&'static self) {
        unsafe {
            core::arch::asm!(
                "mov ds, eax",
                "mov es, eax",
                "mov gs, eax",
                "mov fs, eax",
                "mov ss, eax",
                in("eax") Self::GDT_DATA
            );

            core::arch::asm!("ljmp ${},$3f", "3:", const Self::GDT_CODE, options(att_syntax));
        }
    }

    pub fn long_mode_segment(&self) -> u32 {
        Self::GDT_CODE64
    }
}
