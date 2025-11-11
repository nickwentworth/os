pub mod generic;
pub mod raspi;

pub unsafe trait MmioDevice {
    fn base_addr(&self) -> *mut u32;

    /// Read 32 bits from this device's base, at a given offset
    fn read(&self, offset: usize) -> u32 {
        unsafe { self.base_addr().byte_add(offset / 8).read_volatile() }
    }

    /// Writes 32 bits from this device's base, at a given offset
    fn write(&mut self, offset: usize, data: u32) {
        unsafe { self.base_addr().byte_add(offset / 8).write_volatile(data) };
    }

    /// Set the given range of bits from `msb` to `lsb` (both inclusize) to `1`.
    fn set_bits(&mut self, offset: usize, msb: u8, lsb: u8) {
        let curr = self.read(offset);

        // general idea for building these masks (ex: bits 7 to 5):
        // ones:   0 ... 0 1 1 1 1 1 1 1
        //
        // zeros:  0 ... 0 0 0 0 1 1 1 1
        // !zeros: 1 ... 1 1 1 1 0 0 0 0
        //
        // mask:   0 ... 0 1 1 1 0 0 0 0

        let ones_mask = (1 << (msb + 1)) - 1;
        let zeros_mask = (1 << lsb) - 1;
        let mask = ones_mask & !zeros_mask;

        self.write(offset, curr | mask);
    }

    fn set_bit(&mut self, offset: usize, bit: u8) {
        self.set_bits(offset, bit, bit);
    }

    /// Set the given range of bits from `msb` to `lsb` (both inclusize) to `0`.
    fn clear_bits(&mut self, offset: usize, msb: u8, lsb: u8) {
        let curr = self.read(offset);

        // (same as set_bits here)
        let ones_mask = (1 << (msb + 1)) - 1;
        let zeros_mask = (1 << lsb) - 1;
        let mask = ones_mask & !zeros_mask;

        self.write(offset, curr & !mask);
    }

    fn clear_bit(&mut self, offset: usize, bit: u8) {
        self.clear_bits(offset, bit, bit);
    }

    fn write_bit(&mut self, offset: usize, bit: u8, to: bool) {
        if to {
            self.set_bit(offset, bit);
        } else {
            self.clear_bit(offset, bit);
        }
    }
}
