use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct Jump {
    address: u16,
    offset: Register,
}

impl Instruction<2> for Jump {
    fn execute(&mut self, system: &mut Machine) -> Line {
        let address = self.address + system.register(self.offset).u16();

        return Line::Jump(address);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, _, offset, _) = instruction[0].hex_digits();

        Jump {
            address: instruction[1].u16(),
            offset: Register::from(offset),
        }
    }
}
