## 2024-05-27 - [Fix Panics and Unsafe Account Operations]
**Vulnerability:** Found uses of panicking `.unwrap()` on `Option`/`Result`, direct RefCell borrow panics (`.borrow_mut()`), and unsafe arithmetic (`+=`, `-=`) on account lamports in `programs/mpl-core/src/utils/account.rs`.
**Learning:** In Solana smart contracts, relying on `.unwrap()`, direct `.borrow_mut()` on AccountInfo fields, or basic arithmetic can lead to ungraceful program aborts (panics) or underflow/overflow vulnerabilities, which act as potential vectors for DoS or loss of funds.
**Prevention:** Always use safe fallbacks or bubble up errors: `.try_borrow_mut_lamports()?`, `.try_borrow_mut_data()?`, and `.checked_add()` / `.checked_sub()`.
