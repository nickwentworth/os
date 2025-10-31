use crate::graphics::{
    framebuffer::{Drawable, FrameBuffer},
    window::Window,
};
use alloc::vec::Vec;

pub struct Graphics {
    windows: Vec<Window>,
    framebuffer: FrameBuffer,
}

impl Graphics {
    pub fn new(framebuffer: FrameBuffer) -> Self {
        Self {
            windows: Vec::new(),
            framebuffer,
        }
    }

    pub fn register_window(&mut self, window: Window) {
        self.windows.push(window);
    }

    pub fn draw_frame(&mut self) {
        for window in self.windows.iter_mut().rev() {
            let mut view = self.framebuffer.view(window.bounds());
            window.draw(&mut view);
        }
    }
}
