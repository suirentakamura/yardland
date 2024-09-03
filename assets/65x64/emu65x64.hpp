//==============================================================================
//                                          .ooooo.     .o      .ooo
//                                         d88'   `8. o888    .88'
//  .ooooo.  ooo. .oo.  .oo.   oooo  oooo  Y88..  .8'  888   d88'
// d88' `88b `888P"Y88bP"Y88b  `888  `888   `88888b.   888  d888P"Ybo.
// 888ooo888  888   888   888   888   888  .8'  ``88b  888  Y88[   ]88
// 888    .o  888   888   888   888   888  `8.   .88P  888  `Y88   88P
// `Y8bod8P' o888o o888o o888o  `V88V"V8P'  `boood8'  o888o  `88bod8'
//
// A Portable C++ Nozotech 65x64 Emulator
//------------------------------------------------------------------------------
// 2024 KyokoToreno
//
// Based on work by:
//
// Copyright (C),2016 Andrew John Jacobs
// All rights reserved.
//
// This work is made available under the terms of the Creative Commons
// Attribution-NonCommercial-ShareAlike 4.0 International license. Open the
// following URL to see the details.
//
// http://creativecommons.org/licenses/by-nc-sa/4.0/
//------------------------------------------------------------------------------

#ifndef EMU65X64_H
#define EMU65X64_H

#include <stdckdint.h>
#include <limits>
#include <stdlib.h>
#include <stdint.h>
#include <iostream>
#include <cstring>
#include <string>

// Rust ffi IN

extern "C" {
    unsigned char mmu_read_byte(unsigned long long a);
    unsigned short mmu_read_word(unsigned long long a);
    unsigned long mmu_read_dword(unsigned long long a);
    unsigned long long mmu_read_qword(unsigned long long a);

    void mmu_write_byte(unsigned long long a, unsigned char v);
    void mmu_write_word(unsigned long long a, unsigned short v);
    void mmu_write_dword(unsigned long long a, unsigned long v);
    void mmu_write_qword(unsigned long long a, unsigned long long v);
}

#define INLINE inline

#define TRACE(MNEM)       { if (trace) dump(MNEM, ea); }
#define TRACE_SIZE(MNEM)  { if (trace) switch(size) { case Size::BYTE: dump(MNEM ".b", ea); break; case Size::WORD: dump(MNEM ".w", ea); break; case Size::DWORD: dump(MNEM ".d", ea); break; case Size::QWORD: dump(MNEM ".q", ea); break; } }
#define BYTES(N)          { if (trace) bytes(N); pc += N; }
#define SHOWPC()          { if (trace) show(); }

// The wdc65x64 class defines common types for 8-, 16-, 32- and 64-bit data values and
// a set of common functions for manipulating them.

// The mem816 class defines a set of standard methods for defining and accessing
// the emulated memory area.

// Defines the WDC 65C65x64 emulator.
class emu65x64
{
public:
    // Common types for memory and register sizes
    typedef unsigned char      Bit;
    typedef unsigned char      Byte;
    typedef unsigned short     Word;
    typedef unsigned long      DWord;
    typedef unsigned long long QWord;

    enum Size: Byte {
        BYTE    = 0,
        WORD    = 1,
        DWORD   = 2,
        QWORD   = 3
    };

    enum StopReason: int {
        RUNNING     = 0,
        COPROCESSOR = 1,
        WAIT        = 2,
        STOP        = 3
    };

    // Convert a value to a hex string.
    static char *toHex(unsigned long long value, unsigned int digits);

    // Return the low byte of a word
    INLINE static Byte lo(Word value)
    {
        return ((Byte) value);
    }

    // Return the high byte of a word
    INLINE static Byte hi(Word value)
    {
        return (lo((Word) (value >> 8)));
    }

    // Return the low word of a double word
    INLINE static Word lo(DWord value)
    {
        return ((Word) value);
    }

    // Return the high word of a double word
    INLINE static Word hi(DWord value)
    {
        return (lo(value >> 16));
    }

    // Return the low double word of a quad word
    INLINE static DWord lo(QWord value)
    {
        return ((DWord) value);
    }

    // Return the high double word of a quad word
    INLINE static DWord hi(QWord value)
    {
        return (lo(value >> 32));
    }

    // Combine two bytes into a word
    INLINE static Word join(Byte l, Byte h)
    {
        return (l | (h << 8));
    }

    // Combine two words into a double word
    INLINE static DWord join(Word l, Word h)
    {
        return (l | (h << 16));
    }

    // Combine two double words into a quad word
    INLINE static QWord join(DWord l, DWord h)
    {
        return (l | (h << 32));
    }

    // Swap the high and low bytes of a word
    INLINE static Word swap(Word value)
    {
        return ((value >> 8) | (value << 8));
    }

    // Swap the high and low words of a double word
    INLINE static DWord swap(DWord value)
    {
        return ((value >> 16) | (value << 16));
    }

