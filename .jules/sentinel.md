## 2024-05-18 - [Integer Overflow and Panic Prevention in Lamport Mutations]
**Vulnerability:** Use of direct `.borrow_mut()` and unsafe arithmetic operators (`+=`, `-=`) on account lamports in `utils/account.rs`, which could lead to double-borrow panics or integer overflows (denial of service/exploit vector).
**Learning:** In Rust/Solana, mutating `RefCell`s with `+=`/`-=` while also calculating the delta inline can trigger an ungraceful program panic. Direct mutable borrows bypass safe error propagation.
**Prevention:** Always compute delta values first, then use `.checked_add()` or `.checked_sub()`, and explicitly assign the result using `.try_borrow_mut_lamports()?` to propagate `Result` gracefully rather than panicking.
