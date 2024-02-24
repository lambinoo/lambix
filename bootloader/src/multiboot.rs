use core::ffi::CStr;
use core::mem::{align_of, size_of};
use core::ptr::NonNull;

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

/// A specific boot information
#[derive(Debug)]
pub enum Tag<'a> {
    CommandLine(&'a CStr),
}

/// Iterator over the boot tags from a boot information structure
pub struct TagIter<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> TagIter<'a> {
    fn current_tag(&self, header: &'a TagHeader) -> Option<Tag<'a>> {
        let tag_start = self.cursor + size_of::<TagHeader>();
        let tag_end = tag_start + header.size as usize;
        let tag_data = self.buffer.get(tag_start..tag_end)?;

        match header.typ {
            TagHeader::COMMAND_LINE => {
                let cmd_line = CStr::from_bytes_until_nul(tag_data).unwrap();
                Some(Tag::CommandLine(cmd_line))
            }

            _ => None,
        }
    }
}

impl<'a> Iterator for TagIter<'a> {
    type Item = Option<Tag<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        let header = self
            .buffer
            .get(self.cursor..self.cursor + size_of::<TagHeader>())
            .map(|raw_header| NonNull::from(&raw_header[0]))
            .map(|header_ptr| unsafe { header_ptr.cast::<TagHeader>().as_ref() })
            .filter(|header| header.typ != TagHeader::END_TAG);

        if let Some(header) = header {
            let tag = self.current_tag(header);

            self.cursor += (header.size as usize);
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
