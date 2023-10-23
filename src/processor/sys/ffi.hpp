#ifndef FFI_HPP
#define FFI_HPP

#include <stdint.h>

extern "C" {
    extern uint8_t readb(uint32_t addr);
    extern void writeb(uint32_t addr, uint8_t byte);
}

#endif /* FFI_HPP */
