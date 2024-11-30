#![no_std]
#![no_main]

mod registers;
mod uart;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    unsafe {
        let mut el: u64;
        core::arch::asm!("mrs {}, CurrentEL", out(reg) el);
        println!("Entering kernel at EL{}", (el >> 2) & 0b11);
    }

    unsafe {
        core::ptr::read_volatile(0xFFFF_FFFF_FFFF_FFFF as *mut u32);
    }

    println!("Back from the exception!");

    loop {}
}

#[no_mangle]
pub extern "C" fn _handle_exception(x0: u64) {
    println!("Got an exception! Origin/Type Data: {}", x0);

    // TODO: we should normally return, but the test invalid memory
    // access would just cause us to keep raising exceptions forever
    // because we aren't advancing the exception return address, so
    // for now we will just loop forever in here
    loop {}
}
