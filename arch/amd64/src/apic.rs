use crate::io_write_port;
use crate::println;

use crate::MSR;

pub fn disable_legacy_pic() {
    unsafe {
        io_write_port!(u8, 0x21, 0xff);
        io_write_port!(u8, 0xA1, 0xff);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct LocalAPIC(*const u32);

impl LocalAPIC {
    const MSR_REGISTER: u32 = 0x1B;

    pub fn get_local() -> LocalAPIC {
        let value = MSR::read(Self::MSR_REGISTER);
        println!("{value:x?}");

        let mut address: usize = (value.low & 0xfffff000) as usize;
        address |= (value.high & 0xfffff) as usize;

        LocalAPIC(address as *const _)
    }

    pub fn apic_id(&self) -> u32 {
        unsafe { self.0.offset(8).read_volatile() }
    }

    pub fn apic_version(&self) -> u32 {
        unsafe { self.0.offset(12).read_volatile() }
    }
}
