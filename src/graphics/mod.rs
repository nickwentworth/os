pub mod color;
pub mod framebuffer;
pub mod graphics;
pub mod window;

use crate::{
    devices::raspi::videocore::{PixelOrder, VideoCore},
    graphics::{color::Color, framebuffer::FrameBuffer, graphics::Graphics, window::Window},
    println,
    util::rect::Rect,
};

pub fn init_graphics() {
    println!("Initializing graphics...");

    println!(
        "VideoCore firmware version: 0x{:x}",
        VideoCore::get_firmware_version()
    );

    VideoCore::set_depth(32);
    VideoCore::set_pixel_order(PixelOrder::RGB);
    // VideoCore::set_display_dimensions(1920, 1080);

    let (width, height) = VideoCore::get_display_dimensions();
    let depth = VideoCore::get_depth();

    println!("Allocating {width} x {height} pixel framebuffer, {depth} bits per pixel");

    assert_eq!(VideoCore::get_pixel_order(), PixelOrder::RGB);

    let (buf_base, buf_len) = VideoCore::allocate_frame_buffer();

    assert_eq!(
        buf_len,
        (width * height * depth / 8) as usize,
        "Allocated frame buffer is not the expected size!"
    );

    println!("Graphics initialized!");

    let framebuffer = FrameBuffer::new(buf_base, width as usize, height as usize, depth as usize);

    let mut graphics = Graphics::new(framebuffer);
    graphics.register_window(Window::new(Rect::new(0, 0, 200, 200), Color::RED));
    graphics.register_window(Window::new(Rect::new(100, 100, 300, 200), Color::GREEN));
    graphics.register_window(Window::new(Rect::new(100, 50, 300, 150), Color::BLUE));

    graphics.draw_frame();

    // for r in 0..255 {
    //     for g in 0..255 {
    //         for b in 0..255 {
    //             let c = Color::rgb(r, g, b);
    //             println!("{}", c);
    //             framebuffer.fill(c);
    //         }
    //     }
    // }
}
