use core::arch::naked_asm;

use crate::exception::frame::ExceptionFrame;

pub mod frame;
pub mod irq;

/// Loads the given `ExceptionFrame` to general purpose and system registers
/// and resumes at the given `ExceptionFrame.elr` address.
///
/// ### Safety:
/// This is a direct branch instruction, so the current stack is basically
/// invalid afterwards.
///
/// This still works because when we take an exception, we store the old
/// process's `sp`, which is pointing to its own `ExceptionFrame`. When we
/// eventually get back to that process, that extra junk from the
/// exception_handler's stack is effectively ignored.
#[unsafe(naked)]
pub unsafe extern "C" fn branch_load_frame(_frame: *mut ExceptionFrame) -> ! {
    naked_asm!("mov sp, x0", "b _load_exception_frame");
}
