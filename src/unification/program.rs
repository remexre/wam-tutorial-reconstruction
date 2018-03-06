use std::collections::HashSet;

use common::{Functor, Term};

use super::control::Instruction;
use super::flatten::{FlatTerm, FlatTermValue};

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
pub fn compile_program(term: &Term) -> Vec<Instruction> {
    let flat = FlatTerm::flatten(term).0;
    let mut seen = HashSet::with_capacity(flat.len());
    let mut code = Vec::new();

    for (i, v) in flat.into_iter().enumerate() {
        if let FlatTermValue::Structure(f, args) = v {
            code.push(compile(&mut seen, i, Some(Functor(f, args.len()))));
            for arg in args {
                code.push(compile(&mut seen, arg, None));
            }
        }
    }

    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_program_term;

    #[test]
    fn compiles_example_term() {
        assert_eq!(
            compile_program(&example_program_term()),
            vec![
                Instruction::GetStructure(functor!(p / 3), 0),
                Instruction::UnifyVariable(1),
                Instruction::UnifyVariable(2),
                Instruction::UnifyVariable(3),
                Instruction::GetStructure(functor!(f / 1), 1),
                Instruction::UnifyVariable(4),
                Instruction::GetStructure(functor!(h / 2), 2),
                Instruction::UnifyValue(3),
                Instruction::UnifyVariable(5),
                Instruction::GetStructure(functor!(f / 1), 5),
                Instruction::UnifyVariable(6),
                Instruction::GetStructure(functor!(a / 0), 6),
            ]
        );
    }
}
