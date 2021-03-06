// Copyright (C) 2020 Julian Blaauboer

pub mod resolution;
pub mod unification;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Term type of which predicates and queries are built.
pub enum Term {
    Atom(String),
    Variable(String),
    Compound(String, Vec<Term>),
}

// For easier construction
pub mod helper {
    use super::*;
    pub fn atom(s: &str) -> Term {
        Term::Atom(String::from(s))
    }
    pub fn variable(s: &str) -> Term {
        Term::Variable(String::from(s))
    }

    #[macro_export]
    macro_rules! compound {
        ($name:tt : $($s:expr),+) => {
            $crate::Term::Compound(String::from($name), vec![$($s),+])
        };
        ($name:tt) => {
            $crate::Term::Compound(String::from($name), Vec::new())
        };
    }
}