mod memory;
mod processor;

use std::thread;
use sdl2::{
    render::TextureAccess,
    pixels::PixelFormatEnum
};

pub fn main() {
    let bin = std::fs::read("assets/test/rom.bin").unwrap();
    memory::dma_moveb_in(bin.as_slice(), 0xF000);
    let bin = std::fs::read("assets/test.bgr").unwrap();
    memory::dma_moveb_in(bin.as_slice(), 0x10000);

    let sdl = sdl2::init().unwrap();
    let sdl_video = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let window = sdl_video.window("Yardland", 1024, 720)
        .position_centered()
        .vulkan()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().accelerated().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture(Some(PixelFormatEnum::RGB565), TextureAccess::Streaming, 1024, 720).unwrap();

    let trace = !std::env::args().nth(1).unwrap_or(String::from("F")).eq("T");

    thread::spawn(move || processor::processor_func(trace));

    'main: loop {
        for event in event_pump.poll_iter() {
            use sdl2::event::Event;

            match event {
                Event::Quit {..} => {
                    break 'main
                }
                _ => {}
            }
        }

        canvas.clear();

        texture.with_lock(None, |pixels: &mut [u8], pitch| {
            memory::dma_moveb_out_r(pixels, 0xA0000, pitch * 720);
        }).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
    }
}
