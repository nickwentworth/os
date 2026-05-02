use crate::mem::addr::KernelVirtAddr;

// TODO: maybe relocate these helper functions?
unsafe fn read_u32_be(ptr: *mut u32) -> u32 {
    u32::from_be(*ptr)
}
unsafe fn read_null_terminated_str<'a>(start: *mut u8) -> &'a str {
    let mut len = 0;

    while *start.add(len) != 0 {
        len += 1;
    }

    let slice = core::slice::from_raw_parts(start, len);
    core::str::from_utf8(slice).unwrap()
}
fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[derive(Debug)]
pub enum DtbParseErr {
    InvalidMagic(u32),
    MisalignedStructOffset(u32),
}

pub struct DeviceTree {
    base_addr: KernelVirtAddr,
}

impl DeviceTree {
    const EXPECTED_MAGIC: u32 = 0xD00DFEED;

    /// Define an implicit `DeviceTree` from a base address in memory.
    /// Applies some simple checks to make sure the tree looks initially good.
    ///
    /// ### Safety
    /// Following methods essentially require that the provided DTB address,
    /// and entire DTB in memory, is fully valid and formatted correctly.
    ///
    /// Besides this method checking the magic header value, currently there are
    /// no substantial checks later on to ensure the `DeviceTree` is valid.
    pub unsafe fn from_addr(addr: KernelVirtAddr) -> Result<Self, DtbParseErr> {
        let dtb = Self { base_addr: addr };
        let header = dtb.header();

        if header.magic != Self::EXPECTED_MAGIC {
            return Err(DtbParseErr::InvalidMagic(header.magic));
        } else if header.offset_dt_struct % 4 != 0 {
            return Err(DtbParseErr::MisalignedStructOffset(header.offset_dt_struct));
        }

        Ok(dtb)
    }

    pub fn header(&self) -> DeviceTreeHeader {
        unsafe {
            let ptr = self.base_addr.to_ptr().cast::<u32>();

            DeviceTreeHeader {
                magic: read_u32_be(ptr),
                total_size: read_u32_be(ptr.add(1)),
                offset_dt_struct: read_u32_be(ptr.add(2)),
                offset_dt_strings: read_u32_be(ptr.add(3)),
                offset_mem_rsvmap: read_u32_be(ptr.add(4)),
                version: read_u32_be(ptr.add(5)),
                last_compat_version: read_u32_be(ptr.add(6)),
                boot_cpu_id: read_u32_be(ptr.add(7)),
                size_dt_strings: read_u32_be(ptr.add(8)),
                size_dt_struct: read_u32_be(ptr.add(9)),
            }
        }
    }

    pub fn nodes<'a>(&'a self) -> DtbNodeIter<'a> {
        let header = self.header();

        let start = self
            .base_addr
            .to_ptr()
            .wrapping_byte_add(header.offset_dt_struct as usize)
            .cast();

        DtbNodeIter {
            dt: self,
            cursor: start,
        }
    }
}

#[derive(Debug)]
/// A mainly internal struct describing sizes/offsets of the device tree
pub struct DeviceTreeHeader {
    magic: u32,
    total_size: u32,
    offset_dt_struct: u32,
    offset_dt_strings: u32,
    offset_mem_rsvmap: u32,
    version: u32,
    last_compat_version: u32,
    boot_cpu_id: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

#[derive(Debug)]
enum DtbToken {
    BeginNode { name_start_ptr: *mut u8 },
    EndNode,
    Prop { value_len: u32, name_offset: u32 },
    Nop,
    End,
}

impl DtbToken {
    unsafe fn from_ptr(ptr: *mut u32) -> Result<Self, u32> {
        let token = read_u32_be(ptr);

        match token {
            0x00000001 => Ok(Self::BeginNode {
                name_start_ptr: ptr.add(1).cast::<u8>(),
            }),
            0x00000002 => Ok(Self::EndNode),
            0x00000003 => Ok(Self::Prop {
                value_len: read_u32_be(ptr.add(1)),
                name_offset: read_u32_be(ptr.add(2)),
            }),
            0x00000004 => Ok(Self::Nop),
            0x00000009 => Ok(Self::End),
            _ => Err(token),
        }
    }

    fn size(&self) -> usize {
        match self {
            Self::BeginNode { name_start_ptr } => {
                let name = unsafe { read_null_terminated_str(*name_start_ptr) };
                4 + align_up(name.len() + 1, 4)
            }
            Self::EndNode => 4,
            Self::Prop { value_len, .. } => 12 + align_up(*value_len as usize, 4),
            Self::Nop => 4,
            Self::End => 4,
        }
    }
}

/// Helper struct to iterate through the main nodes of the `DeviceTree`
pub struct DtbNodeIter<'dtb> {
    dt: &'dtb DeviceTree,
    cursor: *mut u32,
}

impl<'dtb> Iterator for DtbNodeIter<'dtb> {
    type Item = DtbNode<'dtb>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // println!("Reading token @ {:x}", self.cursor as usize);
            let token = unsafe { DtbToken::from_ptr(self.cursor).unwrap() };
            // println!("Found token: {:?}", token);

            match token {
                DtbToken::BeginNode { .. } => {
                    let node = DtbNode {
                        dt: self.dt,
                        base_addr: self.cursor.cast(),
                    };

                    self.cursor = self.cursor.wrapping_byte_add(token.size());

                    return Some(node);
                }
                DtbToken::End => return None,
                _ => self.cursor = self.cursor.wrapping_byte_add(token.size()),
            }
        }
    }
}

pub struct DtbNode<'dtb> {
    dt: &'dtb DeviceTree,
    base_addr: *mut u8,
}

impl<'dtb> DtbNode<'dtb> {
    pub fn name(&self) -> &str {
        let name_ptr = self.base_addr.wrapping_add(4);
        unsafe { read_null_terminated_str(name_ptr) }
    }
}
