use crate::AppSettings;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use egui_extras::{TableBuilder, Column};
use rfd::FileDialog;

pub struct MenusPlugin;

impl Plugin for MenusPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            settings_window_system,
            info_window_system,
        ));
    }
}

fn settings_window_system(mut contexts: EguiContexts, mut settings: ResMut<AppSettings>) {
    if let Some(ctx_mut) = contexts.try_ctx_mut() {
        egui::Window::new("Settings").show(ctx_mut, |ui| {
            ui.label("Hello World!");

            ui.checkbox(&mut settings.running, "Running?");
            ui.checkbox(&mut settings.trace, "Trace?");

            ui.horizontal(|ui| {
                if ui.button("Open input file...").clicked() {
                    settings.input_file = FileDialog::new()
                        .pick_file();
                }

                if let Some(input_file) = &settings.input_file {
                    ui.label(format!("Selected input file: {:#?}", input_file));
                }
            });
        });
    }
}

fn info_window_system(mut contexts: EguiContexts, settings: Res<AppSettings>) {
    if let Some(ctx_mut) = contexts.try_ctx_mut() {
        egui::Window::new("Info").show(ctx_mut, |ui| {
            ui.collapsing("CPU", |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Reset").clicked() {
                        // Reset
                    }

                    if ui.add_enabled(!settings.running, egui::Button::new("Step")).clicked() {
                        // Step
                    }
                });

                ui.label("Frequency: 1.0 MHz");

                ui.separator();

                ui.collapsing("Registers", |ui| {
                    ui.label("A: 0x00000000");
                    ui.label("B: 0x00000000");
                    ui.label("C: 0x00000000");
                    ui.label("X: 0x00000000");
                    ui.label("Y: 0x00000000");
                    ui.label("Z: 0x00000000");
                    ui.label("PC: 0x00000000");
                    ui.label("SP: 0x00000000");
                    ui.label("TP: 0x00000000");
                });
            });

            ui.collapsing("GPU", |ui| {
                ui.label("Type: VGA-compatible dumb framebuffer");
                ui.label("Resolution: 640x480");
                ui.label("Color depth: 16-bpp color");

                use crate::machine::video::{VRAM_BASE, VRAM_SIZE};

                ui.label(format!("VRAM Range: 0x{:016X} - 0x{:016X}", VRAM_BASE, VRAM_BASE + VRAM_SIZE));
                ui.label(format!("VRAM Size: {} bytes", VRAM_SIZE));
            });

            ui.collapsing("Memory", |ui| {
                /*
                let buffer_example = [
                    0xDEu8, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    0xDE, 0xAD, 0xBE, 0xEF, 0xDE, 0xAD, 0xBE, 0xEF,
                    b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W',
                    b'o', b'r', b'l', b'd', b'!', 0, 0, 0
                ];
                */

                let buffer_example = unsafe { Box::<[u8; u16::MAX as usize]>::new_uninit().assume_init() };

                memory_map(ui, buffer_example.as_ref());
            });
        });
    }
}

/// Memory map widget
fn memory_map(ui: &mut egui::Ui, memory: &[u8]) {
    let mut table = TableBuilder::new(ui);

    /*
    ui.horizontal(|ui| {
        ui.label("Goto address: ");

        let mut goto = String::new();
        ui.text_edit_singleline(&mut goto);

        if ui.button("Go").clicked() {
            table = table.scroll_to_row(usize::from_str_radix(&goto, 16).unwrap() / 16, Some(egui::Align::Min));
        }
    });

    ui.separator();
    */

    // Table begins here

    table = table.striped(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::auto())
        .columns(Column::auto(), 16)
        .column(Column::auto());

    table
        .header(20., |mut header| {
            header.col(|ui| {
                ui.strong("Offset");
            });

            for i in 0..16 {
                header.col(|ui| {
                    ui.strong(format!("{:02X}", i));
                });
            }

            header.col(|ui| {
                ui.strong("Decoded text");
            });
        })
        .body(|body| {
            let chunks = memory.chunks(16usize).collect::<Vec<&[u8]>>();

            body.rows(18., chunks.len(), |mut row| {
                let i = row.index();
                let bytes = chunks[i];

                row.col(|ui| {
                    ui.monospace(format!("{:016X}", i * 16));
                });

                for j in 0..16usize {
                    if j < bytes.len() {
                        row.col(|ui| {
                            ui.monospace(format!("{:02X}", bytes[j]));
                        });
                    } else {
                        row.col(|ui| {
                            ui.monospace("");
                        });
                    }
                }

                row.col(|ui| {
                    ui.monospace(format!("{}",
                        String::from_utf8(bytes.iter().map(|b| {
                            if b.is_ascii() && !b.is_ascii_control() { *b } else { b'.' }
                        }).collect()).unwrap()
                    ));
                });
            });
        });
}
