use instructions::{
    add::Add, addc::AddC, cmp::Cmp, div::Div, divn::DivN, jal::Jal, jump::Jump, jumpc0::JumpC0,
    jumpc1::JumpC1, lea::Lea, load::Load, mul::Mul, muln::MulN, store::Store, sub::Sub, trap::Trap,
    Instruction,
};

mod instructions;
mod machine;
mod word;

fn main() {
    let mut machine = machine::Machine::from_instructions("lines.txt");

    // dbg!(&machine.memory);

    loop {
        let instruction = machine.memory[machine.pc as usize];
        let digits = instruction.hex_digits();

        if !match digits {
            (0x0, _, _, _) => machine.execute::<Add, { Add::WIDTH }>(),
            (0x1, _, _, _) => machine.execute::<Sub, { Sub::WIDTH }>(),
            (0x2, _, _, _) => machine.execute::<Mul, { Mul::WIDTH }>(),
            (0x3, _, _, _) => machine.execute::<Div, { Div::WIDTH }>(),
            (0x4, _, _, _) => machine.execute::<Cmp, { Cmp::WIDTH }>(),
            (0x5, _, _, _) => machine.execute::<AddC, { AddC::WIDTH }>(),
            (0x6, _, _, _) => machine.execute::<MulN, { MulN::WIDTH }>(),
            (0x7, _, _, _) => machine.execute::<DivN, { DivN::WIDTH }>(),
            (0xC, _, _, _) => machine.execute::<Trap, { Trap::WIDTH }>(),

            (0xF, _, _, 0) => machine.execute::<Lea, { Lea::WIDTH }>(),
            (0xF, _, _, 1) => machine.execute::<Load, { Load::WIDTH }>(),
            (0xF, _, _, 2) => machine.execute::<Store, { Store::WIDTH }>(),
            (0xF, _, _, 3) => machine.execute::<Jump, { Jump::WIDTH }>(),
            (0xF, _, _, 4) => machine.execute::<JumpC0, { JumpC0::WIDTH }>(),
            (0xF, _, _, 5) => machine.execute::<JumpC1, { JumpC1::WIDTH }>(),
            (0xF, _, _, 6) => machine.execute::<Jal, { Jal::WIDTH }>(),

            _ => panic!("Invalid instruction"),
        } {
            println!("breaking at pc {} {:?}", machine.pc, digits);
            break;
        }
    }
}
