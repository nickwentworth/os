use crate::{
    graphics::{color::Color, framebuffer},
    util::rect::Rect,
};

pub trait Drawable {
    fn draw(&self, buf: &mut FrameBufferView);
}

pub struct FrameBuffer {
    base_addr: *mut u8,
    width: usize,
    height: usize,
    color_mode: ColorMode,
}

enum ColorMode {
    Rgb32,
}

impl FrameBuffer {
    pub fn new(base_addr: usize, width: usize, height: usize, depth: usize) -> Self {
        Self {
            base_addr: base_addr as *mut u8,
            width,
            height,
            color_mode: match depth {
                32 => ColorMode::Rgb32,
                _ => panic!("Depth value {} not handled!", depth),
            },
        }
    }

    pub fn view(&'_ self, bounds: Rect) -> FrameBufferView<'_> {
        FrameBufferView {
            framebuffer: &self,
            bounds: bounds,
        }
    }
}

pub struct FrameBufferView<'a> {
    framebuffer: &'a FrameBuffer,
    bounds: Rect,
}

impl FrameBufferView<'_> {
    fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        let idx = y * self.framebuffer.width + x;

        match self.framebuffer.color_mode {
            ColorMode::Rgb32 => unsafe {
                self.framebuffer
                    .base_addr
                    .cast::<u32>()
                    .add(idx)
                    .write(color.to_u32_be());
            },
        }
    }

    pub fn fill(&mut self, color: Color) {
        for (x, y) in self.bounds.points() {
            self.set_pixel(x, y, color);
        }
    }
}
