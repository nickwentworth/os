use crate::{kernel::scheduler::Scheduler, mutex::Mutex};

static CPU_ARR: [Mutex<Cpu>; 4] = {
    [
        Mutex::new(Cpu::new(0)),
        Mutex::new(Cpu::new(1)),
        Mutex::new(Cpu::new(2)),
        Mutex::new(Cpu::new(3)),
    ]
};

pub struct Cpu {
    cpu_id: usize,
    scheduler: Scheduler,
}

impl Cpu {
    const fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            scheduler: Scheduler::new(),
        }
    }

    pub fn me() -> &'static Mutex<Cpu> {
        // TODO: fetch from system register
        let cpu_id = 0;

        CPU_ARR.get(cpu_id).unwrap()
    }

    pub fn scheduler(&self) -> &Scheduler {
        &self.scheduler
    }
    pub fn scheduler_mut(&mut self) -> &mut Scheduler {
        &mut self.scheduler
    }
}
