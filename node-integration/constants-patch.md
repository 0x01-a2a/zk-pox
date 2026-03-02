# Constants Patch — Adding ZK-PoX Program ID

## File: `node/crates/zerox1-node/src/constants.rs`

### Add after the existing program ID constants:

```rust
/// ZK-PoX program (experience credential storage).
#[cfg(feature = "devnet")]
pub const ZKPOX_PROGRAM_ID: &str = "ZKPoX1111111111111111111111111111111111111";
#[cfg(not(feature = "devnet"))]
pub const ZKPOX_PROGRAM_ID: &str = "REPLACE_WITH_MAINNET_PROGRAM_ID";
```

### Note:

After deploying the ZK-PoX Anchor program to devnet, replace the placeholder
`ZKPoX111...` ID with the actual deployed program ID.
