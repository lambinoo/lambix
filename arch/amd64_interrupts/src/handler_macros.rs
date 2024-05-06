use arch_amd64::get_cr2;
use arch_amd64::interrupts::StackFrame;
use arch_amd64::println;

use super::idt::InterruptVector;

macro_rules! isr_entry_error_code {
    ($name:ident, $isr_no:literal) => {
        unsafe {
            #[link_section = ".idt"]
            #[naked]
            unsafe extern "C" fn $name() -> ! {
                core::arch::asm!(
                    "push rdi",
                    "mov rdi, {isr}",
                    "jmp {entry}",
                    isr = const $isr_no,
                    entry = sym $crate::handler_macros::interrupt_entrypoint,
                    options(noreturn)
                )
            }

            use arch_amd64::interrupts::InterruptWithErrorCodeHandler;
            InterruptWithErrorCodeHandler::new($name)
        }
    };
}

macro_rules! isr_entry {
    ($name:ident, $isr_no:literal) => {
        unsafe {
            #[link_section = ".idt"]
            #[naked]
            unsafe extern "C" fn $name() -> ! {
                core::arch::asm!(
                    "push 0",
                    "push rdi",
                    "mov rdi, {isr}",
                    "jmp {entry}",
                    isr = const $isr_no,
                    entry = sym $crate::handler_macros::interrupt_entrypoint,
                    options(noreturn)
                )
            }

            use arch_amd64::interrupts::InterruptHandler;
            InterruptHandler::new($name)
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct SavedRegisters {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    error_code: u64,
    interrupt_stack_frame: StackFrame,
}

/// Entrypoint for interrupts
/// # Safety
/// Do not call this function directly, it's part of the interrupt handling mechanism
#[link_section = ".idt"]
#[naked]
pub unsafe extern "C" fn interrupt_entrypoint() {
    core::arch::asm!(
        "push rsi",
        "push rdx",
        "mov rsi, [rsp+24]",
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
        "mov rdx, rsp",
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
        sym crate::handler_macros::interrupt_handler,
        options(noreturn)
    )
}

/// Shared handler for interrupts
/// # Safety
/// Do not call this function directly, it's part of the interrupt handling mechanism
#[link_section = ".idt"]
pub unsafe extern "C" fn interrupt_handler(
    isr: InterruptVector,
    error_code: u64,
    registers: &SavedRegisters,
) {
    match isr {
        InterruptVector::PageFault => {
            let fault_address = get_cr2();
            panic!("page fault raised when accessing address {fault_address:?} ({error_code:x})");
        }

        InterruptVector::Breakpoint => {
            println!("Breakpoint hit, dumping registers: {:#x?}", registers);
        }

        _ => panic!("{isr:?} raised (error code: {error_code:x}), aborting: {registers:#?}"),
    }
}
