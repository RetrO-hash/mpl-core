## 2024-05-15 - [RefCell Borrow Mut Panic in Solana Arithmetic]
**Vulnerability:** In `programs/mpl-core/src/utils/account.rs`, arithmetic operations combined with immediate assignment (like `**funds_dest_account_info.lamports.borrow_mut() = dest_starting_lamports.checked_add(...)`) can lead to runtime `RefCell` borrow panics because `.lamports()` might be called while a mutable borrow is held, or if an untracked multiple borrow happens. Also, using direct `borrow_mut()` on account `lamports` or `data` bypasses the safe check of `try_borrow_mut_lamports()`.

**Learning:** Safe state updates in Solana require using `try_borrow_mut_lamports()?` and computing new values before performing the mutable borrow. Using `try_borrow_mut_data()` rather than `borrow_mut()` ensures errors are gracefully handled rather than crashing the program.

**Prevention:** Ensure assignment lines like `**account.try_borrow_mut_lamports()? = new_value;` have `new_value` completely evaluated before the assignment to avoid unexpected evaluation orders causing borrow panics. Avoid `.unwrap()` and avoid direct `.borrow_mut()` on account info fields.
