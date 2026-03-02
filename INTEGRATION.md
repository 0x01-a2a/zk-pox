# ZK-PoX Integration Guide

Step-by-step instructions for integrating ZK-PoX into the `mobile` and `node` repositories.

**Nothing in this folder modifies existing repos directly.**
All files are self-contained. Copy them to the locations specified below.

---

## Prerequisites

- Rust toolchain (1.75+) with `cargo-ndk` for Android cross-compilation
- Android NDK r25+ (for building the `zkpox-mobile` shared library)
- Solana CLI + Anchor CLI (for deploying the on-chain program)

---

## 1. Mobile App — Android (Kotlin)

### 1.1 Copy Kotlin files

```
zk-pox/android/GpsLogger.kt   → mobile/android/app/src/main/java/world/zerox1/node/GpsLogger.kt
zk-pox/android/GpsDatabase.kt → mobile/android/app/src/main/java/world/zerox1/node/GpsDatabase.kt
zk-pox/android/ZkPoxModule.kt → mobile/android/app/src/main/java/world/zerox1/node/ZkPoxModule.kt
```

### 1.2 What each file does

| File | Role |
|------|------|
| `GpsLogger.kt` | Passive GPS collection every 5 min. Signs each point with Ed25519 (API 33+) or HMAC-SHA256 fallback. |
| `GpsDatabase.kt` | Encrypted SQLite. Stores signed GPS points with time-range queries and night counting. |
| `ZkPoxModule.kt` | React Native bridge. Exposes `getGpsStats`, `generateProof`, `verifyProof`, `analyzeSpoofRisk`, `countNightsNear`. |

### 1.3 JNI signatures

`ZkPoxModule.kt` calls these native functions from `libzkpox_mobile.so`:

```kotlin
external fun generateProofNative(pointsJson: String, requestJson: String): String
external fun verifyProofNative(resultJson: String): Int      // 1 = valid, 0 = invalid, -1 = error
external fun analyzeSpoofRiskNative(pointsJson: String): String
```

These match the `#[no_mangle] pub extern "system"` functions in `rust/crates/zkpox-mobile/src/lib.rs`.

### 1.4 Add Android dependencies

In `mobile/android/app/build.gradle`, add inside `dependencies { }`:

```gradle
implementation("com.google.android.gms:play-services-location:21.3.0")
```

### 1.5 Wire GPS Logger into NodeService

Follow the instructions in `zk-pox/android/NodeService.patch`:

1. Add `gpsLogger` and `gpsDatabase` fields to `NodeService`
2. Start the logger in `onCreate()` after wakeLock acquisition
3. Stop the logger in `onDestroy()`

### 1.6 Register ZkPoxModule in React Native

In `MainApplication.kt` (or wherever `ReactPackage` modules are registered), add `ZkPoxModule` to the list of native modules. Follow the same pattern as `NodeModule`.

### 1.7 Build the native library

Cross-compile `zkpox-mobile` for Android:

```bash
cd zk-pox/rust
cargo ndk -t arm64-v8a -t armeabi-v7a -o ../mobile/android/app/src/main/jniLibs build --release -p zkpox-mobile
```

This produces:
- `jniLibs/arm64-v8a/libzkpox_mobile.so`
- `jniLibs/armeabi-v7a/libzkpox_mobile.so`

### 1.8 Add location permissions (if not already present)

In `mobile/android/app/src/main/AndroidManifest.xml`:

```xml
<uses-permission android:name="android.permission.ACCESS_FINE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_COARSE_LOCATION" />
<uses-permission android:name="android.permission.ACCESS_BACKGROUND_LOCATION" />
```

---

## 2. Mobile App — React Native

### 2.1 Copy React Native files

```
zk-pox/react-native/Credentials.tsx → mobile/src/screens/Credentials.tsx
zk-pox/react-native/useZkPox.ts     → mobile/src/hooks/useZkPox.ts
zk-pox/react-native/ZkPoxModule.ts  → mobile/src/native/ZkPoxModule.ts
```

### 2.2 What each file does

| File | Role |
|------|------|
| `Credentials.tsx` | Full screen: GPS stats, anti-spoofing analysis, claim type selection, proof generation, result display. |
| `useZkPox.ts` | Hook wrapping all native calls. Manages stats, proof state, spoof analysis, verify. |
| `ZkPoxModule.ts` | TypeScript types for `GpsStats`, `ProofRequest`, `SpoofAnalysis`, `ZkPoxModuleInterface`. |

### 2.3 Add navigation

```tsx
<Stack.Screen name="Credentials" component={CredentialsScreen} />
```

### 2.4 Fix import paths

Adjust relative imports based on final directory structure:

```tsx
// In Credentials.tsx:
import { useZkPox } from '../hooks/useZkPox';

// In useZkPox.ts:
import ZkPoxModule from '../native/ZkPoxModule';
```

---

## 3. Solana — Anchor Program

### 3.1 Create program directory

```bash
mkdir -p node/programs/workspace/programs/zk-pox/src
```

### 3.2 Copy files

```
zk-pox/solana/Cargo.toml  → node/programs/workspace/programs/zk-pox/Cargo.toml
zk-pox/solana/src/lib.rs   → node/programs/workspace/programs/zk-pox/src/lib.rs
```

### 3.3 On-chain credential fields (v2)

The `ExperienceCredential` PDA stores:

