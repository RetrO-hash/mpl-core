## 2024-03-08 - Rust panic on None collection
**Vulnerability:** unwrap() call on an Option could cause panic if collection is None.
**Learning:** The previous code had a check for collection.is_none(), but this approach is brittle. If the check is modified or removed by a future developer, it will panic and cause the Solana program to halt unexpectedly.
**Prevention:** Use pattern matching (if let Some) instead of unwrap() to extract the value from Option.
