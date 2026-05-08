use crate::{
    allocator::LinkedListAllocator, devices::registry::DeviceRegistry, mem::addr::PhysAddr,
    mutex::Mutex, println, sys::cpu::Cpu, util::dtb::DeviceTree,
};
use core::{alloc::GlobalAlloc, fmt::Write, ptr::null_mut};

struct GlobalKernel(Option<Kernel>);

#[global_allocator]
static mut KERNEL: GlobalKernel = GlobalKernel(None);

unsafe impl GlobalAlloc for GlobalKernel {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        println!("Allocating {} bytes", layout.size());
        Kernel::get()
            .allocator
            .lock()
            .alloc(layout)
            .unwrap_or(null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        println!("Deallocating {} bytes", layout.size());
        Kernel::get().allocator.lock().dealloc(ptr, layout);
    }
}

pub struct Kernel {
    cpus: [Cpu; 4],
    allocator: Mutex<LinkedListAllocator>,
    drivers: DeviceRegistry,
}

impl Kernel {
    // -------------------- Global Kernel Methods -------------------- //

    /// Constructs and initializes the global `Kernel` object.
    ///
    /// ### Safety
    /// This must only be called once. Any usages of `kernel()` before calling this
    /// will result in a panic.
    pub unsafe fn init_kernel(dt: DeviceTree) {
        let kernel = Kernel::init(dt);
        KERNEL.0 = Some(kernel);
    }

    /// Returns a reference to a presumed initialized kernel. In most cases outside of
    /// `init_kernel()`, this can be used.
    pub fn get() -> &'static Kernel {
        Self::try_get().expect("Kernel should be initialized")
    }

    /// Returns either a reference to an initialized kernel, or `None` if `init_kernel()`
    /// hasn't yet been called or finished.
    ///
    /// Useful for systems that may be used during or after kernel initialization.
    pub fn try_get() -> Option<&'static Kernel> {
        unsafe { KERNEL.0.as_ref() }
    }

    // -------------------- Kernel Instance Methods -------------------- //

    const HEAP_START: PhysAddr = PhysAddr::new(0x5000_0000);
    const HEAP_SIZE: usize = 1024 * 1024;

    fn init(dt: DeviceTree) -> Self {
        let mut drivers = DeviceRegistry::empty();
        for node in dt.nodes() {
            drivers.try_bind(&node);
        }

        let allocator =
            unsafe { LinkedListAllocator::new(Self::HEAP_START.into(), Self::HEAP_SIZE) };

        Self {
            cpus: [Cpu::new(0), Cpu::new(1), Cpu::new(2), Cpu::new(3)],
            allocator: Mutex::new(allocator),
            drivers,
        }
    }

    pub fn cpu_me(&self) -> &Cpu {
        let cpu_idx = 0; // TODO: fetch from system register
        &self.cpus[cpu_idx]
    }

    pub fn serial_write_fmt(&self, args: core::fmt::Arguments) -> core::fmt::Result {
        self.drivers
            .uart
            .as_ref()
            .ok_or(core::fmt::Error)?
            .lock()
            .write_fmt(args)
    }
}
