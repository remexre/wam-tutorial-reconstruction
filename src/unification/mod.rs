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
            Instruction::PutStructure(Functor(_atom, _arity), _reg) => {
                unimplemented!()
            }
            Instruction::SetValue(reg) => {
                let n = self.s.push_with(|n| HeapCell::Ref(n));
                self.e[reg] = self.s.get(n);
            }
            Instruction::SetVariable(reg) => {
                self.s.push(self.e[reg]);
            }

            Instruction::GetStructure(Functor(_atom, _arity), _reg) => {
                unimplemented!()
            }
            Instruction::UnifyVariable(_reg) => unimplemented!(),
            Instruction::UnifyValue(_reg) => unimplemented!(),
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

    proptest!{
        #[test]
        fn query_compile(ref term in arb_term(5, 10)) {
            let mut machine = Machine::empty();
            let query = FlatTerm::flatten_term(term.clone());
            for instr in compile_query(&query, 1) {
                machine.run_instruction(instr);
            }
            let term2 = machine.s.extract_term(machine.e[1], None).unwrap();
            assert_eq!(term, &term2);
        }
    }
}
