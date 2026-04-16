## 2024-05-30 - Fix integer overflow/underflow vulnerabilities

**Vulnerability:**
The `try_borrow_mut_lamports()? += ...` and `-=` operators were being used, which are inherently prone to causing unexpected program panics on integer overflow or underflow. In Solana, uncontrolled panics can lead to unexpected behaviors or could be used as a vector for denial-of-service, or just act as bugs.

**Learning:**
I learned that using `try_borrow_mut_lamports()? += lamports_diff` is an unsafe pattern. Because it does not use checked arithmetic, it can overflow. Rust evaluaties the left side first anyway which means a panic may occur if it reads from the same account. But here, the primary issue is the uncontrolled integer arithmetic which causes a program panic instead of a graceful error like `MplCoreError::NumericalOverflowError`. Also we should use `.checked_add` and `.checked_sub`.

**Prevention:**
Always use `.checked_add` and `.checked_sub` when dealing with `lamports`, and map the result to an explicit error type like `MplCoreError::NumericalOverflowError` to handle it safely.
