use proptest::prelude::*;

use common::{Atom, Clause, Functor, Structure, Term, Variable};

macro_rules! parse_tests {
    ($($parser:ident($input:expr, $expected:expr);)*) => {
        #[test]
        fn parse_tests() {
            $({
                let res = $crate::common::ParseError::from_iresult($parser($input), $input)
                    .expect(concat!("Failed to parse \"", $input, "\""));
                assert_eq!(res, $expected);
            })*
        }
    }
}

/// Returns the term `p(Z, h(Z, W), f(W))`.
pub fn example_query() -> Structure {
    Structure(
        atom!(p),
        vec![
            Term::Variable(variable!("Z")),
            Term::Structure(Structure(
                atom!(h),
                vec![
                    Term::Variable(variable!("Z")),
                    Term::Variable(variable!("W")),
                ],
            )),
            Term::Structure(Structure(
                atom!(f),
                vec![Term::Variable(variable!("W"))],
            )),
        ],
    )
}

/// Returns the term `p(Z, h(Z, W), f(W))`.
pub fn example_query_term() -> Term {
    Term::Structure(example_query())
}

/// Returns the term `p(f(X), h(Y, f(a)), Y)`.
pub fn example_program() -> Structure {
    Structure(
        atom!(p),
        vec![
            Term::Structure(Structure(
                atom!(f),
                vec![Term::Variable(variable!("X"))],
            )),
            Term::Structure(Structure(
                atom!(h),
                vec![
                    Term::Variable(variable!("Y")),
                    Term::Structure(Structure(
                        atom!(f),
                        vec![Term::Structure(Structure(atom!(a), vec![]))],
                    )),
                ],
            )),
            Term::Variable(variable!("Y")),
        ],
    )
}

/// Returns the term `p(f(X), h(Y, f(a)), Y)`.
pub fn example_program_term() -> Term {
    Term::Structure(example_program())
}

prop_compose! {
    [pub] fn arb_atom()(s in "\\PC*") -> Atom {
        Atom::from(s)
    }
}

prop_compose! {
    [pub] fn arb_functor(max_arity: usize)
                  (atom in arb_atom(),
                   arity in 0..max_arity)
                  -> Functor {
        Functor(atom, arity)
    }
}

prop_compose! {
    [pub] fn arb_variable()(s in "(_[a-zA-Z_0-9]|[A-Z])[a-zA-Z_0-9]*") -> Variable {
        Variable::from_str(s).unwrap()
    }
}

pub fn arb_term(
    max_functor_arity: usize,
    max_depth: usize,
) -> BoxedStrategy<Term> {
    prop_oneof![
        Just(Term::Anonymous),
        arb_variable().prop_map(Term::Variable),
    ].prop_recursive(
        max_depth as u32,
        (max_functor_arity * max_depth) as u32,
        max_functor_arity as u32,
        move |inner| {
            arb_atom().prop_flat_map(move |atom| {
                prop::collection::vec(inner.clone(), 0..max_functor_arity)
                    .prop_map(move |subterms| {
                        Term::Structure(Structure(atom, subterms))
                    })
            })
        },
    )
        .boxed()
}

pub fn arb_structure(
    max_functor_arity: usize,
    max_depth: usize,
) -> BoxedStrategy<Structure> {
    arb_atom()
        .prop_flat_map(move |name| {
            let term = arb_term(max_functor_arity, max_depth - 1);
            prop::collection::vec(term, 0..max_functor_arity)
                .prop_map(move |ts| Structure(name, ts))
        })
        .boxed()
}

pub fn arb_clause(
    max_functor_arity: usize,
    max_term_depth: usize,
    max_clause_depth: usize,
) -> BoxedStrategy<Clause> {
    arb_structure(max_functor_arity, max_term_depth)
        .prop_flat_map(move |head| {
            let structure = arb_structure(max_functor_arity, max_term_depth);
            prop::collection::vec(structure, 0..max_clause_depth)
                .prop_map(move |tail| Clause(head.clone(), tail))
        })
        .boxed()
}
