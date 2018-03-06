use std::collections::HashMap;
use std::collections::HashSet;

use common::{Clause, FlatTermValue, Functor, Structure, Term};

use super::control::Instruction;

fn compile(
    seen: &mut HashSet<usize>,
    reg: usize,
    functor: Option<Functor>,
) -> Instruction {
    let new = seen.insert(reg);
    if let Some(functor) = functor {
        Instruction::GetStructure(functor, reg)
    } else if new {
        Instruction::UnifyVariable(reg)
    } else {
        Instruction::UnifyValue(reg)
    }
}

/// Compiles a "program" (a term to unify against) into instructions.
pub fn compile_program(
    program: Vec<Clause>,
) -> (Vec<Instruction>, HashMap<Functor, usize>) {
    let mut code = Vec::new();
    let mut labels = HashMap::new();

    for Clause(head, body) in program {
        //labels.insert(clause.functor(), code.len());

        unimplemented!();
        code.push(Instruction::Proceed);
    }

    (code, labels)
}
