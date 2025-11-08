use crate::{allocator::LinkedListAllocator, kernel::cpu::Cpu, mutex::Mutex, println};
use core::{alloc::GlobalAlloc, cell::OnceCell};

pub struct Kernel {
    cpus: [Cpu; 4],
    allocator: Mutex<OnceCell<LinkedListAllocator>>,
}

impl Kernel {
    pub const fn uninitialized() -> Self {
        Self {
            cpus: [Cpu::new(0), Cpu::new(1), Cpu::new(2), Cpu::new(3)],
            allocator: Mutex::new(OnceCell::new()),
        }
    }

    const HEAP_START: *mut u8 = 0xFFFF_0000_4000_0000 as *mut u8;
    const HEAP_SIZE: usize = 1024 * 1024;

    pub fn init(&self) {
        self.allocator
            .lock()
            .set(unsafe { LinkedListAllocator::new(Self::HEAP_START, Self::HEAP_SIZE) })
            .unwrap_or_else(|_| println!("Allocator was already initialized"));
    }

    pub fn cpu_me(&self) -> &Cpu {
        let cpu_idx = 0; // TODO: fetch from system register
        &self.cpus[cpu_idx]
    }
}

unsafe impl GlobalAlloc for Kernel {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        println!("Allocating {} bytes", layout.size());

        self.allocator
            .lock()
            .get_mut()
            .expect("Allocator not yet initialized")
            .alloc(layout)
            .expect("Unable to allocate")
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        println!("Deallocating {} bytes", layout.size());

        self.allocator
            .lock()
            .get_mut()
            .expect("Allocator not yet initialized")
            .dealloc(ptr, layout);
    }
}
