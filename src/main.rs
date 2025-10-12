#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod devices;
mod exception;
mod kernel;
mod mutex;
mod registers;

use alloc::vec::Vec;
use core::hint::spin_loop;

use crate::{
    devices::generic::gic::GICv2,
    exception::{branch_load_frame, frame::ExceptionFrame, irq::IRQ},
    kernel::{cpu::Cpu, process::Process},
};

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

    let p1 = Process::init(test::<1>);
    let p2 = Process::init(test::<2>);
    let p3 = Process::init(test::<3>);

    let mut cpu = Cpu::me().lock();
    let scheduler = cpu.scheduler_mut();
    scheduler.register_process(p1);
    scheduler.register_process(p2);
    scheduler.register_process(p3);

    unsafe {
        // TODO: would be nice for this to be managed separately, lump
        //       in set_timer_interrupt as well with it
        GICv2::init();
        GICv2::enable_irq(IRQ::GenericPhysTimer);
    }

    unsafe {
        scheduler.start();
    }
}

fn test<const X: usize>() -> ! {
    let mut i = 0u64;
    loop {
        println!("{X}: {i}");
        for _ in 0..100000000 {
            spin_loop();
        }
        i += 1;
    }
}

#[no_mangle]
pub extern "C" fn _handle_exception(x0: *mut ExceptionFrame) {
    let frame = unsafe { x0.as_ref().unwrap() };

    // println!("Got an exception! Data:\n{:?}", frame);

    // TODO: still need to actually differentiate exception kinds/IRQs

    let mut cpu = Cpu::me().lock();
    let next_process = cpu.scheduler_mut().next(frame as *const ExceptionFrame);
    let next_process_sp = next_process.unwrap().sp();

    unsafe { branch_load_frame(next_process_sp as *mut ExceptionFrame) };

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
