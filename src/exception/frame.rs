#[derive(Debug)]
#[repr(C)]
pub struct ExceptionFrame {
    regs: [usize; 31],
    sp: usize,
    elr: usize,
    spsr: usize,
    esr: usize,
    far: usize,
    kind: usize,
}

#[no_mangle]
pub static EXCEPTION_FRAME_SIZE: usize = size_of::<ExceptionFrame>();

impl ExceptionFrame {
    pub fn new_el1(frame_addr: usize, entry_addr: usize) -> Self {
        Self {
            regs: [0; 31],
            sp: frame_addr,
            elr: entry_addr,
            spsr: 0b_0101,
            esr: 0,
            far: 0,
            kind: 0,
        }
    }

    pub fn elr(&self) -> usize {
        self.elr
    }
}
