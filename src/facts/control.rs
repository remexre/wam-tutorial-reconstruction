use std::fmt::{Display, Formatter, Result as FmtResult};

use common::Functor;

/// A single M<sub>1</sub> instruction.
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

    /// Jumps to the code for the fact with the given functor.
    Call(Functor),

    /// A no-op.
    Proceed,

    /// x
    PutVariable(usize, usize),
    PutValue(usize, usize),
    GetVariable(usize, usize),
    GetValue(usize, usize),
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
            Instruction::Call(f) => write!(fmt, "call {}", f),
            Instruction::Proceed => fmt.write_str("proceed"),
            Instruction::PutVariable(a, x) => {
                write!(fmt, "put_variable {}, {}", a, x)
            }
            Instruction::PutValue(a, x) => {
                write!(fmt, "put_value {}, {}", a, x)
            }
            Instruction::GetVariable(a, x) => {
                write!(fmt, "get_variable {}, {}", a, x)
            }
            Instruction::GetValue(a, x) => {
                write!(fmt, "get_value {}, {}", a, x)
            }
        }
    }
}
