## 2026-06-16 - [Safe RefCell Borrows and Checked Arithmetic in Solana Smart Contracts]
**Vulnerability:** Ungraceful panics using `.borrow_mut()` for lamports/data and potential integer overflows/underflows with `+=` and `-=` operators on lamports balances.
**Learning:** In Solana programs, `.borrow_mut()` can panic if a `RefCell` is already borrowed, and direct arithmetic can silently overflow/underflow, causing severe smart contract vulnerabilities (DoS or token minting/draining bugs).
**Prevention:** Strictly use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` to safely propagate errors. Always use `.checked_add()` and `.checked_sub()` mapping to an explicit error like `MplCoreError::NumericalOverflowError` when mutating balances.
