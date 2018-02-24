use common::{Atom, Env, Term, Variable};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatTerm(pub Vec<FlatTermValue>);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlatTermValue {
    Structure(Atom, Vec<usize>),
    Variable,
}

pub fn flatten_term(term: Term) -> FlatTerm {
    fn flatten_term_onto(regs: &mut Vec<FlatTermValue>, env: &mut Env<Variable, usize>, term: Term) -> usize {
        match term {
            Term::Anonymous => {
                let n = regs.len();
                regs.push(FlatTermValue::Variable);
                n
            },
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
    FlatTerm(regs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flatten_term_basic() {
        // p(Z, h(Z, W), f(W))
        let term = Term::Structure(
            "p".into(),
            vec![
                Term::Variable(variable!("Z")),
                Term::Structure(
                    "h".into(),
                    vec![
                        Term::Variable(variable!("Z")),
                        Term::Variable(variable!("W")),
                    ],
                ),
                Term::Structure("f".into(), vec![
                    Term::Variable(variable!("W")),
                ]),
            ],
        );
        let flat = flatten_term(term);

        println!("{:#?}", flat);

        assert_eq!(
            flat,
            FlatTerm(vec![
                FlatTermValue::Structure("p".into(), vec![1, 2, 4]),
                FlatTermValue::Variable,
                FlatTermValue::Structure("h".into(), vec![1, 3]),
                FlatTermValue::Variable,
                FlatTermValue::Structure("f".into(), vec![3]),
            ])
        );
    }
}
