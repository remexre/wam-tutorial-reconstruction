//! A Rust implementation of the machines in "Warren's Abstract Machine: A
//! Tutorial Reconstruction."

extern crate either;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
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

use std::collections::HashMap;

use failure::Error;

use common::{Term, Variable};

/// A trait for an abstract machine based on CESK semantics.
pub trait Machine {
    /// Runs a query against the program.
    fn run_query(
        &mut self,
        query: Vec<Term>,
    ) -> Box<Iterator<Item = Result<HashMap<Variable, Term>, Error>>>;
}
