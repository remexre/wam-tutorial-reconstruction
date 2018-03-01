//! Nom parsers for various syntactic elements.
//!
//! Currently does not support:
//!
//!  - Infix operators
#![allow(missing_docs)]

use std::char;
use std::str::FromStr;

use nom::{digit, hex_digit, multispace, IResult, Needed};

use common::{Atom, Clause, Functor, Term, Variable};

macro_rules! from_str {
    ($($(#[$meta:meta])* $parser:ident => $ty:ty),*$(,)*) => {
        $(impl $ty {
            $(#[$meta])*
            pub fn parse(s: &str) -> Result<Self, $crate::common::ParseError> {
                $crate::common::ParseError::from_iresult($parser(s), s)
            }
        })*
    }
}

from_str! {
    /// Parses an Atom.
    atom => Atom,

    /// Parses a Clause.
    clause => Clause,

    /// Parses a Functor.
    functor => Functor,

    /// Parses a Term.
    term => Term,
}

/// Matches whitespace or a line comment.
named!(whitespace_or_comment(&str) -> &str, recognize!(many0!(alt!(
    value!((), one_of!(" \n\r\t")) |
    value!((), tuple!(tag_s!("%"), take_till_s!(|ch| ch == '\r' || ch == '\n')))
))));

/// Removes whitespace and line comments.
macro_rules! remove_whitespace_and_comments {
    ($i:expr, $($args:tt)*) => {
        sep!($i, whitespace_or_comment, $($args)*)
    };
}

// The "basic" common types.

/// Parses an `Atom`.
named!(pub atom(&str) -> Atom, remove_whitespace_and_comments!(alt!(
    map!(
        delimited!(tag_s!("'"), many0!(atom_quoted_char), tag_s!("'")),
        |cs| Atom::from(cs.into_iter().filter_map(|x| x).collect::<String>())
    ) |
    map!(unquoted_atom, |cs| cs.into())
)));

/// Parses a `Clause`.
named!(pub clause(&str) -> Clause, remove_whitespace_and_comments!(do_parse!(
    clause: alt!(clause_rule | clause_fact) >>
    tag_s!(".") >>
    ( clause )
)));

/// Parses a `Functor`.
named!(pub functor(&str) -> Functor, remove_whitespace_and_comments!(do_parse!(
    atom: atom >>
    tag_s!("/") >>
    arity: map_res!(digit, FromStr::from_str) >>
    ( Functor(atom, arity) )
)));

/// Parses a series of `Clause`s.
named!(pub program(&str) -> Vec<Clause>,
    remove_whitespace_and_comments!(many0!(clause)));

/// Parses a query, which is a conjunctive list of `Term`s.
named!(pub query(&str) -> Vec<Term>, remove_whitespace_and_comments!(do_parse!(
    terms: separated_list!(tag_s!(","), term) >>
    tag_s!(".") >>
    ( terms )
)));

/// Parses a `Term`.
named!(pub term(&str) -> Term, remove_whitespace_and_comments!(alt!(
    map!(variable, |s| if s == "_" {
        Term::Anonymous
    } else {
        Term::Variable(Variable::from_str(s).unwrap())
    }) | term_structure
)));

/// Parses a valid `Variable`.
named!(pub variable(&str) -> &str, remove_whitespace_and_comments!(recognize!(tuple!(
    take_while1_s!(is_variable_start_char),
    take_while_s!(is_plain_char)
))));

// Smaller primitives.

named!(atom_quoted_char(&str) -> Option<char>, alt_complete!(
    value!(Some('\x07'), tag_s!("\\a")) |
    value!(Some('\x08'), tag_s!("\\b")) |
    do_parse!(tag_s!("\\c") >> multispace >> ( None )) |
    value!(None, tag_s!("\\\n")) |
    value!(Some('\x1b'), tag_s!("\\e")) |
    value!(Some('\x0c'), tag_s!("\\f")) |
    value!(Some('\n'), tag_s!("\\n")) |
    value!(Some('\r'), tag_s!("\\r")) |
    value!(Some(' '), tag_s!("\\s")) |
    value!(Some('\t'), tag_s!("\\t")) |
    value!(Some('\x0b'), tag_s!("\\v")) |
    map!(atom_quoted_hex_escape, Some) |
    map!(map_opt!(do_parse!(
        tag_s!("\\") >>
        digits: take_while1_s!(|c| '0' <= c && c <= '7') >>
        ( digits )
    ), |s| u32::from_str_radix(s, 8).ok().and_then(char::from_u32)), Some) |
    value!(Some('\\'), tag_s!("\\\\")) |
    value!(Some('\''), tag_s!("\\'")) |
    value!(Some('"'), tag_s!("\\\"")) |
    value!(Some('`'), tag_s!("\\`")) |
    map!(verify!(one_char, |ch| ch != '\\' && ch != '\''), Some)
));

named!(atom_quoted_hex_escape(&str) -> char, map_opt!(alt!(
    do_parse!(
        tag_s!("\\x") >>
        digits: hex_digit >>
        tag_s!("\\") >>
        ( digits )) |
    do_parse!(
        tag_s!("\\u") >>
        digits: verify!(take!(4), all_hex_digits) >>
        ( digits )) |
    do_parse!(
        tag_s!("\\U") >>
        digits: verify!(take!(8), all_hex_digits) >>
        ( digits ))),
    |s| u32::from_str_radix(s, 16).ok().and_then(char::from_u32)));

named!(unquoted_atom(&str) -> &str, recognize!(tuple!(
    take_while1_s!(is_atom_start_char),
    take_while_s!(is_plain_char)
)));

named!(clause_fact(&str) -> Clause, map!(term, |hd| Clause(hd, vec![])));

named!(clause_rule(&str) -> Clause, do_parse!(
    hd: term >>
    tag_s!(":-") >>
    tl: separated_list!(tag_s!(","), term) >>
    ( Clause(hd, tl) )
));

named!(term_structure(&str) -> Term, do_parse!(
    atom: atom >>
    subterms: opt!(complete!(delimited!(
        tag_s!("("),
        separated_list!(tag_s!(","), term),
        tag_s!(")")))) >>
    ( Term::Structure(atom, subterms.unwrap_or_else(Vec::new)) )
));

// Helper functions.

fn all_hex_digits(s: &str) -> bool {
    s.chars().all(|c| match c {
        'a'...'f' | 'A'...'F' | '0'...'9' => true,
        _ => false,
    })
}

fn is_atom_start_char(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('0' <= ch && ch <= '9')
}

fn is_variable_start_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_plain_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z') || ('a' <= ch && ch <= 'z')
        || ('0' <= ch && ch <= '9') || ch == '_'
}

fn one_char(input: &str) -> IResult<&str, char> {
    let mut iter = input.chars();
    match iter.next() {
        Some(ch) => IResult::Done(iter.as_str(), ch),
        None => IResult::Incomplete(Needed::Size(1)),
    }
}
