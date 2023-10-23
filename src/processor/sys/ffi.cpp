#include "emu816.h"

extern "C" {
    void emu816_reset(bool trace) {
        emu816::reset(trace);
    }

    void emu816_step() {
        emu816::step();
    }

    unsigned long emu816_getCycles() {
        return emu816::getCycles();
    }

    bool emu816_isStopped() {
        return emu816::isStopped();
    }

    void emu816_resume() {
        emu816::resume();
    }

    int emu816_getStopReason() {
        return (int) emu816::getStopReason();
    }

    void emu816_interrupt() {
        emu816::interrupt();
    }

    unsigned char emu816_getCopInstSize() {
        return emu816::getCopInstSize();
    }

    unsigned char emu816_getCopInst(unsigned short *inst) {
        return emu816::getCopInst(inst);
    }
}
