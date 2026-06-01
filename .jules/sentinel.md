## 2024-06-01 - [Plugin Lifecycle Validation Bypass]
**Vulnerability:** The `burn_collection` instruction in `mpl-core` completely bypassed the plugin validation framework, omitting the call to `validate_collection_permissions`.
**Learning:** In smart contracts with generic plugin frameworks, failure to explicitly invoke validation checks on every entrypoint effectively nullifies the framework's guarantees, leading to severe authorization and security bypasses for managed assets.
**Prevention:** Ensure that all lifecycle events (e.g., create, burn, transfer, update) have corresponding, uniformly-applied validation checks for both discrete assets and collections before any critical state changes or account closures occur.
