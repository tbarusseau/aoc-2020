use aoc_2020::vm::*;

pub fn solve_part1(input: &str) -> isize {
    let mut vm = Vm::from(input, false);

    loop {
        if vm.step() == VmState::RepeatingInstruction {
            break;
        }
    }

    vm.acc
}

fn fix_instruction(op: Op) -> Op {
    match op {
        Op::Jmp(i) => Op::Nop(i),
        Op::Nop(i) => Op::Jmp(i),
        _ => op,
    }
}

pub fn solve_part2(input: &str) -> isize {
    let mut vm = Vm::from(input, false);
    let mut fixed = false;
    let mut fixed_instructions: Vec<usize> = vec![];

    loop {
        let op = vm.memory[vm.ip];
        match op {
            Op::Jmp(_) | Op::Nop(_) => {
                if !fixed && !fixed_instructions.contains(&vm.ip) {
                    vm.memory[vm.ip] = fix_instruction(op);
                    fixed_instructions.push(vm.ip);
                    fixed = true;
                }
            }
            _ => {}
        }

        let r = vm.step();

        if r == VmState::RepeatingInstruction && fixed {
            vm.reset();
            fixed = false;
        }

        if r == VmState::Terminated {
            break;
        }
    }

    vm.acc
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
