//! A Rust implementation of the machines in "Warren's Abstract Machine: A
//! Tutorial Reconstruction."

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

#[cfg(test)]
#[macro_use]
mod test_utils;

pub mod common;
pub mod unification;

use failure::Error;

use common::Term;

/// A trait for an abstract machine based on CESK semantics.
pub trait Machine {
    /// Compiles a program.
    fn compile(&self) -> Result<(), Error>;

    /// Runs a query against the program.
    fn run_query(&self, query: Vec<Term>) -> Result<(), Error>;
}
