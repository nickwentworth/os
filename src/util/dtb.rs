use core::ffi::CStr;

use crate::{mem::addr::KernelVirtAddr, println};

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
                magic: Self::read_u32(ptr),
                total_size: Self::read_u32(ptr.wrapping_add(1)),
                offset_dt_struct: Self::read_u32(ptr.wrapping_add(2)),
                offset_dt_strings: Self::read_u32(ptr.wrapping_add(3)),
                offset_mem_rsvmap: Self::read_u32(ptr.wrapping_add(4)),
                version: Self::read_u32(ptr.wrapping_add(5)),
                last_compat_version: Self::read_u32(ptr.wrapping_add(6)),
                boot_cpu_id: Self::read_u32(ptr.wrapping_add(7)),
                size_dt_strings: Self::read_u32(ptr.wrapping_add(8)),
                size_dt_struct: Self::read_u32(ptr.wrapping_add(9)),
            }
        }
    }

    pub fn nodes(&self) -> DeviceTreeNodeIter {
        let header = self.header();

        let start = self
            .base_addr
            .to_ptr()
            .wrapping_byte_add(header.offset_dt_struct as usize);

        DeviceTreeNodeIter::from_addr(start)
    }

    unsafe fn read_u32(ptr: *mut u32) -> u32 {
        u32::from_be(*ptr)
    }

    fn align_up(addr: *mut u8, align: usize) -> *mut u8 {
        let addr_u = addr as usize;
        ((addr_u + align - 1) & !(align - 1)) as *mut u8
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

/// Helper struct to iterate through the main nodes of the `DeviceTree`
pub struct DeviceTreeNodeIter {
    cursor: *mut u8,
}

impl DeviceTreeNodeIter {
    fn from_addr(start: *mut u8) -> Self {
        Self { cursor: start }
    }
}

impl Iterator for DeviceTreeNodeIter {
    type Item = DeviceTreeNode;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let curr_token = unsafe { DeviceTree::read_u32(self.cursor.cast()) };

            match curr_token {
                0x00000001 => {
                    let node = DeviceTreeNode::from_addr(self.cursor);

                    // We found the start of the node, but we still need to advance to the next token
                    let offset = 4 + node.name().len() + 1; // handle size of current token + name + null char
                    self.cursor = self.cursor.wrapping_add(offset);
                    self.cursor = DeviceTree::align_up(self.cursor, 4);

                    return Some(node);
                }
                0x00000002 => self.cursor = self.cursor.wrapping_add(4),
                0x00000003 => {
                    self.cursor = self.cursor.wrapping_add(4); // advance to property info struct

                    let prop_len = unsafe { DeviceTree::read_u32(self.cursor.cast()) };

                    let offset = 8 + prop_len as usize; // handle size of info struct + property value
                    self.cursor = self.cursor.wrapping_add(offset);
                    self.cursor = DeviceTree::align_up(self.cursor, 4);
                }
                0x00000004 => self.cursor = self.cursor.wrapping_add(4),
                0x00000009 => return None,
                _ => return None, // TODO: should this panic?
            }
        }
    }
}

pub struct DeviceTreeNode {
    base_addr: *mut u8,
}

impl DeviceTreeNode {
    fn from_addr(addr: *mut u8) -> Self {
        Self { base_addr: addr }
    }

    pub fn name(&self) -> &str {
        let name_ptr = self.base_addr.wrapping_add(4);
        let mut name_len = 0;

        unsafe {
            while *name_ptr.add(name_len) != 0 {
                name_len += 1;
            }

            let slice = core::slice::from_raw_parts(name_ptr, name_len);
            core::str::from_utf8(slice).unwrap()
        }
    }
}
