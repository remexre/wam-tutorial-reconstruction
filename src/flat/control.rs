use std::fmt::{Display, Formatter, Result as FmtResult};

use common::Functor;

/// A local address, which corresponds to either a register or an entry on the
/// local stack.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Location {
    /// A local variable, aka a local stack entry.
    Local(usize),

    /// A register.
    Register(usize),
}

impl Display for Location {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Location::Local(n) => write!(fmt, "Y{}", n),
            Location::Register(n) => write!(fmt, "X{}", n),
        }
    }
}

/// A single M<sub>2</sub> instruction.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Inspects the value pointed to by the numbered register in preparation
    /// for unification with a functor.
    ///
    /// If the cell points to the same functor, unification proceeds with the
    /// machine in read mode, which unifies with each argument.
    ///
    /// If the cell points to an unbound reference, unification proceeds with
    /// the machine in write mode, which constructs the term on the heap.
    GetStructure(Functor, Location),

    GetValue(Location, usize),
    GetVariable(Location, usize),

    /// Places the functor part of a structure onto the heap, storing its
    /// address in the location given by the second argument.
    PutStructure(Functor, Location),

    PutValue(Location, usize),
    PutVariable(Location, usize),

    /// Attempts to unify a value. See `Machine::unify`.
    UnifyValue(Location),

    /// Attempts to unify a variable.
    UnifyVariable(Location),

    /// Calls the code for the fact with the given functor, with support for
    /// faster leaf calls.
    Call(Functor),

    /// A return from a leaf function / a fact.
    Proceed,

    /// An allocation of n + 2 stack slots.
    Allocate(usize),

    /// A `leave` and `ret` in one instruction.
    Deallocate,
}

impl Display for Instruction {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Instruction::GetStructure(f, reg) => {
                write!(fmt, "get_structure {}, {}", f, reg)
            }
            Instruction::GetValue(loc, reg) => {
                write!(fmt, "get_value {}, {}", loc, reg)
            }
            Instruction::GetVariable(loc, reg) => {
                write!(fmt, "get_variable {}, {}", loc, reg)
            }

            Instruction::PutStructure(f, reg) => {
                write!(fmt, "put_structure {}, {}", f, reg)
            }
            Instruction::PutValue(loc, reg) => {
                write!(fmt, "put_value {}, {}", loc, reg)
            }
            Instruction::PutVariable(loc, reg) => {
                write!(fmt, "put_variable {}, {}", loc, reg)
            }

            Instruction::UnifyValue(r) => write!(fmt, "unify_value {}", r),
            Instruction::UnifyVariable(r) => {
                write!(fmt, "unify_variable {}", r)
            }

            Instruction::Call(f) => write!(fmt, "call {}", f),
            Instruction::Proceed => fmt.write_str("proceed"),

            Instruction::Allocate(n) => write!(fmt, "allocate {}", n),
            Instruction::Deallocate => fmt.write_str("deallocate"),
        }
    }
}
