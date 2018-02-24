//! Nom parsers for various syntactic elements.
//!
//! Currently does not support:
//!
//!  - Infix operators
//!  - Octal escapes

use std::char;
use std::str::FromStr;

use nom::{digit, hex_digit, multispace, IError, IResult};

use common::{Atom, Clause, Functor, Term, Variable};

macro_rules! from_str {
    ($($(#[$meta:meta])* $parser:ident => $ty:ty),*$(,)*) => {
        $(impl $ty {
            $(#[$meta])*
            pub fn parse(s: &str) -> Result<Self, IError<&str>> {
                $parser(s).to_full_result()
            }
        })*
    }
}

from_str! {
    /// Parses an Atom.
    atom => Atom,

    /// Parses a Functor.
    functor => Functor,

    /// Parses a Term.
    term => Term,
}

// The "basic" common types.

named!(atom(&str) -> Atom, alt!(
    map!(
        delimited!(tag_s!("'"), many0!(atom_quoted_char), tag_s!("'")),
        |cs| Atom::from(cs.into_iter().filter_map(|x| x).collect::<String>())
    ) |
    map!(unquoted_atom, |cs| cs.into())
));

named!(functor(&str) -> Functor, do_parse!(
    atom: atom >>
    tag_s!("/") >>
    arity: map_res!(digit, FromStr::from_str) >>
    ( Functor(atom, arity) )));

named!(term(&str) -> Term, alt!(
    map!(variable, |s| Term::Variable(Variable::from_str(s).unwrap()))
));

named!(variable(&str) -> &str, recognize!(tuple!(
    take_while1_s!(is_variable_start_char),
    take_while_s!(is_plain_char)
)));

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
    map!(verify!(one_char, is_quoted_plain_char), Some)
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

// Helper functions.

fn all_hex_digits(s: &str) -> bool {
    s.chars().all(|c| match c {
        'a'...'f' | 'A'...'F' | '0'...'9' => true,
        _ => false,
    })
}

fn is_atom_start_char(ch: char) -> bool {
    'a' <= ch && ch <= 'z'
}

fn is_variable_start_char(ch: char) -> bool {
    ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_plain_char(_ch: char) -> bool {
    unimplemented!()
}

fn one_char(input: &str) -> IResult<&str, char> {
    let mut iter = input.chars();
    match iter.next() {
        Some(ch) => IResult::Done(iter.as_str(), ch),
        None => IResult::Incomplete(unimplemented!()),
    }
}

pub fn is_quoted_plain_char(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_' || ch == ' '
}
