use std::collections::HashMap;

use common::{Atom, Structure, Term, Variable};

/// A type for flattened terms.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FlatTerm {
    Functor(Atom, Vec<usize>),
    Ref(usize),
    Variable(Option<Variable>),
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
            flattened[i] = Some(FlatTerm::Variable(None));
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
            flattened[i] = Some(FlatTerm::Variable(Some(v)));
            vars.insert(v, i);
        }
    }
}

/// Flattens a structure to the form needed for fact compilation.
pub fn flatten(structure: &Structure) -> Vec<FlatTerm> {
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
                flattened.push(Some(FlatTerm::Variable(None)));
                flattened[i] = Some(FlatTerm::Ref(n));
            }
            Term::Variable(v) => {
                if let Some(&n) = vars.get(&v) {
                    flattened[i] = Some(FlatTerm::Ref(n));
                } else {
                    let n = flattened.len();
                    flattened.push(Some(FlatTerm::Variable(Some(v))));
                    flattened[i] = Some(FlatTerm::Ref(n));
                    vars.insert(v, n);
                }
            }
            _ => { /* This should have already been handled. */ }
        }
    }

    flattened.into_iter().map(|f| f.unwrap()).collect()
}
