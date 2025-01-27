use std::{fs, io};

use crate::{
    instructions::{Instruction, Line},
    word::Word,
};

pub struct Machine {
    pub registers: [Word; 16],
    pub memory: [Word; 65536],
    pub pc: u16,
}

impl Machine {
    pub fn from_instructions(path: &str) -> Self {
        let mut memory = [Word::ZERO; 65536];

        memory
            .iter_mut()
            .zip(
                fs::read_to_string(path)
                    .expect("Failed to read file")
                    .split(',')
                    .map(|x| {
                        u16::from_str_radix(x.trim(), 16)
                            .expect(format!("{x} invalid hex value").as_str())
                    })
                    .map(|x| Word(x)),
            )
            .for_each(|(slot, value)| *slot = value);

        Machine {
            registers: [Word::ZERO; 16],
            memory,
            pc: 0,
        }
    }

    pub fn register(&self, register: Register) -> &Word {
        &self.registers[register as usize]
    }

    pub fn register_mut(&mut self, register: Register) -> &mut Word {
        &mut self.registers[register as usize]
    }

    fn take_instructions<const N: usize>(&self, pc: usize) -> [Word; N] {
        self.memory[pc..pc + N]
            .try_into()
            .expect("Invalid slice length")
    }

    pub fn execute<T: Instruction<N>, const N: usize>(&mut self) -> bool {
        let line = T::compile(self.take_instructions(self.pc as usize)).execute(self);

        match line {
            Line::Increment(amount) => {
                self.pc += amount;
                true
            }
            Line::Jump(address) => {
                self.pc = address;
                true
            }
            Line::Breakpoint => {
                println!("Breakpoint hit at {:#06x}", self.pc);
                // wait until return key is pressed
                //
                let mut _buffer = String::new();
                io::stdin().read_line(&mut _buffer).unwrap();

                self.pc += 1;
                true
            }
            Line::Halt => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register {
    R0 = 0,
    R1 = 1,
    R2 = 2,
    R3 = 3,
    R4 = 4,
    R5 = 5,
    R6 = 6,
    R7 = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl From<u8> for Register {
    fn from(value: u8) -> Self {
        match value {
            0 => Register::R0,
            1 => Register::R1,
            2 => Register::R2,
            3 => Register::R3,
            4 => Register::R4,
            5 => Register::R5,
            6 => Register::R6,
            7 => Register::R7,
            8 => Register::R8,
            9 => Register::R9,
            10 => Register::R10,
            11 => Register::R11,
            12 => Register::R12,
            13 => Register::R13,
            14 => Register::R14,
            15 => Register::R15,
            _ => panic!("Invalid register"),
        }
    }
}

pub struct Flags {
    pub g: bool,
    pub G: bool,
    pub E: bool,
    pub L: bool,
    pub l: bool,
    pub v: bool,
    pub V: bool,
    pub c: bool,
    pub S: bool,
    pub s: bool,
    pub f: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Flags {
            g: false,
            G: false,
            E: false,
            L: false,
            l: false,
            v: false,
            V: false,
            c: false,
            S: false,
            s: false,
            f: false,
        }
    }
}

impl From<Flags> for Word {
    fn from(value: Flags) -> Self {
        return Word(
            (value.g as u16)
                + ((value.G as u16) << 1)
                + ((value.E as u16) << 2)
                + ((value.L as u16) << 3)
                + ((value.l as u16) << 4)
                + ((value.v as u16) << 5)
                + ((value.V as u16) << 6)
                + ((value.c as u16) << 7)
                + ((value.S as u16) << 8)
                + ((value.s as u16) << 9)
                + ((value.f as u16) << 10),
        );
    }
}
