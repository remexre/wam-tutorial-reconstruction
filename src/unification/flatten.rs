use common::{Atom, Env, Structure, Term, Variable};

/// A breadth-first search to flatten a term.
///
/// This isn't able to be the (faster) depth-first search, since indices are
/// assigned in traversal order.
fn flatten_term_onto<'t>(
    regs: &mut Vec<FlatTermValue>,
    env: &mut Env<Variable, usize>,
    i: usize,
    term: &'t Term,
) {
    match *term {
        Term::Anonymous => {
            regs[i] = FlatTermValue::Variable(None);
        }
        Term::Structure(Structure(f, ref ts)) => {
            let mut is = Vec::new();
            let mut subterms = Vec::new();
            for t in ts.iter() {
                match *t {
                    Term::Variable(var) => {
                        is.push(if let Some(&n) = env.get(var) {
                            n
                        } else {
                            let n = regs.len();
                            regs.push(FlatTermValue::Variable(Some(var)));
                            env.push(var, n);
                            n
                        });
                    }
                    _ => {
                        let n = regs.len();
                        // Push a placeholder.
                        regs.push(FlatTermValue::Variable(None));
                        subterms.push((n, t));
                        is.push(n);
                    }
                }
            }
            regs[i] = FlatTermValue::Structure(f, is);
            for (i, t) in subterms {
                flatten_term_onto(regs, env, i, t);
            }
        }
        Term::Variable(var) => {
            regs[i] = FlatTermValue::Variable(Some(var));
        }
    }
}

/// A value in a flattened term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlatTermValue {
    /// A structure.
    Structure(Atom, Vec<usize>),

    /// A variable, possibly with a name.
    Variable(Option<Variable>),
}

/// A flattened term.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatTerm(pub Vec<FlatTermValue>);

impl FlatTerm {
    /// Converts a Term into a flattened form.
    pub fn flatten(term: &Term) -> FlatTerm {
        // We start with a placeholder.
        let mut regs = vec![FlatTermValue::Variable(None)];
        let mut env = Env::new();
        flatten_term_onto(&mut regs, &mut env, 0, term);
        FlatTerm(regs)
    }
}

#[cfg(test)]
mod tests {
    use test_utils::{arb_term, example_query_term};

    use super::*;

    #[test]
    fn flattens_example_query_term() {
        assert_eq!(
            FlatTerm::flatten(&example_query_term()),
            FlatTerm(vec![
                FlatTermValue::Structure(atom!(p), vec![1, 2, 3]),
                FlatTermValue::Variable(Some(variable!("Z"))),
                FlatTermValue::Structure(atom!(h), vec![1, 4]),
                FlatTermValue::Structure(atom!(f), vec![4]),
                FlatTermValue::Variable(Some(variable!("W"))),
            ])
        );
    }

    #[test]
    fn flattens_simple_terms() {
        assert_eq!(
            FlatTerm::flatten(&Term::Anonymous),
            FlatTerm(vec![FlatTermValue::Variable(None)])
        );

        assert_eq!(
            FlatTerm::flatten(&Term::Variable(variable!("X"))),
            FlatTerm(vec![FlatTermValue::Variable(Some(variable!("X")))])
        );

        assert_eq!(
            FlatTerm::flatten(&Term::Structure(Structure(
                "foo".into(),
                vec![
                    Term::Variable(variable!("X")),
                    Term::Variable(variable!("X")),
                ]
            ))),
            FlatTerm(vec![
                FlatTermValue::Structure(atom!(foo), vec![1, 1]),
                FlatTermValue::Variable(Some(variable!("X"))),
            ])
        );
    }

    proptest! {
        #[test]
        fn flatten_term_doesnt_crash(ref term in arb_term(5, 5)) {
            FlatTerm::flatten(term);
        }
    }
}
