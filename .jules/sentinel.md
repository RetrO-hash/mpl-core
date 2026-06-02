## 2024-11-21 - RefCell and Integer Overflow Security Risks
**Vulnerability:** Critical potential for panics and mathematical manipulation due to unsafe `.borrow_mut()` and non-checked `+=`/`-=` operators when modifying account lamports.
**Learning:** Smart contracts in Rust must never trust un-checked mathematics (due to wrapping/panic properties of numbers in Rust) nor standard cell borrows (due to the single mutable borrow rule failing dynamically when a caller or nested call already holds a lock).
**Prevention:** Always use `.lamports()`, `try_borrow_mut_lamports()`, and `.checked_add()` / `.checked_sub()` for account math and data modification. Never use `+=`, `-=`, or standard `borrow_mut()`.
