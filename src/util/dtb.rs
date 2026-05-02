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

        let magic = dtb.magic();
        if magic != Self::EXPECTED_MAGIC {
            return Err(DtbParseErr::InvalidMagic(magic));
        }

        Ok(dtb)
    }

    // -------------------- Useful External Methods -------------------- //

    pub fn nodes<'a>(&'a self) -> DtbNodeIter<'a> {
        DtbNodeIter {
            dt: self,
            cursor: self.struct_section_ptr().cast(),
        }
    }

    // -------------------- Header-Related Getters -------------------- //

    fn magic(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>()) }
    }

    fn size(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().add(1)) }
    }

    fn struct_section_ptr(&self) -> *mut u8 {
        let ptr = self.base_addr.to_ptr().cast::<u32>().wrapping_add(2);
        let offset = unsafe { read_u32_be(ptr) };
        self.base_addr.to_ptr().wrapping_byte_add(offset as usize)
    }

    fn strings_section_ptr(&self) -> *mut u8 {
        let ptr = self.base_addr.to_ptr().cast::<u32>().wrapping_add(3);
        let offset = unsafe { read_u32_be(ptr) };
        self.base_addr.to_ptr().wrapping_byte_add(offset as usize)
    }

    fn mem_rsvmap_section_ptr(&self) -> *mut u8 {
        let ptr = self.base_addr.to_ptr().cast::<u32>().wrapping_add(4);
        let offset = unsafe { read_u32_be(ptr) };
        self.base_addr.to_ptr().wrapping_byte_add(offset as usize)
    }

    fn version(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().wrapping_add(5)) }
    }

    fn last_compat_version(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().wrapping_add(6)) }
    }

    fn boot_cpu_id(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().wrapping_add(7)) }
    }

    fn strings_section_size(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().wrapping_add(8)) }
    }

    fn struct_section_size(&self) -> u32 {
        unsafe { read_u32_be(self.base_addr.to_ptr().cast::<u32>().wrapping_add(9)) }
    }
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

    /// Calculates the size of the current token, including any included strings/values paired
    /// with this token.
    ///
    /// In essence, this returns the number of bytes to the start of the next token.
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

    pub fn props(&self) -> DtbPropIter<'dtb> {
        // Calculate offset to first token after this node's name
        let offset = 4 + align_up(self.name().len() + 1, 4);

        DtbPropIter {
            dt: self.dt,
            cursor: self.base_addr.wrapping_add(offset),
        }
    }
}

pub struct DtbPropIter<'dtb> {
    dt: &'dtb DeviceTree,
    cursor: *mut u8,
}

impl<'dtb> Iterator for DtbPropIter<'dtb> {
    type Item = DtbProp<'dtb>;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // println!("Reading token @ {:x}", self.cursor as usize);
            let token = unsafe { DtbToken::from_ptr(self.cursor.cast()).unwrap() };
            // println!("Found token: {:?}", token);

            match token {
                DtbToken::Prop { .. } => {
                    let prop = DtbProp {
                        dt: self.dt,
                        base_addr: self.cursor,
                    };

                    self.cursor = self.cursor.wrapping_byte_add(token.size());

                    return Some(prop);
                }
                DtbToken::Nop => self.cursor = self.cursor.wrapping_byte_add(token.size()),
                DtbToken::BeginNode { .. } => return None,
                DtbToken::EndNode => return None,
                DtbToken::End => panic!("Unexpected end node!"),
            }
        }
    }
}

pub struct DtbProp<'dtb> {
    dt: &'dtb DeviceTree,
    base_addr: *mut u8,
}

impl<'dtb> DtbProp<'dtb> {
    pub fn value_raw(&self) -> &[u8] {
        unsafe {
            let len = read_u32_be(self.base_addr.add(4).cast());
            core::slice::from_raw_parts(self.base_addr.add(12), len as usize)
        }
    }

    pub fn name(&self) -> &str {
        unsafe {
            let offset = read_u32_be(self.base_addr.add(8).cast());
            let name_ptr = self.dt.strings_section_ptr().byte_add(offset as usize);
            read_null_terminated_str(name_ptr)
        }
    }
}
