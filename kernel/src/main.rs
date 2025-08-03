#![no_std]
#![no_main]

mod paging;

#[macro_use]
extern crate arch_amd64;

use core::panic::PanicInfo;
use core::ptr::NonNull;

use amd64_interrupts::DEFAULT_IDT;
use arch_amd64::apic;
use bootloader::KernelInformation;

use crate::paging::initialize_early_kernel_memory;

#[panic_handler]
fn panic_handler(panic_info: &PanicInfo) -> ! {
    let (filename, lineno) = panic_info
        .location()
        .map(|loc| (loc.file(), loc.line()))
        .unwrap_or(("<?>", 0));

    println!("\n============================================");
    println!("Lambix panicked at {filename}:{lineno}");
    println!("{}", panic_info.message());
    println!("============================================");

    loop {
        unsafe { core::arch::asm!("cld", "cli", "hlt") };
    }
}

#[unsafe(no_mangle)]
extern "C" fn _start(kernel_info_ptr: u32) -> ! {
    println!("done.\n");

    let kernel_info = unsafe {
        let info_ptr = NonNull::new(kernel_info_ptr as usize as *mut KernelInformation)
            .expect("Invalid kernel information passed");
        info_ptr.as_ref()
    };

    println!("{kernel_info:#?}");

    apic::disable_legacy_8259_pic();
    let local_apic = apic::LocalAPIC::get_local();
    println!(
        "Setting up APIC with id {:x} / {:x} at address {:x?}",
        local_apic.apic_id(),
        local_apic.apic_version(),
        local_apic
    );

    DEFAULT_IDT.load_idt();

    println!("IDT has been applied");

    unsafe {
        let kernel_info_ptr = usize::try_from(kernel_info_ptr).unwrap() as *mut KernelInformation;
        initialize_early_kernel_memory(kernel_info_ptr);

        loop {
            core::arch::asm!("hlt");
        }
    }
}
