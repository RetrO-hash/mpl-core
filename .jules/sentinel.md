## 2024-10-24 - Rust Evaluation Order RefCell Panics
**Vulnerability:** A DoS vulnerability via program crash, caused by runtime "RefCell already borrowed" panics in lamport assignments (`**dest.lamports.borrow_mut() = dest.lamports().checked_add(...)`).
**Learning:** Rust evaluates assignments left-to-right. The left operand (`borrow_mut()`) takes a mutable borrow over the RefCell before the right side evaluates. When the right side evaluates `.lamports()`, it attempts an immutable borrow on the *same* RefCell, which panics because the mutable borrow is actively held.
**Prevention:** Compute the right-hand-side expression entirely in a separate local variable *before* invoking `try_borrow_mut_lamports()?` on the left side of the assignment.
