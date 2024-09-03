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
