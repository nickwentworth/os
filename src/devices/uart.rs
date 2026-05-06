use crate::{
    devices::{Device, MmioDevice},
    mem::addr::KernelVirtAddr,
    util::dtb::DtbNode,
};

pub struct UartPl011Device {
    base_addr: KernelVirtAddr,
}

impl Device for UartPl011Device {
    fn name(&self) -> &str {
        "PrimeCell UART PL011"
    }
}

unsafe impl MmioDevice for UartPl011Device {
    fn base_addr(&self) -> *mut u32 {
        self.base_addr.to_ptr().cast()
    }
}

impl UartPl011Device {
    pub fn try_from_node(node: &DtbNode) -> Option<Self> {
        if !node.is_compatible("arm,pl011") {
            return None;
        }

        let base_addr = node.prop_reg()?.0;
        Some(Self {
            base_addr: base_addr.into(),
        })
    }
}

/// Driver for the PrimeCell UART PL011. Enables basic functions,
/// including transmitting and receiving characters.
///
/// ### Resources
/// https://documentation-service.arm.com/static/5e8e36c2fd977155116a90b5
pub struct UartPl011 {
    device: UartPl011Device,
}

/// Configuration helper for the UART
struct UartPl011Config {
    enabled: bool,
    mode: UartPl011Mode,
}

/// Describes operation mode of the UART
#[derive(Clone, Copy, PartialEq, Eq)]
enum UartPl011Mode {
    Transmit,
    Receive,
}

impl UartPl011 {
    /// Data Register
    const UART_DR: usize = 0x000;
    const UART_DR_DATA: u8 = 0; // Receive or transmit char

    /// Flag Register
    const UART_FR: usize = 0x018;
    const UART_FR_BUSY: u8 = 3; // UART busy

    /// Line Control Register
    const UART_LCRH: usize = 0x02C;
    const UART_LCRH_FEN: u8 = 4; // FIFO enable

    /// Control Register
    const UART_CR: usize = 0x030;
    const UART_CR_RXE: u8 = 9; // Receive enable
    const UART_CR_TXE: u8 = 8; // Transmit enable
    const UART_CR_UARTEN: u8 = 0; // UART enable

    pub fn bind(device: UartPl011Device) -> Self {
        let mut uart = Self { device };
        uart.configure(UartPl011Config {
            enabled: true,
            mode: UartPl011Mode::Transmit,
        });
        uart
    }

    pub fn init(&mut self) {
        self.configure(UartPl011Config {
            enabled: true,
            mode: UartPl011Mode::Transmit,
        });
    }

    pub fn transmit_str(&mut self, s: &str) {
        self.set_mode(UartPl011Mode::Transmit);

        s.chars()
            .map(|ch| ch as u8)
            .for_each(|byte| self.transmit_byte(byte));
    }

    pub fn receieve_char(&mut self) -> char {
        self.set_mode(UartPl011Mode::Receive);
        self.receieve_byte().into()
    }

    // -------------------- Configuration -------------------- //

    fn configure(&mut self, options: UartPl011Config) {
        self.device
            .write_bit(Self::UART_CR, Self::UART_CR_UARTEN, options.enabled);

        self.set_mode(options.mode);

        self.device
            .write_bit(Self::UART_LCRH, Self::UART_LCRH_FEN, true); // always enable FIFOs
    }

    fn set_mode(&mut self, mode: UartPl011Mode) {
        self.device.write_bit(
            Self::UART_CR,
            Self::UART_CR_RXE,
            mode == UartPl011Mode::Receive,
        );
        self.device.write_bit(
            Self::UART_CR,
            Self::UART_CR_TXE,
            mode == UartPl011Mode::Transmit,
        );
    }

    // -------------------- Data Read & Write -------------------- //

    fn transmit_byte(&mut self, byte: u8) {
        while self.is_busy() {}
        self.device.write(Self::UART_DR, byte.into());
    }

    fn receieve_byte(&self) -> u8 {
        while self.is_busy() {}
        self.device.read(Self::UART_DR) as u8
    }

    fn is_busy(&self) -> bool {
        let fr = self.device.read(Self::UART_FR);
        fr & (1 << Self::UART_FR_BUSY) != 0
    }
}

impl core::fmt::Write for UartPl011 {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.transmit_str(s))
    }
}
