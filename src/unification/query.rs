use std::collections::HashSet;

use common::{Atom, Env, Functor, Term, Variable};
use unification::control::Instruction;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FlatTermValue {
    Structure(Atom, Vec<usize>),
    Variable,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FlatTerm(pub Vec<FlatTermValue>);

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
        FlatTerm(regs)
    }

    /// Compiles a term into instructions that will construct the term on the
    /// heap, storing the root into the given register number.
    pub fn compile(&self, base: usize) -> Vec<Instruction> {
        let mut instrs = Vec::with_capacity(self.0.len());
        let mut seen = HashSet::with_capacity(self.0.len());
        for (i, ft) in self.0.iter().enumerate() {
            seen.insert(i);
            match *ft {
                FlatTermValue::Structure(atom, ref subterms) => {
                    unimplemented!()
                }
                FlatTermValue::Variable => unimplemented!(),
            }
        }
        instrs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_utils::example_term;

    #[test]
    fn flattens_example_term() {
        let term = example_term();
        let flat = FlatTerm::flatten_term(term);

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

    #[test]
    fn compiles_example_term() {
        let term = example_term();
        let flat = FlatTerm::flatten_term(term);
        let code = flat.compile(1);

        assert_eq!(
            code,
            vec![
                Instruction::PutStructure(Functor("h".into(), 2), 3),
                Instruction::SetVariable(2),
                Instruction::SetVariable(5),
                Instruction::PutStructure(Functor("f".into(), 1), 4),
                Instruction::SetVariable(5),
                Instruction::PutStructure(Functor("p".into(), 3), 1),
                Instruction::SetVariable(2),
                Instruction::SetVariable(3),
                Instruction::SetVariable(4),
            ]
        );
    }
}
