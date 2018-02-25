//! Chapter 2 -- Unification, Pure and Simple.

pub mod control;
pub mod state;
mod query;

use failure::Error;

use common::{Functor, Term};

use self::control::{Control, Instruction};
pub use self::query::*;
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
    /// Creates a new Machine.
    pub fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            c: Control { code, ip: 0 },
            e: Vec::new(),
            s: State::new(),
            k: (), // TODO
        }
    }

    /// Runs an instruction.
    pub fn run_instruction(&mut self, instr: Instruction) {
        match instr {
            Instruction::PutStructure(Functor(atom, arity), reg) => {
                unimplemented!()
            }
            Instruction::SetValue(reg) => {
                let n = self.s.push_with(|n| HeapCell::Ref(n));
                self.e[reg] = self.s.get(n);
            }
            Instruction::SetVariable(reg) => {
                self.s.push(self.e[reg]);
            }
        }
    }
}

impl ::Machine for Machine {
    fn compile(&self) -> Result<(), Error> {
        bail!("TODO compile")
    }

    fn run_query(&self, query: Vec<Term>) -> Result<(), Error> {
        if query.len() != 1 {
            bail!("The unification machine doesn't support conjunctions in queries.");
        }
        bail!("TODO run_query")
    }
}
