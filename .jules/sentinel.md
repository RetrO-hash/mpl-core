## 2024-05-24 - [Replace borrow_mut() with try_borrow_mut_lamports() / try_borrow_mut_data()]
**Vulnerability:** Double borrow of `lamports` or `data` RefCell causes ungraceful program panics in Solana.
**Learning:** Using `.borrow_mut()` directly on an `AccountInfo`'s `lamports` or `data` can lead to program crashes if the `RefCell` is already borrowed mutably or immutably elsewhere. Instead, one should use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` to allow the program to fail gracefully with an error rather than panicking.
**Prevention:** Always use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` on `AccountInfo` struct.
