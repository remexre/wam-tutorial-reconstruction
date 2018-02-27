use std::collections::HashSet;

use common::{FlatTermValue, Functor, Term};

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
    current: usize,
) {
    if let Some(val) = flats[current].take() {
        if let FlatTermValue::Structure(a, ref args) = val {
            for &arg in args {
                compile_visitor(code, seen, flats, arg);
            }
            code.push(compile(seen, current, Some(Functor(a, args.len()))));
            for &arg in args {
                code.push(compile(seen, arg, None));
            }
        }
    }
}

/// Compiles a term into instructions that will construct the term on the
/// heap, storing the root into the given register number.
pub fn compile_query(term: Term) -> Vec<Instruction> {
    let mut flat = term.flatten().0.into_iter().map(Some).collect::<Vec<_>>();
    let mut code = Vec::with_capacity(flat.len());
    let mut seen = HashSet::with_capacity(flat.len());

    for i in 0..flat.len() {
        compile_visitor(&mut code, &mut seen, &mut flat, i);
    }
    assert!(flat.iter().all(Option::is_none));

    // This might happen if the FlatTerm is only variables, which should only
    // occur for a variable (or anonymous) term.
    if code.is_empty() {
        assert!(match term {
            Term::Anonymous | Term::Variable(_) => true,
            _ => false,
        });
        vec![Instruction::SetVariable(0)]
    } else {
        code
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_term;

    #[test]
    fn compiles_example_term() {
        assert_eq!(
            compile_query(example_term()),
            vec![
                Instruction::PutStructure(Functor("h".into(), 2), 2),
                Instruction::SetVariable(1),
                Instruction::SetVariable(4),
                Instruction::PutStructure(Functor("f".into(), 1), 3),
                Instruction::SetValue(4),
                Instruction::PutStructure(Functor("p".into(), 3), 0),
                Instruction::SetValue(1),
                Instruction::SetValue(2),
                Instruction::SetValue(3),
            ]
        );
    }
}
