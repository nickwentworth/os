#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod devices;
mod exception;
mod mutex;
mod registers;

use alloc::vec::Vec;
use core::time::Duration;
use devices::generic::{gic::GICv2, timer::ArmPhysTimer};
use exception::irq::IRQ;

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    unsafe {
        let mut el: u64;
        core::arch::asm!("mrs {}, CurrentEL", out(reg) el);
        println!("Entering kernel at EL{}", (el >> 2) & 0b11);
    }

    allocator::init_global_allocator();

    // TODO: would be cool to have some way to easily test things, like cargo test
    // lets test out the allocator
    let mut v = Vec::new();
    let size = 10000;
    for i in 0..size {
        v.push(i);
    }
    for i in 0..size {
        assert_eq!(v.get(i), Some(&i));
    }
    println!("Basic allocation test passed!");

    unsafe {
        ArmPhysTimer::set_timer_interrupt(Duration::from_secs(3));
        GICv2::init();
        GICv2::enable_irq(IRQ::GenericPhysTimer);
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn _handle_exception(x0: u64) {
    panic!("Got an exception! Origin/Type Data: {}", x0);

    // TODO: we should normally return, but the test invalid memory
    // access would just cause us to keep raising exceptions forever
    // because we aren't advancing the exception return address
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
static mut __L0_TABLE: TranslationTable = TranslationTable::new();
#[no_mangle]
static mut __L1_TABLE: TranslationTable = TranslationTable::new();

#[repr(align(4096))]
struct TranslationTable([u64; 512]);

impl TranslationTable {
    const fn new() -> Self {
        Self([0; 512])
    }
}
