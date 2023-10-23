#![allow(dead_code)]

//! [emu816](https://github.com/andrew-jacobs/emu816) rust port.

use num_enum::TryFromPrimitive;

#[link(name = "emu816")]
extern "C" {
    fn emu816_reset(trace: bool);
    fn emu816_step();
    fn emu816_getCycles() -> u32;
    fn emu816_isStopped() -> bool;
    fn emu816_resume();
    fn emu816_getStopReason() -> isize;
    fn emu816_interrupt();
    fn emu816_getCopInstSize() -> u8;
    fn emu816_getCopInst(inst: *mut u16) -> u8;
}

// Memory access functions

#[no_mangle]
pub extern "C" fn readb(addr: u32) -> u8 {
    use crate::memory::readb;

    readb(addr)
}

#[no_mangle]
pub extern "C" fn writeb(addr: u32, byte: u8) {
    use crate::memory::writeb;

    writeb(addr, byte);
}

#[derive(TryFromPrimitive)]
#[repr(isize)]
pub enum StopReason {
    Coprocessor = 1,
    WaitInterrupt,
    Stop
}

#[derive(TryFromPrimitive)]
#[repr(u8)]
pub enum CoprocessorOpcode {
    MmuMapBanks,
    MmuDmaTransferBVR,
    MmuDmaTransferBV,
    MmuDmaTransferBR,
}

pub struct CoprocessorInst {
    pub opcode: CoprocessorOpcode,
    pub args: Vec<u16>
}

/// Resets the CPU.
/// 
/// `trace`: If true, emulator traces debug information to console.
pub fn reset(trace: bool) {
    unsafe {
        emu816_reset(trace);
    }
}

/// Step through one cycle.
pub fn step() {
    unsafe {
        emu816_step();
    }
}

/// Get cycles.
pub fn get_cycles() -> u32 {
    unsafe {
        emu816_getCycles()
    }
}

/// Returns true if the CPU has halted execution.
pub fn is_stopped() -> bool {
    unsafe {
        emu816_isStopped()
    }
}

/// Resumes execution.
pub fn resume() {
    unsafe {
        emu816_resume();
    }
}

/// Returns the reason the CPU has halted execution.
pub fn get_stop_reason() -> Option<StopReason> {
    unsafe {
        StopReason::try_from(emu816_getStopReason()).ok()
    }
}

/// Sends an IRQ to the CPU.
pub fn interrupt() {
    unsafe {
        emu816_interrupt();
    }
}

/// Fetches the coprocessor id and arguments from the last COP instruction.
pub fn get_coprocessor_inst() -> Option<CoprocessorInst> {
    unsafe {
        let size = emu816_getCopInstSize() as usize;

        let mut inst: Vec<u16> = Vec::with_capacity(size);
        let opcode = CoprocessorOpcode::try_from(emu816_getCopInst(inst.as_mut_ptr())).unwrap();
        inst.set_len(size);

        Some(
            CoprocessorInst {
                opcode, 
                args: inst.clone()
            }
        )
    }
}
