## 2024-05-24 - [CRITICAL] Prevent Solana Account Lamports Borrow Panic

**Vulnerability:** Double borrow of `RefCell` using `.borrow_mut()` when assigning calculated lamports

**Learning:** `solana_program::account_info::AccountInfo` stores `lamports` as an `Rc<RefCell<&mut u64>>`. If you use `.borrow_mut()` on the left hand side of an assignment, and the right hand side calculates a value using `.lamports()` (which internally uses `.borrow()`), the assignment order in Rust causes the `borrow_mut()` to occur *first*, resulting in a `RefCell already borrowed` panic at runtime. This allows an attacker to repeatedly intentionally trigger panics, leading to Denial of Service (DoS) and failed transactions.
**Prevention:**
1. Never use `**account_info.lamports.borrow_mut() = account_info.lamports().checked_add(...).unwrap();`.
2. Calculate the new value *first* into a local variable, and only *then* mutably borrow the account to assign it.
3. For accounts, prefer using `**account_info.try_borrow_mut_lamports()? = new_lamports;` to ensure any double-borrows are returned gracefully as an error rather than a runtime panic.
