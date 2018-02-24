//! Common code used by multiple chapters.

mod env;
mod parser;
#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use regex::Regex;
use symbol::Symbol;

pub use common::env::Env;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Atom(pub Symbol);

impl AsRef<str> for Atom {
    fn as_ref(&self) -> &str{ self.0.as_ref() }
}

impl Display for Atom {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        lazy_static! {
            static ref PLAIN: Regex = Regex::new("^[a-z][a-zA-Z_0-9]+$").unwrap();
        }
        let atom = self.0.as_str();
        if PLAIN.is_match(atom) {
            fmt.write_str(atom)
        } else {
            fmt.write_char('\'')?;
            for ch in atom.chars() {
                if parser::is_quoted_plain_char(ch) {
                    fmt.write_char(ch)?;
                } else {
                    match ch {
                        '\x07' => fmt.write_str("\\a")?,
                        '\x08' => fmt.write_str("\\b")?,
                        '\x1b' => fmt.write_str("\\e")?,
                        '\x0c' => fmt.write_str("\\f")?,
                        '\n' => fmt.write_str("\\n")?,
                        '\r' => fmt.write_str("\\r")?,
                        '\t' => fmt.write_str("\\t")?,
                        '\x0b' => fmt.write_str("\\v")?,
                        '\\' => fmt.write_str("\\\\")?,
                        '\'' => fmt.write_str("\\'")?,
                        '"' => fmt.write_str("\\\"")?,
                        _ => {
                            let n = ch as u32;
                            if n < 0x10000 {
                                write!(fmt, "\\u{:04x}", n)?;
                            } else {
                                write!(fmt, "\\U{:08x}", n)?;
                            }
                        }
                    }
                }
            }
            fmt.write_char('\'')
        }
    }
}

impl<'a> From<&'a str> for Atom {
    fn from(s: &'a str) -> Atom {
        Atom(s.into())
    }
}

impl<'a> From<Cow<'a, str>> for Atom {
    fn from(s: Cow<'a, str>) -> Atom {
        Atom(s.into())
    }
}

impl From<String> for Atom {
    fn from(s: String) -> Atom {
        Atom(s.into())
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Functor(pub Atom, pub usize);

impl Display for Functor {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}/{}", self.0, self.1)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable(Symbol);

lazy_static! {
    static ref VARIABLE_NAME: Regex =
        Regex::new("^(_[a-zA-Z_0-9]|[A-Z])[a-zA-Z_0-9]*$").unwrap();
}

impl Variable {
    /// Attempts to create a variable from the given string, returning `None`
    /// if the given string is not a valid variable name.
    pub fn from_str<S: AsRef<str>>(name: S) -> Option<Variable> {
        if VARIABLE_NAME.is_match(name.as_ref()) {
            Some(Variable(name.into()))
        } else {
            None
        }
    }
}

impl AsRef<str> for Variable {
    fn as_ref(&self) -> &str{ self.0.as_ref() }
}

impl Display for Variable {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let var = self.0.as_str();
        assert!(VARIABLE_NAME.is_match(var));
        fmt.write_str(var)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Term {
    Anonymous,
    Structure(Atom, Vec<Term>),
    Variable(Variable),
}

impl Term {
    /// Return whether the given term is a subterm of self.
    ///
    /// Note that this does *not* perform any kind of unification -- `X` is a
    /// subterm of `f(X)`, but `Y` is not.
    pub fn contains(&self, sub: &Term) -> bool {
        if self == sub {
            return true;
        }
        match *self {
            Term::Anonymous => false,
            Term::Structure(_, ref ts) => ts.iter().any(|t| t.contains(sub)),
            Term::Variable(ref v) => match *sub {
                Term::Anonymous => false,
                Term::Variable(ref v2) => v == v2,
                Term::Structure(_, _) => false,
            },
        }
    }
}

impl Display for Term {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Term::Anonymous => fmt.write_char('_'),
            Term::Structure(ref atom, ref args) => {
                Display::fmt(atom, fmt)?;
                fmt.write_char('(')?;
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    } else {
                        fmt.write_str(", ")?;
                    }
                    Display::fmt(arg, fmt)?;
                }
                fmt.write_char(')')
            },
            Term::Variable(ref v) => Display::fmt(v, fmt),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Clause(Term, Vec<Term>);

impl Display for Clause {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let &Clause(ref hd, ref tl) = self;
        Display::fmt(hd, fmt)?;
        if !tl.is_empty() {
            let mut first = true;
            for term in tl {
                if first {
                    fmt.write_str(" :-\n    ")?;
                    first = false;
                } else {
                    fmt.write_str(",\n    ")?;
                }
                Display::fmt(term, fmt)?;
            }
        }
        fmt.write_char('.')
    }
}
