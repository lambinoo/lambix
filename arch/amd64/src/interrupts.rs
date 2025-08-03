use core::ops::Deref;

use crate::println;

#[derive(Debug)]
#[repr(C)]
pub struct StackFrame {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
}

pub type HandlerType = unsafe extern "C" fn() -> !;
pub type HandlerWithCodeType = unsafe extern "C" fn() -> !;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptHandler {
    interrupt: HandlerType,
}

impl Deref for InterruptHandler {
    type Target = HandlerType;

    fn deref(&self) -> &Self::Target {
        &self.interrupt
    }
}

impl InterruptHandler {
    /// Marks a given function as a valid new interrupt handler. This should not be called manually,
    /// but only be created through the interrupt_handler!() macro
    ///
    /// ```rust,ignore
    /// interrupt_handler! my_handler() {
    ///     println!("hello there");
    /// };
    /// ```
    ///
    /// # Safety
    /// The function pointer passed to this function needs to be a valid interrupt handler
    /// for the amd64 architecture.
    ///
    pub unsafe fn new(interrupt: HandlerType) -> Self {
        Self { interrupt }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct InterruptWithErrorCodeHandler {
    interrupt: HandlerWithCodeType,
}

impl Deref for InterruptWithErrorCodeHandler {
    type Target = HandlerWithCodeType;

    fn deref(&self) -> &Self::Target {
        &self.interrupt
    }
}

impl InterruptWithErrorCodeHandler {
    /// Marks a given function as a valid new interrupt handler. This should not be called manually,
    /// but only be created through the interrupt_handler!() macro
    ///
    /// ```rust,ignore
    /// interrupt_handler! my_handler() {
    ///     println!("hello there");
    /// };
    /// ```
    ///
    /// # Safety
    /// The function pointer passed to this function needs to be a valid interrupt handler
    /// for the amd64 architecture.
    ///
    pub unsafe fn new(interrupt: HandlerWithCodeType) -> Self {
        Self { interrupt }
    }
}

#[macro_export]
macro_rules! interrupt_handler {
    (fn $name:ident($stack:ident: $stacktype:ty) $impl:block) => {
        unsafe {
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name() -> ! {
                extern "C" fn __impl($stack: $stacktype) {
                    $impl
                }

                core::arch::naked_asm!(
                    "push rdi",
                    "push rsi",
                    "lea rdi, [rsp+16]",
                    "push rdx",
                    "push rcx",
                    "push rbx",
                    "push rax",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",
                    "push r13",
                    "push r14",
                    "push r15",
                    "call {}",
                    "pop r15",
                    "pop r14",
                    "pop r13",
                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rax",
                    "pop rbx",
                    "pop rcx",
                    "pop rdx",
                    "pop rsi",
                    "pop rdi",
                    "iretq",
                    sym __impl,
                )
            }

            $crate::interrupts::InterruptHandler::new($name)
        }
    };

    (fn $name:ident($stack:ident: $stacktype:ty, $varname:ident: u64) $impl:block) => {
        unsafe {
            #[unsafe(naked)]
            pub unsafe extern "C" fn $name() -> ! {
                #[inline(always)]
                extern "C" fn __impl($stack: $stacktype, $varname: u64) {
                    $impl
                }

                core::arch::naked_asm!(
                    "push rdi",
                    "push rsi",
                    "mov rsi, [rsp+16]",
                    "lea rdi, [rsp+24]",
                    "push rdx",
                    "push rcx",
                    "push rbx",
                    "push rax",
                    "push r8",
                    "push r9",
                    "push r10",
                    "push r11",
                    "push r12",
                    "push r13",
                    "push r14",
                    "push r15",
                    "call {}",
                    "pop r15",
                    "pop r14",
                    "pop r13",
                    "pop r12",
                    "pop r11",
                    "pop r10",
                    "pop r9",
                    "pop r8",
                    "pop rax",
                    "pop rbx",
                    "pop rcx",
                    "pop rdx",
                    "pop rsi",
                    "pop rdi",
                    "add rsp, 8",
                    "iretq",
                    sym __impl,
                )
            }
            // (rsp)
            //   v
            // [rsi][rdi][err][rip][cs][rflags][rsp][ss]
            //             v
            //                  v

            $crate::interrupts::InterruptWithErrorCodeHandler::new($name)
        }
    };
}

macro_rules! default_handler {
    ($name:ident) => {
        Interrupt::new(interrupt_handler!(
            fn $name(stack_frame: &StackFrame) {
                let name = stringify!($name);

                let mut cr2: usize;
                unsafe { core::arch::asm!("mov {}, cr2", out(reg) cr2) };
                println!("cr2 value: {:?}", cr2 as *const ());

                panic!(
                    "Got {}, handling with default handler {:x?}",
                    name, stack_frame
                );
            }
        ))
    };

    ($name:ident, with_error_code) => {
        InterruptWithErrorCode::new(interrupt_handler!(
            fn $name(stack_frame: &StackFrame, error_code: u64) {
                let name = stringify!($name);


                let mut cr2: usize;
                unsafe { core::arch::asm!("mov {}, cr2", out(reg) cr2) };
                println!("cr2 value: {:?}", cr2 as *const ());

                panic!(
                    "Got {}, handling with default handler ({:x}) {:x?}",
                    name, error_code, stack_frame
                );
            }
        ))
    };
}

#[repr(C)]
struct InterruptDescriptor {
    low: u64,
    high: u64,
}

impl core::fmt::Debug for InterruptDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value_high = u64::to_be_bytes(self.high);
        let value_low = u64::to_be_bytes(self.low);
        f.write_fmt(format_args!("{:x?} {:x?}", &value_high, &value_low))
    }
}

