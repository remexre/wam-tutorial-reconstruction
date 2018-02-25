use common::Functor;

/// The control component of the CESK machine.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Control {
    pub code: Vec<Instruction>,
    pub ip: usize,
}

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
