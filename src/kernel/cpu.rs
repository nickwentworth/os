use crate::{kernel::scheduler::Scheduler, mutex::Mutex};
use core::{
    arch::asm,
    sync::atomic::{AtomicU64, Ordering},
};

static CPU: Cpu = Cpu::new(0);

pub struct Cpu {
    cpu_id: usize,
    scheduler: Mutex<Scheduler>,
    preempt_rc: AtomicU64,
}

impl Cpu {
    const fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            scheduler: Mutex::new(Scheduler::new()),
            preempt_rc: AtomicU64::new(0),
        }
    }

    pub fn me() -> &'static Cpu {
        &CPU
    }

    pub fn scheduler(&self) -> &Mutex<Scheduler> {
        &self.scheduler
    }

    pub fn preempt_counter(&self) -> u64 {
        self.preempt_rc.load(Ordering::SeqCst)
    }

    pub fn increment_preempt_counter(&self) -> u64 {
        let prev = self.preempt_rc.fetch_add(1, Ordering::SeqCst);

        if prev == 0 {
            unsafe { asm!("msr DAIFSET, #0b1111") }
        }

        prev + 1
    }

    pub fn decrement_preempt_counter(&self) -> u64 {
        let prev = self.preempt_rc.fetch_sub(1, Ordering::SeqCst);

        if prev == 1 {
            unsafe { asm!("msr DAIFCLR, #0b1111") }
        }

        prev - 1
    }
}
