## 2024-05-01 - Prevent RefCell Borrow Panics and Integer Overflows in Lamport Operations
**Vulnerability:** Core lamport manipulation logic used unsafe `.borrow_mut()` and operators (`+=`, `-=`) during multiple simultaneous references to account balances.
**Learning:** Rust's `RefCell` and the way the left side of the assignment is resolved early caused unexpected program panics ("double borrow"), exposing the program to Denial of Service (DoS) and potential overflows when interacting with user lamports.
**Prevention:** Strictly use `.try_borrow_mut_lamports()?` to gracefully bubble up errors instead of panicking, and exclusively use `.checked_add()` / `.checked_sub()` to prevent any possibility of integer overflow. Pre-compute the new value before assignment to satisfy borrowing rules.
