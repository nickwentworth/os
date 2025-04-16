use core::{arch::asm, time::Duration};

pub struct HardwareTimer;

impl HardwareTimer {
    const NANOS_PER_SEC: u128 = 1_000_000_000;

    /// Returns the duration of time that the kernel has been running for
    pub fn runtime() -> Duration {
        let mut counter: u64;
        unsafe { asm!( "mrs {}, CNTPCT_EL0", out(reg) counter) }
        let nanos = counter as u128 * Self::NANOS_PER_SEC / Self::freq();
        Duration::from_nanos(nanos as u64)
    }

    /// Setup a timer interrupt to be fired after the given duration
    pub fn set_timer_interrupt(duration: Duration) {
        let now = Self::runtime();
        let target = now + duration;
        let target_counter = target.as_nanos() * Self::freq() / Self::NANOS_PER_SEC;
        unsafe { asm!("msr CNTP_CVAL_EL0, {}", in(reg) target_counter as u64) }
    }

    fn freq() -> u128 {
        let mut freq: u64;
        unsafe { asm!("mrs {}, CNTFRQ_EL0", out(reg) freq) }
        freq as u128
    }
}
