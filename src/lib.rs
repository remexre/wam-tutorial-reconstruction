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
