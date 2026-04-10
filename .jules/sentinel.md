## 2024-05-31 - [RefCell Double Borrow Panics]
**Vulnerability:** Ungraceful panic via double borrows (`RefCell::borrow_mut()`) overlapping with `try_borrow_mut_lamports()`.
**Learning:** In Rust/Solana, when assigning to a mutably borrowed account field, if the expression calculation includes borrowing from the same account, it causes a `RefCell` borrow panic (e.g., `*account.borrow_mut() = account.lamports()`).
**Prevention:** Always compute the new value into a variable *before* acquiring the mutable borrow and assigning the new value. Also, use `try_borrow_mut_lamports()` over `.borrow_mut()` to return a clean program error instead of panicking.
