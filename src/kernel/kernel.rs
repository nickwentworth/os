use crate::{
    allocator::LinkedListAllocator,
    devices::generic::uart::{UartController, UartPl011},
    kernel::cpu::Cpu,
    mutex::Mutex,
    println,
};
use core::{alloc::GlobalAlloc, cell::OnceCell};

pub struct Kernel {
    cpus: [Cpu; 4],
    allocator: Mutex<OnceCell<LinkedListAllocator>>,
    serial: Mutex<OnceCell<UartController>>,
}

impl Kernel {
    pub const fn uninitialized() -> Self {
        Self {
            cpus: [Cpu::new(0), Cpu::new(1), Cpu::new(2), Cpu::new(3)],
            allocator: Mutex::new(OnceCell::new()),
            serial: Mutex::new(OnceCell::new()),
        }
    }

    const HEAP_START: *mut u8 = 0xFFFF_0000_4000_0000 as *mut u8;
    const HEAP_SIZE: usize = 1024 * 1024;

    const UART0_BASE: *mut u8 = 0xFFFF_0000_FE20_1000 as *mut u8;

    pub fn init(&self) {
        let uart_driver = UartPl011::new(Self::UART0_BASE.into());
        let mut uart_controller = UartController::new(uart_driver);
        uart_controller.init();
        self.serial.lock().set(uart_controller);

        let allocator = unsafe { LinkedListAllocator::new(Self::HEAP_START, Self::HEAP_SIZE) };
        self.allocator
            .lock()
            .set(allocator)
            .unwrap_or_else(|_| println!("Allocator was already initialized"));
    }

    pub fn cpu_me(&self) -> &Cpu {
        let cpu_idx = 0; // TODO: fetch from system register
        &self.cpus[cpu_idx]
    }

    // TODO: this method feels a bit messy, like its leaking some extra implementation details...
    pub fn get_serial(&self) -> &Mutex<OnceCell<UartController>> {
        &self.serial
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
