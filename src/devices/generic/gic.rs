use crate::exception::irq::IRQ;

pub struct GICv2;

impl GICv2 {
    const GIC_BASE: u64 = 0xFFFF_0000_FF84_0000;

    // register offsets
    const GICD: usize = 0x1000;
    const GICD_CTLR: usize = Self::GICD;
    const GICD_ISENABLER_BASE: usize = Self::GICD + 0x100;

    const GICC: usize = 0x2000;
    const GICC_CTLR: usize = Self::GICC;
    const GICC_PMR: usize = Self::GICC + 0x4;

    pub unsafe fn init() {
        Self::write_reg(Self::GICC_PMR, 255);
        Self::write_reg(Self::GICC_CTLR, 1);
        Self::write_reg(Self::GICD_CTLR, 1);
    }

    pub unsafe fn enable_irq(irq: IRQ) {
        let isenabler_n = irq.gic_int_id() / 32;
        let isenabler_offset = Self::GICD_ISENABLER_BASE + 4 * isenabler_n;

        let bit_m = irq.gic_int_id() % 32;

        Self::write_reg(isenabler_offset, 1 << bit_m);
    }

    // TODO: remove or sync this, brought over from uart
    unsafe fn write_reg(offset: usize, value: u32) {
        let ptr = Self::GIC_BASE as *mut u32;
        ptr.byte_add(offset).write_volatile(value);
    }
}
