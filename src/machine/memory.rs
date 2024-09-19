use std::{
    any::Any,
    sync::RwLock,
    intrinsics::transmute_unchecked,
    mem::{size_of, transmute}
};

use bevy::prelude::*;
use bevy_egui::egui::util::id_type_map::TypeId;
use desert::{FromBytesLE, ToBytesLE};

pub struct MemoryPlugin;

impl Plugin for MemoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<PhysicalMemory>();
    }
}

pub const MEMORY_SIZE: usize = u16::MAX as usize;

#[derive(Resource)]
pub struct PhysicalMemory(RwLock<Box<[u8; MEMORY_SIZE]>>);

impl Default for PhysicalMemory {
    fn default() -> Self {
        Self(RwLock::new(
            unsafe { Box::new_uninit().assume_init() }
        ))
    }
}

impl PhysicalMemory {
    /// Reads a u8 from the memory.
    fn read_u8(&self, index: usize) -> u8 {
        self.0.read().unwrap()[index]
    }

    /// Writes a u8 to the memory.
    fn write_u8(&mut self, index: usize, value: u8) {
        self.0.write().unwrap()[index] = value;
    }

    /// Reads a slice of bytes from the memory.
    fn read_slice_u8(&self, index: usize, size: usize, slice: &mut [u8]) {
        let lock = self.0.read().unwrap();
        slice.copy_from_slice(&lock[index..index + size]);
    }

    /// Writes a slice of bytes to the memory.
    fn write_slice_u8(&mut self, index: usize, slice: &[u8]) {
        let mut lock = self.0.write().unwrap();
        lock[index..index + slice.len()].copy_from_slice(slice);
    }

    /// Reads a slice of a given type from the memory.
    ///
    /// # Beware
    ///
    /// This function discards the remaining bytes if the slice is not aligned.
    pub fn read_slice<T: Any>(&self, index: usize, size: usize, slice: &mut [T]) {
        unsafe {
            self.read_slice_u8(index, size, transmute(slice))
        }
    }

    /// Writes a slice of a given type to the memory.
    pub fn write_slice<T: Any>(&mut self, index: usize, slice: &[T]) {
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            return unsafe {
                self.write_slice_u8(index,  transmute(slice))
            }
        }

        let (_, m, _) = unsafe {
            slice.align_to::<u8>()
        };

        self.write_slice_u8(index, m);
    }

    /// Reads a value of a given type from the memory.
    pub fn read<T: FromBytesLE + Any>(&self, index: usize) -> Result<T, desert::Error> {
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            return Ok(unsafe {
                transmute_unchecked::<u8, T>(self.read_u8(index))
            });
        }

        let mut v: Vec<u8> = Vec::with_capacity(std::mem::size_of::<T>() + 69);
        self.read_slice_u8(index, std::mem::size_of::<T>(), v.as_mut_slice());
        T::from_bytes_le(v.as_slice()).map(|(_, t)| t)
    }

    /// Writes a value of a given type to the memory.
    pub fn write<T: ToBytesLE + Any>(&mut self, index: usize, value: T) -> Result<(), desert::Error> {
        if TypeId::of::<u8>() == TypeId::of::<T>() {
            self.write_u8(index, unsafe {
                transmute_unchecked::<T, u8>(value)
            });
            return Ok(());
        }

        let s = value.to_bytes_le()?;
        self.write_slice_u8(index, s.as_slice());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read_write() {
        let mut memory = PhysicalMemory::default();

        memory.write_u8(0, 0x12);
        memory.write_u8(1, 0x34);
        memory.write_u8(2, 0x56);
        memory.write_u8(3, 0x78);

        assert_eq!(memory.read_u8(0), 0x12);
        assert_eq!(memory.read_u8(1), 0x34);
        assert_eq!(memory.read_u8(2), 0x56);
        assert_eq!(memory.read_u8(3), 0x78);

        assert_eq!(memory.read::<u8>(0).unwrap(), 0x12);
        assert_eq!(memory.read::<u8>(1).unwrap(), 0x34);
        assert_eq!(memory.read::<u8>(2).unwrap(), 0x56);
        assert_eq!(memory.read::<u8>(3).unwrap(), 0x78);
    }

    #[test]
    fn test_memory_read_write_slice() {
        let mut memory = PhysicalMemory::default();

        memory.write_slice_u8(0, &[0x12, 0x34, 0x56, 0x78]);

        assert_eq!(memory.read::<u32>(0).unwrap(), 0x78563412);
    }

    #[test]
    fn test_memory_read_write_slice_unaligned() {
        let mut memory = PhysicalMemory::default();

        memory.write_slice_u8(0, &[0x12, 0x34, 0x56, 0x78, 0x9A]);

        assert_eq!(memory.read::<u32>(0).unwrap(), 0x78563412);
    }

    #[test]
    fn test_memory_read_write_slice_type() {
        let mut memory = PhysicalMemory::default();

        memory.write_slice(0, "Hello, World!".as_bytes());

        let mut v: Vec<u8> = Vec::with_capacity(13);
        memory.read_slice(0, 13, v.as_mut_slice());

        assert_eq!(v.as_slice(), "Hello, World!".as_bytes());
    }

    #[test]
    fn test_memory_read_write_type() {
        let mut memory = PhysicalMemory::default();

        memory.write(0, 0x78563412u32).unwrap();

        assert_eq!(memory.read::<u32>(0).unwrap(), 0x78563412);
    }
}

