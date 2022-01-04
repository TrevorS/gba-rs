mod flags;
mod registers;
mod state;

use crate::cpu::flags::Flags;
use crate::cpu::registers::Registers;
use crate::cpu::state::State;

#[derive(Default)]
pub struct CPU {
    registers: Registers,
    flags: Flags,
    state: State,
}

impl CPU {
    pub fn step(&self) {}
}
