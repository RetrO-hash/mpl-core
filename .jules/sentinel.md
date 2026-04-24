## 2024-05-24 - Double Borrow Panics & Overflow Vulnerabilities

**Vulnerability:** Found multiple instances of `account.lamports.borrow_mut()` and unchecked math (`+=` / `-=`) on lamports in `programs/mpl-core/src/utils/account.rs` during account close and reallocation.
**Learning:** `RefCell` borrows (`borrow_mut()`) will panic the Solana program if the cell is already borrowed elsewhere (a common risk in complex call stacks). Unchecked arithmetic on lamports can panic via overflow/underflow, acting as a Denial of Service vector.
**Prevention:** Always use the safer, fallible alternatives `try_borrow_mut_lamports()?` and `try_borrow_mut_data()?`. Wrap all lamport math in `checked_add` or `checked_sub` and cleanly return an error (like `NumericalOverflowError`) to ensure graceful transaction failures rather than raw panics.
