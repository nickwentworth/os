use crate::{
    allocator::LinkedListAllocator,
    devices::uart::{UartPl011, UartPl011Device},
    kernel::cpu::Cpu,
    mem::addr::PhysAddr,
    mutex::Mutex,
    println,
    util::dtb::DeviceTree,
};
use core::{alloc::GlobalAlloc, cell::OnceCell, fmt::Write};

pub struct Kernel {
    cpus: [Cpu; 4],
    allocator: Mutex<OnceCell<LinkedListAllocator>>,
    serial: Mutex<OnceCell<UartPl011>>,
}

impl Kernel {
    pub const fn uninitialized() -> Self {
        Self {
            cpus: [Cpu::new(0), Cpu::new(1), Cpu::new(2), Cpu::new(3)],
            allocator: Mutex::new(OnceCell::new()),
            serial: Mutex::new(OnceCell::new()),
        }
    }

    const HEAP_START: PhysAddr = PhysAddr::new(0x5000_0000);
    const HEAP_SIZE: usize = 1024 * 1024;

    // const UART0_BASE: PhysAddr = PhysAddr::new(0x0900_0000);

    pub fn init(&self, dt: DeviceTree) {
        for node in dt.nodes() {
            if let Some(uart_device) = UartPl011Device::try_from_node(&node) {
                let uart = UartPl011::bind(uart_device);
                self.serial.lock().set(uart);
                println!("Uart initialized!");
            }
        }

        let allocator =
            unsafe { LinkedListAllocator::new(Self::HEAP_START.into(), Self::HEAP_SIZE) };
        self.allocator
            .lock()
            .set(allocator)
            .unwrap_or_else(|_| println!("Allocator was already initialized"));
    }

    pub fn cpu_me(&self) -> &Cpu {
        let cpu_idx = 0; // TODO: fetch from system register
        &self.cpus[cpu_idx]
    }

    pub fn serial_write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        self.serial
            .lock()
            .get_mut()
            .ok_or(core::fmt::Error)?
            .write_fmt(args)
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
