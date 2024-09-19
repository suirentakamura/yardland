#![feature(ascii_char)]
#![feature(new_uninit)]
#![feature(min_specialization)]
#![feature(core_intrinsics)]

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod machine;

#[derive(Resource, Debug)]
struct AppSettings {
    trace: bool,
    input_file: std::path::PathBuf
}

pub fn main() {
    // let trace = !std::env::args().nth(1).unwrap_or(String::from("F")).eq("T");
    // let input_file = std::path::PathBuf::from(&std::env::args().nth(2).unwrap());

    App::new()
        .add_plugins(DefaultPlugins
            .build()
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Yardland".into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(EguiPlugin)
        // .insert_resource(AppSettings { trace, input_file })
        .add_systems(Update, ui_example_system)
        .run();

    /*

    let mut mmu = MmuDevice::new();
    let machine = Machine::new(&mut mmu);

    let mut ram_slice = [0u8; 0x10000];
    let ram_size = ram_slice.len() as u64;
    let mut ram = RamDevice::new(&mut ram_slice);
    machine.mmu.map_device(0, ram_size, &mut ram);

    let serial = SerialDevice::new();
    let mut running_serial = serial.start();
    machine.mmu.map_device(0x10000, SERIAL_REG_PIPE2, &mut running_serial);

    machine.mmu.write_byte(0x10000 + SERIAL_REG_CONTROL, SERIAL_CONTROL_PIPE1_ENABLE).unwrap();

    let string = format!("{:?}\n", machine);

    for (i, c) in string.as_ascii().unwrap().iter().enumerate() {
        machine.mmu.write_byte(i as u64, c.to_u8()).unwrap();
        machine.mmu.write_byte(0x10000 + SERIAL_REG_PIPE1, c.to_u8()).unwrap();
    }

    //println!("{}", string);

    loop {
        let status = machine.mmu.read_byte(0x10000 + SERIAL_REG_STATUS).unwrap();

        machine.mmu.write_byte(0x10000 + SERIAL_REG_PIPE1, b'A').unwrap();

        //if status & SERIAL_STATUS_PIPE1_RX != 0 {
        /*
            let c = machine.mmu.read_byte(0x10000 + SERIAL_REG_PIPE1).unwrap();
            machine.mmu.write_byte(0x10000 + SERIAL_REG_PIPE1, c).unwrap();
            */
        //}
    }

    // drop(running_serial);

    */

}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("World!");
    });
}
