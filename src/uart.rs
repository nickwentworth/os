use core::{
    fmt::{self, Arguments, Write},
    ptr::{read_volatile, write_volatile},
};

pub struct UartPl011;

impl UartPl011 {
    const UART0_BASE: u32 = 0xFE20_1000;

    // register offsets
    const DR: usize = 0x00;
    const FR: usize = 0x18;

    // TODO: qemu comes with UART0 setup, need to set it up here in the future

    /// Transmit a string through UART0's FIFO buffer, character by character
    pub fn transmit_str(str: &str) {
        for c in str.bytes() {
            unsafe {
                // wait for UART to finish transmitting data
                while Self::read_reg(Self::FR) & (1 << 3) == 1 {}

                Self::write_reg(Self::DR, c.into());
            }
        }
    }

    unsafe fn read_reg(offset: usize) -> u32 {
        let reg = (Self::UART0_BASE as *mut u32).byte_add(offset);
        read_volatile(reg)
    }

    unsafe fn write_reg(offset: usize, value: u32) {
        let reg = (Self::UART0_BASE as *mut u32).byte_add(offset);
        write_volatile(reg, value);
    }
}

impl core::fmt::Write for UartPl011 {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        UartPl011::transmit_str(s);
        Ok(())
    }
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
        $crate::uart::_print(format_args!($($args)*))
    };
}

/// Helper function passed to print! macro
pub fn _print(args: Arguments<'_>) {
    UartPl011::write_fmt(&mut UartPl011, args).unwrap()
}
