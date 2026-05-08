use crate::exception::frame::{ExceptionFrame, EXCEPTION_FRAME_SIZE};
use alloc::boxed::Box;
use core::ptr;

pub struct Process {
    stack: Box<[u8; Self::STACK_SIZE]>,
    pub(super) sp: usize,
}

impl Process {
    const STACK_SIZE: usize = 1024;

    pub fn init(entry: fn() -> !) -> Self {
        let mut stack = Box::new([0; Self::STACK_SIZE]);

        // write initial exception frame to stack
        let sp = unsafe {
            let stack_low = ptr::from_mut(stack.get_mut(0).unwrap());
            let stack_high = stack_low.add(Self::STACK_SIZE);

            let frame_addr = stack_high
                .sub(EXCEPTION_FRAME_SIZE)
                .cast::<ExceptionFrame>();

            let initial_frame = ExceptionFrame::new_el1(frame_addr as usize, entry as usize);

            frame_addr.write(initial_frame);
            frame_addr
        };

        Self {
            stack,
            sp: sp as usize,
        }
    }

    pub fn sp(&self) -> usize {
        self.sp
    }
}
