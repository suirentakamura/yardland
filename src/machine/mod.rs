pub mod memory;
pub mod mmu;

pub use mmu::MmuDevice;

use bevy::prelude::*;

pub trait Device {
    fn read_byte(&self, address: u64) -> Result<u8, &'static str>;
    fn write_byte(&mut self, address: u64, data: u8) -> Result<(), &'static str>;

    fn read_stream(&self, address: u64, stream: &mut [u8]) -> Result<(), &'static str>;
    fn write_stream(&mut self, address: u64, stream: &[u8]) -> Result<(), &'static str>;

    fn read_word(&self, address: u64) -> Result<u16, &'static str> {
        let mut stream = [0u8; 2];
        self.read_stream(address, &mut stream)?;
        Ok(u16::from_le_bytes(stream))
    }

    fn read_dword(&self, address: u64) -> Result<u32, &'static str> {
        let mut stream = [0u8; 4];
        self.read_stream(address, &mut stream)?;
        Ok(u32::from_le_bytes(stream))
    }

    fn read_qword(&self, address: u64) -> Result<u64, &'static str> {
        let mut stream = [0u8; 8];
        self.read_stream(address, &mut stream)?;
        Ok(u64::from_le_bytes(stream))
    }

    fn write_word(&mut self, address: u64, data: u16) -> Result<(), &'static str> {
        self.write_stream(address, &data.to_le_bytes())
    }

    fn write_dword(&mut self, address: u64, data: u32) -> Result<(), &'static str> {
        self.write_stream(address, &data.to_le_bytes())
    }

    fn write_qword(&mut self, address: u64, data: u64) -> Result<(), &'static str> {
        self.write_stream(address, &data.to_le_bytes())
    }

    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}

pub struct RamDevice<'a> {
    ram: &'a mut [u8],
}

impl<'a> RamDevice<'a> {
    pub fn new(ram: &'a mut [u8]) -> Self {
        RamDevice { ram }
    }

    fn check_address(&self, address: u64, length: usize) -> Result<(), &'static str> {
        if (address as usize) + length > self.ram.len() {
            Err("Address out of bounds")
        } else {
            Ok(())
        }
    }
}

impl<'a> Device for RamDevice<'a> {
    fn read_byte(&self, address: u64) -> Result<u8, &'static str> {
        self.check_address(address, 1)?;
        Ok(self.ram[address as usize])
    }

    fn write_byte(&mut self, address: u64, data: u8) -> Result<(), &'static str> {
        self.check_address(address, 1)?;
        self.ram[address as usize] = data;
        Ok(())
    }

    fn read_stream(&self, address: u64, stream: &mut [u8]) -> Result<(), &'static str> {
        let length = stream.len();
        self.check_address(address, length)?;
        stream.copy_from_slice(&self.ram[address as usize..address as usize + length]);
        Ok(())
    }

    fn write_stream(&mut self, address: u64, stream: &[u8]) -> Result<(), &'static str> {
        let length = stream.len();
        self.check_address(address, length)?;
        self.ram[address as usize..address as usize + length].copy_from_slice(stream);
        Ok(())
    }
}

fn machine_setup_system(mut commands: Commands) {

}

fn machine_system() {

}

pub struct MachinePlugin;

impl Plugin for MachinePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, machine_setup_system)
            .add_systems(Update, machine_system);
    }
}
