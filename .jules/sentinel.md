## 2024-05-18 - [CRITICAL] RefCell Double Borrow Panics in Solana Programs

**Vulnerability:**
Calculations for new lamport values were performed simultaneously with mutably borrowing the target account's lamports in a single assignment statement (e.g., `**dest.try_borrow_mut_lamports()? = dest.lamports().checked_add(amount)?`). This caused `RefCell` double borrow panics because Rust evaluates the left-hand side of the assignment (the mutable borrow) before completing the right-hand side (which requires a read borrow of the same `RefCell`). Additionally, unsafe operator assignments like `+=` and `-=` were used with `.try_borrow_mut_lamports()?`, ignoring checked arithmetic and leading to potential panics on integer overflow/underflow.

**Learning:**
Solana program accounts wrap their lamports and data in `RefCell`s. Borrowing them mutably on the left side of an assignment while concurrently reading from them on the right side reliably crashes the program via a double borrow panic. Furthermore, using compound assignment operators (`+=`, `-=`) on these borrows is fundamentally unsafe because they bypass checked math.

**Prevention:**
1. Compute the new value entirely and store it in an intermediate local variable *before* acquiring the mutable borrow (e.g., `let new_lamports = dest.lamports().checked_add(amount)?;`).
2. Only after the calculation is complete, perform the assignment using a safe borrow (e.g., `**dest.try_borrow_mut_lamports()? = new_lamports;`).
3. Always strictly enforce the use of `.checked_add()` and `.checked_sub()` to prevent arithmetic panics, rejecting any code that uses `+=` or `-=` on account lamports.
