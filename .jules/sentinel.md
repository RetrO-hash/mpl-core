## 2024-05-24 - Replace unsafe borrow_mut with try_borrow_mut in Solana programs
**Vulnerability:** Direct usage of `.borrow_mut()` on `RefCell` types (like `AccountInfo` lamports and data) can cause the Solana program to panic if there is an active borrow. This leads to silent failures and unexpected reentrancy effects where errors are bypassed by panicking instead of securely reverting the transaction.
**Learning:** `borrow_mut()` causes panics. Safely return errors instead to prevent bypassing other error handling paths.
**Prevention:** Always use `try_borrow_mut_lamports()?` and `try_borrow_mut_data()?` for Solana account fields.
