use std::convert::TryFrom;

use super::VmError;

#[non_exhaustive]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Op {
    /// No-op
    Nop,
    /// Add to the accumulator
    Acc,
    /// Jump
    Jmp,
}

use Op::*;

impl TryFrom<&str> for Op {
    type Error = VmError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "nop" => Ok(Nop),
            "acc" => Ok(Acc),
            "jmp" => Ok(Jmp),
            _ => Err(VmError::InvalidOperation),
        }
    }
}
