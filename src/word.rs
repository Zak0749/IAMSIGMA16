use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Word(pub u16);

impl Word {
    pub const ZERO: Word = Word(0);

    pub fn u16(&self) -> u16 {
        self.0
    }

    pub fn i16(&self) -> i16 {
        self.0 as i16
    }

    pub fn set_bit(&mut self, bit: u16, value: bool) {
        if value {
            self.0 |= 1 << bit;
        } else {
            println!("{} {}", self.0, bit);
            self.0 &= !(1 << bit) & 0xFFFF;
        }
    }

    pub fn set_bits(&mut self, values: [bool; 16]) {
        let mut word = 0;
        for (i, value) in values.iter().enumerate() {
            if *value {
                word |= 1 << i;
            }
        }
        self.0 = word;
    }

    pub fn get_bit(&self, bit: u16) -> bool {
        (self.0 & (1 << bit)) != 0
    }

    pub fn hex_digits(&self) -> (u8, u8, u8, u8) {
        let hex = self.0;
        let d1 = (hex >> 12) & 0xF;
        let d2 = (hex >> 8) & 0xF;
        let d3 = (hex >> 4) & 0xF;
        let d4 = hex & 0xF;
        (d1 as u8, d2 as u8, d3 as u8, d4 as u8)
    }

    pub fn bit(&self, n: u8) -> bool {
        self.0 & (1 << n) != 0
    }

    pub fn with_carry(&self, carry: bool) -> Word {
        return Word(self.0 + (carry as u16));
    }
}

pub enum Overflow {
    Good,
    UnsignedOverflow,
    SignedOverflow,
}

impl Add for &Word {
    type Output = (Word, Overflow);

    fn add(self, rhs: Self) -> Self::Output {
        let (num, unsigned_overflow) = self.u16().overflowing_add(rhs.u16());
        let signed_overflow = self.i16().overflowing_add(rhs.i16()).1;

        (
            Word(num),
            match (unsigned_overflow, signed_overflow) {
                (false, false) => Overflow::Good,
                (true, false) => Overflow::UnsignedOverflow,
                (false, true) => Overflow::SignedOverflow,
                (true, true) => panic!("Impossible to have both unsigned and signed overflow"),
            },
        )
    }
}

impl Sub for &Word {
    type Output = (Word, Overflow);

    fn sub(self, rhs: Self) -> Self::Output {
        let (num, unsigned_overflow) = self.u16().overflowing_sub(rhs.u16());
        let signed_overflow = self.i16().overflowing_sub(rhs.i16()).1;

        (
            Word(num),
            match (unsigned_overflow, signed_overflow) {
                (false, false) => Overflow::Good,
                (true, false) => Overflow::UnsignedOverflow,
                (false, true) => Overflow::SignedOverflow,
                (true, true) => panic!("Impossible to have both unsigned and signed overflow"),
            },
        )
    }
}

impl Mul for &Word {
    type Output = (Word, Overflow);

    fn mul(self, rhs: Self) -> Self::Output {
        let (num, unsigned_overflow) = self.u16().overflowing_mul(rhs.u16());
        let signed_overflow = self.i16().overflowing_mul(rhs.i16()).1;

        (
            Word(num),
            match (unsigned_overflow, signed_overflow) {
                (false, false) => Overflow::Good,
                (true, false) => Overflow::UnsignedOverflow,
                (false, true) => Overflow::SignedOverflow,
                (true, true) => panic!("Impossible to have both unsigned and signed overflow"),
            },
        )
    }
}

impl Div for &Word {
    type Output = (Word, Word);

    fn div(self, rhs: Self) -> Self::Output {
        let num = self.u16() / rhs.u16();
        let rem = self.u16() % rhs.u16();

        (Word(num), Word(rem))
    }
}
