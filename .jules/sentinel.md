## 2024-05-18 - Prevent Panics via Safe Borrowing & Checked Math

**Vulnerability:** Direct `RefCell` borrowing (`borrow_mut()`) and unsafe arithmetic (`+=`, `-=`) on Solana account balances.
**Learning:** In Solana smart contracts, using `borrow_mut()` on `AccountInfo` fields like lamports or data can lead to immediate program panics if double borrows accidentally occur during execution. Likewise, unsafe math can cause overflows/underflows or panics.
**Prevention:** Strictly utilize `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` to securely bubble up errors instead of panicking. Also replace unsafe lamport math with `checked_add`/`checked_sub` and map failures to specific errors like `MplCoreError::NumericalOverflowError`.
