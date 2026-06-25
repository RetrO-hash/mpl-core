## 2026-06-25 - Prevent ungraceful panics in lamport transfers
**Vulnerability:** Ungraceful panics caused by direct `.borrow_mut()` on RefCells, left-hand side assignment evaluation panics, and unsafe `+=` / `-=` operator usage.
**Learning:** In Solana, assigning a computed value back to a mutably borrowed account field using `.lamports.borrow_mut() = ...` or using `+=` panics ungracefully or causes double borrows. Using `.try_borrow_mut_lamports()?` ensures failures return `ProgramError` gracefully.
**Prevention:** Always read balances first, compute new balances using safe math (`.checked_add()`), and then assign using `.try_borrow_mut_lamports()?`.
