## 2024-05-18 - Fix panics and overflow risks in lamport updates
**Vulnerability:** Use of direct `.borrow_mut()` on `.lamports` and `.data` fields of `AccountInfo` and using `+=`/`-=` for lamport arithmetic.
**Learning:** `borrow_mut()` causes ungraceful program panics on double borrows. `+=`/`-=` are unsafe for lamport updates as they don't gracefully return an error if underflow/overflow occurs. Left-to-right evaluation combined with right-side operations can lead to panics if an account alias is encountered.
**Prevention:** Strictly use `.try_borrow_mut_lamports()?` and `.try_borrow_mut_data()?`. Compute the new value *before* assigning it to the mutable borrow. Use `.checked_add()` and `.checked_sub()` to prevent overflows.
