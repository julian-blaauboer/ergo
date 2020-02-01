extern crate ergo;

fn main() {
    // For readability of terms.
    use ergo::helper::{atom, variable};
    use ergo::compound;

    // Create a new universe.
    let mut uni = ergo::resolution::Universe::new();

    // Add some facts and rules to the universe
    // The equivalent prolog clauses are:
    // human(socrates).
    // human(plato).
    // mortal(X) :- human(X).
    uni.add_fact(compound!("human": atom("socrates")));
    uni.add_fact(compound!("human": atom("plato")));
    uni.add_rule(compound!("mortal": variable("X")), vec![
        compound!("human": variable("X"))
    ]);

    // Query the universe
    let results = uni.query(vec![
        compound!("mortal": variable("X"))
    ]);

    // Universe::query returns an iterator.
    // Queries are lazily evaluated.
    for result in results {
        // Instantiate the variable from the instantiations
        let x = ergo::unification::instantiate(variable("X"), &result);
        println!("X = {:?}", x);
        // Should output:
        // X = Atom("socrates")
        // X = Atom("plato")
    }
}