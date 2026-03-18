## 2024-05-24 - [Fix Integer Overflow/Underflow Risk in Lamport Transfers]
**Vulnerability:** Unsafe operators (`+=`, `-=`) were used on account lamports, which could panic programs on overflow/underflow. Also, `.borrow_mut()` was being called directly on `data`, which can trigger double borrow panics during execution, terminating the transaction unexpectedly.
**Learning:** `try_borrow_mut_lamports` returns a `RefMut` whose value should be updated cautiously via safely checked arithmetic functions instead of operators to avoid unhandled panics that act as a DOS attack vector.
**Prevention:** Avoid `+=` and `-=` entirely in Solana programs for value manipulation like lamports, and use `.checked_add()` and `.checked_sub()`. Avoid `.borrow_mut()` in favor of `.try_borrow_mut_data()?`.
