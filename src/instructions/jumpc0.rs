use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

pub struct JumpC0 {
    bit: u16,
    address: u16,
    offset: Register,
}

impl Instruction<2> for JumpC0 {
    fn execute(&mut self, system: &mut Machine) -> Line {
        let address = self.address + system.register(self.offset).u16();

        if system.register(Register::R15).u16() & self.bit << 0 == 0 {
            return Line::Jump(address);
        }

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, bit, offset, _) = instruction[0].hex_digits();

        JumpC0 {
            bit: bit as u16,
            address: instruction[1].u16(),
            offset: Register::from(offset),
        }
    }
}
