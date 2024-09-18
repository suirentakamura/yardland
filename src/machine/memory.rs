use std::mem::MaybeUninit;

use bevy::prelude::*;

pub const MEMORY_SIZE: usize = u32::MAX as usize / 4;

#[derive(Component)]
struct PhysicalMemory(Box<[u8; MEMORY_SIZE]>);

impl Default for PhysicalMemory {
    fn default() -> Self {
        Self(unsafe { Box::new_uninit().assume_init() })
    }
}

impl PhysicalMemory {
    /// Reads a u8 from the memory.
    fn read_u8(&self, index: usize) -> u8 {
        self.0[index]
    }

    /// Writes a u8 to the memory.
    fn write_u8(&mut self, index: usize, value: u8) {
        self.0[index] = value;
    }

    /// Reads a slice of bytes from the memory.
    fn read_slice_u8(&self, index: usize, size: usize) -> &[u8] {
        &self.0[index..index + size]
    }

    /// Writes a slice of bytes to the memory.
    fn write_slice_u8(&mut self, index: usize, slice: &[u8]) {
        self.0[index..index + slice.len()].copy_from_slice(slice);
    }

    /// Reads a slice of a given type from the memory.
    ///
    /// # Beware
    ///
    /// This function discards the remaining bytes if the slice is not aligned.
    fn read_slice<T: From<u8>>(&self, index: usize, size: usize) -> &[T] {
        let (_, m, _) = unsafe {
            self.0[index..index + size].align_to::<T>()
        };

        m
    }

    /// Writes a slice of a given type to the memory.
    fn write_slice<T: Into<u8>>(&mut self, index: usize, slice: &[T]) {
        let (_, m, _) = unsafe {
            slice.align_to::<u8>()
        };

        self.0[index..index + m.len()].copy_from_slice(m);
    }

    fn read<T: From<u8>>(&self, index: usize) -> T {
        let s = self.read_slice::<T>(index, std::mem::size_of::<T>());
        T::
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_read_write() {
        let mut memory = PhysicalMemory::default();

        memory.write(0, 0x12);
        memory.write(1, 0x34);
        memory.write(2, 0x56);
        memory.write(3, 0x78);

        assert_eq!(memory.read::<u8>(0), 0x12);
        assert_eq!(memory.read::<u8>(1), 0x34);
        assert_eq!(memory.read::<u8>(2), 0x56);
        assert_eq!(memory.read::<u8>(3), 0x78);
    }

    #[test]
    fn test_memory_read_write_slice() {
        let mut memory = PhysicalMemory::default();

        memory.write_slice_u8(0, &[0x12, 0x34, 0x56, 0x78]);

        assert_eq!(memory.read_slice::<u32>(0, 4), &[0x78563412]);
    }

    #[test]
    fn test_memory_read_write_slice_unaligned() {
        let mut memory = PhysicalMemory::default();

        memory.write_slice_u8(0, &[0x12, 0x34, 0x56, 0x78, 0x9A]);

        assert_eq!(memory.read_slice::<u32>(0, 5), &[0x78563412]);
    }
}

/*#![allow(dead_code)]

#[cfg(test)]
mod tests;

use std::sync::RwLock;
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
