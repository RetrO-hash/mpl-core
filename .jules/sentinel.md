## 2024-03-21 - [Fix Double Borrow RefCell Panics]
**Vulnerability:** In `programs/mpl-core/src/processor/collect.rs` and `programs/mpl-core/src/utils/account.rs`, the program was modifying lamports by using inline operations that caused double borrowing of a RefCell, e.g. `**dest.lamports.borrow_mut() = dest.lamports().checked_add(...)`. Because Rust evaluates the left-hand side of assignments before the right-hand side, a mutable borrow of `lamports` was held while an immutable borrow (`.lamports()`) was attempted on the RHS. In Solana, account `lamports` and `data` are wrapped in `Rc<RefCell<...>>`. Double borrowing the RefCell caused the program to crash ungracefully (panic). In addition, direct usage of `+=` and `-=` operators on lamports inside `account.rs` could lead to panics on arithmetic underflow/overflow.

**Learning:** Solanas `AccountInfo` uses `Rc<RefCell>` internally. The order of operations in Rust assignment statements causes left side to evaluate first. Unsafe operations like `.lamports.borrow_mut()` and `+=` bypass safe error handling. This could be used for Denial of Service, or to inadvertently prevent necessary state changes because a transaction crashes entirely instead of failing gracefully.

**Prevention:**
1. Always calculate values before assigning them when updating `lamports` or `data` fields.
2. Use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?` which return an error gracefully rather than using `.borrow_mut()` which panics.
3. Always use `.checked_add()` and `.checked_sub()` for all arithmetic.