    // Swap the high and low double words of a quad word
    INLINE static QWord swap(QWord value)
    {
        return ((value >> 32) | (value << 32));
    }

    // Fetch a byte from memory.
	INLINE static Byte getByte(QWord ea)
	{
		// return readb(ea);
        return 0;
	}

	// Fetch a word from memory
	INLINE static Word getWord(QWord ea)
	{
        // return (join(getByte(ea + 0), getByte(ea + 1)));
        return 0;
	}

	// Fetch a double word from memory
	INLINE static DWord getDWord(QWord ea)
	{
		// return (join(getByte(ea + 2), getWord(ea + 0)));
        return 0;
	}

    // Fetch a quad word from memory
	INLINE static QWord getQWord(QWord ea)
	{
		// return (join(getDWord(ea + 2), getDWord(ea + 0)));
        return 0;
	}

    // Fetch a value from memory using size
    INLINE static QWord getData(QWord ea, Size size)
    {
        switch (size)
        {
        case Size::BYTE:
            return getByte(ea);
        case Size::WORD:
            return getWord(ea);
        case Size::DWORD:
            return getDWord(ea);
        case Size::QWORD:
            return getQWord(ea);
        }
    }

	// Write a byte to memory
	INLINE static void setByte(QWord ea, Byte data)
	{
		// writeb(ea, data);
	}

	// Write a word to memory
	INLINE static void setWord(QWord ea, Word data)
	{
		// setByte(ea + 0, lo(data));
		// setByte(ea + 1, hi(data));
	}

    // Write a double word to memory
	INLINE static void setDWord(QWord ea, Word data)
	{
		// setByte(ea + 0, lo(data));
		// setByte(ea + 1, hi(data));
	}

    // Write a quad word to memory
	INLINE static void setQWord(QWord ea, Word data)
	{
		// setByte(ea + 0, lo(data));
		// setByte(ea + 1, hi(data));
	}

    // Write a value to memory using size
    INLINE static void setData(QWord ea, QWord data, Size size)
    {
        switch (size)
        {
        case Size::BYTE:
            setByte(ea, (Byte)data);
            break;
        case Size::WORD:
            setWord(ea, (Word)data);
            break;
        case Size::DWORD:
            setDWord(ea, (DWord)data);
            break;
        case Size::QWORD:
            setQWord(ea, (QWord)data);
            break;
        }
    }

    emu65x64();
    ~emu65x64();

    static void reset(bool trace);
    static void step();

    INLINE static unsigned long getCycles()
    {
        return (cycles);
    }

    INLINE static bool isStopped()
    {
        return (stopped);
    }

    INLINE static void resume()
    {
        stopped = false;
    }

    INLINE static StopReason getStopReason()
    {
        return (stop_reason);
    }

    INLINE static void interrupt()
    {
        interrupted = true;
    }

    INLINE static Byte getCopInstSize() {
        return (cop_size);
    }

    INLINE static Byte getCopInst(Word *dest) {
        if (cop != NULL) {
            std::memcpy(dest, cop, cop_size * sizeof(Word));
            free(cop);
            cop = NULL;
            cop_size = 0;
        }

        return cop_op;
    }

private:
    static union FLAGS {
        struct {
            Bit     f_c : 1; // carry
            Bit     f_z : 1; // zero
            Bit     f_i : 1; // interrupt disable
            Bit     f_d : 1; // decimal
            Bit     f_t : 1; // trap
            Bit     f_p : 1; // parity
            Bit     f_v : 1; // overflow
            Bit     f_n : 1; // negative
        };
        Byte    b;
        Word    w;
        DWord   d;
        QWord   q;
    }   p;

    static union REGS {
        Byte    b;
        Word    w;
        DWord   d;
        QWord   q;
    }   a, b, x, y, sp, z[128];

    static QWord     pc, tpc;

    static Byte             cop_size, cop_op;
    static Word             *cop;
    static bool             stopped;
    static StopReason       stop_reason;
    static bool             interrupted;
    static unsigned long    cycles;
    static bool             trace;

    static void show();
    static void bytes(unsigned int);
    static void dump(const char *, QWord);

    // Push a byte on the stack
    INLINE static void pushByte(Byte value)
    {
        setByte(sp.q--, value);
    }

    // Push a word on the stack
    INLINE static void pushWord(Word value)
    {
        pushByte(hi(value));
        pushByte(lo(value));
    }

    // Push a double word on the stack
    INLINE static void pushDWord(DWord value)
    {
        pushWord(hi(value));
        pushWord(lo(value));
    }

    // Push a quad word on the stack
    INLINE static void pushQWord(QWord value)
    {
        pushDWord(hi(value));
        pushDWord(lo(value));
    }

    // Pull a byte from the stack
    INLINE static Byte pullByte()
    {
        return (getByte(++sp.q));
    }

    // Pull a word from the stack
    INLINE static Word pullWord()
    {
        Byte    l = pullByte();
        Byte    h = pullByte();

        return (join(l, h));
    }

