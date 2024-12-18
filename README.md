# Rust Vector Out of Bounds Error

This repository demonstrates a common error in Rust when accessing elements in a vector using the `get()` method.  The code attempts to access an index that is out of bounds, leading to a runtime panic.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file shows how to correctly handle this scenario using pattern matching or the `unwrap_or` method.

This example highlights the importance of handling `Option` types returned by methods like `get()` to avoid unexpected panics in your Rust programs.