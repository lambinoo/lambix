use core::ffi::CStr;
use core::fmt::Debug;
use core::mem::{align_of, size_of};
use core::ops::{Deref, Range};
use core::ptr::NonNull;
use core::slice::Chunks;

/// Boot infromation passed by the bootloader, as part of the multiboot v2.0 spec
#[repr(C, align(8))]
pub struct BootInformation {
    total_size: u32,
    _reserved: u32,
}

impl BootInformation {
    const BOOT_MAGIC: u32 = 0x36d76289;

    /// Verifies the magic against the expected value, and returns boot information after
    /// some basic checks around the pointer's validity. This includes checking for alignment
    pub unsafe fn from_ptr(
        ptr: *mut BootInformation,
        magic: u32,
    ) -> Option<&'static BootInformation> {
        if magic == Self::BOOT_MAGIC {
            NonNull::new(ptr)
                .filter(|ptr| is_aligned::<BootInformation>(ptr.as_ptr() as _))
                .map(|ptr| ptr.as_ref())
                .filter(|info| info.size() >= size_of::<Self>())
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        self.total_size as usize
    }

    /// Return boot information as a buffer
    pub fn as_bytes(&self) -> &[u8] {
        let ptr = NonNull::from(self).cast::<u8>().as_ptr();
        unsafe { core::slice::from_raw_parts(ptr as *const u8, self.total_size as usize) }
    }

    /// Returns an iterator over the different information tags
    pub fn iter(&self) -> TagIter {
        TagIter {
            buffer: self.as_bytes(),
            cursor: core::mem::size_of::<Self>(),
        }
    }
}

#[derive(Debug)]
#[repr(C, align(8))]
struct TagHeader {
    typ: u32,
    size: u32,
}

impl TagHeader {
    const MEMORY_INFORMATION: u32 = 4;
    const BIOS_BOOT_DEVICE: u32 = 5;
    const COMMAND_LINE: u32 = 1;
    const MODULES: u32 = 3;
    const ELF_SYMBOLS: u32 = 9;
    const MEMORY_MAP: u32 = 6;
    const BOOTLOADER_NAME: u32 = 2;
    const FRAMEBUFFER_INFO: u32 = 8;
    const END_TAG: u32 = 0;
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct MemoryMap<'a> {
    entry_size: usize,
    entry_version: u32,
    data: &'a [u8],
}

impl MemoryMap<'_> {
    fn from_slice(buffer: &[u8]) -> Option<MemoryMap<'_>> {
        let entry_size = buffer.get(0..size_of::<u32>())?;
        let entry_version = buffer.get(size_of::<u32>()..size_of::<u32>() * 2)?;

        let entry_size: [u8; size_of::<u32>()] = entry_size.try_into().ok()?;
        let entry_version: [u8; size_of::<u32>()] = entry_version.try_into().ok()?;

        let map = MemoryMap {
            entry_size: u32::from_ne_bytes(entry_size) as usize,
            entry_version: u32::from_ne_bytes(entry_version),
            data: buffer.get(size_of::<u32>() * 2..)?,
        };

        Some(map)
    }

    pub fn iter(&self) -> MemoryInfoIter<'_> {
        MemoryInfoIter {
            chunks: self.data.chunks(self.entry_size as usize),
        }
    }
}

#[derive(Clone, Copy)]
pub struct MemoryRange {
    base: u64,
    size: u64,
}

impl MemoryRange {
    pub fn base_as_ptr(&self) -> Option<*const ()> {
        usize::try_from(self.base).ok().map(|ptr| ptr as *const ())
    }

    pub fn end(&self) -> Option<u64> {
        self.base.checked_add(self.size)
    }
}

impl Debug for MemoryRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MemoryRange")
            .field("base", &format_args!("0x{:016x}", self.base))
            .field("length", &self.size)
            .finish()
    }
}

#[derive(Debug)]
pub enum MemoryInfo {
    Available(MemoryRange),
    Reserved(MemoryRange),
    ACPIReclaimable(MemoryRange),
    NVS(MemoryRange),
    BadRam(MemoryRange),
    Unknown(MemoryRange, u32),
}

