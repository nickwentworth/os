#[derive(Clone, Copy)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }
}

#[derive(Clone, Copy)]
pub struct KernelVirtAddr(usize);

impl KernelVirtAddr {
    pub fn to_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }
}

impl From<PhysAddr> for KernelVirtAddr {
    fn from(value: PhysAddr) -> Self {
        Self(value.0 | 0xFFFF_0000_0000_0000)
    }
}
