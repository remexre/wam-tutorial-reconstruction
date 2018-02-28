use std::fmt::{Display, Formatter, Result as FmtResult};

use common::Functor;

/// A single machine instruction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    PutStructure(Functor, usize),
    SetVariable(usize),
    SetValue(usize),

    GetStructure(Functor, usize),
    UnifyVariable(usize),
    UnifyValue(usize),
}

impl Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Instruction::PutStructure(f, r) => {
                write!(fmt, "put_structure {}, {}", f, r)
            }
            Instruction::SetVariable(r) => write!(fmt, "set_variable {}", r),
            Instruction::SetValue(r) => write!(fmt, "set_value {}", r),

            Instruction::GetStructure(f, r) => {
                write!(fmt, "get_structure {}, {}", f, r)
            }
            Instruction::UnifyVariable(r) => {
                write!(fmt, "unify_variable {}", r)
            }
            Instruction::UnifyValue(r) => write!(fmt, "unify_value {}", r),
        }
    }
}
