mod memory;
mod devices;
// mod processor;

use std::{sync::{Arc, RwLock}, thread};
use sdl2::{
    render::TextureAccess,
    pixels::PixelFormatEnum
};

pub fn main() {
    let mut dman = devices::DeviceManager::new();

    let memory_ref = Arc::new(RwLock::new(vec![0u8; u32::MAX as usize / 4]));
    let memory = memory::Memory::new_from_ref(memory_ref.clone());

    // dman.add_device(0, Box::new(memory));

    let prom: devices::RomDevice = std::fs::read("assets/test/rom.bin").unwrap().into();
    let drom: devices::RomDevice = std::fs::read("assets/test.bgr").unwrap().into();

    dman.add_device(0x00000, Box::new(prom));
    dman.add_device(0xA0000, Box::new(drom));

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

    // thread::spawn(move || processor::processor_func(trace));

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
            let memory = memory::Memory::new_from_ref(memory_ref.clone());
            // memory::dma_moveb_out_r(pixels, 0xA0000, pitch * 720);

            memory.copy_into_slice(0xA0000, pitch * 720, pixels);
        }).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
    }
}
