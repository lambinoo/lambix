#![no_std]

use arch_amd64::interrupts::Interrupt;
use arch_amd64::interrupts::InterruptHandler;
use arch_amd64::interrupts::InterruptWithErrorCode;
use arch_amd64::interrupts::InterruptWithErrorCodeHandler;
use arch_amd64::interrupts::ReservedInterrupt;
use lazy_static::lazy_static;

use crate::idt::IDT;

#[macro_use]
pub mod handler_macros;
pub mod idt;

lazy_static! {
    pub static ref DEFAULT_IDT: IDT = IDT {
        divide_by_zero: Interrupt::new(*DIVIDE_BY_ZERO),
        debug: Interrupt::new(*DEBUG),
        non_maskable_interrupt: Interrupt::new(*NON_MASKABLE_INTERRUPT),
        breakpoint: Interrupt::new(*BREAKPOINT),
        overflow: Interrupt::new(*OVERFLOW),
        bound_range: Interrupt::new(*BOUND_RANGE),
        invalid_opcode: Interrupt::new(*INVALID_OPCODE),
        device_not_available: Interrupt::new(*DEVICE_NOT_AVAILABLE),
        double_fault: Interrupt::new(*DOUBLE_FAULT),
        reserved_coprocessor_segment_overrun: ReservedInterrupt::default(),
        invalid_tss: InterruptWithErrorCode::new(*INVALID_TSS),
        segment_not_present: InterruptWithErrorCode::new(*SEGMENT_NOT_PRESENT),
        stack: InterruptWithErrorCode::new(*STACK),
        general_protection: InterruptWithErrorCode::new(*GENERAL_PROTECTION),
        page_fault: InterruptWithErrorCode::new(*PAGE_FAULT),
        reserved_15: ReservedInterrupt::default(),
        x86_floating_point_exception_pending: Interrupt::new(*X86_FLOATING_POINT_EXCEPTION_PENDING),
        alignmnent_check: InterruptWithErrorCode::new(*ALIGNMNENT_CHECK),
        machine_check: Interrupt::new(*MACHINE_CHECK),
        simd_floating_point: Interrupt::new(*SIMD_FLOATING_POINT),
        reserved_20_28: core::array::from_fn(|_| ReservedInterrupt::default()),
        vmm_communication_exception: InterruptWithErrorCode::new(*VMM_COMMUNICATION_EXCEPTION),
        security_exception: InterruptWithErrorCode::new(*SECURITY_EXCEPTION),
        reserved_31: ReservedInterrupt::default(),
        user_defined: core::array::from_fn(|_| Interrupt::new(*USER_DEFINED))
    };
}

lazy_static! {
    pub static ref DIVIDE_BY_ZERO: InterruptHandler = isr_entry!(divide_by_zero, 0);
    pub static ref DEBUG: InterruptHandler = isr_entry!(debug, 1);
    pub static ref NON_MASKABLE_INTERRUPT: InterruptHandler = isr_entry!(non_maskable_interrupt, 2);
    pub static ref BREAKPOINT: InterruptHandler = isr_entry!(breakpoint, 3);
    pub static ref OVERFLOW: InterruptHandler = isr_entry!(overflow, 4);
    pub static ref BOUND_RANGE: InterruptHandler = isr_entry!(bound_range, 5);
    pub static ref INVALID_OPCODE: InterruptHandler = isr_entry!(invalid_opcode, 6);
    pub static ref DEVICE_NOT_AVAILABLE: InterruptHandler = isr_entry!(device_not_available, 7);
    pub static ref DOUBLE_FAULT: InterruptHandler = isr_entry!(double_fault, 8);
    pub static ref INVALID_TSS: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(invalid_tss, 10);
    pub static ref SEGMENT_NOT_PRESENT: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(segment_not_present, 11);
    pub static ref STACK: InterruptWithErrorCodeHandler = isr_entry_error_code!(stack, 12);
    pub static ref GENERAL_PROTECTION: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(general_protection, 13);
    pub static ref PAGE_FAULT: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(page_fault, 14);
    pub static ref X86_FLOATING_POINT_EXCEPTION_PENDING: InterruptHandler =
        isr_entry!(x86_floating_point_exception_pending, 16);
    pub static ref ALIGNMNENT_CHECK: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(alignmnent_check, 17);
    pub static ref MACHINE_CHECK: InterruptHandler = isr_entry!(machine_check, 18);
    pub static ref SIMD_FLOATING_POINT: InterruptHandler = isr_entry!(simd_floating_point, 19);
    pub static ref VMM_COMMUNICATION_EXCEPTION: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(vmm_communication_exception, 29);
    pub static ref SECURITY_EXCEPTION: InterruptWithErrorCodeHandler =
        isr_entry_error_code!(security_exception, 30);
    pub static ref USER_DEFINED: InterruptHandler = isr_entry!(user_defined, 255);
}
