//! A Rust implementation of the machines in "Warren's Abstract Machine: A
//! Tutorial Reconstruction."

extern crate either;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[cfg(test)]
#[macro_use]
extern crate proptest;
extern crate regex;
extern crate symbol;

#[macro_use]
mod macros;

#[cfg(test)]
#[macro_use]
mod test_utils;

pub mod common;
pub mod unification;

use failure::Error;

use common::Term;

/// A trait for an abstract machine based on CESK semantics.
pub trait Machine {
    /// Runs a query against the program.
    fn run_query(&mut self, query: Vec<Term>) -> Result<(), Error>;
}

/// A machine for debugging queries.
pub struct QueryDebugMachine;

impl Machine for QueryDebugMachine {
    fn run_query(&mut self, mut query: Vec<Term>) -> Result<(), Error> {
        use unification::compile_query;

        ensure!(query.len() == 1, "Bad query length");
        let query = query.remove(0);

        let code = compile_query(query);
        for instr in code {
            println!("    {}", instr);
        }
        bail!("TODO run_query")
    }
}
