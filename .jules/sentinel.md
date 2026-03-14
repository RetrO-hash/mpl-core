## 2024-05-18 - Fix panic vector on unwrap in VerifiedCreators
**Vulnerability:** The VerifiedCreators plugin validator used an unwrap on an Option when calculating removed signatures, creating a potential panic vector that could be exploited to halt execution.
**Learning:** Safe pattern matching (if let Some) should always be used over unwrap or expect in Solana programs to prevent denial of service vectors and ungraceful panics.
**Prevention:** Adopt a strict no-unwrap policy for options in the Solana programs, using pattern matching and proper error propagation instead.
