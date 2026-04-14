## 2024-10-27 - [CRITICAL] RefCell overlapping borrow panics
**Vulnerability:** Double mut-borrow panics could trigger a Denial of Service.
**Learning:** `AccountInfo` fields evaluate mathematically right to left in rust, mutating earlier than when evaluated leading to a RefCell double borrow panic.
**Prevention:** In Rust/Solana, when assigning to a mutably borrowed account field (e.g., `try_borrow_mut_lamports()`), compute the new value before the assignment statement. Avoid using `.borrow_mut()` safely. Use `.try_borrow_mut_lamports()?`
