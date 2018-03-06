use std::collections::{HashMap, HashSet};

use common::{Atom, Functor, Structure, Term, Variable};

use super::super::{Instruction, Location};

/// A type for flattened terms.
#[derive(Clone, Debug, Eq, PartialEq)]
enum FlatTerm {
    Functor(Atom, Vec<usize>),
    Ref(usize),
    Variable,
}

/// Used in the first pass to flatten functor arguments, storing variables in a
/// map.
fn flatten_term_to(
    flattened: &mut Vec<Option<FlatTerm>>,
    vars: &mut HashMap<Variable, usize>,
    i: usize,
    term: &Term,
) {
    match *term {
        Term::Anonymous => {
            flattened[i] = Some(FlatTerm::Variable);
        }
        Term::Structure(Structure(f, ref ts)) => {
            let mut is = Vec::new();
            let n = flattened.len();
            for _ in 0..ts.len() {
                flattened.push(None);
            }
            for (j, t) in ts.iter().enumerate() {
                match *t {
                    Term::Variable(ref v) if vars.contains_key(v) => {
                        is.push(vars[v]);
                    }
                    _ => {
                        let i = n + j;
                        flatten_term_to(flattened, vars, i, t);
                        is.push(i);
                    }
                }
            }
            flattened[i] = Some(FlatTerm::Functor(f, is));
        }
        Term::Variable(v) => {
            assert!(!vars.contains_key(&v));
            flattened[i] = Some(FlatTerm::Variable);
            vars.insert(v, i);
        }
    }
}

/// Flattens a structure to the form needed for fact compilation.
fn flatten(structure: &Structure) -> Vec<FlatTerm> {
    // Initially contains placeholders for argument registers.
    let mut flattened = vec![None; structure.1.len()];
    let mut vars = HashMap::new();

    // The first pass flattens functor arguments only.
    for (i, term) in structure.1.iter().enumerate() {
        match *term {
            Term::Structure(_) => {
                flatten_term_to(&mut flattened, &mut vars, i, term)
            }
            _ => { /* Wait for the second pass. */ }
        }
    }

    // The second pass handles variable and anonymous arguments.
    for (i, term) in structure.1.iter().enumerate() {
        match *term {
            Term::Anonymous => {
                // It's quite possible that this can be optimized to
                // flattened[i] = Some(FlatTerm::Variable);
                let n = flattened.len();
                flattened.push(Some(FlatTerm::Variable));
                flattened[i] = Some(FlatTerm::Ref(n));
            }
            Term::Variable(v) => {
                if let Some(&n) = vars.get(&v) {
                    flattened[i] = Some(FlatTerm::Ref(n));
                } else {
                    let n = flattened.len();
                    flattened.push(Some(FlatTerm::Variable));
                    flattened[i] = Some(FlatTerm::Ref(n));
                    vars.insert(v, n);
                }
            }
            _ => { /* This should have already been handled. */ }
        }
    }

    flattened.into_iter().map(|f| f.unwrap()).collect()
}

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
                code.push(Instruction::GetValue(
                    Location::Register(j),
                    Location::Register(i),
                ));
            }
            FlatTerm::Variable => { /* No code needs be emitted here. */ }
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
                FlatTerm::Variable,                   // X
                FlatTerm::Variable,                   // Y
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
                Instruction::GetValue(
                    Location::Register(4),
                    Location::Register(2),
                ),
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
