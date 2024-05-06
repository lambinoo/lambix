use arch_amd64::get_cr2;
use arch_amd64::interrupts::StackFrame;

use super::idt::InterruptVector;

macro_rules! isr_entry_error_code {
    ($name:ident, $isr_no:literal) => {
        unsafe {
            #[naked]
            #[link_section = ".idt"]
            unsafe extern "C" fn $name() -> ! {
                core::arch::asm!(
                    "push rdi",
                    "mov rdi, {}",
                    "jmp {}",
                    const $isr_no,
                    sym $crate::handler_macros::interrupt_entrypoint,
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
            #[naked]
            #[link_section = ".idt"]
            unsafe extern "C" fn $name() -> ! {
                core::arch::asm!(
                    "push 0",
                    "push rdi",
                    "push r15",
                    "mov rdi, {}",
                    "mov r15, {}",
                    "jmp {}",
                    const $isr_no,
                    sym $crate::handler_macros::interrupt_handler,
                    sym $crate::handler_macros::interrupt_entrypoint,
                    options(noreturn)
                )
            }

            use arch_amd64::interrupts::InterruptHandler;
            InterruptHandler::new($name)
        }
    }
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
        "lea rdx, [rsp+32]",
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
        "pop r15",
        "pop rdi",
        "add rsp, 8",
        "iretq",
        sym crate::handler_macros::interrupt_handler,
        options(noreturn)
    )
}

pub unsafe extern "C" fn interrupt_handler(
    isr: InterruptVector,
    error_code: u64,
    stack_frame: *const StackFrame,
) {
    match isr {
        InterruptVector::PageFault => {
            let fault_address = get_cr2();
            panic!("page fault raised when accessing address {fault_address:?} ({error_code:x})");
        }

        _ => panic!("{isr:?} raised (error code: {error_code:x}), aborting: {stack_frame:#?}"),
    }
}
