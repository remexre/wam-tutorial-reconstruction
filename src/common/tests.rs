use proptest::prelude::*;

use common::{Atom, Functor, Term};
use test_utils::example_term;

#[test]
fn term_contains() {
    let term = example_term();

    assert!(term.contains(&term));

    assert!(term.contains(&Term::Variable(variable!("Z"))));
    assert!(!term.contains(&Term::Variable(variable!("Y"))));

    assert!(term.contains(&Term::Structure(
        "f".into(),
        vec![Term::Variable(variable!("W"))]
    )));
    assert!(!term.contains(&Term::Structure("f".into(), vec![])));
}

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
    fn arb_term(max_functor_arity: usize, max_depth: usize)
               (functor in arb_functor(max_functor_arity),
                asdf in 0..1usize)
               -> Term {
        let leaf = prop_oneof![
            //
        ];
        leaf.prop_recursive(
            max_depth,
            max_functor_arity * max_depth,
            max_functor_arity,
            |inner| {
            })
    }
}

prop_compose! {
    fn arb_clause(max_functor_arity: usize, max_term_depth: usize,
                  max_clause_depth: usize)
                 (term in arb_term(max_functor_arity, max_term_depth),
                  asdf in 0..1usize)
               -> Term {
        unimplemented!()
    }
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
}
