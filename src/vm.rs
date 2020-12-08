use itertools::Itertools;
use std::collections::HashSet;

use Op::*;

pub struct Vm {
    pub memory: Vec<Op>,
    pub acc: isize,
    pub lp: Option<usize>,
    pub ip: usize,
    debug: bool,
    pub inner: VmInner,
}

pub struct VmInner {
    memory_snapshot: Vec<Op>,
    pub repeat_detection: HashSet<usize>,
}

#[derive(PartialEq)]
pub enum VmState {
    Running,
    RepeatingInstruction,
    Terminated,
}
use VmState::*;

impl Vm {
    pub fn from(input: &str, debug: bool) -> Vm {
        Vm {
            memory: input.lines().map(Op::from).collect(),
            acc: 0,
            lp: None,
            ip: 0,
            debug,
            inner: VmInner {
                memory_snapshot: input.lines().map(Op::from).collect(),
                repeat_detection: HashSet::new(),
            },
        }
    }

    pub fn reset(&mut self) {
        self.acc = 0;
        self.ip = 0;
        self.lp = None;
        self.memory = self.inner.memory_snapshot.clone();
        self.inner.repeat_detection = HashSet::new();
    }

    pub fn step(&mut self) -> VmState {
        self.lp = Some(self.ip);

        let op = self.memory[self.ip];
        op.step(self);

        if self.inner.repeat_detection.contains(&self.ip) {
            if self.debug {
                let op = self.memory[self.ip];
                println!("Repeating instruction: index {}, {:?}", self.ip, op);
            }
            return RepeatingInstruction;
        } else {
            self.inner.repeat_detection.insert(self.ip);
        }

        if self.ip == self.memory.len() {
            Terminated
        } else {
            Running
        }
    }
}

#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Op {
    pub fn from(line: &str) -> Op {
        let (instruction, value) = line
            .split(' ')
            .collect_tuple()
            .expect("Expected two elements");
        let value = value
            .parse::<isize>()
            .unwrap_or_else(|_| panic!("Couldn't parse value: {}", value));

        match instruction {
            "nop" => Nop(value),
            "acc" => Acc(value),
            "jmp" => Jmp(value),
            _ => panic!("Invalid instruction: {}", instruction),
        }
    }

    pub fn step(&self, vm: &mut Vm) {
        if vm.debug {
            println!("{:?}", self)
        }

        let mut increment_ip = true;

        match self {
            Acc(i) => vm.acc += i,
            Jmp(i) => {
                vm.ip = (vm.ip as isize + i) as usize;
                increment_ip = false;
            }
            Nop(_) => {}
        }

        if increment_ip {
            vm.ip += 1;
        }
    }
}
