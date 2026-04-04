## 2024-05-24 - Account Try Borrow Mut Lamports
**Vulnerability:** Double borrow of `lamports` causing program panic.
**Learning:** In Solana, using `**account.try_borrow_mut_lamports()? += lamports_diff;` can result in a `RefCell` borrow panic if the program tries to borrow mutably multiple times when the assignment expands to read, calculate, and assign.
**Prevention:** Always use `.checked_add` / `.checked_sub` and calculate the new lamports value before assigning it to the dereferenced mutably borrowed lamports, or prefer `checked_add` and `checked_sub` directly to `+=` and `-=` operator to prevent underflow and overflow as well as to keep borrow references limited in scope.
