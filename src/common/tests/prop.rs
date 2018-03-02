use common::{Atom, Clause, Functor, Term, Variable};
use test_utils::{arb_atom, arb_clause, arb_functor, arb_term, arb_variable};

proptest! {
    #[test]
    fn parse_tostring_for_atom(atom in arb_atom()) {
        let atom_str = atom.to_string();
        let atom2 = Atom::parse(&atom_str).expect("Failed to parse atom");
        assert_eq!(atom, atom2);
    }

    #[test]
    fn parse_tostring_for_variable(var in arb_variable()) {
        let var_str = var.to_string();
        let var2 = Variable::from_str(&var_str).expect("Failed to parse variable");
        assert_eq!(var, var2);
    }

    #[test]
    fn parse_tostring_for_functor(functor in arb_functor(100)) {
        let functor_str = functor.to_string();
        let functor2 = Functor::parse(&functor_str)
            .expect("Failed to parse functor");
        assert_eq!(functor, functor2);
    }

    #[test]
    fn parse_tostring_for_term(ref term in arb_term(10, 4)) {
        let term_str = term.to_string();
        let term2 = Term::parse(&term_str).expect("Failed to parse term");
        assert_eq!(term, &term2);
    }

    #[test]
    fn parse_tostring_for_clause(ref clause in arb_clause(5, 3, 3)) {
        let clause_str = clause.to_string();
        let clause2 = Clause::parse(&clause_str)
            .expect("Failed to parse clause");
        assert_eq!(clause, &clause2);
    }
}
