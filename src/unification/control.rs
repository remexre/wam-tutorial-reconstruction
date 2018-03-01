use std::fmt::{Display, Formatter, Result as FmtResult};

use common::Functor;

/// A single machine instruction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Places the functor part of a structure onto the heap, storing its
    /// address in the register numbered by the second argument.
    PutStructure(Functor, usize),

    /// Places an unbound variable cell onto the heap, storing its address in
    /// the register numbered by the argument.
    SetVariable(usize),

    /// Places an reference to the value referenced by the register numbered by
    /// the argument onto the heap.
    SetValue(usize),

    /// Inspects the value pointed to by the numbered register in preparation
    /// for unification with a functor.
    ///
    /// If the cell points to the same functor, unification proceeds with the
    /// machine in read mode, which unifies with each argument.
    ///
    /// If the cell points to an unbound reference, unification proceeds with
    /// the machine in write mode, which constructs the term on the heap.
    GetStructure(Functor, usize),

    /// Attempts to unify a variable.
    UnifyVariable(usize),

    /// Attempts to unify a value. See `Machine::unify`.
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