| Field | Type | Description |
|-------|------|-------------|
| `version` | u8 | Schema version (currently 2) |
| `agent_id` | [u8; 32] | SATI agent public key |
| `claim_type` | u8 | 0=Residency, 1=Commute, ..., 5=Travel |
| `proof_hash` | [u8; 32] | SHA-256 of the Bulletproof bytes |
| `public_inputs_hash` | [u8; 32] | SHA-256 of public inputs |
| `commitments_hash` | [u8; 32] | SHA-256 of Pedersen commitments (binds proof to credential) |
| `count_proven` | u32 | Number of GPS points cryptographically proven |
| `witness_count` | u8 | Number of mesh peer attestations |
| `issued_at` | i64 | Unix timestamp |
| `revoked` | bool | Revocation flag |
| `witnesses` | [[u8; 32]; 8] | Up to 8 peer attestation keys |

### 3.4 Add to workspace

In `node/programs/workspace/Cargo.toml`, add `"programs/zk-pox"` to the `members` array.

### 3.5 Update Anchor.toml

Add under `[programs.localnet]` and `[programs.devnet]`:

```toml
zk_pox = "ZKPoX1111111111111111111111111111111111111"
```

### 3.6 Build and deploy

```bash
cd node/programs/workspace
anchor build -p zk_pox
anchor deploy -p zk_pox --provider.cluster devnet
```

After deployment, replace the placeholder program ID (`ZKPoX111...`) in both
`lib.rs` (`declare_id!()`) and `Anchor.toml` with the actual deployed ID.

---

## 4. Node — Mesh Integration (Rust)

### 4.1 Copy module

```
zk-pox/node-integration/zkpox.rs → node/crates/zerox1-node/src/zkpox.rs
```

### 4.2 Register module

Add to `node/crates/zerox1-node/src/lib.rs`:

```rust
pub mod zkpox;
```

### 4.3 Add message handling

Follow `zk-pox/node-integration/node-patch.md` to add:
- `CORROBORATE_REQUEST` and `CORROBORATE_RESPONSE` match arms in `handle_pubsub_message`
- `pending_corroborations` field to the `Node` struct

### 4.4 Add program ID constant

Follow `zk-pox/node-integration/constants-patch.md` to add
`ZKPOX_PROGRAM_ID` to `constants.rs`.

### 4.5 Add zkpox-core dependency

In `node/crates/zerox1-node/Cargo.toml`, add:

```toml
zkpox-core = { path = "../../zk-pox/rust/crates/zkpox-core" }
```

---

## 5. Core Rust Library

The `zk-pox/rust/` directory is a standalone Rust workspace containing:

| Crate | Purpose |
|---|---|
| `zkpox-core` | Types, commitments, circuit, prover, verifier, antispoof |
| `zkpox-mobile` | JNI bridge for Android (compiles to `.so`) |

### Modules in zkpox-core

| Module | Description |
|--------|-------------|
| `types.rs` | `SignedGPSPoint`, `ClaimType`, `ProofRequest`, `ProofResult`, `PublicInputs`, etc. |
| `commitment.rs` | SHA-256 position/time commitments, proof hashing, GPS point message hashing |
| `circuit.rs` | Bounding-box geofence approximation, coordinate scaling, point filtering |
| `prover.rs` | Aggregate Bulletproofs range proofs on Pedersen-committed GPS coordinate offsets |
| `verifier.rs` | Full cryptographic verification of aggregate range proofs against commitments |
| `antispoof.rs` | Teleportation detection, velocity analysis, zero-noise mock GPS detection |

### Build and test

```bash
cd zk-pox/rust
cargo build
cargo test   # 28 tests
```

### Key dependencies

| Crate | Version | Purpose |
|---|---|---|
| `bulletproofs` | 4 | ZK range proofs (no trusted setup) |
| `curve25519-dalek-ng` | 4 | Elliptic curve for Bulletproofs / Pedersen |
| `merlin` | 3 | Fiat-Shamir transcript |
| `sha2` | 0.10 | Position/time commitments |
| `ed25519-dalek` | 2 | GPS point signing |
| `jni` | 0.21 | JNI bridge for Android |

---

## File Map Summary

| Source | Destination | Type |
|---|---|---|
| `android/GpsLogger.kt` | `mobile/android/.../world/zerox1/node/GpsLogger.kt` | Copy |
| `android/GpsDatabase.kt` | `mobile/android/.../world/zerox1/node/GpsDatabase.kt` | Copy |
| `android/ZkPoxModule.kt` | `mobile/android/.../world/zerox1/node/ZkPoxModule.kt` | Copy |
| `android/NodeService.patch` | Apply to `mobile/.../NodeService.kt` | Manual edit |
| `react-native/Credentials.tsx` | `mobile/src/screens/Credentials.tsx` | Copy |
| `react-native/useZkPox.ts` | `mobile/src/hooks/useZkPox.ts` | Copy |
| `react-native/ZkPoxModule.ts` | `mobile/src/native/ZkPoxModule.ts` | Copy |
| `solana/Cargo.toml` | `node/programs/workspace/programs/zk-pox/Cargo.toml` | Copy |
| `solana/src/lib.rs` | `node/programs/workspace/programs/zk-pox/src/lib.rs` | Copy |
| `node-integration/zkpox.rs` | `node/crates/zerox1-node/src/zkpox.rs` | Copy |
| `node-integration/constants-patch.md` | Apply to `node/.../constants.rs` | Manual edit |
| `node-integration/node-patch.md` | Apply to `node/.../node.rs` | Manual edit |
| `rust/` (entire workspace) | Standalone or vendored | Build dependency |
