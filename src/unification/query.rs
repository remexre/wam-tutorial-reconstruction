use std::collections::{HashMap, HashSet};

use common::{FlatTermValue, Functor, Term, Variable};

use super::control::Instruction;

fn compile(
    seen: &mut HashSet<usize>,
    reg: usize,
    functor: Option<Functor>,
) -> Instruction {
    let new = seen.insert(reg);
    if let Some(functor) = functor {
        Instruction::PutStructure(functor, reg)
    } else if new {
        Instruction::SetVariable(reg)
    } else {
        Instruction::SetValue(reg)
    }
}

/// This is just a topological sort implemented via a depth-first search. It
/// should be called on each index of `flats`.
fn compile_visitor(
    code: &mut Vec<Instruction>,
    seen: &mut HashSet<usize>,
    flats: &mut Vec<Option<FlatTermValue>>,
    vars: &mut HashMap<Variable, usize>,
    current: usize,
) {
    if let Some(val) = flats[current].take() {
        match val {
            FlatTermValue::Structure(a, ref args) => {
                for &arg in args {
                    compile_visitor(code, seen, flats, vars, arg);
                }
                code.push(compile(seen, current, Some(Functor(a, args.len()))));
                for &arg in args {
                    code.push(compile(seen, arg, None));
                }
            }
            FlatTermValue::Variable(Some(var)) => {
                let overwrote = vars.insert(var, current);
                assert!(overwrote.is_none());
            }
            FlatTermValue::Variable(None) => {}
        }
    }
}

/// Compiles a term into instructions that will construct the term on the
/// heap, storing the root into the given register number.
pub fn compile_query(
    term: Term,
) -> (Vec<Instruction>, HashMap<Variable, usize>) {
    let mut flat = term.flatten().0.into_iter().map(Some).collect::<Vec<_>>();
    let mut seen = HashSet::with_capacity(flat.len());
    let mut code = Vec::new();
    let mut vars = HashMap::new();

    for i in 0..flat.len() {
        compile_visitor(&mut code, &mut seen, &mut flat, &mut vars, i);
    }
    assert!(flat.iter().all(Option::is_none));

    // This might happen if the FlatTerm is only variables, which should only
    // occur for a variable (or anonymous) term.
    if code.is_empty() {
        assert!(match term {
            Term::Anonymous | Term::Variable(_) => true,
            _ => false,
        });
        code = vec![Instruction::SetVariable(0)];
    }

    (code, vars)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_query_term;

    #[test]
    fn compiles_example_term() {
        assert_eq!(
            compile_query(example_query_term()),
            (
                vec![
                    Instruction::PutStructure(functor!(h / 2), 2),
                    Instruction::SetVariable(1),
                    Instruction::SetVariable(4),
                    Instruction::PutStructure(functor!(f / 1), 3),
                    Instruction::SetValue(4),
                    Instruction::PutStructure(functor!(p / 3), 0),
                    Instruction::SetValue(1),
                    Instruction::SetValue(2),
                    Instruction::SetValue(3),
                ],
                vec![(variable!("Z"), 1), (variable!("W"), 4)]
                    .into_iter()
                    .collect()
            )
        );
    }
}
