static mut REG: u16 = 0b1010101011001011u16;

unsafe fn shift() {
    let  bit1 = (REG & 0b0000000000000010) >>  1;
    let  bit7 = (REG & 0b0000000010000000) >> (7 + bit1);
    let bit12 = (REG & 0b0001000000000000) >> 12;
    let bit15 = (REG & 0b1000000000000000) >> (15 - bit12);

    let next1 =  bit1 ^  bit7;
    let next2 = bit12 ^ bit15;

    REG >>= 2;
    REG |= next1 << 14;
    REG |= next2 << 15;
}

pub fn gen() -> u8 {
    unsafe {
        for _ in 0..(REG as u8 & 0xF) {
            shift()
        }

        REG as u8 & 0xFF
    }
}
