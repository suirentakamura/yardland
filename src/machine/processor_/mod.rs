mod sys;

use sys::{reset, step, is_stopped, StopReason, CoprocessorOpcode, get_stop_reason, resume, interrupt, get_coprocessor_inst};

use crate::memory;

macro_rules! le_u16_as_u32 {
    ($lsb:expr, $msb:expr) => {
        (($msb as u32) << 16) + $lsb as u32
    };
}

pub struct Processor {
    trace: bool
}

pub fn processor_func(trace: bool) {
    reset(trace);

    loop {
        while !is_stopped() {
            step();
        }

        match get_stop_reason() {
            Some(StopReason::Coprocessor) => {
                if let Some(inst) = get_coprocessor_inst() {
                    match inst.opcode {
                        CoprocessorOpcode::MmuMapBanks => { // MMU MAP BANKS
                            for arg in inst.args.chunks_exact(2) {
                                memory::map_bank(arg[0] as u8, arg[1]);
                            }
                        },
                        CoprocessorOpcode::MmuDmaTransferBVR => { // MMU DMA TRANSFERB VR
                            assert_eq!(inst.args.len(), 6);

                            let src  = le_u16_as_u32!(inst.args[0], inst.args[1]);
                            let dest = ((inst.args[3] as u32) << 16) + inst.args[2] as u32;
                            let size = ((inst.args[5] as u32) << 16) + inst.args[4] as u32;

                            memory::dma_transferb_vr(src, dest, size);
                        },
                        CoprocessorOpcode::MmuDmaTransferBV => { // MMU DMA TRANSFERB V
                            assert_eq!(inst.args.len(), 6);

                            let src  = ((inst.args[1] as u32) << 16) + inst.args[0] as u32;
                            let dest = ((inst.args[3] as u32) << 16) + inst.args[2] as u32;
                            let size = ((inst.args[5] as u32) << 16) + inst.args[4] as u32;

                            memory::dma_transferb_v(src, dest, size);
                        },
                        CoprocessorOpcode::MmuDmaTransferBR => { // MMU DMA TRANSFERB R
                            assert_eq!(inst.args.len(), 6);

                            let src  = ((inst.args[1] as u32) << 16) + inst.args[0] as u32;
                            let dest = ((inst.args[3] as u32) << 16) + inst.args[2] as u32;
                            let size = ((inst.args[5] as u32) << 16) + inst.args[4] as u32;

                            memory::dma_transferb_r(src, dest, size);
                        }
                    }
                }
            },
            Some(StopReason::WaitInterrupt) => {
                interrupt();
            },
            Some(StopReason::Stop) | None => break,
        }

        resume();
    }

    println!("Stop!");
}
