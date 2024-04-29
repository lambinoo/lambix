#![no_std]
#![no_main]
#![feature(format_args_nl)]
#![feature(naked_functions)]

#[macro_use]
extern crate arch_amd64;

use core::{arch::asm, panic::PanicInfo, ptr::NonNull};

use arch_amd64::{
    apic,
    idt::IDT,
};
use bootloader::{multiboot2::MemoryInfo, KernelInformation};

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("{:#?}", info);
    loop {
        unsafe { asm!("hlt") };
    }
}

#[no_mangle]
extern "C" fn _start(kernel_info_ptr: u32) -> ! {
    println!("done.\n");

    let kernel_info = unsafe {
        let info_ptr = NonNull::new(kernel_info_ptr as usize as *mut KernelInformation)
            .expect("Invalid kernel information passed");
        info_ptr.as_ref()
    };

    let idt = IDT::default();
    let loaded_idt = idt.load_idt();
    println!("IDT loaded: {:x?}", loaded_idt);

    let mut memory_info: [Option<MemoryInfo>; 20] = core::array::from_fn(|_| None);
    for (idx, mem) in kernel_info
        .boot_info()
        .memory_map()
        .unwrap()
        .iter()
        .enumerate()
    {
        memory_info[idx] = Some(mem.clone());
    }

    let local_apic = apic::LocalAPIC::get_local();
    println!(
        "Setting up APIC with id {:x} at address {:x?}",
        local_apic.apic_id(),
        local_apic
    );

    println!("Goodbye..");
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
