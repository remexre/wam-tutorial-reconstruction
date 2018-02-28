#![allow(unused_macros)]

macro_rules! atom {
    ($name:ident) => { $crate::common::Atom::from(stringify!($name)) }
}

macro_rules! functor {
    ($name:ident / $arity:expr) => {
        $crate::common::Functor(
            $crate::common::Atom::from(stringify!($name)),
            $arity,
        )
    }
}

macro_rules! variable {
    ($name:expr) => { $crate::common::Variable::from_str($name).unwrap() }
}
