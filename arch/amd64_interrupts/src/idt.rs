use arch_amd64::interrupts::Interrupt;
use arch_amd64::interrupts::InterruptWithErrorCode;
use arch_amd64::interrupts::ReservedInterrupt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
#[repr(u8)]
#[allow(dead_code)]
pub enum InterruptVector {
    DivideByZero = 0,
    Debug,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRange,
    InvalidOpcode,
    DeviceNotAvailable,
    DoubleFault,
    ReservedCoprocessorSegmentOverrun,
    InvalidTss,
    SegmentNotPresent,
    Stack,
    GeneralProtection,
    PageFault,
    Reserved15,
    X86FloatingPointExceptionPending,
    AlignmnentCheck,
    MachineCheck,
    SimdFloatingPoint,
    Reserved20,
    Reserved21,
    Reserved22,
    Reserved23,
    Reserved24,
    Reserved25,
    Reserved26,
    Reserved27,
    Reserved28,
    VmmCommunicationException,
    SecurityException,
    Reserved31,
    UserDefined = 255,
}

#[derive(Debug)]
#[repr(C, align(8))]
pub struct IDT {
    pub divide_by_zero: Interrupt,
    pub debug: Interrupt,
    pub non_maskable_interrupt: Interrupt,
    pub breakpoint: Interrupt,
    pub overflow: Interrupt,
    pub bound_range: Interrupt,
    pub invalid_opcode: Interrupt,
    pub device_not_available: Interrupt,
    pub double_fault: Interrupt,
    pub reserved_coprocessor_segment_overrun: ReservedInterrupt,
    pub invalid_tss: InterruptWithErrorCode,
    pub segment_not_present: InterruptWithErrorCode,
    pub stack: InterruptWithErrorCode,
    pub general_protection: InterruptWithErrorCode,
    pub page_fault: InterruptWithErrorCode,
    pub reserved_15: ReservedInterrupt,
    pub x86_floating_point_exception_pending: Interrupt,
    pub alignmnent_check: InterruptWithErrorCode,
    pub machine_check: Interrupt,
    pub simd_floating_point: Interrupt,
    pub reserved_20_28: [ReservedInterrupt; 9],
    pub vmm_communication_exception: InterruptWithErrorCode,
    pub security_exception: InterruptWithErrorCode,
    pub reserved_31: ReservedInterrupt,
    pub user_defined: [Interrupt; 244],
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Register(u16, usize);

impl IDT {
    fn register(&self) -> Register {
        Register(
            (core::mem::size_of::<Self>() as u16) - 1,
            self as *const _ as usize,
        )
    }

    pub fn load_idt(&'static self) -> Register {
        let mut register = self.register();
        unsafe {
            core::arch::asm!(
                "lidt [{}]",
                "sidt [{}]",
                "sti",
                in(reg) &register,
                in(reg) &mut register
            );
        }

        register
    }
}
