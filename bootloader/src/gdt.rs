#[derive(Clone, Copy)]
#[repr(C)]
pub struct Descriptor(u64);

impl core::fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut f = f.debug_list();
        for v in u64::to_be_bytes(self.0) {
            f.entry(&format_args!("{:08b}", v));
        }
        f.finish()
    }
}


impl Descriptor {
    const Present: u32 = 1 << 15;
    const Available: u32 = 1 << 20;
    const DefaultOperandSize: u32 = 1 << 22;
    const Granulariry: u32 =  1 << 23;

    const fn new(base_value: u64, base_address: u32, segment_limit: u32) -> Self {
        let base_address = base_address as u64;
        let segment_limit = segment_limit as u64;

        let mut descriptor: u64 = base_value;
        descriptor |= segment_limit & 0xffff;
        descriptor |= (segment_limit & 0xf0000) << 48;

        descriptor |= (base_address & 0x00ffffff) << 8;
        descriptor |= (base_address & 0xff000000) << 56;

        Self(descriptor)
    }

    const fn with_flag(self, flag: u32) -> Self {
        Self(self.0 | (flag as u64) << 32)
    }
}


#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct CodeDescriptor(Descriptor);

impl CodeDescriptor {
    const CODE_TYPE: u64 = 0b11 << 33;

    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(Descriptor::new(Self::CODE_TYPE, base_address, segment_limit)
            .with_flag(Descriptor::Present)
            .with_flag(Descriptor::DefaultOperandSize)
            .with_flag(Descriptor::Granulariry))
    }
    
    pub const fn readable(self) -> Self {
        Self(self.0.with_flag(1 << 9))
    }

    pub const fn conforming(self) -> Self {
        Self(self.0.with_flag(1 << 10))
    }

    pub const fn descriptor(self) -> Descriptor {
        self.0
    }
}


#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DataDescriptor(Descriptor);

impl DataDescriptor {
    const DATA_TYPE: u64 = 0b10 << 33;

    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(Descriptor::new(Self::DATA_TYPE, base_address, segment_limit)
            .with_flag(Descriptor::Present)
            .with_flag(Descriptor::DefaultOperandSize)
            .with_flag(Descriptor::Granulariry))
    }
    
    pub const fn writable(self) -> Self {
        Self(self.0.with_flag(1 << 9))
    }

    pub const fn expand_down(self) -> Self {
        Self(self.0.with_flag(1 << 10))
    }

    pub const fn descriptor(self) -> Descriptor {
        self.0
    }
}

#[repr(C, packed)]
struct GlobalDescriptorTableRegister(u32, u16);

#[derive(Debug)]
#[repr(C, packed)]
pub struct GlobalDescriptorTable<const N: usize>([Descriptor; N]);

impl<const N: usize> GlobalDescriptorTable<N> {
    pub fn apply(&'static self) {
        let register = GlobalDescriptorTableRegister(
            self as *const _ as usize as _,
            core::mem::size_of::<Self>() as u16
        );

        unsafe {
            core::arch::asm!("lgdt [{}]", in(reg) &register);
        }
    }

    pub const fn selector_for(&self, idx: usize) -> u32 {
        (idx * 8) as u32
    }
}

#[no_mangle]
pub static EARLY_GDT: GlobalDescriptorTable<3> = GlobalDescriptorTable([
    Descriptor::new(0, 0, 0),
    CodeDescriptor::new(0, 0xfffff).readable().descriptor(),
    DataDescriptor::new(0, 0xfffff).writable().descriptor()
]);

const EARLY_GDT_CODE: u32 = 0x08;
const EARLY_GDT_DATA: u32 = 0x10;