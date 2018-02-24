use common::Term;

macro_rules! variable {
    ($name:expr) => { $crate::common::Variable::from_str($name).unwrap() }
}

/// Returns the term `p(Z, h(Z, W), f(W))`.
pub fn example_term() -> Term {
    Term::Structure(
        "p".into(),
        vec![
            Term::Variable(variable!("Z")),
            Term::Structure(
                "h".into(),
                vec![
                    Term::Variable(variable!("Z")),
                    Term::Variable(variable!("W")),
                ],
            ),
            Term::Structure("f".into(), vec![
                Term::Variable(variable!("W")),
            ]),
        ],
    )
}
