use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// `BasicIo` is a trait that defines the basic read and write operations for a
/// device in the 64-bit bus.
pub trait BasicIo {
    /// Returns the length of the memory buffer in bytes.
    fn len(&self) -> usize;

    /// Reads a u64 from the memory buffer at the specified index.
    fn read_u64(&self, index: usize) -> u64;

    fn read_u8(&self, index: usize) -> u8 {
        self.read_u64(index) as u8
    }
    fn read_u16(&self, index: usize) -> u16 {
        self.read_u64(index) as u16
    }
    fn read_u32(&self, index: usize) -> u32 {
        self.read_u64(index) as u32
    }

    /// Writes a u64 to the memory buffer at the specified index.
    fn write_u64(&self, index: usize, data: u64);

    fn write_u8(&self, index: usize, data: u8) {
        self.write_u64(index, data as u64);
    }
    fn write_u16(&self, index: usize, data: u16) {
        self.write_u64(index, data as u64);
    }
    fn write_u32(&self, index: usize, data: u32) {
        self.write_u64(index, data as u64);
    }

    fn copy_into_slice(&self, index: usize, size: usize, dest: &mut [u8]);
}

#[derive(Clone)]
pub struct Memory {
    buffer: Arc<RwLock<Vec<u64>>>,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        Self::new_from_ref(Arc::new(RwLock::new(vec![0u64; size / 8])))
    }

    pub fn new_from_ref(buffer: Arc<RwLock<Vec<u64>>>) -> Self {
        Memory {
            buffer
        }
    }
}

impl BasicIo for Memory {
    fn len(&self) -> usize {
        self.buffer.read().unwrap().len()
    }

    fn read_u64(&self, index: usize) -> u64 {
        let buffer = self.buffer.read().unwrap();
        buffer[index]
    }

    fn write_u64(&self, index: usize, data: u64) {
        let mut buffer = self.buffer.write().unwrap();
        buffer[index] = data;
    }

    fn write_u8(&self, index: usize, data: u8) {
        let mut buffer = self.buffer.write().unwrap();
        buffer[index / 8] = (buffer[index / 8] & !(0xFF << (index % 8))) | ((data as u64) << (index % 8));
    }

    fn write_u16(&self, index: usize, data: u16) {
        let mut buffer = self.buffer.write().unwrap();
        buffer[index / 8] = (buffer[index / 8] & !(0xFFFF << (index % 8))) | ((data as u64) << (index % 8));
    }

    fn write_u32(&self, index: usize, data: u32) {
        let mut buffer = self.buffer.write().unwrap();
        buffer[index / 8] = (buffer[index / 8] & !(0xFFFFFFFF << (index % 8))) | ((data as u64) << (index % 8));
    }

    fn copy_into_slice(&self, index: usize, size: usize, dest: &mut [u8]) {
        let buffer = self.buffer.read().unwrap();
        dest.copy_from_slice(bytemuck::cast_slice(&buffer[index..index + size]));
    }
}

/*
#![allow(dead_code)]

#[cfg(test)]
mod tests;

use std::sync::RwLock;
use std::collections::HashMap;
use once_cell::sync::Lazy;

pub const MEMORY_SIZE: u32 = u32::MAX / 16;

static mut BUFFER: RwLock<[u8; MEMORY_SIZE as usize]> = RwLock::new([0u8; MEMORY_SIZE as usize]);
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