/*#![allow(dead_code)]

#[cfg(test)]
mod tests;


use std::collections::HashMap;
use once_cell::sync::Lazy;


static mut BUFFER: RwLock<[u8; MEMORY_SIZE]> = RwLock::new([0u8; MEMORY_SIZE]);
static mut BANK_TABLE: Lazy<RwLock<HashMap<u8, u16>>> = Lazy::new(|| {
    let mut bank_table = HashMap::new();
    for i in 0..u8::MAX {
        bank_table.insert(i, i as u16);
    }
    RwLock::new(bank_table)
});

macro_rules! buffer {
    [$addr:expr] => {{
        let buffer = BUFFER.read().unwrap();
        buffer[$addr]
    }};
    ($addr:expr, $byte:expr) => {{
        let mut buffer = BUFFER.write().unwrap();
        buffer[$addr] = $byte;
    }};
}

macro_rules! bank_table {
    [$bank:expr] => {{
        let bank_table = BANK_TABLE.read().unwrap();
        bank_table[$bank]
    }};
    ($virt:expr, $real:expr) => {{
        let mut bank_table = BANK_TABLE.write().unwrap();
        bank_table.insert($virt, $real);
    }};
}

macro_rules! real_addr {
    ($virt:expr) => {{
        let virt_bank = ($virt >> 16) as u8;
        let real_bank = bank_table![&virt_bank];
        let mut real_addr = (real_bank as u32) << 16;
        real_addr |= ($virt & u16::MAX as u32) as u32;
        real_addr as usize
    }};
}

pub fn map_bank(virt: u8, real: u16) {
    println!("MMU MAP BANK: VIRT {{{:X}}} REAL {{{:X}}}", virt, real);

    unsafe {
        bank_table!(virt, real);
    }
}

pub fn readb(addr: u32) -> u8 {
    unsafe {
        buffer![real_addr!(addr)]
    }
}

pub fn writeb(addr: u32, byte: u8) {
    unsafe {
        buffer!(real_addr!(addr), byte);
    }
}

pub fn dma_moveb_out_v(dest: &mut [u8], start: u32, size: usize) {
    unsafe {
        let real_addr = real_addr!(start);
        let buffer = BUFFER.read().unwrap();
        dest.copy_from_slice(&buffer[real_addr..(real_addr + size)]);
    }
}

pub fn dma_moveb_out_r(dest: &mut [u8], start: u32, size: usize) {
    unsafe {
        let real_addr = start as usize;
        let buffer = BUFFER.read().unwrap();
        dest.copy_from_slice(&buffer[real_addr..(real_addr + size)]);
    }
}

pub fn dma_moveb_in(src: &[u8], start: u32) {
    unsafe {
        let real_addr = real_addr!(start);
        let mut buffer = BUFFER.write().unwrap();
        buffer[real_addr..(real_addr + src.len())].copy_from_slice(src);
    }
}

unsafe fn dma_transferb (src: usize, dest: usize, size: usize) {
    let src_buf = {
        let mut src_buf = Vec::with_capacity(size);
        let buffer = BUFFER.read().unwrap();
        src_buf.set_len(size);
        src_buf.as_mut_slice().copy_from_slice(&buffer[src..src + size]);
        src_buf
    };

    let mut buffer = BUFFER.write().unwrap();
    buffer[dest..dest + size].copy_from_slice(src_buf.as_slice());
}

pub fn dma_transferb_vr(src: u32, dest: u32, size: u32) {
    unsafe {
        let real_src = real_addr!(src);
        let real_dest = dest as usize;
        let real_size = size as usize;

        println!("DMA TRANSFER B VR: VSRC {{{:X}}} RSRC {{{:X}}} DEST {{{:X}}} SIZE {{{:X}}}", src, real_src, real_dest, real_size);

        dma_transferb(real_src, real_dest, real_size);
    }
}

pub fn dma_transferb_v(src: u32, dest: u32, size: u32) {
    unsafe {
        let real_src = real_addr!(src);
        let real_dest = real_addr!(dest);
        let real_size = size as usize;

        println!("DMA TRANSFER B V: VSRC {{{:X}}} RSRC {{{:X}}} VDEST {{{:X}}} RDEST {{{:X}}} SIZE {{{:X}}}", src, real_src, dest, real_dest, real_size);

        dma_transferb(real_src, real_dest, real_size);
    }
}

pub fn dma_transferb_r(src: u32, dest: u32, size: u32) {
    unsafe {
        let real_src = src as usize;
        let real_dest = dest as usize;
        let real_size = size as usize;

        println!("DMA TRANSFER B R: SRC {{{:X}}} DEST {{{:X}}} SIZE {{{:X}}}", real_src, real_dest, real_size);

        dma_transferb(real_src, real_dest, real_size);
    }
}
*/
