use crate::{
    graphics::{color::Color, framebuffer::Drawable},
    util::rect::Rect,
};

pub struct Window {
    bounds: Rect,
    color: Color,
}

impl Window {
    pub fn new(bounds: Rect, color: Color) -> Self {
        Self { bounds, color }
    }

    pub fn bounds(&self) -> Rect {
        self.bounds
    }
}

impl Drawable for Window {
    fn draw(&self, buf: &mut super::framebuffer::FrameBufferView) {
        buf.fill(self.color);
    }
}
