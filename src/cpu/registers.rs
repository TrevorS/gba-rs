const INITIAL_A: Byte = 0x01;
const INITIAL_B: Byte = 0x00;
const INITIAL_C: Byte = 0x13;
const INITIAL_D: Byte = 0x00;
const INITIAL_E: Byte = 0xd8;
const INITIAL_F: Byte = 0xb0;

const INITIAL_HL: Word = 0x014d;
const INITIAL_PC: Word = 0x0100;
const INITIAL_SP: Word = 0xfffe;

use crate::types::{from_word, to_word, Byte, Word};

pub struct Registers {
    a: Byte,
    b: Byte,
    c: Byte,
    d: Byte,
    e: Byte,
    f: Byte,
    h: Byte,
    l: Byte,
    pc: Word,
    sp: Word,
}

impl Registers {
    pub fn a(&self) -> Byte {
        self.a
    }

    pub fn b(&self) -> Byte {
        self.b
    }

    pub fn c(&self) -> Byte {
        self.c
    }

    pub fn d(&self) -> Byte {
        self.d
    }

    pub fn e(&self) -> Byte {
        self.e
    }

    pub fn f(&self) -> Byte {
        self.f
    }
    pub fn h(&self) -> Byte {
        self.h
    }

    pub fn l(&self) -> Byte {
        self.l
    }

    pub fn af(&self) -> Word {
        to_word(self.a, self.f)
    }

    pub fn bc(&self) -> Word {
        to_word(self.b, self.c)
    }

    pub fn de(&self) -> Word {
        to_word(self.d, self.e)
    }

    pub fn hl(&self) -> Word {
        to_word(self.h, self.l)
    }

    pub fn sp(&self) -> Word {
        self.sp
    }

    pub fn pc(&self) -> Word {
        self.pc
    }

    pub fn set_a(&mut self, value: Byte) {
        self.a = value
    }

    pub fn set_b(&mut self, value: Byte) {
        self.b = value
    }

    pub fn set_c(&mut self, value: Byte) {
        self.c = value
    }

    pub fn set_d(&mut self, value: Byte) {
        self.d = value
    }

    pub fn set_e(&mut self, value: Byte) {
        self.e = value
    }

    pub fn set_f(&mut self, value: Byte) {
        self.f = value
    }

    pub fn set_af(&mut self, value: Word) {
        let (a, f) = from_word(value);

        self.a = a;
        self.f = f;
    }

    pub fn set_bc(&mut self, value: Word) {
        let (b, c) = from_word(value);

        self.b = b;
        self.c = c;
    }

    pub fn set_de(&mut self, value: Word) {
        let (d, e) = from_word(value);

        self.d = d;
        self.e = e;
    }

    pub fn set_hl(&mut self, value: Word) {
        let (h, l) = from_word(value);

        self.h = h;
        self.l = l;
    }

    pub fn set_pc(&mut self, value: Word) {
        self.pc = value;
    }

    pub fn set_sp(&mut self, value: Word) {
        self.sp = value;
    }
}

impl Default for Registers {
    fn default() -> Self {
        let mut rs = Self {
            a: INITIAL_A,
            b: INITIAL_B,
            c: INITIAL_C,
            d: INITIAL_D,
            e: INITIAL_E,
            f: INITIAL_F,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        };

        rs.set_hl(INITIAL_HL);
        rs.set_pc(INITIAL_PC);
        rs.set_sp(INITIAL_SP);

        rs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hl_post_construction() {
        let rs = Registers::default();

        assert_eq!(rs.hl(), INITIAL_HL);
    }
}
