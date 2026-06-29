## 2024-05-25 - Prevent RefCell Double Borrow Panics
**Vulnerability:** Double borrow panic from attempting to borrow an account's `lamports` mutably while evaluating a value that also reads from it (e.g. `**account.lamports.borrow_mut() = account.lamports() + X`).
**Learning:** Rust evaluates the left-hand operand first. This means `borrow_mut()` holds a mutable borrow while the right-hand operand invokes `lamports()`, which also borrows from the same RefCell, resulting in a runtime panic that crashes the program.
**Prevention:** First compute the new balance using `let new_balance = account.lamports() + X;`, and then assign it securely via `**account.try_borrow_mut_lamports()? = new_balance;`.
