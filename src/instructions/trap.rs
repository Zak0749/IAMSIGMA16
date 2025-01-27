use std::io::Read;

use crate::{
    machine::{Machine, Register},
    word::Word,
};

use super::{Instruction, Line};

const HALT: u16 = 0;
const NON_BLOCKING_READ: u16 = 1;
const NON_BLOCKING_WRITE: u16 = 2;
const BLOCKING_READ: u16 = 3;
const BREAKPOINT: u16 = 4;

pub struct Trap {
    code: Register,
    r_a: Register,
    r_b: Register,
}

impl Instruction<1> for Trap {
    fn execute(&mut self, data: &mut Machine) -> Line {
        match data.register(self.code).u16() {
            HALT => {
                return Line::Halt;
            }
            NON_BLOCKING_READ => {
                let start = data.register(self.r_a).u16() as usize;
                let length = data.register(self.r_b).u16() as usize;

                let mut input = Vec::with_capacity(length);
                std::io::stdin().read_exact(&mut input).unwrap();

                for i in 0..length {
                    let c = input.get(i).unwrap_or(&0);
                    data.memory[start + i] = Word(*c as u16);
                }
            }
            NON_BLOCKING_WRITE => {
                let start = data.register(self.r_a).u16() as usize;
                let length = data.register(self.r_b).u16() as usize;

                let out = data.memory[start..start + length]
                    .iter()
                    .map(|&c| c.u16() as u8 as char)
                    .collect::<String>();

                print!("{out}")
            }
            BLOCKING_READ => {
                let start = data.register(self.r_a).u16() as usize;
                let length = data.register(self.r_b).u16() as usize;

                let mut input = Vec::with_capacity(length);
                std::io::stdin().read_exact(&mut input).unwrap();

                for (i, c) in input.iter().enumerate() {
                    data.memory[start + i] = Word(*c as u16);
                }

                println!(
                    "{} read: {}",
                    data.pc,
                    input.iter().map(|c| *c as char).collect::<String>()
                )
            }
            BREAKPOINT => {
                return Line::Breakpoint;
            }
            _ => panic!("Invalid Trap code"),
        }

        return Line::Increment(Self::WIDTH as u16);
    }

    fn compile(instruction: [Word; Self::WIDTH]) -> Self {
        let (_, r_d, r_a, r_b) = instruction[0].hex_digits();

        Trap {
            code: Register::from(r_d),
            r_a: Register::from(r_a),
            r_b: Register::from(r_b),
        }
    }
}
