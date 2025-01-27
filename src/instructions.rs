use crate::{machine::Machine, word::Word};

pub mod add;
pub mod addc;
pub mod cmp;
pub mod div;
pub mod divn;
pub mod jal;
pub mod jump;
pub mod jumpc0;
pub mod jumpc1;
pub mod lea;
pub mod load;
pub mod mul;
pub mod muln;
pub mod store;
pub mod sub;
pub mod trap;

pub enum Line {
    Increment(u16),
    Jump(u16),
    Halt,
    Breakpoint,
}

pub trait Instruction<const WIDTH: usize> {
    const WIDTH: usize = WIDTH;

    fn execute(&mut self, system: &mut Machine) -> Line;

    fn compile(instruction: [Word; WIDTH]) -> Self;
}
