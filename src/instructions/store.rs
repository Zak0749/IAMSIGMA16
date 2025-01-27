use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct Store {
    r_i: Register,
    address: u16,
    offset: Register,
}

impl Instruction<2> for Store {
    fn execute(&mut self, system: &mut Machine) -> Line {
        let address = self.address + system.register(self.offset).u16();

        system.memory[address as usize] = *system.register(self.r_i);

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, r_d, offset, _) = instruction[0].hex_digits();

        Store {
            r_i: Register::from(r_d),
            address: instruction[1].u16(),
            offset: Register::from(offset),
        }
    }
}
