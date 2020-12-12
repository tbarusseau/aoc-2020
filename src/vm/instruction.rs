use std::convert::TryFrom;

use nom::{
    branch::alt,
    character::complete::{alpha1, char, digit1, space1},
    combinator::{map_res, opt, recognize},
    sequence::{terminated, tuple},
    Finish, IResult,
};

use super::{
    op::Op::{self, *},
    Vm, VmError,
};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Instruction {
    pub op: Op,
    pub lhs: isize,
    pub rhs: Option<isize>,
}

impl Instruction {
    pub fn from(instruction: &Instruction) -> Instruction {
        instruction.clone()
    }

    pub fn step(self, vm: &mut Vm) {
        if vm.debug {
            println!("Stepping with: {:?}", self);
        }

        let mut increment_ip = true;
        match self.op {
            Nop => {}
            Acc => {
                vm.acc += self.lhs;
            }
            Jmp => {
                vm.ip = (vm.ip as isize + self.lhs) as usize;
                increment_ip = false;
            }
        }

        if increment_ip {
            vm.ip += 1;
        }
    }
}

fn op(input: &str) -> IResult<&str, Op> {
    map_res(alpha1, |s| Op::try_from(s))(input)
}

fn signed_integer(input: &str) -> IResult<&str, isize> {
    map_res(
        recognize(tuple((alt((char('-'), char('+'))), digit1))),
        |s: &str| s.parse::<isize>(),
    )(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Result<Instruction, VmError>> {
    let (input, op) = terminated(op, space1)(input)?;
    let (input, lhs) = terminated(signed_integer, opt(space1))(input)?;
    let (input, rhs) = opt(signed_integer)(input)?;

    Ok((input, Instruction::try_from((op, lhs, rhs))))
}

impl TryFrom<(Op, isize, Option<isize>)> for Instruction {
    type Error = VmError;

    fn try_from(value: (Op, isize, Option<isize>)) -> Result<Self, Self::Error> {
        let i = Instruction {
            op: value.0,
            lhs: value.1,
            rhs: value.2,
        };

        // Check number of operands
        match i.op {
            Nop | Acc | Jmp => {
                if i.rhs.is_some() {
                    return Err(VmError::InvalidNumberOfOperands);
                } else {
                    Ok(i)
                }
            }
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = VmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match parse_instruction(value).finish() {
            Ok((_, Ok(i))) => Ok(i),
            Ok((_, Err(e))) => Err(e),
            Err(e) => Err(VmError::NomError(e.to_string())),
        }
    }
}
