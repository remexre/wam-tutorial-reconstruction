use common::{Clause, Functor, Structure, Term};
use common::parsers::{atom, clause, functor, query, term, variable};
use test_utils::example_query_term;

mod prop;

#[test]
fn term_contains() {
    let term = example_query_term();

    assert!(term.contains(&term));

    assert!(term.contains(&Term::Variable(variable!("Z"))));
    assert!(!term.contains(&Term::Variable(variable!("Y"))));

    assert!(term.contains(&Term::Structure(Structure(
        "f".into(),
        vec![Term::Variable(variable!("W"))]
    ))));
    assert!(!term.contains(&Term::Structure(Structure(atom!(f), vec![]))));
}

parse_tests! {
    atom("asdf", atom!(asdf));
    atom("'Asdf'", atom!(Asdf));
    atom("''", "".into());
    atom("12", "12".into());
    atom("'asdf'", atom!(asdf));
    atom("foo_bar", atom!(foo_bar));
    atom("'foo bar'", "foo bar".into());
    atom("'Hello, world!'", "Hello, world!".into());
    atom("'HELLO\\nWORLD'", "HELLO\nWORLD".into());

    variable("X", "X");
    variable("Foo", "Foo");
    variable("_", "_");
    variable("_Foo", "_Foo");

    functor("asdf/2", functor!(asdf/2));
    functor("''/0", Functor("".into(), 0));
    functor("12/3", Functor("12".into(), 3));
    functor("qwerty/123456", functor!(qwerty/123456));

    term("_", Term::Anonymous);
    term("X", Term::Variable(variable!("X")));
    term("123", Term::Structure(Structure("123".into(), vec![])));
    term("123()", Term::Structure(Structure("123".into(), vec![])));
    term("''()", Term::Structure(Structure("".into(), vec![])));
    term("a_b(c(D, E), F, _, '')", Term::Structure(Structure(atom!(a_b), vec![
        Term::Structure(Structure(atom!(c), vec![
            Term::Variable(variable!("D")),
            Term::Variable(variable!("E")),
        ])),
        Term::Variable(variable!("F")),
        Term::Anonymous,
        Term::Structure(Structure("".into(), vec![])),
    ])));
    term("p(Z, h(Z, W), f(W))", example_query_term());

    query(".", vec![]);
    query("true.", vec![ Structure(atom!(true), vec![]) ]);
    query("fail.", vec![ Structure(atom!(fail), vec![]) ]);
    query("halt.", vec![ Structure(atom!(halt), vec![]) ]);
    query("append(cons(1, nil), X, cons(1, cons(2, nil))).", vec![
        Structure(atom!(append), vec![
            Term::Structure(Structure(atom!(cons), vec![
                Term::Structure(Structure("1".into(), vec![])),
                Term::Structure(Structure(atom!(nil), vec![])),
            ])),
            Term::Variable(variable!("X")),
            Term::Structure(Structure(atom!(cons), vec![
                Term::Structure(Structure("1".into(), vec![])),
                Term::Structure(Structure(atom!(cons), vec![
                    Term::Structure(Structure("2".into(), vec![])),
                    Term::Structure(Structure(atom!(nil), vec![])),
                ])),
            ])),
        ]),
    ]);
    query("even(X), prime(X).", vec![
        Structure(atom!(even), vec![
            Term::Variable(variable!("X")),
        ]),
        Structure(atom!(prime), vec![
            Term::Variable(variable!("X")),
        ]),
    ]);
    query("atom_length('Hello, world!', Len).", vec![
        Structure(atom!(atom_length), vec![
            Term::Structure(Structure("Hello, world!".into(), vec![])),
            Term::Variable(variable!("Len")),
        ]),
    ]);

    clause("append(nil, L, L).", Clause(Structure(atom!(append), vec![
        Term::Structure(Structure(atom!(nil), vec![])),
        Term::Variable(variable!("L")),
        Term::Variable(variable!("L")),
    ]), vec![]));
    clause("append(cons(H, T), L2, cons(H, L3)) :- append(T, L2, L3).",
        Clause(Structure(atom!(append), vec![
            Term::Structure(Structure(atom!(cons), vec![
                Term::Variable(variable!("H")),
                Term::Variable(variable!("T")),
            ])),
            Term::Variable(variable!("L2")),
            Term::Structure(Structure(atom!(cons), vec![
                Term::Variable(variable!("H")),
                Term::Variable(variable!("L3")),
            ])),
        ]), vec![
            Structure(atom!(append), vec![
                Term::Variable(variable!("T")),
                Term::Variable(variable!("L2")),
                Term::Variable(variable!("L3")),
            ]),
        ]));
}
