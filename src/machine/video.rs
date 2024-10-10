use crate::machine::memory::PhysicalMemory;
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, render_resource::{
        Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
    }},
};
// use bevy_pixels::prelude::*;

pub const VRAM_BASE: usize = 0xA000000;
pub const VRAM_SIZE: usize = 1_228_800; // VRAM ends at 0xA12C000

pub struct VideoPlugin;

impl Plugin for VideoPlugin {
    fn build(&self, app: &mut App) {
        app
        // .add_systems(Draw, copy_fb);
            .insert_resource(DrawFbTimer(
                Timer::from_seconds(1.0 / 60.0, TimerMode::Repeating)
            ))
            .add_systems(Startup, (setup_camera, setup_fb_texture))
            .add_systems(PostUpdate, draw_fb);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed {
                width: 640.0,
                height: 480.0,
            },
            ..default()
        },
        ..default()
    });
}

fn setup_fb_texture(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 640,
        height: 480,
        ..default()
    };

    let mut fb_image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("Yardland 640x480 Framebuffer"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Uint,
            usage: TextureUsages::all(),
            view_formats: &[],
        },
        ..default()
    };

    fb_image.resize(size);

    commands.spawn(SpriteBundle {
        texture: images.add(fb_image),
        ..default()
    });
}

#[derive(Resource)]
struct DrawFbTimer(Timer);

/// Draws the framebuffer
fn draw_fb(
    mut images: ResMut<Assets<Image>>,
    mut timer: ResMut<DrawFbTimer>,
    time: Res<Time>,
    memory: Res<PhysicalMemory>,
    fb_handle_query: Query<&Handle<Image>, With<Sprite>>
) {
    timer.0.tick(time.delta());

    if timer.0.just_finished() && memory.can_read() {
        let fb_handle = fb_handle_query.get_single().unwrap();
        let fb_image = images.get_mut(fb_handle).unwrap();

        assert_eq!(fb_image.data.len(), VRAM_SIZE, "Framebuffer image data size mismatch!");
        memory.copy_to_slice(VRAM_BASE, VRAM_SIZE, fb_image.data.as_mut_slice());
    }
}

/*
fn draw_fb(memory: Res<PhysicalMemory>, mut wrapper_query: Query<&mut PixelsWrapper>) {
    if memory.can_read() {
        // Query the `PixelsWrapper` component that owns an instance of `Pixels` for the given window.
        let Ok(mut wrapper) = wrapper_query.get_single_mut() else { return };

        // Get a mutable slice for the pixel buffer.
        let frame = wrapper.pixels.frame_mut();

        memory.read_slice_u8(VRAM_BASE, VRAM_SIZE, frame);
    }
}
*/

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
