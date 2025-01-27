use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct DivN {
    r_d: Register,
    r_a: Register,
    r_b: Register,
}

impl Instruction<1> for DivN {
    fn execute(&mut self, data: &mut Machine) -> Line {
        let divend = data.register(self.r_a).u16() as u32
            + (data.register(Register::R15).u16() as u32)
            << 16;

        let result = divend / data.register(self.r_b).u16() as u32;
        let remainder = divend % data.register(self.r_b).u16() as u32;

        *data.register_mut(Register::R15) = Word((result & 0xFFFF0000 >> 16) as u16);
        *data.register_mut(self.r_d) = Word((result & 0x0000FFFF) as u16);
        *data.register_mut(self.r_a) = Word(remainder as u16);

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, r_d, r_a, r_b) = instruction[0].hex_digits();

        DivN {
            r_d: Register::from(r_d),
            r_a: Register::from(r_a),
            r_b: Register::from(r_b),
        }
    }
}
