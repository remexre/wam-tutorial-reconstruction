//! Chapter 2 -- Unification, Pure and Simple.

pub mod control;
pub mod state;
pub mod query;

use unification::control::{Control, Instruction};
use unification::state::State;

/// An abstract machine for M0.
#[derive(Debug)]
pub struct Machine {
    pub c: Control,
    // TODO: env
    pub e: (),
    pub s: State,
    // TODO: kont
    pub k: (),
}

impl Machine {
    /// Creates a new Machine.
    pub fn new(code: Vec<Instruction>) -> Machine {
        Machine {
            c: Control { code, ip: 0 },
            e: unimplemented!(),
            s: State::new(),
            k: unimplemented!(),
        }
    }
}
