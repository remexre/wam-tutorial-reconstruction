//! Chapter 2 -- Unification, Pure and Simple.

pub mod control;
pub mod state;
mod program;
mod query;

use failure::Error;

use common::{FlatTerm, Functor, Term};

use self::control::{Control, Instruction};
pub use self::program::compile_program;
pub use self::query::compile_query;
use self::state::{HeapCell, State};

/// An abstract machine for M0.
#[derive(Debug)]
pub struct Machine {
    /// The control component, which contains all loaded code.
    pub c: Control,

    /// The environment, which is currently just a growable sequence of indexed
    /// registers, each of which contains a heap cell.
    pub e: Vec<HeapCell>,

    /// The state, which is the heap (aka the global stack).
    pub s: State,

    /// The continuation, which is the (local) stack.
    /// TODO: Define k.
    pub k: (),
}

impl Machine {
    /// Creates a new Machine, given the term to unify against.
    pub fn new(program: Term) -> Machine {
        let mut machine = Machine::empty();
        let program = FlatTerm::flatten_term(program);
        machine.c.code = compile_program(&program);
        machine
    }

    /// Creates a new Machine with no code loaded.
    pub fn empty() -> Machine {
        Machine {
            c: Control {
                code: vec![],
                ip: 0,
            },
            e: Vec::new(),
            s: State::new(),
            k: (), // TODO
        }
    }

    /// Runs an instruction.
    pub fn run_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::PutStructure(functor, reg) => {
                let n = self.s.push_with(|n| HeapCell::Str(n + 1));
                self.s.push(HeapCell::Functor(functor));
                let val = self.s.get(n);
                self.set_or_add_register(reg, val);
            }
            Instruction::SetVariable(reg) => {
                let n = self.s.push_with(|n| HeapCell::Ref(n));
                let val = self.s.get(n);
                self.set_or_add_register(reg, val);
            }
            Instruction::SetValue(reg) => {
                self.s.push(self.e[reg]);
            }

            Instruction::GetStructure(Functor(_atom, _arity), _reg) => {
                unimplemented!()
            }
            Instruction::UnifyVariable(_reg) => unimplemented!(),
            Instruction::UnifyValue(_reg) => unimplemented!(),
        }
    }

    fn set_or_add_register(&mut self, n: usize, val: HeapCell) {
        if self.e.len() == n {
            self.e.push(val);
        } else {
            self.e[n] = val;
        }
    }
}

impl ::Machine for Machine {
    fn run_query(&self, query: Vec<Term>) -> Result<(), Error> {
        if query.len() != 1 {
            bail!("M0 doesn't support conjunctions in queries.");
        }
        bail!("TODO run_query")
    }
}

#[cfg(test)]
mod tests {
    use test_utils::arb_term;

    use super::*;

    fn compile_query_roundtrip(term: Term) -> Term {
        let mut machine = Machine::empty();
        println!("===== {} =====", term);
        let flat = FlatTerm::flatten_term(term);
        for instr in compile_query(&flat, 0) {
            println!("{}", instr);
            machine.run_instruction(instr);
        }
        println!("----------");
        machine.s.extract_term(machine.e[0], None).unwrap()
    }

    proptest!{
        #[test]
        fn query_compile_idempotency(ref term in arb_term(5, 10)) {
            let term2 = compile_query_roundtrip(term.clone());
            let term3 = compile_query_roundtrip(term2.clone());
            assert_eq!(term2, term3);
        }
    }
}
