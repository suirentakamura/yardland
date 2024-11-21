use std::{io::Read, sync::{Arc, Mutex}};

use log::warn;

use crate::memory::BasicIo;

pub struct DeviceDefinition {
    pub start: usize,
    pub io: Box<dyn BasicIo>,
}

pub struct DeviceManager {
    devices: Vec<DeviceDefinition>,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, start: usize, io: Box<dyn BasicIo>) {
        self.devices.push(DeviceDefinition {
            start,
            io,
        });
    }

    pub fn read_u8(&self, index: usize) -> u8 {
        for device in &self.devices {
            if index >= device.start && index < device.start + device.io.len() {
                return device.io.read_u8(index - device.start);
            }
        }

        0
    }

    pub fn write_u8(&self, index: usize, data: u8) {
        for device in &self.devices {
            if index >= device.start && index < device.start + device.io.len() {
                device.io.write_u8(index - device.start, data);
                return;
            }
        }
    }

    pub fn copy_into_slice(&self, index: usize, size: usize, dest: &mut [u8]) {
        //let mut size = size;

        for device in &self.devices {
            let device_len = device.io.len();

            if index >= device.start && index < device.start + device_len {
                //assert!(device.io.len() != size, "device io len mismatch {} != {}", device.io.len(), size);
                //assert!(dest.len() != size, "dest slice len mismatch {} != {}", dest.len(), size);

                let device_index = index - device.start;
                // size -= device_len - device_index;
                device.io.copy_into_slice(device_index, device_len, &mut dest[..device_len]);
                return;
            }
        }
    }
}

pub struct KeyboardDevice;

impl BasicIo for KeyboardDevice {
    fn len(&self) -> usize {
        2
    }

    fn read_u8(&self, index: usize) -> u8 {
        match index {
            0 => { // Data port
                0
            },
            1 => { // Status port
                0
            },
            _ => {
                0
            },
        }
    }

    fn write_u8(&self, index: usize, data: u8) {
        unimplemented!()
    }

    fn copy_into_slice(&self, index: usize, size: usize, dest: &mut [u8]) {
        unimplemented!()
    }
}

pub struct RomDevice(Arc<Mutex<Vec<u8>>>);

impl From<Vec<u8>> for RomDevice {
    fn from(data: Vec<u8>) -> Self {
        RomDevice(Arc::new(Mutex::new(data)))
    }
}

impl BasicIo for RomDevice {
    fn len(&self) -> usize {
        let data = self.0.lock().unwrap();
        data.len()
    }

    fn read_u8(&self, index: usize) -> u8 {
        let data = self.0.lock().unwrap();
        data[index]
    }

    fn write_u8(&self, index: usize, data: u8) {
    }

    fn copy_into_slice(&self, index: usize, size: usize, dest: &mut [u8]) {
        let buffer = self.0.lock().unwrap();
        dest.copy_from_slice(&buffer[index..index + size]);
    }
}
