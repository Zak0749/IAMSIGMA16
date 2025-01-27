use crate::{
    machine::{Flags, Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct Cmp {
    r_a: Register,
    r_b: Register,
}

impl Instruction<1> for Cmp {
    fn execute(&mut self, data: &mut Machine) -> Line {
        let r_a = data.register(self.r_a);
        let r_b = data.register(self.r_b);

        *data.register_mut(Register::R15) = Word::from(Flags {
            G: r_a.u16() > r_b.u16(),
            g: r_a.i16() > r_b.i16(),
            E: r_a == r_b,
            l: r_a.u16() < r_b.u16(),
            L: r_a.i16() < r_b.i16(),
            ..Default::default()
        });

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, _, r_a, r_b) = instruction[0].hex_digits();

        Cmp {
            r_a: Register::from(r_a),
            r_b: Register::from(r_b),
        }
    }
}
