pub mod instruction;
pub mod op;

use instruction::Instruction;
use std::{convert::TryFrom, num::ParseIntError};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum VmError {
    #[error("couldn't parse input")]
    InputError,
    #[error("invalid operation")]
    InvalidOperation,
    #[error("invalid number of operands")]
    InvalidNumberOfOperands,
    #[error("numeral parsing error")]
    ParsingError(#[from] ParseIntError),
    #[error("nom parsing error: {0}")]
    NomError(String),
}

pub struct Vm {
    /// Instructions held in memory
    pub instructions: Vec<Instruction>,
    /// Accumulator value, can be modified with the `acc` instruction
    pub acc: isize,
    /// Last instruction pointer
    pub lp: Option<usize>,
    /// Instruction pointer
    pub ip: usize,
    /// Debug flag: if enabled, logs execution
    debug: bool,
    /// Used to reset the virtual machine's state if needed
    snapshot: Vec<Instruction>,
}

#[derive(PartialEq)]
pub enum VmState {
    Running,
    Terminated,
}
use VmState::*;

impl TryFrom<&str> for Vm {
    type Error = VmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let instructions: Vec<Result<Instruction, _>> =
            value.lines().map(Instruction::try_from).collect();
        if instructions.iter().any(Result::is_err) {
            return Err(VmError::InputError);
        }

        Ok(Vm {
            instructions: instructions.iter().flatten().cloned().collect(),
            acc: 0,
            lp: None,
            ip: 0,
            debug: false,
            snapshot: instructions.iter().flatten().cloned().collect(),
        })
    }
}

impl Vm {
    pub fn from(input: &str, debug: bool) -> Result<Vm, VmError> {
        let mut vm = Vm::try_from(input)?;
        vm.debug = debug;
        Ok(vm)
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
        self.lp = None;
        self.instructions = self.snapshot.clone();
    }

    pub fn step(&mut self) -> VmState {
        self.lp = Some(self.ip);

        let op = self.instructions[self.ip];
        op.step(self);

        if self.ip == self.instructions.len() {
            Terminated
        } else {
            Running
        }
    }

    pub fn patch_instruction(&mut self, index: usize, instruction: Instruction) {
        self.instructions[index] = instruction;
    }
}
