## 2026-06-23 - Unsafe Lamport Modification Using Arithmetic Assignment

**Vulnerability:** Found the use of arithmetic assignment operators (`+=` and `-=`) when modifying lamports within account info borrowing. These can lead to critical numerical overflow or underflow panics, which represents a DoS and fund manipulation vulnerability on Solana.

**Learning:** Solana smart contracts must rigorously use safe math operations (like `checked_add` and `checked_sub`) directly mapping `None` to an explicit numerical error instead of defaulting to panic-prone arithmetic assignment for critical lamport operations.

**Prevention:** Completely avoid the `+=` and `-=` operators on lamport balances. Instead, calculate the new value safely using `checked_*` and then reassign.