impl Deref for MemoryInfo {
    type Target = MemoryRange;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Available(e)
            | Self::Reserved(e)
            | Self::ACPIReclaimable(e)
            | Self::NVS(e)
            | Self::BadRam(e)
            | Self::Unknown(e, _) => e,
        }
    }
}

impl MemoryInfo {
    const AVAILABLE: u32 = 1;
    const RESERVED: u32 = 2;
    const ACPI_RECLAIMABLE: u32 = 3;
    const MEMORY_NVS: u32 = 4;
    const BAD_RAM: u32 = 5;
}

pub struct MemoryInfoIter<'a> {
    chunks: Chunks<'a, u8>,
}

impl Iterator for MemoryInfoIter<'_> {
    type Item = MemoryInfo;

    fn next(&mut self) -> Option<Self::Item> {
        let parse_buffer = |buffer: &[u8]| {
            let size = size_of::<u64>();

            let base_addr = buffer.get(0..size)?.try_into().ok()?;
            let length = buffer.get(size..size * 2)?.try_into().ok()?;
            let typ = buffer
                .get(size * 2..size * 2 + size_of::<u32>())?
                .try_into()
                .ok()?;

            let base = u64::from_ne_bytes(base_addr);
            let size = u64::from_ne_bytes(length);

            let data = MemoryRange { base, size };

            let memory_info = match u32::from_ne_bytes(typ) {
                MemoryInfo::AVAILABLE => MemoryInfo::Available(data),
                MemoryInfo::RESERVED => MemoryInfo::Reserved(data),
                MemoryInfo::ACPI_RECLAIMABLE => MemoryInfo::ACPIReclaimable(data),
                MemoryInfo::MEMORY_NVS => MemoryInfo::NVS(data),
                MemoryInfo::BAD_RAM => MemoryInfo::BadRam(data),
                v @ _ => MemoryInfo::Unknown(data, v),
            };

            Some(memory_info)
        };

        self.chunks.next().map(parse_buffer).flatten()
    }
}

/// A specific bootparse_buffer]
#[derive(Debug)]
pub enum Tag<'a> {
    CommandLine(&'a CStr),
    MemoryMap(MemoryMap<'a>),
    Unknown(u32),
}

/// Iterator over the boot tags from a boot information structure
#[derive(Debug)]
pub struct TagIter<'a> {
    cursor: usize,
    buffer: &'a [u8],
}

impl<'a> TagIter<'a> {
    fn current_tag(&self, header: &'a TagHeader) -> Tag<'a> {
        let tag_start = self.cursor + size_of::<TagHeader>();
        let tag_end = tag_start + header.size as usize;

        self.buffer
            .get(tag_start..tag_end)
            .map(|tag_data| match header.typ {
                TagHeader::COMMAND_LINE => {
                    let cmd_line = CStr::from_bytes_until_nul(tag_data).unwrap();
                    Some(Tag::CommandLine(cmd_line))
                }
                TagHeader::MEMORY_MAP => MemoryMap::from_slice(tag_data).map(|m| Tag::MemoryMap(m)),
                _ => None,
            })
            .flatten()
            .unwrap_or(Tag::Unknown(header.typ))
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Tag<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let header = self
            .buffer
            .get(self.cursor..self.cursor + size_of::<TagHeader>())
            .map(|raw_header| NonNull::from(&raw_header[0]))
            .map(|header_ptr| unsafe { header_ptr.cast::<TagHeader>().as_ref() })
            .filter(|header| header.typ != TagHeader::END_TAG);

        if let Some(header) = header {
            let tag = self.current_tag(header);

            self.cursor += header.size as usize;
            if !is_aligned::<TagHeader>(self.cursor) {
                self.cursor &= !(align_of::<TagHeader>() - 1);
                self.cursor += align_of::<TagHeader>();
            }

            Some(tag)
        } else {
            None
        }
    }
}

fn is_aligned<T>(ptr: usize) -> bool {
    let align = align_of::<T>() - 1;
    ptr & align == 0
}
