#[derive(Debug, Default)]
pub struct CPUID {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl CPUID {
    pub fn get_raw(id: u32) -> Self {
        let mut cpuid = Self::default();
        unsafe {
            core::arch::asm!(
                "push rbx",
                "cpuid",
                "mov rdi, rbx",
                "pop rbx",
                in("eax") id,
                lateout("eax") cpuid.eax,
                out("edi") cpuid.ebx,
                out("ecx") cpuid.ecx,
                out("edx") cpuid.edx
            )
        }

        cpuid
    }
}
