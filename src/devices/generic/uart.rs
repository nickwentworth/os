use crate::devices::MmioDevice;

pub struct UartController {
    driver: UartPl011,
}

unsafe impl Send for UartController {}

impl UartController {
    pub fn new(driver: UartPl011) -> Self {
        Self { driver }
    }

    pub fn init(&mut self) {
        self.driver.configure(UartPl011Config {
            enabled: true,
            mode: UartPl011Mode::Transmit,
        });
    }

    pub fn transmit_str(&mut self, s: &str) {
        self.driver.set_mode(UartPl011Mode::Transmit);

        s.chars()
            .map(|ch| ch as u8)
            .for_each(|byte| self.driver.transmit_byte(byte));
    }

    pub fn receieve_char(&mut self) -> char {
        self.driver.set_mode(UartPl011Mode::Transmit);
        self.driver.receieve_char()
    }
}

impl core::fmt::Write for UartController {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        Ok(self.transmit_str(s))
    }
}

pub struct UartPl011 {
    base_addr: *mut u8,
}

struct UartPl011Config {
    enabled: bool,
    mode: UartPl011Mode,
}

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

    pub fn new(base_addr: *mut u8) -> Self {
        Self { base_addr }
    }

    // -------------------- Configuration -------------------- //

    fn configure(&mut self, options: UartPl011Config) {
        self.write_bit(Self::UART_CR, Self::UART_CR_UARTEN, options.enabled);
        self.set_mode(options.mode);

        self.write_bit(Self::UART_LCRH, Self::UART_LCRH_FEN, true); // always enable FIFOs
    }

    fn set_mode(&mut self, mode: UartPl011Mode) {
        self.write_bit(
            Self::UART_CR,
            Self::UART_CR_RXE,
            mode == UartPl011Mode::Receive,
        );
        self.write_bit(
            Self::UART_CR,
            Self::UART_CR_TXE,
            mode == UartPl011Mode::Transmit,
        );
    }

    // -------------------- Data Read & Write -------------------- //

    fn transmit_byte(&mut self, byte: u8) {
        while self.is_busy() {}

        self.write(Self::UART_DR, byte.into());
    }

    fn receieve_char(&self) -> char {
        while self.is_busy() {}

        let data = self.read(Self::UART_DR);
        data as u8 as char
    }

    fn is_busy(&self) -> bool {
        let fr = self.read(Self::UART_FR);
        fr & (1 << Self::UART_FR_BUSY) != 0
    }
}

unsafe impl MmioDevice for UartPl011 {
    fn base_addr(&self) -> *mut u32 {
        self.base_addr.cast()
    }
}
