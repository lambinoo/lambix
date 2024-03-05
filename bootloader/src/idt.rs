use crate::descriptors::InterruptDescriptor;

#[derive(Default, Debug)]
#[repr(transparent)]
pub struct ReservedInterrupt {
    inner: u64,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct InterruptWithErrorCode {
    inner: InterruptDescriptor,
}

impl InterruptWithErrorCode {
    pub fn new(handler: extern "x86-interrupt" fn(u32)) -> Self {
        let inner =
            InterruptDescriptor::new(InterruptDescriptor::INTERRUPT_GATE, handler as u32, 8);

        Self { inner }
    }
}

impl Default for InterruptWithErrorCode {
    fn default() -> Self {
        Self::new(default_error_code_handler)
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Interrupt {
    inner: InterruptDescriptor,
}

impl Interrupt {
    pub fn new(handler: extern "x86-interrupt" fn()) -> Self {
        let inner =
            InterruptDescriptor::new(InterruptDescriptor::INTERRUPT_GATE, handler as u32, 8);

        Self { inner }
    }
}

impl Default for Interrupt {
    fn default() -> Self {
        Self::new(default_handler)
    }
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct IDT {
    divide_by_zero: Interrupt,
    debug: Interrupt,
    non_maskable_interrupt: Interrupt,
    breakpoint: Interrupt,
    overflow: Interrupt,
    bound_range: Interrupt,
    invalid_opcode: Interrupt,
    device_not_available: Interrupt,
    double_fault: InterruptWithErrorCode,
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
    user_defined_255: Interrupt
}

impl IDT {
    pub fn load_idt(&self) {
        #[derive(Debug)]
        #[repr(C, packed)]
        struct Register(u16, u32);
        let register = Register(core::mem::size_of::<Self>() as u16, self as *const _ as u32);

        unsafe {
            core::arch::asm!("lidt [{}]", in(reg) &register);
        }
    }
}

extern "x86-interrupt" fn default_error_code_handler(error_code: u32) {
    loop {}
    handler_body(Some(error_code))
}

extern "x86-interrupt" fn default_handler() {
    loop {}
    handler_body(None);
}

fn handler_body(error_code: Option<u32>) {
    println!("{:?}", error_code);
}
