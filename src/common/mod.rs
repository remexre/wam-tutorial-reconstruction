//! Common code used by multiple chapters.

mod env;
pub mod parsers;
#[cfg(test)]
mod tests;

use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FmtResult, Write};

use nom::{Err as NomErr, IError, IResult, Needed};
use regex::Regex;
use symbol::Symbol;

pub use self::env::Env;

/// An error while parsing.
#[derive(Clone, Debug, Fail, PartialEq)]
pub enum ParseError {
    /// An error, possibly with the byte index at which it occurred.
    Error(Option<usize>),

    /// An error which can be resolved by adding more input.
    Incomplete(Needed),
}

impl ParseError {
    /// Creates a ParseError from a `nom::IError` and the input.
    pub fn from_ierror(err: IError<&str>, src: &str) -> ParseError {
        fn find_position(err: NomErr<&str>) -> Option<&str> {
            match err {
                NomErr::Code(_) => None,
                NomErr::Node(_, errs) => {
                    for err in errs {
                        if let Some(pos) = find_position(err) {
                            return Some(pos);
                        }
                    }
                    None
                }
                NomErr::Position(_, pos) => Some(pos),
                NomErr::NodePosition(_, pos, _) => Some(pos),
            }
        }

        match err {
            IError::Incomplete(needed) => ParseError::Incomplete(needed),
            IError::Error(err) => ParseError::Error(
                find_position(err).map(|pos| src.len() - pos.len()),
            ),
        }
    }

    /// Converts a `nom::IResult` to a `Result`.
    pub fn from_iresult<T>(
        res: IResult<&str, T>,
        src: &str,
    ) -> Result<T, ParseError> {
        let res = match res {
            IResult::Done("", val) => Ok(val),
            IResult::Done(pos, _) => {
                return Err(ParseError::Error(Some(src.len() - pos.len())));
            }
            IResult::Incomplete(needed) => Err(IError::Incomplete(needed)),
            IResult::Error(err) => Err(IError::Error(err)),
        };
        res.map_err(|err| ParseError::from_ierror(err, src))
    }
}

impl Display for ParseError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ParseError::Error(Some(n)) => {
                write!(fmt, "Parse error at character {}", n)
            }
            ParseError::Error(None) => fmt.write_str("Parse error"),
            ParseError::Incomplete(Needed::Unknown) => {
                fmt.write_str("Incomplete input")
            }
            ParseError::Incomplete(Needed::Size(size)) => {
                write!(fmt, "Incomplete input ({} characters needed)", size)
            }
        }
    }
}

/// An interned string literal, used for literals.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Atom(pub Symbol);

impl AsRef<str> for Atom {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

fn is_quoted_plain_char(ch: char) -> bool {
    (' ' <= ch && ch <= '~') && (ch != '\'' && ch != '\\')
}

impl Display for Atom {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        lazy_static! {
            static ref PLAIN: Regex = Regex::new("^[a-z0-9][a-zA-Z_0-9]*$").unwrap();
        }
        let atom = self.0.as_str();
        if PLAIN.is_match(atom) {
            fmt.write_str(atom)
        } else {
            fmt.write_char('\'')?;
            for ch in atom.chars() {
                if is_quoted_plain_char(ch) {
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

/// A functor, which contains the atom and arity of a structure.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Functor(pub Atom, pub usize);

impl Display for Functor {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}/{}", self.0, self.1)
    }
}

/// A variable.
///
/// The name is not public, since it may only be certain values.
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
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for Variable {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let var = self.0.as_str();
        assert!(VARIABLE_NAME.is_match(var));
        fmt.write_str(var)
    }
}

/// Any value, which may have unresolved variables within it.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Term {
    /// An anonymous variable. Unification with an anonymous variable will not
    /// affect other anonymous variables in the term.
    Anonymous,

    /// A structure literal.
    Structure(Structure),

    /// A variable. All instances of a variable within a term will be
    /// instantiated to the same value.
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
            Term::Structure(ref s) => s.1.iter().any(|t| t.contains(sub)),
            Term::Variable(ref v) => match *sub {
                Term::Anonymous => false,
                Term::Variable(ref v2) => v == v2,
                Term::Structure(_) => false,
            },
        }
    }
}

impl Display for Term {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Term::Anonymous => fmt.write_char('_'),
            Term::Structure(ref s) => Display::fmt(s, fmt),
            Term::Variable(ref v) => Display::fmt(v, fmt),
        }
    }
}

/// A structure, which has a functor and a sequence of terms.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Structure(pub Atom, pub Vec<Term>);

impl Structure {
    /// Returns the functor for this structure.
    pub fn functor(&self) -> Functor {
        Functor(self.0, self.1.len())
    }
}

impl Display for Structure {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        Display::fmt(&self.0, fmt)?;
        if self.1.is_empty() {
            Ok(())
        } else {
            fmt.write_char('(')?;
            let mut first = true;
            for arg in &self.1 {
                if first {
                    first = false;
                } else {
                    fmt.write_str(", ")?;
                }
                Display::fmt(arg, fmt)?;
            }
            fmt.write_char(')')
        }
    }
}

/// A clause, which is a fact or a rule.
///
/// Facts are always true, and have an empty Vec as their second value.
///
/// Rules are true if all of the terms in their second value are true.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Clause(pub Structure, pub Vec<Structure>);

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

/// A single cell on the heap.
#[derive(Copy, Clone, Debug)]
pub enum HeapCell {
    /// A functor.
    Functor(Functor),

    /// A reference to another cell.
    Ref(usize),

    /// A structure reference, which points to a functor cell.
    Str(usize),
}

impl HeapCell {
    /// Returns whether the given cell is a Ref.
    pub fn is_ref(self) -> bool {
        match self {
            HeapCell::Ref(_) => true,
            _ => false,
        }
    }
}
