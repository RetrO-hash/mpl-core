## 2025-02-14 - Prevent RefCell double-borrow panics in lamport arithmetic
**Vulnerability:** Double-borrow RefCell panics.
**Learning:** Rust evaluates assignments right-to-left. Directly modifying lamports with `**account.try_borrow_mut_lamports()? = account.lamports().checked_add(x)?` triggers a panic because `.lamports()` borrows immutably, and `.try_borrow_mut_lamports()` attempts to borrow mutably *while* the immutable borrow is still active during the evaluation of the right-hand side. The same applies for direct assignment operators like `+=`.
**Prevention:** Always compute the new lamport/data values in a separate, dedicated variable *before* performing the `try_borrow_mut_lamports()?` assignment on the next line.
