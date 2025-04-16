use core::ptr::write_volatile;

pub struct Gic;

impl Gic {
    const GIC_BASE: u64 = 0xFFFF_0000_FF84_0000;

    // register offsets
    const GICD: usize = 0x1000;
    const GICD_CTLR: usize = Self::GICD;
    const GICD_ISENABLER0: usize = Self::GICD + 0x100;

    const GICC: usize = 0x2000;
    const GICC_CTLR: usize = Self::GICC;
    const GICC_PMR: usize = Self::GICC + 0x4;

    pub unsafe fn init() {
        // to get timer interrupts setup, we need to configure interrupts for the
        // EL1 phyiscal timer interrupt, INTID = 30

        // TODO: these can be generalized, ex: INTID > 32 will have different requirements

        // TODO: this only initializes CPU 0, any further CPUs need a similar, but separate
        //       init step

        // set 30th bit to enable INTID 30
        Self::write_reg(Self::GICD_ISENABLER0, 1 << 30);

        // don't mask any interrupts based on priotity
        Self::write_reg(Self::GICC_PMR, 255);

        // finally enable the cpu interface and distributor
        Self::write_reg(Self::GICC_CTLR, 1);
        Self::write_reg(Self::GICD_CTLR, 1);
    }

    // TODO: copied from uart.rs, should move to a common struct
    unsafe fn write_reg(offset: usize, value: u32) {
        let reg = (Self::GIC_BASE as *mut u32).byte_add(offset);
        write_volatile(reg, value);
    }
}
