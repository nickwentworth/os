use crate::kernel::kernel::Kernel;

pub mod cpu;
pub mod kernel;
pub mod process;
pub mod scheduler;

#[global_allocator]
static KERNEL: Kernel = Kernel::uninitialized();

pub unsafe fn init_kernel() {
    KERNEL.init();
}

pub fn get_kernel() -> &'static Kernel {
    &KERNEL
}
