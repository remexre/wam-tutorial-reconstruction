//! Chapter 2 -- Unification, Pure and Simple.

pub mod control;
pub mod env;
pub mod store;
mod program;
mod query;

use std::collections::HashMap;
use std::iter::once;

use failure::Error;

use common::{Term, Variable};

use self::control::Instruction;
use self::env::Env;
pub use self::program::compile_program;
pub use self::query::compile_query;
use self::store::{HeapCell, Mode, Store};

/// An abstract machine for M0.
///
/// TODO: This shouldn't really be CESK. More like CS, with both inlined.
#[derive(Debug)]
pub struct Machine {
    /// The control component, which contains all loaded code.
    pub c: Vec<Instruction>,

    /// The environment, which is currently just a growable sequence of indexed
    /// registers, each of which containing a heap address.
    pub e: Env,

    /// The store, which is the heap (aka the global stack).
    pub s: Store,

    /// The continuation, which doesn't really exist for M0.
    pub k: (),
}

impl Machine {
    /// Creates a new Machine, given the term to unify against.
    pub fn new(program: Term) -> Machine {
        let mut machine = Machine::empty();
        machine.c = compile_program(program);
        machine
    }

    /// Creates a new Machine with no code loaded.
    pub fn empty() -> Machine {
        Machine {
            c: Vec::new(),
            e: Env::new(),
            s: Store::new(),
            k: (),
        }
    }

    /// Runs an instruction.
    pub fn run_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::PutStructure(functor, reg) => {
                let n = self.s.push_with(|n| HeapCell::Str(n + 1));
                self.s.push(HeapCell::Functor(functor));
                self.e[reg] = n;
            }
            Instruction::SetVariable(reg) => {
                let n = self.s.push_with(|n| HeapCell::Ref(n));
                self.e[reg] = n;
            }
            Instruction::SetValue(reg) => {
                let cell = self.s.get(self.e[reg]);
                self.s.push(cell);
            }

            Instruction::GetStructure(functor, reg) => {
                let addr = self.s.deref(self.e[reg]);
                match self.s.get(addr) {
                    HeapCell::Ref(_) => {
                        let n = self.s.push_with(|n| HeapCell::Str(n + 1));
                        self.s.push(HeapCell::Functor(functor));
                        self.s.bind(addr, n);
                        self.s.s = 0;
                        self.s.mode = Mode::Write;
                    }
                    HeapCell::Str(a) => {
                        if self.s.get_functor(a) == functor {
                            self.s.s = a + 1;
                            self.s.mode = Mode::Read;
                        } else {
                            self.s.s = 0;
                            self.s.fail = true;
                        }
                    }
                    _ => panic!("Invalid deref in {}", instr),
                }
            }
            Instruction::UnifyVariable(reg) => {
                match self.s.mode {
                    Mode::Read => {
                        self.e[reg] = self.s.s;
                    }
                    Mode::Write => {
                        self.e[reg] = self.s.push_with(|n| HeapCell::Ref(n));
                    }
                }
                self.s.s += 1;
            }
            Instruction::UnifyValue(reg) => {
                match self.s.mode {
                    Mode::Read => {
                        let a1 = self.e[reg];
                        let a2 = self.s.s;
                        self.unify(a1, a2);
                    }
                    Mode::Write => {
                        let val = self.s.get(self.e[reg]);
                        self.s.push(val);
                    }
                }
                self.s.s += 1;
            }
        }
    }

    /// Unifies the values at the two addresses.
    fn unify(&mut self, a1: usize, a2: usize) {
        let mut pdl = vec![a1, a2];
        while !pdl.is_empty() && !self.s.fail {
            let d1 = self.s.deref(pdl.pop().unwrap());
            let d2 = self.s.deref(pdl.pop().unwrap());
            if d1 != d2 {
                match (self.s.get(d1), self.s.get(d2)) {
                    (HeapCell::Str(v1), HeapCell::Str(v2)) => {
                        let f1 = self.s.get_functor(v1);
                        let f2 = self.s.get_functor(v2);
                        if f1 == f2 {
                            for i in 1..f1.1 {
                                pdl.push(v1 + i);
                                pdl.push(v2 + i);
                            }
                        } else {
                            self.s.fail = true;
                        }
                    }
                    _ => self.s.bind(d1, d2),
                }
            }
        }
    }
}

impl ::Machine for Machine {
    fn run_query(
        &mut self,
        mut query: Vec<Term>,
    ) -> Box<Iterator<Item = Result<HashMap<Variable, Term>, Error>>> {
        if query.len() != 1 {
            let err =
                format_err!("M0 doesn't support conjunctions in queries.");
            return Box::new(once(Err(err)));
        }
        let query = query.remove(0);

        self.e.clear();
        self.s.reset();

        let (query_code, vars) = compile_query(query);

        for instr in query_code.into_iter().chain(self.c.clone()) {
            trace!("> {}", instr);
            println!("> {}", instr);
            self.run_instruction(instr);
            if self.s.fail {
                let err = format_err!("Failed to unify ({}).", instr);
                return Box::new(once(Err(err)));
            }
        }

        let mut names = HashMap::new();
        Box::new(once(
            vars.into_iter()
                .map(|(var, reg)| {
                    self.s.extract_term(self.e[reg], Some(&names)).map(|val| {
                        names.insert(self.s.deref(self.e[reg]), var);
                        (var, val)
                    })
                })
                .collect(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use Machine as MachineTrait;
    use test_utils::{arb_term, example_program_term, example_query_term};

    use super::*;

    fn compile_query_roundtrip(term: Term) -> Term {
        let mut machine = Machine::empty();
        for instr in compile_query(term).0 {
            machine.run_instruction(instr);
            assert!(!machine.s.fail);
        }
        machine.s.extract_term(machine.e[0], None).unwrap()
    }

    proptest!{
        #[test]
        fn compile_query_idempotency(ref term in arb_term(5, 10)) {
            let term2 = compile_query_roundtrip(term.clone());
            let term3 = compile_query_roundtrip(term2.clone());
            assert_eq!(term2, term3);
        }
    }

    #[test]
    fn works() {
        let res = Machine::new(example_program_term())
            .run_query(vec![example_query_term()])
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to run query");
        assert_eq!(
            res,
            vec![
                vec![
                    (
                        variable!("W"),
                        Term::Structure(
                            atom!(f),
                            vec![Term::Structure(atom!(a), vec![])],
                        ),
                    ),
                    (
                        variable!("Z"),
                        Term::Structure(
                            atom!(f),
                            vec![
                                Term::Structure(
                                    atom!(f),
                                    vec![Term::Structure(atom!(a), vec![])],
                                ),
                            ],
                        ),
                    ),
                ].into_iter()
                    .collect(),
            ]
        );
    }
}
