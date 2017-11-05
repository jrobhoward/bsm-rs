# Commands
* ```cargo build```
* ```cargo build --release```
* ```cargo test```
* ```cargo update```
* ```cargo run --example main```
* ```rustfmt src/lib.rs```


# Notes
## goals
* Safe, Concurrent Systems Language

* Cargo: package management
    - source code
    - dependencies

* IDE
    - Yes, Visual Studio (code)
        + Linux!
        + Plugins - code completion
            * Rusty Code
            * Vim
            * Native Debug
    - For FreeBSD: Sublime + modules

* Variables are immuable by default
    - use ```mut``` with ```let``` to make mutable
    - var can be reassigned with ```let``` a second time
* String::new "associated function" (static)
* Compiler provides excellent feedback
    - e.g. reassignment of immutable variables
    - e.g. unable to infer type
        + ```let guess = "42".parse().expect("Not a number!");```
        + ```let guess: u32 = "42".parse().expect("Not a number!");```
    - e.g. (runtime) index out of bounds
* Numbers can contain underscores for readability
    - e.g. ```let million = 1_000_000```
* Type inference
    - e.g. ```let million = 1_000_000```
    - e.g. ```let million:u64 = 1_000_000```
* Machine size/achitecture: ```isize``` & ```usize```
    - similar to ```size_t```
* Documentation accepts markdown, not html
* ```char``` is unicode scalar variable
* Tuple
    - comma-separated list inside parenthesis
    - pattern matching, ```let x y z = (0,1.2,100);``` can be used to decompose a tuple
* Arrays
    - cannot grow or shrink in size
    - out-of-bounds is checked at runtime too
*

## Types
## Operators

## Control Flow
* if
    - else if
    - if in a let statement
* loop
    - until ```break```
* while
    - until condition is met
* for

## Ownership
* Enables language to make memory safety guarantees without needing a garbage collector.
* Checked at compile time, no runtime overhead
* Managing heap data is why ownership exists
* rust calls ```drop``` upon a closing ```}``` automatically for us.

### Rules
* Each value has a variable that's its owner
* There can only be one owner at a time
* When a variable goes out of scope, the value will be dropped

### Borrowing
### Slices
### Structs
### Enums
### Macros


* Find out if contract/collaboration tests are possible (mocks / verify)
* Rust's approach is to separate data (struct & enum) from behavior (impl), and using trait to group the behaviors.  Helps advocate composition over inheritence.
* look into rust's enums: https://users.rust-lang.org/t/why-not-just-add-classes/4618/3
* rust used to have classes, used to have a GC
* separating data from state has advantages (e.g. serialization & FFI)
* git server for cargo: http://doc.crates.io/specifying-dependencies.html





