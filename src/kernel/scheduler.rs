use crate::{
    devices::generic::timer::ArmPhysTimer,
    exception::{branch_load_frame, frame::ExceptionFrame},
    kernel::process::Process,
};
use alloc::collections::vec_deque::VecDeque;
use core::{ptr, time::Duration};

pub struct Scheduler {
    active_process: Option<Process>,
    run_queue: VecDeque<Process>,
}

impl Scheduler {
    const PREEMPT_RATE: Duration = Duration::from_millis(10);

    pub const fn new() -> Self {
        Self {
            active_process: None,
            run_queue: VecDeque::new(),
        }
    }

    pub fn register_process(&mut self, process: Process) {
        self.run_queue.push_back(process);
    }

    /// Rotates the current `Process` (if there is one) to the back of this
    /// scheduler's run queue, and chooses the next as its active `Process`.
    ///
    /// Returns the new active `Process`, if one was chosen.
    pub fn next(&mut self, prev_frame: *const ExceptionFrame) -> Option<&Process> {
        if let Some(mut prev) = self.active_process.take() {
            prev.sp = prev_frame as usize;
            self.run_queue.push_back(prev);
        }

        let next = self.run_queue.pop_front();
        self.active_process = next;

        ArmPhysTimer::set_timer_interrupt(Self::PREEMPT_RATE);

        self.active_process.as_ref()
    }

    /// Start this scheduler, which loads and runs the first `Process` in its run queue.
    ///
    /// ### Safety
    /// This marks the end of the kernel initialization phase and should be called just once per CPU.
    ///
    /// Panics if no `Process` was registered via `Self::register_process()`, as there
    /// would be nothing to do
    pub unsafe fn start(&mut self) -> ! {
        let initial_process = self
            .next(ptr::null())
            .expect("No processes were scheduled!");

        ArmPhysTimer::set_timer_interrupt(Self::PREEMPT_RATE);

        branch_load_frame(initial_process.sp as *mut ExceptionFrame);
    }
}
