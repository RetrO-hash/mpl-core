## 2024-05-31 - RefCell Double Borrow Panics and Unsafe Lamport Math
**Vulnerability:**
The `mpl-core` program contains instances of directly modifying lamports using unsafe `+=` and `-=` operators on borrowed RefCell values (e.g., `**account.try_borrow_mut_lamports()? += amount;`). Additionally, there are instances where lamports are calculated and directly assigned to a mutably borrowed account without evaluating the initial balance first (e.g., `**dest.try_borrow_mut_lamports()? = dest.lamports().checked_add(amount)?;`).

**Learning:**
1. In Solana programs, modifying lamports directly using `+=` or `-=` can lead to unhandled overflow/underflow panics, which are not gracefully returned as program errors.
2. Rust evaluates the left-hand operand of an assignment first. When calculating a new balance based on the current balance during the assignment to `try_borrow_mut_lamports()`, the left-hand side establishes a mutable borrow. The right-hand side then attempts an immutable borrow (e.g., `.lamports()`) on the same `RefCell`, resulting in a runtime `BorrowError` panic (a double borrow).
3. Directly calling `.data.borrow_mut()` can cause similar double-borrow panics if there are active immutable borrows.

**Prevention:**
1. Always calculate the new balance or data modification BEFORE establishing a mutable borrow for the assignment. Store the initial balance in a local variable if needed.
2. Avoid unsafe operators like `+=` and `-=` for lamport modifications. Use `.checked_add()` and `.checked_sub()`, propagating the `NumericalOverflowError` on failure.
3. Consistently use `try_borrow_mut_lamports()?` and `try_borrow_mut_data()?` instead of direct `.borrow_mut()` to ensure failures return gracefully rather than panicking the program.