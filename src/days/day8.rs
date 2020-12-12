use aoc_2020::vm::{instruction::Instruction, op::Op::*, *};
use std::collections::HashSet;

pub fn solve_part1(input: &str) -> Result<isize, VmError> {
    let mut vm = Vm::from(input, false)?;
    let mut repeat = HashSet::new();

    while !repeat.contains(&vm.ip) {
        repeat.insert(vm.ip);
        vm.step();
    }

    Ok(vm.acc)
}

pub fn solve_part2(input: &str) -> Result<isize, VmError> {
    let mut vm = Vm::from(input, false)?;
    let mut repeat = HashSet::new();
    let mut fixed = false;
    let mut fixed_instructions: Vec<usize> = vec![];

    loop {
        let index = vm.ip;
        let instruction = vm.instructions[index];
        match instruction.op {
            o @ Jmp | o @ Nop => {
                if !fixed && !fixed_instructions.contains(&index) {
                    let mut patched_instruction = Instruction::from(&instruction);
                    if o == Jmp {
                        patched_instruction.op = Nop;
                    } else {
                        patched_instruction.op = Jmp;
                    }

                    vm.patch_instruction(index, patched_instruction);
                    fixed_instructions.push(index);
                    fixed = true;
                }
            }
            _ => {}
        }

        let r = vm.step();

        if repeat.contains(&vm.ip) {
            if fixed {
                repeat.clear();
                vm.reset();
                fixed = false;
            }
        } else {
            repeat.insert(vm.ip);
        }

        if r == VmState::Terminated {
            break;
        }
    }

    Ok(vm.acc)
}

#[test]
pub fn test_part1() {
    assert_eq!(
        solve_part1(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
        ),
        5,
    );
}

#[test]
pub fn test_part2() {
    assert_eq!(
        solve_part2(
            "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"
        ),
        8,
    )
}
