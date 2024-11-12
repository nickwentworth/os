#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    let test_buffer = 0x100000 as *mut u8;

    unsafe {
        *test_buffer.offset(0) = b'H';
        *test_buffer.offset(1) = b'E';
        *test_buffer.offset(2) = b'L';
        *test_buffer.offset(3) = b'L';
        *test_buffer.offset(4) = b'O';
    }

    loop {}
}
