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

pub fn atom(s: &str) -> Term {
    Term::Atom(String::from(s))
}
pub fn variable(s: &str) -> Term {
    Term::Variable(String::from(s))
}
pub fn compound(s: &str, v: Vec<Term>) -> Term {
    Term::Compound(String::from(s), v)
}

#[macro_export]
macro_rules! compound {
    ($name:tt : $($s:expr),+) => {
        compound($name, vec![$($s),+])
    };
    ($name:tt) => {
        compound($name, Vec::new())
    };
}