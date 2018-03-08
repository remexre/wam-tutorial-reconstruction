use std::collections::HashSet;

use common::{Functor, Structure};

use super::flatten::{flatten, FlatTerm};
use super::super::{Instruction, Location};

/// Compiles a fact into a series of instructions.
pub fn compile(fact: &Structure) -> Vec<Instruction> {
    let mut code = Vec::new();
    let mut seen = HashSet::new();
    for (i, flat) in flatten(fact).into_iter().enumerate() {
        match flat {
            FlatTerm::Functor(a, js) => {
                code.push(Instruction::GetStructure(
                    Functor(a, js.len()),
                    Location::Register(i),
                ));
                for j in js {
                    if seen.contains(&j) {
                        code.push(Instruction::UnifyValue(
                            Location::Register(j),
                        ));
                    } else {
                        code.push(Instruction::UnifyVariable(
                            Location::Register(j),
                        ));
                        seen.insert(j);
                    }
                }
            }
            FlatTerm::Ref(j) => {
                assert!(j > i);
                code.push(Instruction::GetValue(Location::Register(j), i));
            }
            FlatTerm::Variable(_) => { /* No code needs be emitted here. */ }
        }
    }
    code.push(Instruction::Proceed);
    code
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_program;

    #[test]
    fn flattens_example_program() {
        assert_eq!(
            flatten(&example_program()),
            vec![
                FlatTerm::Functor(atom!(f), vec![3]), // f(X)
                FlatTerm::Functor(atom!(h), vec![4, 5]), // h(Y, f(a))
                FlatTerm::Ref(4),                     // points to Y
                FlatTerm::Variable(Some(variable!("X"))), // X
                FlatTerm::Variable(Some(variable!("Y"))), // Y
                FlatTerm::Functor(atom!(f), vec![6]), // f(a)
                FlatTerm::Functor(atom!(a), vec![]),  // a
            ],
        );
    }

    #[test]
    fn compiles_example_program() {
        assert_eq!(
            compile(&example_program()),
            vec![
                Instruction::GetStructure(
                    functor!(f / 1),
                    Location::Register(0),
                ),
                Instruction::UnifyVariable(Location::Register(3)),
                Instruction::GetStructure(
                    functor!(h / 2),
                    Location::Register(1),
                ),
                Instruction::UnifyVariable(Location::Register(4)),
                Instruction::UnifyVariable(Location::Register(5)),
                Instruction::GetValue(Location::Register(4), 2),
                Instruction::GetStructure(
                    functor!(f / 1),
                    Location::Register(5),
                ),
                Instruction::UnifyVariable(Location::Register(6)),
                Instruction::GetStructure(
                    functor!(a / 0),
                    Location::Register(6),
                ),
                Instruction::Proceed,
            ],
        );
    }
}