    // Pull a double word from the stack
    INLINE static DWord pullDWord()
    {
        Word    l = pullWord();
        Word    h = pullWord();

        return (join(l, h));
    }

    // Pull a quad word from the stack
    INLINE static QWord pullQWord()
    {
        DWord    l = pullDWord();
        DWord    h = pullDWord();

        return (join(l, h));
    }

    // Absolute - a
    INLINE static QWord am_absl()
    {
        QWord    ea = pc;

        BYTES(16);
        cycles += 2;
        return (ea);
    }

    // Absolute Indexed X - a,X
    INLINE static QWord am_absx()
    {
        QWord    ea = pc + x.q;

        BYTES(16);
        cycles += 2;
        return (ea);
    }

    // Absolute Indexed Y - a,Y
    INLINE static QWord am_absy()
    {
        QWord    ea = pc + y.q;

        BYTES(16);
        cycles += 2;
        return (ea);
    }

    // Absolute Indirect - (a)
    INLINE static QWord am_absi()
    {
        QWord ia = pc;

        BYTES(16);
        cycles += 4;
        return (getQWord(ia));
    }

    // Absolute Indexed Indirect - (a,X)
    INLINE static QWord am_abxi()
    {
        QWord ia = pc + x.q;

        BYTES(16);
        cycles += 4;
        return (getQWord(ia));
    }

    /*

    // Zero-cache Indirect - ([z])
    INLINE static QWord am_zci(Byte i)
    {
        Byte b = getByte(pc);
        QWord ia = z[b % 128].q;

        BYTES(16);
        cycles += 5;
        return (getQWord(ia));
    }

    // Zero-cache Indexed Indirect - ([z],x)
    INLINE static QWord am_zcxi(Byte b)
    {
        QWord ia = z[(b + x.b) % 128].q;

        BYTES(16);
        cycles += 5;
        return (getQWord(ia));
    }

    // Zero-cache Indirect Indexed - ([z]),y
    INLINE static QWord am_zcix(Byte b)
    {
        QWord ia = z[b % 128].q;

        BYTES(16);
        cycles += 5;
        return (getQWord(ia) + y.q);
    }

    */
    /*

    // Direct Page - d
    INLINE static QWord am_dpag()
    {
        Byte offset = getByte(bank(pbr) | pc);

        BYTES(1);
        cycles += 1;
        return (bank(0) | (Word)(dp.w + offset));
    }

    // Direct Page Indexed X - d,X
    INLINE static QWord am_dpgx()
    {
        Byte offset = getByte(bank(pbr) | pc) + x.b;

        BYTES(1);
        cycles += 1;
        return (bank(0) | (Word)(dp.w + offset));
    }

    // Direct Page Indexed Y - d,Y
    INLINE static QWord am_dpgy()
    {
        Byte offset = getByte(bank(pbr) | pc) + y.b;

        BYTES(1);
        cycles += 1;
        return (bank(0) | (Word)(dp.w + offset));
    }

    // Direct Page Indirect - (d)
    INLINE static QWord am_dpgi()
    {
        Byte disp = getByte(bank(pbr) | pc);

        BYTES(1);
        cycles += 3;
        return (bank(dbr) | getWord(bank(0) | (Word)(dp.w + disp)));
    }

    // Direct Page Indexed Indirect - (d,x)
    INLINE static QWord am_dpix()
    {
        Byte disp = getByte(join(pbr, pc));

        BYTES(1);
        cycles += 3;
        return (bank(dbr) | getWord(bank(0) | (Word)(dp.w + disp + x.w)));
    }

    // Direct Page Indirect Indexed - (d),Y
    INLINE static QWord am_dpiy()
    {
        Byte disp = getByte(join(pbr, pc));

        BYTES(1);
        cycles += 3;
        return (bank(dbr) | (getWord(bank(0) | (dp.w + disp)) + y.w));
    }

    // Direct Page Indirect Long - [d]
    INLINE static QWord am_dpil()
    {
        Byte disp = getByte(join(pbr, pc));

        BYTES(1);
        cycles += 4;
        return (getAddr(bank(0) | (Word)(dp.w + disp)));
    }

    // Direct Page Indirect Long Indexed - [d],Y
    INLINE static QWord am_dily()
    {
        Byte disp = getByte(join(pbr, pc));

        BYTES(1);
        cycles += 4;
        return (getAddr(bank(0) | (Word)(dp.w + disp)) + y.w);
    }

    */

    // Implied/Stack
    INLINE static QWord am_impl()
    {
        BYTES(0);
        return (0);
    }

    // Accumulator
    INLINE static QWord am_acc()
    {
        BYTES(0);
        return (0);
    }

    // Immediate Byte
    INLINE static QWord am_immb()
    {
        QWord ea = pc;

        BYTES(1);
        cycles += 0;
        return (ea);
    }

    // Immediate Word
    INLINE static QWord am_immw()
    {
        QWord ea = pc;

        BYTES(2);
        cycles += 1;
        return (ea);
    }

