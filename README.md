# Multiple Mutable Borrows in Rust
This repository demonstrates a common error in Rust: attempting to create multiple mutable borrows of the same variable.  Rust's ownership and borrowing system prevents data races by ensuring that only one mutable reference to a given piece of data can exist at any time. 

The `bug.rs` file contains code that attempts to violate this rule, resulting in a compile-time error. The `bugSolution.rs` file provides a solution illustrating how to correctly handle this situation.