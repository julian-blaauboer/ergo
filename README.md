# Ergo
Minimal subset of a Prolog engine written in Rust

## Why?
This library is meant as part of a school project, it is by no means complete and isn't meant
to be used professionally in any way. A lot of features are missing or incomplete.
It doesn't support:
- Anonymous variables
- Built-in predicates
- Lists
- And a lot more

## Tutorial
*Full source code available in [examples](examples/socrates.rs)*

We start off by using a few helper functions to help us build terms:
```rust
// For readability of terms.
use ergo::helper::{atom, variable};
use ergo::compound;
```
Then we instantiate a new universe. An universe is a set of clauses that can be queried.
```rust
// Create a new universe.
let mut uni = ergo::resolution::Universe::new();
```
For this example we will add the following prolog clauses to our universe:
```prolog
human(socrates).
human(plato).
mortal(X) :- human(X).
```
Translated this becomes:
```rust
uni.add_fact(compound!("human": atom("socrates")));
uni.add_fact(compound!("human": atom("plato")));
uni.add_rule(compound!("mortal": variable("X")), vec![
    compound!("human": variable("X"))
]);
```
After this the universe is ready to be queried, an universe can be queried in the following
way:
```rust
// Query the universe
let results = uni.query(vec![
    compound!("mortal": variable("X"))
]);
```
The variable "results" is an iterator that lazily evaluates the next solution.
It returns an `HashMap<String,ergo::Term>` containing all the instantiations of the variables.
```rust
for result in results {
    // Instantiate the variable from the instantiations
    let x = ergo::unification::instantiate(variable("X"), &result);
    println!("X = {:?}", x);
    // Should output:
    // X = Atom("socrates")
    // X = Atom("plato")
}
```
