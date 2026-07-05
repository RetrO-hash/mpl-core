## 2024-05-24 - [CRITICAL] Prevent panic from RefCell double borrow and integer overflow on lamports
**Vulnerability:** Lamport modifications used `.borrow_mut()` and unsafe `+=`/`-=` operators, and modifying `data` used `.borrow_mut()`, risking double borrow panics and integer overflow.
**Learning:** Must use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` instead of `.borrow_mut()`. When modifying lamports, avoid `+=` and compute new values before assigning with `.checked_add` and `.checked_sub`.
**Prevention:** Ensure read-compute-write order for lamport updates and always use `try_borrow` methods to map borrow failures into program errors instead of panics.
