mod clause;
mod fact;
mod flatten;

use std::collections::HashMap;

use failure::Error;

use common::{Clause, Functor, Structure, Variable};

use super::control::Instruction;
use self::clause::compile as compile_clause_helper;
use self::fact::compile as compile_fact;

/// Compiles a single clause in a program into a series of instructions.
fn compile_clause(clause: &Clause) -> Vec<Instruction> {
    let Clause(ref head, ref body) = *clause;

    if body.is_empty() {
        compile_fact(head)
    } else {
        compile_clause_helper(Some(head), body).0
    }
}

/// Compiles a program into a series of instructions. Also returns a list of
/// labels.
pub fn compile_program(
    program: &[Clause],
) -> Result<(Vec<Instruction>, HashMap<Functor, usize>), Error> {
    let mut code = Vec::new();
    let mut labels = HashMap::new();

    for clause in program {
        let addr = code.len();
        ensure!(
            labels.insert(clause.0.functor(), addr).is_none(),
            "M2 doesn't support disjunctions"
        );
        code.extend(compile_clause(clause));
    }
    assert_eq!(program.len(), labels.len());

    Ok((code, labels))
}

/// Compiles a query into a series of instructions. Also returns a list of
/// variable assignments.
pub fn compile_query(query: &[Structure]) -> (Vec<Instruction>, Vec<Variable>) {
    compile_clause_helper(None, query)
}

#[cfg(test)]
mod tests {
    use common::Term;
    use super::*;

    #[test]
    fn compiles_example_conjunctive_program() {
        let program = Clause(
            Structure(
                atom!(p),
                vec![
                    Term::Variable(variable!("X")),
                    Term::Variable(variable!("Y")),
                ],
            ),
            vec![
                Structure(
                    atom!(q),
                    vec![
                        Term::Variable(variable!("X")),
                        Term::Variable(variable!("Z")),
                    ],
                ),
                Structure(
                    atom!(q),
                    vec![
                        Term::Variable(variable!("Z")),
                        Term::Variable(variable!("Y")),
                    ],
                ),
            ],
        );
        assert_eq!(
            compile_clause(&program),
            vec![
                Instruction::Allocate(2),
                // get_variable X3, A1
                // get_variable Y1, A2
                // put_value X3, A1
                // put_variable Y2, A2
                Instruction::Call(functor!(q / 2)),
                // put_value Y2, A1
                // put_value Y1, A2
                Instruction::Call(functor!(r / 2)),
                Instruction::Deallocate,
            ],
        );
    }
}
