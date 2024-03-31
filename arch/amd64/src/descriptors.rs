#[derive(Clone, Copy)]
#[repr(C)]
pub struct Descriptor(u64);

impl core::fmt::Debug for Descriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut metadata = f.debug_struct("Descriptor");
        metadata.field(
            "base_address",
            &format_args!("0x{:08x}", self.base_address()),
        );
        metadata.field("segment", &format_args!("0x{:08x}", self.segment()));

        let flags = (self.0 >> 32) as u32;
        metadata.field("flags", &format_args!("{:032b}", flags));
        metadata.field("flags_hex", &format_args!("{:08x}", flags));
        metadata.field("raw", &format_args!("0x{:016x}", self.0));
        metadata.finish()
    }
}

impl Descriptor {
    const PRESENT: u32 = 1 << 15;
    // const AVAILABLE: u32 = 1 << 20;
    const LONG_MODE: u32 = 1 << 21;
    const DEFAULT_OPERAND_SIZE: u32 = 1 << 22;
    const GRANULARITY: u32 = 1 << 23;

    pub const fn new(type_value: u64, base_address: u32, segment_limit: u32) -> Self {
        let base_address = base_address as u64;
        let segment_limit = segment_limit as u64;

        let mut descriptor: u64 = type_value << 40;
        descriptor |= segment_limit & 0xffff;
        descriptor |= (segment_limit & 0xf0000) << 32;

        descriptor |= (base_address & 0x00ffffff) << 16;
        descriptor |= (base_address & 0xff000000) << 56;

        Self(descriptor)
    }

    pub const fn with_flag(self, flag: u32) -> Self {
        Self(self.0 | (flag as u64) << 32)
    }

    pub const fn base_address(&self) -> u64 {
        ((self.0 >> 16) & 0xffffff) | ((self.0 >> 32) & 0xff000000)
    }

    pub const fn segment(&self) -> u64 {
        (self.0 & 0xffff) | ((self.0 >> 32) & 0xf0000)
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct CodeDescriptor(Descriptor);

impl CodeDescriptor {
    const CODE_TYPE: u64 = 0b11000;

    const CONFORMING: u32 = 1 << 10;
    const READABLE: u32 = 1 << 9;

    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(
            Descriptor::new(Self::CODE_TYPE, base_address, segment_limit)
                .with_flag(Descriptor::PRESENT)
                .with_flag(Descriptor::DEFAULT_OPERAND_SIZE)
                .with_flag(Descriptor::GRANULARITY),
        )
    }

    pub const fn readable(self) -> Self {
        Self(self.0.with_flag(Self::READABLE))
    }

    pub const fn conforming(self) -> Self {
        Self(self.0.with_flag(Self::CONFORMING))
    }

    pub const fn descriptor(self) -> Descriptor {
        self.0
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct DataDescriptor(Descriptor);

impl DataDescriptor {
    const DATA_TYPE: u64 = 0b10000;

    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(
            Descriptor::new(Self::DATA_TYPE, base_address, segment_limit)
                .with_flag(Descriptor::PRESENT)
                .with_flag(Descriptor::DEFAULT_OPERAND_SIZE)
                .with_flag(Descriptor::GRANULARITY),
        )
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

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Code64Descriptor(Descriptor);

impl Code64Descriptor {
    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(
            Descriptor::new(CodeDescriptor::CODE_TYPE, base_address, segment_limit)
                .with_flag(Descriptor::PRESENT)
                .with_flag(Descriptor::LONG_MODE)
                .with_flag(CodeDescriptor::CONFORMING),
        )
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Data64Descriptor(Descriptor);

impl Data64Descriptor {
    pub const fn new(base_address: u32, segment_limit: u32) -> Self {
        Self(
            Descriptor::new(DataDescriptor::DATA_TYPE, base_address, segment_limit)
                .with_flag(Descriptor::PRESENT),
        )
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InterruptDescriptor {
    inner: Descriptor,
}

impl InterruptDescriptor {
    pub const INTERRUPT_GATE: u64 = 0xe;
    pub const TRAP_GATE: u64 = 0xf;

    pub const fn new(selector_type: u64, code_segment_offset: u32, selector: u16) -> Self {
        let inner = Descriptor::new(selector_type, code_segment_offset, selector as u32)
            .with_flag(Descriptor::PRESENT);

        Self { inner }
    }
}