    // Immediate DWord
    INLINE static QWord am_immd()
    {
        QWord ea = pc;

        BYTES(4);
        cycles += 1;
        return (ea);
    }

    // Immediate QWord
    INLINE static QWord am_immq()
    {
        QWord ea = pc;

        BYTES(16);
        cycles += 1;
        return (ea);
    }

    // Relative - d
    INLINE static QWord am_rela()
    {
        Word disp = getWord(pc);

        BYTES(2);
        cycles += 1;
        return ((QWord)(pc + (signed short)disp));
    }

    // Stack Relative - d,S
    INLINE static QWord am_srel()
    {
        Byte disp = getByte(pc);

        BYTES(1);
        cycles += 1;

        return ((QWord)(sp.q + disp));
    }

    // Stack Relative Indirect Indexed Y - (d,S),Y
    INLINE static QWord am_sriy()
    {
        Byte disp = getByte(pc);
        QWord ia;

        BYTES(1);
        cycles += 3;

        ia = getQWord(sp.q + disp);

        return ((QWord)(ia + y.q));
    }

    // Set the Negative flag
    INLINE static void setn(unsigned int flag)
    {
        p.f_n = flag ? 1 : 0;
    }

    // Set the Overflow flag
    INLINE static void setv(unsigned int flag)
    {
        p.f_v = flag ? 1 : 0;
    }

    // Set the decimal flag
    INLINE static void setd(unsigned int flag)
    {
        p.f_d = flag ? 1 : 0;
    }

    // Set the Interrupt Disable flag
    INLINE static void seti(unsigned int flag)
    {
        p.f_i = flag ? 1 : 0;
    }

    // Set the Zero flag
    INLINE static void setz(unsigned int flag)
    {
        p.f_z = flag ? 1 : 0;
    }

    // Set the Carry flag
    INLINE static void setc(unsigned int flag)
    {
        p.f_c = flag ? 1 : 0;
    }

    // Set the Negative and Zero flags from a byte value
    INLINE static void setnz_b(Byte value)
    {
        setn(value & 0x80);
        setz(value == 0);
    }

    // Set the Negative and Zero flags from a word value
    INLINE static void setnz_w(Word value)
    {
        setn(value & 0x8000);
        setz(value == 0);
    }

    // Set the Negative and Zero flags from a double word value
    INLINE static void setnz_d(DWord value)
    {
        setn(value & 0x80000000);
        setz(value == 0);
    }

    // Set the Negative and Zero flags from a quad word value
    INLINE static void setnz_q(QWord value)
    {
        setn(value & 0x8000000000000000);
        setz(value == 0);
    }

    INLINE static void op_adc(QWord ea, Size size)
    {
        TRACE_SIZE("ADC");

        QWord data = getData(ea, size);
        bool carry = a.q > INT64_MAX - data - p.f_c;
        QWord temp = a.q + data + p.f_c;

        if (p.f_d) {
            if ((temp & 0x000000000000000f) > 0x0000000000000009) temp += 0x0000000000000006;
            if ((temp & 0x00000000000000f0) > 0x0000000000000090) temp += 0x0000000000000060;
            if ((temp & 0x0000000000000f00) > 0x0000000000000900) temp += 0x0000000000000600;
            if ((temp & 0x000000000000f000) > 0x0000000000009000) temp += 0x0000000000006000;
            if ((temp & 0x00000000000f0000) > 0x0000000000090000) temp += 0x0000000000060000;
            if ((temp & 0x0000000000f00000) > 0x0000000000900000) temp += 0x0000000000600000;
            if ((temp & 0x000000000f000000) > 0x0000000009000000) temp += 0x0000000006000000;
            if ((temp & 0x00000000f0000000) > 0x0000000090000000) temp += 0x0000000060000000;
            if ((temp & 0x0000000f00000000) > 0x0000000900000000) temp += 0x0000000060000000;
            if ((temp & 0x000000f000000000) > 0x0000009000000000) temp += 0x0000006000000000;
            if ((temp & 0x00000f0000000000) > 0x0000090000000000) temp += 0x0000060000000000;
            if ((temp & 0x0000f00000000000) > 0x0000900000000000) temp += 0x0000600000000000;
            if ((temp & 0x000f000000000000) > 0x0009000000000000) temp += 0x0006000000000000;
            if ((temp & 0x00f0000000000000) > 0x0090000000000000) temp += 0x0060000000000000;
            if ((temp & 0x0f00000000000000) > 0x0900000000000000) temp += 0x0600000000000000;
            if ((temp & 0xf000000000000000) > 0x9000000000000000) temp += 0x6000000000000000;
        }

        setc(carry);
        setv((~(a.q ^ data)) & (a.q ^ temp) & 0x8000000000000000);
        setnz_q(a.q = (QWord)temp);

        cycles += 2;
    }

