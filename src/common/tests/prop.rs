use proptest::prelude::*;

use common::{Atom, Clause, Functor, Term, Variable};

prop_compose! {
    fn arb_atom()(s in "\\PC*") -> Atom {
        Atom::from(s)
    }
}

prop_compose! {
    fn arb_functor(max_arity: usize)
                  (atom in arb_atom(),
                   arity in 0..max_arity)
                  -> Functor {
        Functor(atom, arity)
    }
}

prop_compose! {
    fn arb_variable()(s in "(_[a-zA-Z_0-9]|[A-Z])[a-zA-Z_0-9]*") -> Variable {
        Variable::from_str(s).unwrap()
    }
}

fn arb_term(max_functor_arity: usize, max_depth: usize) -> BoxedStrategy<Term> {
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
                    .prop_map(move |subterms| Term::Structure(atom, subterms))
            })
        },
    )
        .boxed()
}

fn arb_clause(
    max_functor_arity: usize,
    max_term_depth: usize,
    max_clause_depth: usize,
) -> BoxedStrategy<Clause> {
    arb_term(max_functor_arity, max_term_depth)
        .prop_flat_map(move |head| {
            let term = arb_term(max_functor_arity, max_term_depth);
            prop::collection::vec(term, 0..max_clause_depth)
                .prop_map(move |tail| Clause(head.clone(), tail))
        })
        .boxed()
}

proptest! {
    #[test]
    fn parse_tostring_for_atom(atom in arb_atom()) {
        let atom_str = atom.to_string();
        let atom2 = Atom::parse(&atom_str).expect("Failed to parse atom");
        assert_eq!(atom, atom2);
    }

    #[test]
    fn parse_tostring_for_functor(functor in arb_functor(100)) {
        let functor_str = functor.to_string();
        let functor2 = Functor::parse(&functor_str)
            .expect("Failed to parse functor");
        assert_eq!(functor, functor2);
    }

    #[test]
    fn parse_tostring_for_term(ref term in arb_term(10, 10)) {
        let term_str = term.to_string();
        let term2 = Term::parse(&term_str).expect("Failed to parse term");
        assert_eq!(term, &term2);
    }

    #[test]
    fn parse_tostring_for_clause(ref clause in arb_clause(10, 10, 3)) {
        let clause_str = clause.to_string();
        let clause2 = Clause::parse(&clause_str)
            .expect("Failed to parse clause");
        assert_eq!(clause, &clause2);
    }
}
