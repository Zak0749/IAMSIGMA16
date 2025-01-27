use crate::{
    machine::{Flags, Machine, Register},
    word::{Overflow, Word},
};

use super::{Instruction, Line};

pub struct Sub {
    r_d: Register,
    r_a: Register,
    r_b: Register,
}

impl Instruction<1> for Sub {
    fn execute(&mut self, data: &mut Machine) -> Line {
        let (result, overflow) = data.register(self.r_a) - data.register(self.r_b);

        *data.register_mut(Register::R15) = Word::from(Flags {
            G: result.u16() > 0,
            g: result.i16() > 0,
            E: result.i16() == 0,
            l: result.i16() < 0,
            L: false,
            v: matches!(overflow, Overflow::SignedOverflow),
            V: matches!(overflow, Overflow::UnsignedOverflow),
            ..Default::default()
        });

        *data.register_mut(self.r_d) = result;

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, r_d, r_a, r_b) = instruction[0].hex_digits();

        Sub {
            r_d: Register::from(r_d),
            r_a: Register::from(r_a),
            r_b: Register::from(r_b),
        }
    }
}
