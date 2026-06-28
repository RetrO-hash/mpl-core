## 2024-05-24 - Double-Borrow RefCell Panics in Assignment Statements
**Vulnerability:** Double-borrow panic from calling `lamports()` (or `borrow()`) and assigning to `borrow_mut()` on the same account simultaneously, exacerbated by `+=` and `-=` causing math panics.
**Learning:** Rust evaluates the left-hand operand of an assignment first, attempting to calculate the right-hand side simultaneously can cause runtime `RefCell` borrow panics if the calculation also reads from the same account.
**Prevention:** Strictly use an alternating read-compute-write sequence for each account sequentially using `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?`. Map arithmetic results from safe operations like `.checked_add()` to errors like `MplCoreError::NumericalOverflowError`.
