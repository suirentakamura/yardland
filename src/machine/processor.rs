use std::ffi::{c_char, CStr};
use bevy::prelude::*;

pub struct ProcessorPlugin;

impl Plugin for ProcessorPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TestProcessor>()
            .add_systems(Update, test_processor_system);
    }
}

#[derive(Resource, Default)]
pub struct TestProcessor {
    pub frequency: f32,
}

fn test_processor_system(mut processor: ResMut<TestProcessor>, time: Res<Time>) {
    processor.frequency = 1.0 / time.delta_seconds();
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ProcessorStatus {
    pub cycles: u32,
    pub stopped: bool,
    pub flags: u8,
    pub pc: u64,
    pub sp: u64,
    pub tp: u64,
    pub dp: u64,
    pub a: u64,
    pub b: u64,
    pub c: u64,
    pub x: u64,
    pub y: u64,
    pub z: u64
}

#[repr(C)]
struct FfiProcessorInstruction {
    mnem: *const c_char,
    am: *const c_char
}

#[derive(Clone, Debug)]
pub struct ProcessorInstruction {
    pub mnemonic: String,
    pub addressing_mode: String
}

impl From<FfiProcessorInstruction> for ProcessorInstruction {
    fn from(ffi: FfiProcessorInstruction) -> Self {
        let mnemonic = unsafe {
            CStr::from_ptr(ffi.mnem).to_string_lossy().into_owned()
        };

        let addressing_mode = unsafe {
            CStr::from_ptr(ffi.am).to_string_lossy().into_owned()
        };

        ProcessorInstruction {
            mnemonic,
            addressing_mode
        }
    }
}
