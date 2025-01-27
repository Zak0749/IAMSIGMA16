use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct Div {
    r_d: Register,
    r_a: Register,
    r_b: Register,
}

impl Instruction<1> for Div {
    fn execute(&mut self, data: &mut Machine) -> Line {
        let (result, remainder) = data.register(self.r_a) / data.register(self.r_b);

        *data.register_mut(self.r_d) = result;

        *data.register_mut(Register::R15) = remainder;

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, r_d, r_a, r_b) = instruction[0].hex_digits();

        Div {
            r_d: Register::from(r_d),
            r_a: Register::from(r_a),
            r_b: Register::from(r_b),
        }
    }
}
