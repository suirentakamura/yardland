use bevy::prelude::*;
use super::Device;

pub struct MmuMapping<'a> {
    address: u64,
    size: u64,
    device: &'a mut dyn Device
}

impl<'a> std::fmt::Debug for MmuMapping<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MmuMapping")
            .field("address", &format_args!("{0:#x} ({0})", self.address))
            .field("size", &format_args!("{0:#x} ({0})", self.size))
            .field("device", &format_args!("{}", self.device.get_name()))
            .finish()
    }
}

#[derive(Component, Debug)]
pub struct MmuDevice<'a> {
    map: Vec<MmuMapping<'a>>
}

impl<'a> MmuDevice<'a> {
    pub fn new() -> Self {
        MmuDevice {
            map: Vec::new()
        }
    }

    pub fn map_device(&mut self, address: u64, size: u64, device: &'a mut dyn Device) {
        self.map.push(MmuMapping { address, size, device });
    }

    pub fn remap_device(&mut self, address: u64, new_address: u64) -> Result<(), &'static str> {
        if let Some(mapping) =
            self.map.iter_mut().find(|mapping| mapping.address == address)
        {
            mapping.address = new_address;
            Ok(())
        } else {
            Err("Could not find device")
        }
    }

    pub fn decode_address(&self, address: u64) -> Result<(u64, &MmuMapping<'a>), &'static str> {
        self.map.iter().find_map(|mapping| {
            if address >= mapping.address && address <= mapping.address + mapping.size {
                Some((address - mapping.address, mapping))
            } else {
                None
            }
        }).ok_or("Address not found")
    }

    pub fn decode_address_mut(&mut self, address: u64) -> Result<(u64, &mut MmuMapping<'a>), &'static str> {
        self.map.iter_mut().find_map(|mapping| {
            if address >= mapping.address && address <= mapping.address + mapping.size {
                Some((address - mapping.address, mapping))
            } else {
                None
            }
        }).ok_or("Address not found")
    }
}

impl<'a> Device for MmuDevice<'a> {
    fn read_byte(&self, address: u64) -> Result<u8, &'static str> {
        let (offset, mapping) = self.decode_address(address)?;
        mapping.device.read_byte(offset)
    }

    fn write_byte(&mut self, address: u64, data: u8) -> Result<(), &'static str> {
        let (offset, mapping) = self.decode_address_mut(address)?;
        mapping.device.write_byte(offset, data)
    }

    fn read_stream(&self, address: u64, stream: &mut [u8]) -> Result<(), &'static str> {
        let (offset, mapping) = self.decode_address(address)?;
        mapping.device.read_stream(offset, stream)
    }

    fn write_stream(&mut self, address: u64, stream: &[u8]) -> Result<(), &'static str> {
        let (offset, mapping) = self.decode_address_mut(address)?;
        mapping.device.write_stream(offset, stream)
    }
}
