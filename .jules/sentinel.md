## 2024-05-24 - [CRITICAL] Fix ungraceful panic via double borrow
**Vulnerability:** Ungraceful panics caused by double borrows using `.borrow_mut()` to modify lamport balances (e.g. in `collect.rs`).
**Learning:** Using `.borrow_mut()` on account lamports or data (RefCells) in Solana programs is dangerous and can lead to ungraceful program panics if a double borrow occurs.
**Prevention:** Instead, use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` which prevent panics by returning a clean error on failure.
