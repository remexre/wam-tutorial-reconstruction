use super::{Atom, Env, Term, Variable};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlatTermValue {
    Structure(Atom, Vec<usize>),
    Variable,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatTerm(pub Vec<(usize, FlatTermValue)>);

impl FlatTerm {
    /// Converts a Term into this flattened form.
    pub fn flatten_term(term: Term) -> FlatTerm {
        fn flatten_term_onto(
            regs: &mut Vec<FlatTermValue>,
            env: &mut Env<Variable, usize>,
            term: Term,
        ) -> usize {
            match term {
                Term::Anonymous => {
                    let n = regs.len();
                    regs.push(FlatTermValue::Variable);
                    n
                }
                Term::Structure(f, ts) => {
                    // Store the index at which the value will be stored.
                    let n = regs.len();

                    // Push a temporary.
                    regs.push(FlatTermValue::Variable);

                    // Flatten all the subterms.
                    let flat_terms = ts.into_iter()
                        .map(|t| flatten_term_onto(regs, env, t))
                        .collect();

                    // Actually put the flat term value into the registers.
                    assert!(regs[n] == FlatTermValue::Variable);
                    regs[n] = FlatTermValue::Structure(f, flat_terms);

                    // Return the index.
                    n
                }
                Term::Variable(v) => if let Some(&n) = env.get(v) {
                    n
                } else {
                    let n = regs.len();
                    regs.push(FlatTermValue::Variable);
                    env.push(v, n);
                    n
                },
            }
        }

        let mut regs = Vec::new();
        let mut env = Env::new();
        flatten_term_onto(&mut regs, &mut env, term);
        drop(env);

        // This is just a topological sort implemented as a depth-first search.
        fn visit(
            out: &mut Vec<(usize, FlatTermValue)>,
            flats: &mut Vec<Option<FlatTermValue>>,
            current: usize,
        ) {
            if let Some(val) = flats[current].take() {
                if let FlatTermValue::Structure(_, ref args) = val {
                    for &arg in args {
                        visit(out, flats, arg);
                    }
                }
                out.push((current, val));
            }
        }

        let mut regs = regs.into_iter().map(Some).collect::<Vec<_>>();
        let mut sorted = Vec::new();
        for i in 0..regs.len() {
            visit(&mut sorted, &mut regs, i);
        }
        assert_eq!(regs.len(), sorted.len());
        FlatTerm(sorted)
    }
}

#[cfg(test)]
mod tests {
    use test_utils::example_term;

    use super::*;

    #[test]
    fn flattens_example_term() {
        let term = example_term();
        let flat = FlatTerm::flatten_term(term);

        assert_eq!(
            flat,
            FlatTerm(vec![
                (1, FlatTermValue::Variable),
                (3, FlatTermValue::Variable),
                (2, FlatTermValue::Structure("h".into(), vec![1, 3])),
                (4, FlatTermValue::Structure("f".into(), vec![3])),
                (0, FlatTermValue::Structure("p".into(), vec![1, 2, 4])),
            ])
        );
    }
}