    INLINE static void op_and(QWord ea, Size size)
    {
        TRACE_SIZE("AND");

            setnz_w(a.w &= getWord(ea));
            cycles += 3;
    }

    INLINE static void op_asl(QWord ea)
    {
        TRACE("ASL");

            Word data = getWord(ea);

            setc(data & 0x8000);
            setnz_w(data <<= 1);
            setWord(ea, data);
            cycles += 5;
    }

    INLINE static void op_asla(QWord ea)
    {
        TRACE("ASL");

            setc(a.w & 0x8000);
            setnz_w(a.w <<= 1);
            setWord(ea, a.w);
        cycles += 2;
    }

    INLINE static void op_bcc(QWord ea)
    {
        TRACE("BCC");

        if (p.f_c == 0) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bcs(QWord ea)
    {
        TRACE("BCS");

        if (p.f_c == 1) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_beq(QWord ea)
    {
        TRACE("BEQ");

        if (p.f_z == 1) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bit(QWord ea)
    {
        TRACE("BIT");

            Word data = getWord(ea);

            setz((a.w & data) == 0);
            setn(data & 0x8000);
            setv(data & 0x4000);

            cycles += 3;
    }

    INLINE static void op_biti(QWord ea)
    {
        TRACE("BIT");


            Word data = getWord(ea);

            setz((a.w & data) == 0);
        cycles += 2;
    }

    INLINE static void op_bmi(QWord ea)
    {
        TRACE("BMI");

        if (p.f_n == 1) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bne(QWord ea)
    {
        TRACE("BNE");

        if (p.f_z == 0) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bpl(QWord ea)
    {
        TRACE("BPL");

        if (p.f_n == 0) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bra(QWord ea)
    {
        TRACE("BRA");

        //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
        pc = (Word)ea;
        cycles += 3;
    }

    INLINE static void op_brk(QWord ea)
    {
        TRACE("BRK");

        if (false) {
            pushWord(pc);
            pushByte(p.b | 0x10);

            p.f_i = 1;
            p.f_d = 0;
            //pbr = 0;

            pc = getWord(0xfffe);
            cycles += 7;
        }
        else {
            //pushByte(pbr);
            pushWord(pc);
            pushByte(p.b);

            p.f_i = 1;
            p.f_d = 0;
            //pbr = 0;

            pc = getWord(0xffe6);
            cycles += 8;
        }
    }

    INLINE static void op_brl(QWord ea)
    {
        TRACE("BRL");

        pc = (Word)ea;
        cycles += 3;
    }

    INLINE static void op_bvc(QWord ea)
    {
        TRACE("BVC");

        if (p.f_v == 0) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_bvs(QWord ea)
    {
        TRACE("BVS");

        if (p.f_v == 1) {
            //if (e && ((pc ^ ea) & 0xff00)) ++cycles;
            pc = (Word)ea;
            cycles += 3;
        }
        else
            cycles += 2;
    }

    INLINE static void op_clc(QWord ea)
    {
        TRACE("CLC");

        setc(0);
        cycles += 2;
    }

    INLINE static void op_cld(QWord ea)
    {
        TRACE("CLD")

        setd(0);
        cycles += 2;
    }

    INLINE static void op_cli(QWord ea)
    {
        TRACE("CLI")

        seti(0);
        cycles += 2;
    }

    INLINE static void op_clv(QWord ea)
    {
        TRACE("CLD")

        setv(0);
        cycles += 2;
    }

    INLINE static void op_cmp(QWord ea)
    {
        TRACE("CMP");


            Word    data = getWord(ea);
            QWord    temp = a.w - data;

            setc(temp & 0x10000L);
            setnz_w((Word)temp);
            cycles += 3;
    }

    INLINE static void op_cop(QWord ea)
    {
        TRACE("COP");

        stopped = true;
        stop_reason = StopReason::COPROCESSOR;

        cop_size = getByte(ea++);
        cop_op = getByte(ea++);
        pc += 1 + cop_size * 2;

        cop = (Word*) malloc(cop_size * sizeof(Word));

        for (int i = 0; i < cop_size; i++) {
            cop[i] = getWord(ea + i * 2);
        }

        /*
            if (e) {
                pushWord(pc);
                pushByte(p.b);

                p.f_i = 1;
                p.f_d = 0;
                pbr = 0;

                pc = getWord(0xfff4);
                cycles += 7;
            }
            else {
                pushByte(pbr);
                pushWord(pc);
                pushByte(p.b);

                p.f_i = 1;
                p.f_d = 0;
                pbr = 0;

                pc = getWord(0xffe4);
                cycles += 8;
            }
        */
    }

    INLINE static void op_cpx(QWord ea)
    {
        TRACE("CPX");
    /*
        if (e || p.f_x) {
            Byte    data = getByte(ea);
            Word    temp = x.b - data;

            setc(temp & 0x100);
            setnz_b(lo(temp));
            cycles += 2;
        }
        else {
            Word    data = getWord(ea);
            //Addr    temp = x.w - data;

            setc(temp & 0x10000);
            setnz_w((Word) temp);
            cycles += 3;
        }*/
    }

    INLINE static void op_cpy(QWord ea)
    {
        TRACE("CPY");
/*
        if (e || p.f_x) {
            Byte    data = getByte(ea);
            Word    temp = y.b - data;

            setc(temp & 0x100);
            setnz_b(lo(temp));
            cycles += 2;
        }
        else {
            Word    data = getWord(ea);
            Addr    temp = y.w - data;

            setc(temp & 0x10000);
            setnz_w((Word) temp);
            cycles += 3;
        }*/
    }

    INLINE static void op_dec(QWord ea)
    {
        TRACE("DEC");


            Word data = getWord(ea);

            setWord(ea, --data);
            setnz_w(data);
            cycles += 5;
    }

    INLINE static void op_deca(QWord ea)
    {
        TRACE("DEC");

            setnz_w(--a.w);

        cycles += 2;
    }

    INLINE static void op_dex(QWord ea)
    {
        TRACE("DEX");

            setnz_w(x.w -= 1);

        cycles += 2;
    }

    INLINE static void op_dey(QWord ea)
    {
        TRACE("DEY");

            setnz_w(y.w -= 1);

        cycles += 2;
    }

    INLINE static void op_eor(QWord ea)
    {
        TRACE("EOR");

            setnz_w(a.w ^= getWord(ea));
            cycles += 3;
    }

    INLINE static void op_inc(QWord ea)
    {
        TRACE("INC");

            Word data = getWord(ea);

            setWord(ea, ++data);
            setnz_w(data);
            cycles += 5;
    }

    INLINE static void op_inca(QWord ea)
    {
        TRACE("INC");

            setnz_w(++a.w);

        cycles += 2;
    }

    INLINE static void op_inx(QWord ea)
    {
        TRACE("INX");

            setnz_w(++x.w);

        cycles += 2;
    }

    INLINE static void op_iny(QWord ea)
    {
        TRACE("INY");

            setnz_w(++y.w);

        cycles += 2;
    }

    INLINE static void op_jmp(QWord ea)
    {
        TRACE("JMP");

        //pbr = lo(ea >> 16);
        pc = (Word)ea;
        cycles += 1;
    }

    INLINE static void op_jsl(QWord ea)
    {
        TRACE("JSL");

        //pushByte(pbr);
        pushWord(pc - 1);

        //pbr = lo(ea >> 16);
        pc = (Word)ea;
        cycles += 5;
    }

    INLINE static void op_jsr(QWord ea)
    {
        TRACE("JSR");

        pushWord(pc - 1);

        pc = (Word)ea;
        cycles += 4;
    }

    INLINE static void op_lda(QWord ea)
    {
        TRACE("LDA");

            setnz_w(a.w = getWord(ea));
            cycles += 3;
    }

    INLINE static void op_ldx(QWord ea)
    {
        TRACE("LDX");


            setnz_w(x.w = getWord(ea));
            cycles += 3;
    }

    INLINE static void op_ldy(QWord ea)
    {
        TRACE("LDY");


            setnz_w(y.w = getWord(ea));
            cycles += 3;
    }

    INLINE static void op_lsr(QWord ea)
    {
        TRACE("LSR");


            Word data = getWord(ea);

            setc(data & 0x0001);
            setnz_w(data >>= 1);
            setWord(ea, data);
            cycles += 5;
    }

    INLINE static void op_lsra(QWord ea)
    {
        TRACE("LSR");

            setc(a.w & 0x0001);
            setnz_w(a.w >>= 1);
            setWord(ea, a.w);
        cycles += 2;
    }

    INLINE static void op_mvn(QWord ea)
    {
        TRACE("MVN");

        Byte src = getByte(ea + 1);
        Byte dst = getByte(ea + 0);

        //setByte(join(dbr = dst, y.w++), getByte(join(src, x.w++)));
        if (--a.w != 0xffff) pc -= 3;
        cycles += 7;
    }

    INLINE static void op_mvp(QWord ea)
    {
        TRACE("MVP");

        Byte src = getByte(ea + 1);
        Byte dst = getByte(ea + 0);

        //setByte(join(dbr = dst, y.w--), getByte(join(src, x.w--)));
        if (--a.w != 0xffff) pc -= 3;
        cycles += 7;
    }

    INLINE static void op_nop(QWord ea)
    {
        TRACE("NOP");

        cycles += 2;
    }

    INLINE static void op_ora(QWord ea)
    {
        TRACE("ORA");


            setnz_w(a.w |= getWord(ea));
            cycles += 3;
    }

    INLINE static void op_pea(QWord ea)
    {
        TRACE("PEA");

        pushWord(getWord(ea));
        cycles += 5;
    }

    INLINE static void op_pei(QWord ea)
    {
        TRACE("PEI");

        pushWord(getWord(ea));
        cycles += 6;
    }

    INLINE static void op_per(QWord ea)
    {
        TRACE("PER");

        pushWord((Word) ea);
        cycles += 6;
    }

    INLINE static void op_pha(QWord ea)
    {
        TRACE("PHA");


            pushWord(a.w);
            cycles += 4;
    }

    INLINE static void op_phb(QWord ea)
    {
        TRACE("PHB");

        //pushByte(dbr);
        cycles += 3;
    }

    INLINE static void op_phd(QWord ea)
    {
        TRACE("PHD");

        //pushWord(dp.w);
        cycles += 4;
    }

    INLINE static void op_phk(QWord ea)
    {
        TRACE("PHK");

        //pushByte(pbr);
        cycles += 3;
    }

    INLINE static void op_php(QWord ea)
    {
        TRACE("PHP");

        pushByte(p.b);
        cycles += 3;
    }

    INLINE static void op_phx(QWord ea)
    {
        TRACE("PHX");


            pushWord(x.w);
            cycles += 4;
    }

    INLINE static void op_phy(QWord ea)
    {
        TRACE("PHY");

            pushWord(y.w);
            cycles += 4;
    }

    INLINE static void op_pla(QWord ea)
    {
        TRACE("PLA");

            setnz_w(a.w = pullWord());
            cycles += 5;
    }

    INLINE static void op_plb(QWord ea)
    {
        TRACE("PLB");

        //setnz_b(dbr = pullByte());
        cycles += 4;
    }

    INLINE static void op_pld(QWord ea)
    {
        TRACE("PLD");

        //setnz_w(dp.w = pullWord());
        cycles += 5;
    }

    INLINE static void op_plk(QWord ea)
    {
        TRACE("PLK");

        //setnz_b(dbr = pullByte());
        cycles += 4;
    }

    INLINE static void op_plp(QWord ea)
    {
        TRACE("PLP");
/*
        if (e)
            p.b = pullByte() | 0x30;
        else {
            p.b = pullByte();

            if (p.f_x) {
                x.w = x.b;
                y.w = y.b;
            }
        }
        */
        cycles += 4;
    }

    INLINE static void op_plx(QWord ea)
    {
        TRACE("PLX");


            setnz_w(x.w = pullWord());
            cycles += 5;
    }

    INLINE static void op_ply(QWord ea)
    {
        TRACE("PLY");

            setnz_w(y.w = pullWord());
            cycles += 5;
    }

    INLINE static void op_rep(QWord ea)
    {
        TRACE("REP");

        p.b &= ~getByte(ea);
        //if (e) p.f_m = p.f_x = 1;
        cycles += 3;
    }

    INLINE static void op_rol(QWord ea)
    {
        TRACE("ROL");

            Word data = getWord(ea);
            Word carry = p.f_c ? 0x0001 : 0x0000;

            setc(data & 0x8000);
            setnz_w(data = (data << 1) | carry);
            setWord(ea, data);
            cycles += 5;
    }

    INLINE static void op_rola(QWord ea)
    {
        TRACE("ROL");

            Word carry = p.f_c ? 0x0001 : 0x0000;

            setc(a.w & 0x8000);
            setnz_w(a.w = (a.w << 1) | carry);
        cycles += 2;
    }

    INLINE static void op_ror(QWord ea)
    {
        TRACE("ROR");


            Word data = getWord(ea);
            Word carry = p.f_c ? 0x8000 : 0x0000;

            setc(data & 0x0001);
            setnz_w(data = (data >> 1) | carry);
            setWord(ea, data);
            cycles += 5;
    }

    INLINE static void op_rora(QWord ea)
    {
        TRACE("ROR");


            Word carry = p.f_c ? 0x8000 : 0x0000;

            setc(a.w & 0x0001);
            setnz_w(a.w = (a.w >> 1) | carry);
        cycles += 2;
    }

    INLINE static void op_rti(QWord ea)
    {
        TRACE("RTI");


            p.b = pullByte();
            pc = pullWord();
            //pbr = pullByte();
            cycles += 7;
        p.f_i = 0;
    }

    INLINE static void op_rtl(QWord ea)
    {
        TRACE("RTL");

        pc = pullWord() + 1;
        //pbr = pullByte();
        cycles += 6;
    }

    INLINE static void op_rts(QWord ea)
    {
        TRACE("RTS");

        pc = pullWord() + 1;
        cycles += 6;
    }

    INLINE static void op_sbc(QWord ea)
    {
        TRACE("SBC");
/*
        if (e || p.f_m) {
            Byte    data = ~getByte(ea);
            Word    temp = a.b + data + p.f_c;

            if (p.f_d) {
                if ((temp & 0x0f) > 0x09) temp += 0x06;
                if ((temp & 0xf0) > 0x90) temp += 0x60;
            }

            setc(temp & 0x100);
            setv((~(a.b ^ data)) & (a.b ^ temp) & 0x80);
            setnz_b(a.b = lo(temp));
            cycles += 2;
        }
        else {
            Word    data = ~getWord(ea);
            int     temp = a.w + data + p.f_c;

            if (p.f_d) {
                if ((temp & 0x000f) > 0x0009) temp += 0x0006;
                if ((temp & 0x00f0) > 0x0090) temp += 0x0060;
                if ((temp & 0x0f00) > 0x0900) temp += 0x0600;
                if ((temp & 0xf000) > 0x9000) temp += 0x6000;
            }

            setc(temp & 0x10000);
            setv((~(a.w ^ data)) & (a.w ^ temp) & 0x8000);
            setnz_w(a.w = (Word)temp);
            cycles += 3;
        }
        */
    }

    INLINE static void op_sec(QWord ea)
    {
        TRACE("SEC");

        setc(1);
        cycles += 2;
    }

    INLINE static void op_sed(QWord ea)
    {
        TRACE("SED");

        setd(1);
        cycles += 2;
    }

    INLINE static void op_sei(QWord ea)
    {
        TRACE("SEI");

        seti(1);
        cycles += 2;
    }

    INLINE static void op_sep(QWord ea)
    {
        TRACE("SEP");

        p.b |= getByte(ea);
        /*
        if (e) p.f_m = p.f_x = 1;

        if (p.f_x) {
            x.w = x.b;
            y.w = y.b;
        }
        */
        cycles += 3;
    }

    INLINE static void op_sta(QWord ea)
    {
        TRACE("STA");

            setWord(ea, a.w);
            cycles += 3;
    }

    INLINE static void op_stp(QWord ea)
    {
        TRACE("STP");

        /*
        if (!interrupted) {
            pc -= 1;
        }
        else
            interrupted = false;
        */

        stopped = true;
        stop_reason = StopReason::STOP;

        cycles += 3;
    }

    INLINE static void op_stx(QWord ea)
    {
        TRACE("STX");

            setWord(ea, x.w);
            cycles += 3;
    }

    INLINE static void op_sty(QWord ea)
    {
        TRACE("STY");

            setWord(ea, y.w);
            cycles += 3;
    }

    INLINE static void op_stz(QWord ea)
    {
        TRACE("STZ");

            setWord(ea, 0);
            cycles += 3;
    }

    INLINE static void op_tax(QWord ea)
    {
        TRACE("TAX");

            setnz_w(x.w = a.w);

        cycles += 2;
    }

    INLINE static void op_tay(QWord ea)
    {
        TRACE("TAY");

            setnz_w(y.w = a.w);

        cycles += 2;
    }

    INLINE static void op_tcd(QWord ea)
    {
        TRACE("TCD");

        //dp.w = a.w;
        cycles += 2;
    }

    INLINE static void op_tdc(QWord ea)
    {
        TRACE("TDC");
/*
        if (e || p.f_m)
            setnz_b(lo(a.w = dp.w));
        else
            setnz_w(a.w = dp.w);*/

        cycles += 2;
    }

    INLINE static void op_tas(QWord ea)
    {
        TRACE("TAS");

        sp.q = a.q;
        cycles += 2;
    }

    INLINE static void op_trb(QWord ea)
    {
        TRACE("TRB");

        QWord data = getQWord(ea);

        setQWord(ea, data & ~a.q);
        setz((a.q & data) == 0);
        cycles += 5;
    }

    INLINE static void op_tsb(QWord ea)
    {
        TRACE("TSB");

        QWord data = getQWord(ea);

        setQWord(ea, data | a.q);
        setz((a.q & data) == 0);
        cycles += 5;
    }

    INLINE static void op_tsa(QWord ea)
    {
        TRACE("TSA");

        setnz_q(a.q = sp.q);

        cycles += 2;
    }

    INLINE static void op_tsx(QWord ea)
    {
        TRACE("TSX");

        setnz_q(x.q = sp.q);

        cycles += 2;
    }

    INLINE static void op_txa(QWord ea)
    {
        TRACE("TXA");

        setnz_q(a.q = x.q);

        cycles += 2;
    }

    INLINE static void op_txs(QWord ea)
    {
        TRACE("TXS");

        sp.q = x.q;

        cycles += 2;
    }

    INLINE static void op_txy(QWord ea)
    {
        TRACE("TXY");

        setnz_q(y.q = x.q);

        cycles += 2;
    }

    INLINE static void op_tya(QWord ea)
    {
        TRACE("TYA");

        setnz_q(a.q = y.q);

        cycles += 2;
    }

    INLINE static void op_tyx(QWord ea)
    {
        TRACE("TYX");

        setnz_q(x.q = y.q);

        cycles += 2;
    }

    INLINE static void op_tba(QWord ea)
    {
        TRACE("TYX");

        setnz_q(x.q = y.q);

        cycles += 2;
    }

    INLINE static void op_wai(QWord ea)
    {
        TRACE("WAI");

        stopped = true;
        stop_reason = StopReason::WAIT;

        cycles += 3;
    }
};

#endif
