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
    /// Places the functor part of a structure onto the heap, storing its
    /// address in the location given by the second argument.
    PutStructure(Functor, Location),

    /// Places an unbound variable cell onto the heap, storing its address in
    /// the location given by the argument.
    SetVariable(Location),

    /// Places an reference to the value referenced by the register location
    /// given by the argument onto the heap.
    SetValue(Location),

    /// Inspects the value pointed to by the numbered register in preparation
    /// for unification with a functor.
    ///
    /// If the cell points to the same functor, unification proceeds with the
    /// machine in read mode, which unifies with each argument.
    ///
    /// If the cell points to an unbound reference, unification proceeds with
    /// the machine in write mode, which constructs the term on the heap.
    GetStructure(Functor, Location),

    /// Attempts to unify a variable.
    UnifyVariable(Location),

    /// Attempts to unify a value. See `Machine::unify`.
    UnifyValue(Location),

    GetValue(Location, Location),

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
            //Instruction::PutVariable(a, x) => {
            //write!(fmt, "put_variable {}, {}", a, x)
            //}
            //Instruction::PutValue(a, x) => {
            //write!(fmt, "put_value {}, {}", a, x)
            //}
            //Instruction::GetVariable(a, x) => {
            //write!(fmt, "get_variable {}, {}", a, x)
            //}
            Instruction::GetValue(a, x) => {
                write!(fmt, "get_value {}, {}", a, x)
            }
            Instruction::Allocate(n) => write!(fmt, "allocate {}", n),
            Instruction::Deallocate => fmt.write_str("deallocate"),
        }
    }
}
