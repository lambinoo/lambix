
use crate::read_msr;

#[derive(Debug)]
#[repr(C)]
pub struct LocalAPIC(*const u32);

impl LocalAPIC {
    const MSR_REGISTER: u32 = 0x1B;

    pub fn get_local() -> LocalAPIC {
        let value = read_msr(Self::MSR_REGISTER);

        let mut address: usize = (value.low & 0xffffff00) as usize;
        address |= (value.high & 0xffffff) as usize;

        LocalAPIC(address as *const _)
    }

    pub fn apic_id(&self) -> u32 {
        unsafe { self.0.byte_offset(0x20).read() }
    }
}
