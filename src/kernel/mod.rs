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

#[macro_export]
macro_rules! println {
    ($($args:tt)*) => {
        $crate::print!("{}\n", format_args!($($args)*))
    };
}

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => {
        $crate::kernel::_print(format_args!($($args)*))
    };
}

pub fn _print(args: core::fmt::Arguments<'_>) {
    // TODO: should anything be done if this returns an Err?
    let _ = get_kernel().serial_write_fmt(args);
}
