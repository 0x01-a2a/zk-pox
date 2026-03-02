# ZK-PoX Integration Guide

Step-by-step instructions for integrating ZK-PoX into the `mobile` and `node` repositories.

**Nothing in this folder modifies existing repos directly.**
All files are self-contained. Copy them to the locations specified below.

---

## Prerequisites

- Rust toolchain (1.75+) with `cargo-ndk` for Android cross-compilation
- Android NDK (for building the `zkpox-mobile` shared library)
- Solana CLI + Anchor CLI (for deploying the on-chain program)

---

## 1. Mobile App — Android (Kotlin)

### 1.1 Copy Kotlin files

```
zk-pox/android/GpsLogger.kt   → mobile/android/app/src/main/java/world/zerox1/node/GpsLogger.kt
zk-pox/android/GpsDatabase.kt → mobile/android/app/src/main/java/world/zerox1/node/GpsDatabase.kt
zk-pox/android/ZkPoxModule.kt → mobile/android/app/src/main/java/world/zerox1/node/ZkPoxModule.kt
```

### 1.2 Add Android dependencies

In `mobile/android/app/build.gradle`, add inside `dependencies { }`:

```gradle
implementation("com.google.android.gms:play-services-location:21.3.0")
```

### 1.3 Wire GPS Logger into NodeService

Follow the instructions in `zk-pox/android/NodeService.patch`:

1. Add `gpsLogger` and `gpsDatabase` fields to `NodeService`
2. Start the logger in `onCreate()` after wakeLock acquisition
3. Stop the logger in `onDestroy()`

### 1.4 Register ZkPoxModule in React Native

In `MainApplication.kt` (or wherever `ReactPackage` modules are registered), add `ZkPoxModule` to the list of native modules. The exact registration depends on the existing pattern — look for where `NodeModule` is registered and follow the same pattern.

### 1.5 Build the native library

Cross-compile `zkpox-mobile` for Android:

```bash
cd zk-pox/rust
cargo ndk -t arm64-v8a -t armeabi-v7a -o ../mobile/android/app/src/main/jniLibs build --release -p zkpox-mobile
```

This produces:
- `jniLibs/arm64-v8a/libzkpox_mobile.so`
- `jniLibs/armeabi-v7a/libzkpox_mobile.so`

### 1.6 Add location permission (if not already present)

In `mobile/android/app/src/main/AndroidManifest.xml`, ensure these permissions exist:

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

### 2.2 Add navigation

In the app's navigation setup (wherever screens are registered), add:

```tsx
<Stack.Screen name="Credentials" component={CredentialsScreen} />
```

Import:
```tsx
import CredentialsScreen from '../screens/Credentials';
```

### 2.3 Fix import paths

The import in `Credentials.tsx` references `./useZkPox` — adjust the relative path based on the final directory structure. If hooks and screens are in different directories:

```tsx
import { useZkPox } from '../hooks/useZkPox';
```

Similarly, `useZkPox.ts` imports from `./ZkPoxModule` — adjust to:
```tsx
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

### 3.3 Add to workspace

In `node/programs/workspace/Cargo.toml`, add `"programs/zk-pox"` to the `members` array.

### 3.4 Update Anchor.toml

Add under `[programs.localnet]` and `[programs.devnet]`:

```toml
zk_pox = "ZKPoX1111111111111111111111111111111111111"
```

### 3.5 Build and deploy

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

Or vendor the crate into the node workspace.

---

## 5. Core Rust Library

The `zk-pox/rust/` directory is a standalone Rust workspace containing:

| Crate | Purpose |
|---|---|
| `zkpox-core` | Types, commitments, ZK circuit, prover, verifier |
| `zkpox-mobile` | JNI bridge for Android (compiles to `.so`) |

### Build and test

```bash
cd zk-pox/rust
cargo build
cargo test
```

### Key dependencies

| Crate | Version | Purpose |
|---|---|---|
| `bulletproofs` | 4 | ZK range proofs (no trusted setup) |
| `curve25519-dalek` | 4 | Elliptic curve for Bulletproofs |
| `merlin` | 3 | Fiat-Shamir transcript |
| `sha2` | 0.10 | Position/time commitments |
| `ed25519-dalek` | 2 | GPS point signing |

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
