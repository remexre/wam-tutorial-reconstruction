use super::{Atom, Env, Term, Variable};

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
            regs[i] = FlatTermValue::Variable;
        }
        Term::Structure(f, ref ts) => {
            let mut is = Vec::new();
            let mut subterms = Vec::new();
            for t in ts.iter() {
                if let Term::Variable(v) = *t {
                    is.push(if let Some(&n) = env.get(v) {
                        n
                    } else {
                        let n = regs.len();
                        regs.push(FlatTermValue::Variable);
                        env.push(v, n);
                        n
                    });
                } else {
                    let n = regs.len();
                    regs.push(FlatTermValue::Variable);
                    subterms.push((n, t));
                    is.push(n);
                }
            }
            regs[i] = FlatTermValue::Structure(f, is);
            for (i, t) in subterms {
                flatten_term_onto(regs, env, i, t);
            }
        }
        Term::Variable(_) => {
            regs[i] = FlatTermValue::Variable;
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlatTermValue {
    Structure(Atom, Vec<usize>),
    Variable,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatTerm(pub Vec<FlatTermValue>);

impl Term {
    /// Converts a Term into a flattened form.
    pub fn flatten(&self) -> FlatTerm {
        let mut regs = vec![FlatTermValue::Variable];
        let mut env = Env::new();
        flatten_term_onto(&mut regs, &mut env, 0, self);
        FlatTerm(regs)
    }
}

#[cfg(test)]
mod tests {
    use test_utils::{arb_term, example_term};

    use super::*;

    #[test]
    fn flattens_example_term() {
        assert_eq!(
            example_term().flatten(),
            FlatTerm(vec![
                FlatTermValue::Structure("p".into(), vec![1, 2, 3]),
                FlatTermValue::Variable,
                FlatTermValue::Structure("h".into(), vec![1, 4]),
                FlatTermValue::Structure("f".into(), vec![4]),
                FlatTermValue::Variable,
            ])
        );
    }

    #[test]
    fn flattens_simple_things() {
        assert_eq!(
            Term::Anonymous.flatten(),
            FlatTerm(vec![FlatTermValue::Variable])
        );

        assert_eq!(
            Term::Variable(variable!("X")).flatten(),
            FlatTerm(vec![FlatTermValue::Variable])
        );

        assert_eq!(
            Term::Structure(
                "foo".into(),
                vec![
                    Term::Variable(variable!("X")),
                    Term::Variable(variable!("X")),
                ]
            ).flatten(),
            FlatTerm(vec![
                FlatTermValue::Structure("foo".into(), vec![1, 1]),
                FlatTermValue::Variable,
            ])
        );
    }

    proptest! {
        #[test]
        fn flatten_doesnt_crash(ref term in arb_term(5, 5)) {
            term.flatten();
        }
    }
}
