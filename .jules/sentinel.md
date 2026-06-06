## 2026-06-06 - Unsafe Arithmetic Operations Modifying Lamports
**Vulnerability:** Use of += and -= on try_borrow_mut_lamports() instead of checked_add and checked_sub in resize_or_reallocate_account and close_program_account.
**Learning:** In Solana, even when subtracting or adding to an account's lamports directly, it must be done with checked math to prevent underflow/overflow panics. While close_program_account used checked math to calculate new lamports for funds_dest_account_info, it subtracted with -= from the source account. resize_or_reallocate_account used += and -=
**Prevention:** Always use checked_add and checked_sub on try_borrow_mut_lamports() when adjusting lamports directly, returning MplCoreError::NumericalOverflowError if it fails.