#[allow(unused)]
impl InterruptDescriptor {
    pub const GDT_CODE64: u64 = 0x18 << 16;
    pub const PRESENT: u64 = 1 << 47;
    pub const INTERRUPT_GATE: u64 = 0xe << 40;
    pub const TRAP_GATE: u64 = 0xf << 40;

    const fn disabled() -> Self {
        InterruptDescriptor { low: 0, high: 0 }
    }

    fn from_address(address: u64) -> Self {
        let high = address >> 32;
        let low = (address & 0xffff)
            | ((address & 0xffff0000) << 32)
            | Self::GDT_CODE64
            | Self::PRESENT
            | Self::TRAP_GATE;

        Self { high, low }
    }

    fn from_handler(handler: InterruptHandler) -> Self {
        let address = u64::try_from(*handler.deref() as usize).unwrap();
        Self::from_address(address)
    }

    fn from_handler_with_error(handler: InterruptWithErrorCodeHandler) -> Self {
        let address = u64::try_from(*handler.deref() as usize).unwrap();
        Self::from_address(address)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct ReservedInterrupt {
    inner: u128,
}

impl ReservedInterrupt {
    const fn new() -> Self {
        Self { inner: 0 }
    }
}

impl Default for ReservedInterrupt {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InterruptWithErrorCode {
    inner: InterruptDescriptor,
}

impl InterruptWithErrorCode {
    pub const fn disabled() -> Self {
        Self {
            inner: InterruptDescriptor::disabled(),
        }
    }

    pub fn new(handler: InterruptWithErrorCodeHandler) -> Self {
        Self {
            inner: InterruptDescriptor::from_handler_with_error(handler),
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct Interrupt {
    inner: InterruptDescriptor,
}

impl Interrupt {
    pub const fn disabled() -> Self {
        Self {
            inner: InterruptDescriptor::disabled(),
        }
    }

    pub fn new(handler: InterruptHandler) -> Self {
        Self {
            inner: InterruptDescriptor::from_handler(handler),
        }
    }
}

impl Default for Interrupt {
    fn default() -> Self {
        let handler = interrupt_handler!(
            fn default_handler(_stack_frame: &StackFrame) {
                println!("Interrupt without error code");
            }
        );

        Self::new(handler)
    }
}

#[derive(Debug)]
#[repr(C, align(8))]
pub struct IDT {
    divide_by_zero: Interrupt,
    debug: Interrupt,
    non_maskable_interrupt: Interrupt,
    breakpoint: Interrupt,
    overflow: Interrupt,
    bound_range: Interrupt,
    invalid_opcode: Interrupt,
    device_not_available: Interrupt,
    double_fault: Interrupt,
    reserved_coprocessor_segment_overrun: ReservedInterrupt,
    invalid_tss: InterruptWithErrorCode,
    segment_not_present: InterruptWithErrorCode,
    stack: InterruptWithErrorCode,
    general_protection: InterruptWithErrorCode,
    page_fault: InterruptWithErrorCode,
    reserved_15: ReservedInterrupt,
    x86_floating_point_exception_pending: Interrupt,
    alignmnent_check: InterruptWithErrorCode,
    machine_check: Interrupt,
    simd_floating_point: Interrupt,
    reserved_20_28: [ReservedInterrupt; 9],
    vmm_communication_exception: InterruptWithErrorCode,
    security_exception: InterruptWithErrorCode,
    reserved_31: ReservedInterrupt,
    user_defined_32: Interrupt,
    user_defined_33: Interrupt,
    user_defined_34: Interrupt,
    user_defined_35: Interrupt,
    user_defined_36: Interrupt,
    user_defined_37: Interrupt,
    user_defined_38: Interrupt,
    user_defined_39: Interrupt,
    user_defined_40: Interrupt,
    user_defined_41: Interrupt,
    user_defined_42: Interrupt,
    user_defined_43: Interrupt,
    user_defined_44: Interrupt,
    user_defined_45: Interrupt,
    user_defined_46: Interrupt,
    user_defined_47: Interrupt,
    user_defined_48: Interrupt,
    user_defined_49: Interrupt,
    user_defined_50: Interrupt,
    user_defined_51: Interrupt,
    user_defined_52: Interrupt,
    user_defined_53: Interrupt,
    user_defined_54: Interrupt,
    user_defined_55: Interrupt,
    user_defined_56: Interrupt,
    user_defined_57: Interrupt,
    user_defined_58: Interrupt,
    user_defined_59: Interrupt,
    user_defined_60: Interrupt,
    user_defined_61: Interrupt,
    user_defined_62: Interrupt,
    user_defined_63: Interrupt,
    user_defined_64: Interrupt,
    user_defined_65: Interrupt,
    user_defined_66: Interrupt,
    user_defined_67: Interrupt,
    user_defined_68: Interrupt,
    user_defined_69: Interrupt,
    user_defined_70: Interrupt,
    user_defined_71: Interrupt,
    user_defined_72: Interrupt,
    user_defined_73: Interrupt,
    user_defined_74: Interrupt,
    user_defined_75: Interrupt,
    user_defined_76: Interrupt,
    user_defined_77: Interrupt,
    user_defined_78: Interrupt,
    user_defined_79: Interrupt,
    user_defined_80: Interrupt,
    user_defined_81: Interrupt,
    user_defined_82: Interrupt,
    user_defined_83: Interrupt,
    user_defined_84: Interrupt,
    user_defined_85: Interrupt,
    user_defined_86: Interrupt,
    user_defined_87: Interrupt,
    user_defined_88: Interrupt,
    user_defined_89: Interrupt,
    user_defined_90: Interrupt,
    user_defined_91: Interrupt,
    user_defined_92: Interrupt,
    user_defined_93: Interrupt,
    user_defined_94: Interrupt,
    user_defined_95: Interrupt,
    user_defined_96: Interrupt,
    user_defined_97: Interrupt,
    user_defined_98: Interrupt,
    user_defined_99: Interrupt,
    user_defined_100: Interrupt,
    user_defined_101: Interrupt,
    user_defined_102: Interrupt,
    user_defined_103: Interrupt,
    user_defined_104: Interrupt,
    user_defined_105: Interrupt,
    user_defined_106: Interrupt,
    user_defined_107: Interrupt,
    user_defined_108: Interrupt,
    user_defined_109: Interrupt,
    user_defined_110: Interrupt,
    user_defined_111: Interrupt,
    user_defined_112: Interrupt,
    user_defined_113: Interrupt,
    user_defined_114: Interrupt,
    user_defined_115: Interrupt,
    user_defined_116: Interrupt,
    user_defined_117: Interrupt,
    user_defined_118: Interrupt,
    user_defined_119: Interrupt,
    user_defined_120: Interrupt,
    user_defined_121: Interrupt,
    user_defined_122: Interrupt,
    user_defined_123: Interrupt,
    user_defined_124: Interrupt,
    user_defined_125: Interrupt,
    user_defined_126: Interrupt,
    user_defined_127: Interrupt,
    user_defined_128: Interrupt,
    user_defined_129: Interrupt,
    user_defined_130: Interrupt,
    user_defined_131: Interrupt,
    user_defined_132: Interrupt,
    user_defined_133: Interrupt,
    user_defined_134: Interrupt,
    user_defined_135: Interrupt,
    user_defined_136: Interrupt,
    user_defined_137: Interrupt,
    user_defined_138: Interrupt,
    user_defined_139: Interrupt,
    user_defined_140: Interrupt,
    user_defined_141: Interrupt,
    user_defined_142: Interrupt,
    user_defined_143: Interrupt,
    user_defined_144: Interrupt,
    user_defined_145: Interrupt,
    user_defined_146: Interrupt,
    user_defined_147: Interrupt,
    user_defined_148: Interrupt,
    user_defined_149: Interrupt,
    user_defined_150: Interrupt,
    user_defined_151: Interrupt,
    user_defined_152: Interrupt,
    user_defined_153: Interrupt,
    user_defined_154: Interrupt,
    user_defined_155: Interrupt,
    user_defined_156: Interrupt,
    user_defined_157: Interrupt,
    user_defined_158: Interrupt,
    user_defined_159: Interrupt,
    user_defined_160: Interrupt,
    user_defined_161: Interrupt,
    user_defined_162: Interrupt,
    user_defined_163: Interrupt,
    user_defined_164: Interrupt,
    user_defined_165: Interrupt,
    user_defined_166: Interrupt,
    user_defined_167: Interrupt,
    user_defined_168: Interrupt,
    user_defined_169: Interrupt,
    user_defined_170: Interrupt,
    user_defined_171: Interrupt,
    user_defined_172: Interrupt,
    user_defined_173: Interrupt,
    user_defined_174: Interrupt,
    user_defined_175: Interrupt,
    user_defined_176: Interrupt,
    user_defined_177: Interrupt,
    user_defined_178: Interrupt,
    user_defined_179: Interrupt,
    user_defined_180: Interrupt,
    user_defined_181: Interrupt,
    user_defined_182: Interrupt,
    user_defined_183: Interrupt,
    user_defined_184: Interrupt,
    user_defined_185: Interrupt,
    user_defined_186: Interrupt,
    user_defined_187: Interrupt,
    user_defined_188: Interrupt,
    user_defined_189: Interrupt,
    user_defined_190: Interrupt,
    user_defined_191: Interrupt,
    user_defined_192: Interrupt,
    user_defined_193: Interrupt,
    user_defined_194: Interrupt,
    user_defined_195: Interrupt,
    user_defined_196: Interrupt,
    user_defined_197: Interrupt,
    user_defined_198: Interrupt,
    user_defined_199: Interrupt,
    user_defined_200: Interrupt,
    user_defined_201: Interrupt,
    user_defined_202: Interrupt,
    user_defined_203: Interrupt,
    user_defined_204: Interrupt,
    user_defined_205: Interrupt,
    user_defined_206: Interrupt,
    user_defined_207: Interrupt,
    user_defined_208: Interrupt,
    user_defined_209: Interrupt,
    user_defined_210: Interrupt,
    user_defined_211: Interrupt,
    user_defined_212: Interrupt,
    user_defined_213: Interrupt,
    user_defined_214: Interrupt,
    user_defined_215: Interrupt,
    user_defined_216: Interrupt,
    user_defined_217: Interrupt,
    user_defined_218: Interrupt,
    user_defined_219: Interrupt,
    user_defined_220: Interrupt,
    user_defined_221: Interrupt,
    user_defined_222: Interrupt,
    user_defined_223: Interrupt,
    user_defined_224: Interrupt,
    user_defined_225: Interrupt,
    user_defined_226: Interrupt,
    user_defined_227: Interrupt,
    user_defined_228: Interrupt,
    user_defined_229: Interrupt,
    user_defined_230: Interrupt,
    user_defined_231: Interrupt,
    user_defined_232: Interrupt,
    user_defined_233: Interrupt,
    user_defined_234: Interrupt,
    user_defined_235: Interrupt,
    user_defined_236: Interrupt,
    user_defined_237: Interrupt,
    user_defined_238: Interrupt,
    user_defined_239: Interrupt,
    user_defined_240: Interrupt,
    user_defined_241: Interrupt,
    user_defined_242: Interrupt,
    user_defined_243: Interrupt,
    user_defined_244: Interrupt,
    user_defined_245: Interrupt,
    user_defined_246: Interrupt,
    user_defined_247: Interrupt,
    user_defined_248: Interrupt,
    user_defined_249: Interrupt,
    user_defined_250: Interrupt,
    user_defined_251: Interrupt,
    user_defined_252: Interrupt,
    user_defined_253: Interrupt,
    user_defined_254: Interrupt,
    user_defined_255: Interrupt,
}

impl Default for IDT {
    fn default() -> Self {
        IDT {
            divide_by_zero: default_handler!(divide_by_zero),
            alignmnent_check: default_handler!(alignmnent_check, with_error_code),
            bound_range: default_handler!(bound_range),
            breakpoint: default_handler!(breakpoint),
            debug: default_handler!(debug),
            device_not_available: default_handler!(device_not_available),
            double_fault: default_handler!(double_fault),
            general_protection: default_handler!(general_protection, with_error_code),
            non_maskable_interrupt: default_handler!(non_maskable_interrupt),
            invalid_opcode: default_handler!(invalid_opcode),
            overflow: default_handler!(overflow),
            invalid_tss: default_handler!(invalid_tss, with_error_code),
            reserved_coprocessor_segment_overrun: Default::default(),
            segment_not_present: default_handler!(segment_not_present, with_error_code),
            stack: default_handler!(stack, with_error_code),
            page_fault: default_handler!(page_fault, with_error_code),
            reserved_15: Default::default(),
            x86_floating_point_exception_pending: default_handler!(
                x86_floating_point_exception_pending
            ),
            machine_check: default_handler!(machine_check),
            reserved_20_28: Default::default(),
            security_exception: default_handler!(security_exception, with_error_code),
            simd_floating_point: default_handler!(simd_floating_point),
            vmm_communication_exception: default_handler!(
                vmm_communication_exception,
                with_error_code
            ),
            reserved_31: Default::default(),
            user_defined_32: Default::default(),
            user_defined_33: Default::default(),
            user_defined_34: Default::default(),
            user_defined_35: Default::default(),
            user_defined_36: Default::default(),
            user_defined_37: Default::default(),
            user_defined_38: Default::default(),
            user_defined_39: Default::default(),
            user_defined_40: Default::default(),
            user_defined_41: Default::default(),
            user_defined_42: Default::default(),
            user_defined_43: Default::default(),
            user_defined_44: Default::default(),
            user_defined_45: Default::default(),
            user_defined_46: Default::default(),
            user_defined_47: Default::default(),
            user_defined_48: Default::default(),
            user_defined_49: Default::default(),
            user_defined_50: Default::default(),
            user_defined_51: Default::default(),
            user_defined_52: Default::default(),
            user_defined_53: Default::default(),
            user_defined_54: Default::default(),
            user_defined_55: Default::default(),
            user_defined_56: Default::default(),
            user_defined_57: Default::default(),
            user_defined_58: Default::default(),
            user_defined_59: Default::default(),
            user_defined_60: Default::default(),
            user_defined_61: Default::default(),
            user_defined_62: Default::default(),
            user_defined_63: Default::default(),
            user_defined_64: Default::default(),
            user_defined_65: Default::default(),
            user_defined_66: Default::default(),
            user_defined_67: Default::default(),
            user_defined_68: Default::default(),
            user_defined_69: Default::default(),
            user_defined_70: Default::default(),
            user_defined_71: Default::default(),
            user_defined_72: Default::default(),
            user_defined_73: Default::default(),
            user_defined_74: Default::default(),
            user_defined_75: Default::default(),
            user_defined_76: Default::default(),
            user_defined_77: Default::default(),
            user_defined_78: Default::default(),
            user_defined_79: Default::default(),
            user_defined_80: Default::default(),
            user_defined_81: Default::default(),
            user_defined_82: Default::default(),
            user_defined_83: Default::default(),
            user_defined_84: Default::default(),
            user_defined_85: Default::default(),
            user_defined_86: Default::default(),
            user_defined_87: Default::default(),
            user_defined_88: Default::default(),
            user_defined_89: Default::default(),
            user_defined_90: Default::default(),
            user_defined_91: Default::default(),
            user_defined_92: Default::default(),
            user_defined_93: Default::default(),
            user_defined_94: Default::default(),
            user_defined_95: Default::default(),
            user_defined_96: Default::default(),
            user_defined_97: Default::default(),
            user_defined_98: Default::default(),
            user_defined_99: Default::default(),
            user_defined_100: Default::default(),
            user_defined_101: Default::default(),
            user_defined_102: Default::default(),
            user_defined_103: Default::default(),
            user_defined_104: Default::default(),
            user_defined_105: Default::default(),
            user_defined_106: Default::default(),
            user_defined_107: Default::default(),
            user_defined_108: Default::default(),
            user_defined_109: Default::default(),
            user_defined_110: Default::default(),
            user_defined_111: Default::default(),
            user_defined_112: Default::default(),
            user_defined_113: Default::default(),
            user_defined_114: Default::default(),
            user_defined_115: Default::default(),
            user_defined_116: Default::default(),
            user_defined_117: Default::default(),
            user_defined_118: Default::default(),
            user_defined_119: Default::default(),
            user_defined_120: Default::default(),
            user_defined_121: Default::default(),
            user_defined_122: Default::default(),
            user_defined_123: Default::default(),
            user_defined_124: Default::default(),
            user_defined_125: Default::default(),
            user_defined_126: Default::default(),
            user_defined_127: Default::default(),
            user_defined_128: Default::default(),
            user_defined_129: Default::default(),
            user_defined_130: Default::default(),
            user_defined_131: Default::default(),
            user_defined_132: Default::default(),
            user_defined_133: Default::default(),
            user_defined_134: Default::default(),
            user_defined_135: Default::default(),
            user_defined_136: Default::default(),
            user_defined_137: Default::default(),
            user_defined_138: Default::default(),
            user_defined_139: Default::default(),
            user_defined_140: Default::default(),
            user_defined_141: Default::default(),
            user_defined_142: Default::default(),
            user_defined_143: Default::default(),
            user_defined_144: Default::default(),
            user_defined_145: Default::default(),
            user_defined_146: Default::default(),
            user_defined_147: Default::default(),
            user_defined_148: Default::default(),
            user_defined_149: Default::default(),
            user_defined_150: Default::default(),
            user_defined_151: Default::default(),
            user_defined_152: Default::default(),
            user_defined_153: Default::default(),
            user_defined_154: Default::default(),
            user_defined_155: Default::default(),
            user_defined_156: Default::default(),
            user_defined_157: Default::default(),
            user_defined_158: Default::default(),
            user_defined_159: Default::default(),
            user_defined_160: Default::default(),
            user_defined_161: Default::default(),
            user_defined_162: Default::default(),
            user_defined_163: Default::default(),
            user_defined_164: Default::default(),
            user_defined_165: Default::default(),
            user_defined_166: Default::default(),
            user_defined_167: Default::default(),
            user_defined_168: Default::default(),
            user_defined_169: Default::default(),
            user_defined_170: Default::default(),
            user_defined_171: Default::default(),
            user_defined_172: Default::default(),
            user_defined_173: Default::default(),
            user_defined_174: Default::default(),
            user_defined_175: Default::default(),
            user_defined_176: Default::default(),
            user_defined_177: Default::default(),
            user_defined_178: Default::default(),
            user_defined_179: Default::default(),
            user_defined_180: Default::default(),
            user_defined_181: Default::default(),
            user_defined_182: Default::default(),
            user_defined_183: Default::default(),
            user_defined_184: Default::default(),
            user_defined_185: Default::default(),
            user_defined_186: Default::default(),
            user_defined_187: Default::default(),
            user_defined_188: Default::default(),
            user_defined_189: Default::default(),
            user_defined_190: Default::default(),
            user_defined_191: Default::default(),
            user_defined_192: Default::default(),
            user_defined_193: Default::default(),
            user_defined_194: Default::default(),
            user_defined_195: Default::default(),
            user_defined_196: Default::default(),
            user_defined_197: Default::default(),
            user_defined_198: Default::default(),
            user_defined_199: Default::default(),
            user_defined_200: Default::default(),
            user_defined_201: Default::default(),
            user_defined_202: Default::default(),
            user_defined_203: Default::default(),
            user_defined_204: Default::default(),
            user_defined_205: Default::default(),
            user_defined_206: Default::default(),
            user_defined_207: Default::default(),
            user_defined_208: Default::default(),
            user_defined_209: Default::default(),
            user_defined_210: Default::default(),
            user_defined_211: Default::default(),
            user_defined_212: Default::default(),
            user_defined_213: Default::default(),
            user_defined_214: Default::default(),
            user_defined_215: Default::default(),
            user_defined_216: Default::default(),
            user_defined_217: Default::default(),
            user_defined_218: Default::default(),
            user_defined_219: Default::default(),
            user_defined_220: Default::default(),
            user_defined_221: Default::default(),
            user_defined_222: Default::default(),
            user_defined_223: Default::default(),
            user_defined_224: Default::default(),
            user_defined_225: Default::default(),
            user_defined_226: Default::default(),
            user_defined_227: Default::default(),
            user_defined_228: Default::default(),
            user_defined_229: Default::default(),
            user_defined_230: Default::default(),
            user_defined_231: Default::default(),
            user_defined_232: Default::default(),
            user_defined_233: Default::default(),
            user_defined_234: Default::default(),
            user_defined_235: Default::default(),
            user_defined_236: Default::default(),
            user_defined_237: Default::default(),
            user_defined_238: Default::default(),
            user_defined_239: Default::default(),
            user_defined_240: Default::default(),
            user_defined_241: Default::default(),
            user_defined_242: Default::default(),
            user_defined_243: Default::default(),
            user_defined_244: Default::default(),
            user_defined_245: Default::default(),
            user_defined_246: Default::default(),
            user_defined_247: Default::default(),
            user_defined_248: Default::default(),
            user_defined_249: Default::default(),
            user_defined_250: Default::default(),
            user_defined_251: Default::default(),
            user_defined_252: Default::default(),
            user_defined_253: Default::default(),
            user_defined_254: Default::default(),
            user_defined_255: Default::default(),
        }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Register(u16, usize);

impl IDT {
    pub fn register(&self) -> Register {
        Register(
            (core::mem::size_of::<Self>() as u16) - 1,
            self as *const _ as usize,
        )
    }

    pub fn load_idt(&self) -> Register {
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
