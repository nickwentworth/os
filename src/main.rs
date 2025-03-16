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
        core::ptr::read_volatile(0xFFFF_0000_0000_0000 as *mut u32); // lowest physical address
        core::ptr::read_volatile(0xFFFF_0000_7FFF_FFFC as *mut u32); // top-most physical address
        println!("Good so far...");
        core::ptr::read_volatile(0xFFFF_0000_8000_0000 as *mut u32); // now we get an exception!
        println!("Not getting here!");
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
