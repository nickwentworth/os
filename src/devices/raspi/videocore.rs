use crate::println;
use core::ptr::{read_volatile, write_volatile};

pub struct VideoCore {}

impl VideoCore {
    const VC_BASE: usize = 0xFFFF_0000_FE00_B880;
    const VC_READ: usize = 0x00;
    const VC_STATUS: usize = 0x10;
    const VC_WRITE: usize = 0x20;

    const MAILBOX_FULL: u32 = 1 << 31;
    const MAILBOX_EMPTY: u32 = 1 << 30;

    // TODO: should we require &self / &mut self for these?

    pub fn get_firmware_version() -> u32 {
        let res = Self::mailbox_call::<0, 1>(0x00000001, []);
        res[0]
    }

    pub fn get_display_dimensions() -> (u32, u32) {
        let res = Self::mailbox_call::<0, 2>(0x00040003, []);
        (res[0], res[1])
    }

    pub fn get_virtual_buffer_dimensions() -> (u32, u32) {
        let res = Self::mailbox_call::<0, 2>(0x00040004, []);
        (res[0], res[1])
    }

    pub fn get_depth() -> u32 {
        let res = Self::mailbox_call::<0, 1>(0x00040005, []);
        res[0]
    }

    pub fn allocate_frame_buffer() -> (u32, u32) {
        let res = Self::mailbox_call::<1, 2>(0x00040001, [4096]);
        (res[0], res[1])
    }

    const fn max(a: usize, b: usize) -> usize {
        if a > b {
            a
        } else {
            b
        }
    }

    fn mailbox_call<const REQ_LEN: usize, const RES_LEN: usize>(
        tag: u32,
        params: [u32; REQ_LEN],
    ) -> [u32; RES_LEN]
    where
        [(); 6 + Self::max(REQ_LEN, RES_LEN)]:,
    {
        // TODO: sometimes this isn't actually aligned to 16-bytes, and adding
        //       #[repr(align(16))] also isn't aligning it, so we may need
        //       to implement proper alignment in allocator, then allocate it there?

        let mut buf = [0; 6 + Self::max(REQ_LEN, RES_LEN)];

        buf[0] = buf.len() as u32 * 4;
        buf[1] = 0; // full mailbox status code

        buf[2] = tag;
        buf[3] = Self::max(REQ_LEN, RES_LEN) as u32 * 4;
        buf[4] = 0; // this tag's status code

        for i in 0..REQ_LEN {
            buf[5 + i] = params[i];
        }

        buf[buf.len() - 1] = 0; // end tag

        let buf_addr = buf.get(0).unwrap() as *const u32 as u32;
        println!("Buffer addr: 0x{:x}", buf_addr);

        let masked_addr = (buf_addr & !0b1111) | 0b1000;
        unsafe { Self::write_mailbox(masked_addr) }

        let resp_addr = unsafe { Self::read_mailbox() };
        println!("Response addr: 0x{:x}", resp_addr);

        println!("{:?}", buf);
        println!("{:x?}", buf);

        let resp_code = buf[1];
        println!("Response code: 0x{:x}", resp_code);

        let mut response = [0u32; RES_LEN];
        for i in 0..RES_LEN {
            response[i] = buf[5 + i];
        }
        response
    }

    unsafe fn write_mailbox(value: u32) {
        while Self::read_reg(Self::VC_STATUS) & Self::MAILBOX_FULL != 0 {
            println!("ASDF");
        }
        Self::write_reg(Self::VC_WRITE, value);
    }

    unsafe fn read_mailbox() -> u32 {
        loop {
            while Self::read_reg(Self::VC_STATUS) & Self::MAILBOX_EMPTY != 0 {
                println!("ASDF");
            }
            let val = Self::read_reg(Self::VC_READ);
            if val & 0xF == 8 {
                return val & !0xF;
            }
        }
    }

    unsafe fn read_reg(offset: usize) -> u32 {
        let reg = (Self::VC_BASE as *mut u32).byte_add(offset);
        read_volatile(reg)
    }

    unsafe fn write_reg(offset: usize, value: u32) {
        let reg = (Self::VC_BASE as *mut u32).byte_add(offset);
        write_volatile(reg, value);
    }
}
