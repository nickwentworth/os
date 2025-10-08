use core::{ptr, time::Duration};

use alloc::{boxed::Box, collections::vec_deque::VecDeque};

use crate::{
    devices::generic::{gic::GICv2, timer::ArmPhysTimer},
    exception::{
        branch_load_frame,
        frame::{ExceptionFrame, EXCEPTION_FRAME_SIZE},
        irq::IRQ,
    },
    mutex::Mutex,
    println,
};

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

    active_process: Option<Process>,
    run_queue: VecDeque<Process>,
}

impl Cpu {
    const fn new(cpu_id: usize) -> Self {
        Self {
            cpu_id,
            active_process: None,
            run_queue: VecDeque::new(),
        }
    }

    pub fn me() -> &'static Mutex<Cpu> {
        // TODO: fetch from system register
        let cpu_id = 0;

        CPU_ARR.get(cpu_id).unwrap()
    }

    pub fn queue_process(&mut self, process: Process) {
        self.run_queue.push_back(process);
    }

    pub unsafe fn start_scheduler(&mut self) -> ! {
        unsafe {
            ArmPhysTimer::set_timer_interrupt(Duration::from_millis(1));
            GICv2::init();
            GICv2::enable_irq(IRQ::GenericPhysTimer);
        }

        let process = self.run_queue.pop_front();
        self.active_process = process;

        let frame_sp = self.active_process.as_ref().unwrap().sp;
        println!("Frame SP: {:x}", frame_sp);
        branch_load_frame(frame_sp as *mut ExceptionFrame);
    }

    pub fn next_process(&mut self, prev_frame: *const ExceptionFrame) -> Option<&Process> {
        ArmPhysTimer::set_timer_interrupt(Duration::from_millis(1));

        if let Some(mut prev_process) = self.active_process.take() {
            prev_process.sp = prev_frame as usize;

            let next_process = self.run_queue.pop_front().unwrap();
            self.run_queue.push_back(prev_process);

            self.active_process = Some(next_process);
            return self.active_process.as_ref();
        }

        None
    }
}

pub struct Process {
    stack: Box<[u8; Self::STACK_SIZE]>,
    sp: usize,
}

impl Process {
    const STACK_SIZE: usize = 1024;

    pub unsafe fn init(entry: fn() -> !) -> Self {
        let mut stack = Box::new([0; Self::STACK_SIZE]);

        let stack_low = ptr::from_mut(stack.get_mut(0).unwrap());
        let stack_high = stack_low.add(Self::STACK_SIZE);

        let frame_addr = stack_high
            .sub(EXCEPTION_FRAME_SIZE)
            .cast::<ExceptionFrame>();

        let initial_frame = ExceptionFrame::new_el1(frame_addr as usize, entry as usize);

        frame_addr.write(initial_frame);

        Self {
            stack,
            sp: frame_addr as usize,
        }
    }

    pub fn sp(&self) -> usize {
        self.sp
    }
}
