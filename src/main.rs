#![no_std]
#![no_main]
#![feature(generic_const_exprs)]

extern crate alloc;

mod allocator;
mod devices;
mod exception;
mod graphics;
mod kernel;
mod mutex;
mod registers;
mod util;

use crate::{
    devices::generic::gic::GICv2,
    exception::{frame::ExceptionFrame, irq::IRQ},
    kernel::{get_kernel, init_kernel, process::Process, scheduler::Scheduler},
};
use alloc::vec::Vec;
use core::hint::spin_loop;

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    unsafe {
        let mut el: u64;
        core::arch::asm!("mrs {}, CurrentEL", out(reg) el);
        println!("Entering kernel at EL{}", (el >> 2) & 0b11);
    }

    unsafe { init_kernel() };

    // TODO: would be cool to have some way to easily test things, like cargo test
    // test out the allocator
    let mut v = Vec::new();
    let size = 10000;
    for i in 0..size {
        v.push(i);
    }
    for i in 0..size {
        assert_eq!(v.get(i), Some(&i));
    }
    println!("Basic allocation test passed!");

    graphics::init_graphics();

    // initialize some test processes
    let mut scheduler = get_kernel().cpu_me().scheduler().lock();
    scheduler.register_process(Process::init(test::<1>));
    scheduler.register_process(Process::init(test::<2>));
    scheduler.register_process(Process::init(test::<3>));
    drop(scheduler);

    unsafe {
        // TODO: would be nice for this to be managed separately, lump
        //       in set_timer_interrupt as well with it
        GICv2::init();
        GICv2::enable_irq(IRQ::GenericPhysTimer);
    }

    assert_eq!(get_kernel().cpu_me().preempt_counter(), 0);

    Scheduler::start();
}

fn test<const X: usize>() -> ! {
    let mut i = 0u64;
    loop {
        println!("{X}: {i}");
        for _ in 0..100_000_000 {
            spin_loop();
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn _handle_exception(x0: *mut ExceptionFrame) -> usize {
    // println!("Got an exception! Data:\n{:?}", frame);

    // TODO: still need to actually differentiate exception kinds/IRQs

    let mut scheduler = get_kernel().cpu_me().scheduler().lock();
    let next_process = scheduler.next(x0);
    next_process.unwrap().sp()

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
