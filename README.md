# Rust Iterator Exhaustion Panic

This repository demonstrates a common error in Rust: panicking due to accessing an iterator after it has been exhausted.

The `bug.rs` file contains the buggy code.  The `bugSolution.rs` file provides a corrected version.

The error occurs because the code attempts to access an element from the iterator after it has already yielded all elements from the vector.