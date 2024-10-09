use crate::machine::memory::PhysicalMemory;
use bevy::prelude::*;
use bevy_pixels::prelude::*;

pub const VRAM_BASE: usize = 0xA00_0000;
pub const VRAM_SIZE: usize = 614_400;

pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Draw, copy_fb);
    }
}

/// Draws the framebuffer
fn copy_fb(memory: Res<PhysicalMemory>, mut wrapper_query: Query<&mut PixelsWrapper>) {
    if memory.can_read() {
        // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
        let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

        // Get a mutable slice for the pixel buffer.
        let frame = wrapper.pixels.frame_mut();

        memory.read_slice_u8(VRAM_BASE, VRAM_SIZE, frame);
    }
}

/*
use sdl2::{
    render::TextureAccess,
    pixels::PixelFormatEnum
};

pub fn video() {
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

        texture.with_lock(None, |pixels: &mut [u8], _| {
            // memory::dma_moveb_out_r(pixels, 0xA0000, pitch * 720);

            let (_, vram) = machine.mmu.decode_address_mut(0xA0000).unwrap();
            vram.device.read_stream(0, pixels).unwrap();
        }).unwrap();

        canvas.copy(&texture, None, None).unwrap();

        canvas.present();
    }
}
*/
