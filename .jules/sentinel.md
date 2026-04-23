
## 2024-05-18 - RefCell Borrow Panics and Unwrap
**Vulnerability:** Found uses of `.borrow_mut()` on `lamports` and `.data.borrow_mut()` to access or change an account's data, which can lead to double borrow program panics (returning generic runtime errors) if an attacker or unexpected execution path tries to borrow again or if already borrowed. Additionally, found an unnecessary `.unwrap()` that could lead to crashes in failure scenarios.
**Learning:** In Solana smart contracts, it is critical to use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` to safely access RefCells and propagate the borrow failure correctly instead of panicking.
**Prevention:** Avoid using direct `.borrow_mut()` and `.unwrap()` on account fields, ensuring safe matching or `Result` handling in the codebase.
