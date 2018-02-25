use common::{Atom, Clause, Functor, Term, Variable};
use common::parsers::{atom, clause, functor, query, term, variable};
use test_utils::example_term;

mod prop;

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

parse_tests! {
    atom("asdf", Atom("asdf".into()));
    atom("'Asdf'", Atom("Asdf".into()));
    atom("''", Atom("".into()));
    atom("12", Atom("12".into()));
    atom("'asdf'", Atom("asdf".into()));
    atom("foo_bar", Atom("foo_bar".into()));
    atom("'foo bar'", Atom("foo bar".into()));
    atom("'Hello, world!'", Atom("Hello, world!".into()));
    atom("'HELLO\\nWORLD'", Atom("HELLO\nWORLD".into()));

    variable("X", "X");
    variable("Foo", "Foo");
    variable("_", "_");
    variable("_Foo", "_Foo");

    functor("asdf/2", Functor("asdf".into(), 2));
    functor("''/0", Functor("".into(), 0));
    functor("12/3", Functor("12".into(), 3));
    functor("qwerty/123456", Functor("qwerty".into(), 123456));

    term("_", Term::Anonymous);
    term("X", Term::Variable(variable!("X")));
    term("123", Term::Structure("123".into(), vec![]));
    term("123()", Term::Structure("123".into(), vec![]));
    term("''()", Term::Structure("".into(), vec![]));
    term("a_b(c(D, E), F, _, '')", Term::Structure("a_b".into(), vec![
        Term::Structure("c".into(), vec![
            Term::Variable(variable!("D")),
            Term::Variable(variable!("E")),
        ]),
        Term::Variable(variable!("F")),
        Term::Anonymous,
        Term::Structure("".into(), vec![]),
    ]));
    term("p(Z, h(Z, W), f(W))", example_term());

    query(".", vec![]);
    query("true.", vec![ Term::Structure("true".into(), vec![]) ]);
    query("fail.", vec![ Term::Structure("fail".into(), vec![]) ]);
    query("halt.", vec![ Term::Structure("halt".into(), vec![]) ]);
    query("append(cons(1, nil), X, cons(1, cons(2, nil))).", vec![
        Term::Structure("append".into(), vec![
            Term::Structure("cons".into(), vec![
                Term::Structure("1".into(), vec![]),
                Term::Structure("nil".into(), vec![]),
            ]),
            Term::Variable(variable!("X")),
            Term::Structure("cons".into(), vec![
                Term::Structure("1".into(), vec![]),
                Term::Structure("cons".into(), vec![
                    Term::Structure("2".into(), vec![]),
                    Term::Structure("nil".into(), vec![]),
                ]),
            ]),
        ]),
    ]);
    query("even(X), prime(X).", vec![
        Term::Structure("even".into(), vec![
            Term::Variable(variable!("X")),
        ]),
        Term::Structure("prime".into(), vec![
            Term::Variable(variable!("X")),
        ]),
    ]);
    query("atom_length('Hello, world!', Len).", vec![
        Term::Structure("atom_length".into(), vec![
            Term::Structure("Hello, world!".into(), vec![]),
            Term::Variable(variable!("Len")),
        ]),
    ]);

    clause("append(nil, L, L).", Clause(Term::Structure("append".into(), vec![
        Term::Structure("nil".into(), vec![]),
        Term::Variable(variable!("L")),
        Term::Variable(variable!("L")),
    ]), vec![]));
    clause("append(cons(H, T), L2, cons(H, L3)) :- append(T, L2, L3).",
        Clause(Term::Structure("append".into(), vec![
            Term::Structure("cons".into(), vec![
                Term::Variable(variable!("H")),
                Term::Variable(variable!("T")),
            ]),
            Term::Variable(variable!("L2")),
            Term::Structure("cons".into(), vec![
                Term::Variable(variable!("H")),
                Term::Variable(variable!("L3")),
            ]),
        ]), vec![
            Term::Structure("append".into(), vec![
                Term::Variable(variable!("T")),
                Term::Variable(variable!("L2")),
                Term::Variable(variable!("L3")),
            ]),
        ]));
}
