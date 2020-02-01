// Copyright (C) 2020 Julian Blaauboer

use crate::Term;
use std::collections::HashMap;

/// Instantiate the variable as much as possible.
pub fn instantiate(mut term: Term, instantiations: &HashMap<String, Term>) -> Term {
    while let Term::Variable(name) = &term {
        if let Some(next) = instantiations.get(name) {
            term = next.clone();
        } else {
            break;
        }
    }
    term
}

/// Helper function for unifying compound terms.
fn unify_compounds(
    args1: Vec<Term>,
    args2: Vec<Term>,
    mut instantiations: HashMap<String, Term>,
) -> (bool, HashMap<String, Term>) {
    for (arg1, arg2) in args1.into_iter().zip(args2.into_iter()) {
        let (success, i) = unify(arg1, arg2, instantiations);
        instantiations = i;
        if !success {
            return (false, instantiations);
        }
    }
    return (true, instantiations);
}

/// Unify two terms.
pub fn unify(
    mut a: Term,
    mut b: Term,
    mut instantiations: HashMap<String, Term>,
) -> (bool, HashMap<String, Term>) {
    // Instantiate the terms
    a = instantiate(a, &instantiations);
    b = instantiate(b, &instantiations);

    // Make sure the first term is the variable if there is one
    let (a, b) = if let Term::Variable(_) = b {
        (b, a)
    } else {
        (a, b)
    };

    // Unify depending on type
    match a {
        Term::Variable(name) => {
            instantiations.insert(name, b);
            return (true, instantiations);
        }
        Term::Atom(name1) => {
            if let Term::Atom(name2) = b {
                return (name1 == name2, instantiations);
            }
        }
        Term::Compound(functor1, args1) => {
            if let Term::Compound(functor2, args2) = b {
                if functor1 == functor2 && args1.len() == args2.len() {
                    let (success, new_instantiations) =
                        unify_compounds(args1, args2, instantiations.clone());
                    if success {
                        return (true, new_instantiations);
                    }
                }
            }
        }
    };

    // Otherwise, we have no match
    (false, instantiations)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// Test the function `instantiate`.
    fn test_instantiate() {
        let mut mapping = HashMap::new();
        mapping.insert("X".to_string(), Term::Variable("Y".to_string()));
        mapping.insert("Y".to_string(), Term::Atom("foo".to_string()));

        assert_eq!(
            instantiate(Term::Variable("X".to_string()), &mapping),
            Term::Atom("foo".to_string())
        );
    }
}
