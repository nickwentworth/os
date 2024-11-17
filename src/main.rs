#![no_std]
#![no_main]

mod uart;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _kernel_main() -> ! {
    println!("Hello world!");
    println!("{} / {} = {:.6}", 2, 3, 2f32 / 3f32);
    panic!("We should panic here!");
}
