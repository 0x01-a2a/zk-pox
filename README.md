# ZK-PoX — Zero-Knowledge Proof-of-Experience

A privacy-preserving verifiable life credential protocol built on the 0x01 agent mesh.

## What It Does

Your phone passively records GPS coordinates signed with your SATI cryptographic identity (Ed25519).
You can then generate **zero-knowledge proofs** about WHERE you were and WHEN — without
revealing exact coordinates, exact times, or your identity.

No cameras. No microphones. No human action. Just GPS + time + cryptography.

## What Makes It Unique

| Feature | ZK-PoX | zkLocus | OLP Protocol |
|---------|--------|---------|--------------|
| Aggregate range proofs on committed GPS | Yes | No (single-point) | No |
| Anti-spoofing (velocity, teleport, noise) | Yes | No | No |
| Ed25519 signed GPS points | Yes | No | BLS only |
| On-chain soulbound credentials | Yes (Solana) | No | No |
| Mesh peer attestation (witnesses) | Yes | No | Partial |
| Mobile-native (Android JNI) | Yes | No | No |
| Decentralized agent mesh integration | Yes | No | No |

## Project Status

### Done

- [x] **Rust core library** (`zkpox-core`) — types, commitments, circuit, prover, verifier
- [x] **Real Bulletproofs** — aggregate range proofs on Pedersen-committed GPS coordinate offsets (not toy "count >= N")
- [x] **Cryptographic verification** — full prove/verify round-trip with tamper detection
- [x] **Anti-spoofing module** — teleportation detection, impossible velocity analysis, zero-noise mock GPS detection
- [x] **JNI bridge** (`zkpox-mobile`) — proper `jni` crate with `Java_world_zerox1_node_ZkPoxModule_*` signatures, anti-spoof gate before proof generation
- [x] **Ed25519 GPS signing** — PKCS#8 key wrapping for Android API 33+, HMAC-SHA256 fallback for older devices
- [x] **GPS Logger** (Kotlin) — passive background collection with `FusedLocationProviderClient`
- [x] **Encrypted GPS Database** (Kotlin) — SQLite with time-range queries and night counting
- [x] **React Native bridge** (Kotlin) — `ZkPoxModule` exposing stats, proof gen, spoof analysis to JS
- [x] **React Native UI** — `Credentials.tsx` screen with GPS stats, claim type selection, proof generation, spoof risk display
- [x] **Solana Anchor program** — `submit_credential`, `add_witness`, `revoke_credential` with PDA-based soulbound tokens
- [x] **Mesh integration module** — `CORROBORATE_REQUEST/RESPONSE` for peer attestation
- [x] **28/28 Rust tests passing** — circuit, commitment, prover, verifier, antispoof

### TODO

- [ ] Ed25519 signature verification in prover (currently skipped for prototype)
- [ ] Solana program: store `commitments_hash` on-chain for verifier binding
- [ ] Temporal range proofs (prove timestamp is within window without revealing it)
- [ ] Multi-region travel proofs (prove N distinct geofence visits)
- [ ] CI/CD pipeline for Android cross-compilation (`cargo ndk`)
- [ ] Benchmarks: proof generation time on actual Android devices
- [ ] Anchor tests (TypeScript)

## Repository Layout

```
rust/                     Rust workspace
  crates/zkpox-core/        Core: types, commitments, circuit, prover, verifier, antispoof
  crates/zkpox-mobile/      JNI bridge (compiles to libzkpox_mobile.so)
android/                  Kotlin files for mobile app
  GpsLogger.kt               Passive GPS collection with Ed25519 signing
  GpsDatabase.kt             Encrypted SQLite storage
  ZkPoxModule.kt             React Native native module
  NodeService.patch          Integration guide for NodeService
react-native/             React Native layer
  Credentials.tsx            Credentials screen (GPS stats, proof gen, spoof risk)
  useZkPox.ts                React hook for ZK-PoX operations
  ZkPoxModule.ts             TypeScript type definitions
solana/                   Anchor program
  src/lib.rs                 submit_credential, add_witness, revoke_credential
node-integration/         Mesh integration
  zkpox.rs                   CORROBORATE protocol messages
  constants-patch.md         ZKPOX_PROGRAM_ID
  node-patch.md              Message handling in node.rs
INTEGRATION.md            Step-by-step guide: where each file goes
```

## Proof Types

| Type | Proves | Use Case |
|------|--------|----------|
| RESIDENCY | "Near location H for N+ nights in period P" | Visa, rental, proof of address |
| COMMUTE | "Traveled A to B, D days/week, for W weeks" | Employment verification, tax |
| ATTENDANCE | "Within R meters of E for T+ hours on date D" | Conference POAPs, check-ins |
| ABSENCE | "NOT within R meters of X during period P" | Legal alibi, geo-exclusion |
| STABILITY | "Location variance below threshold over period P" | Insurance risk scoring |
| TRAVEL | "In N distinct regions during period P" | Travel credentials, nomad proof |

## How It Works

```
Phone (passive)           Rust Core                  Solana
     |                        |                         |
  GPS fix ──Ed25519 sign──> SignedGPSPoint              |
     |                        |                         |
  [encrypted SQLite]          |                         |
     |                        |                         |
  "Prove I lived here" ──> Anti-spoof check             |
     |                     Bulletproofs range proof      |
     |                     Pedersen commitments          |
     |                        |                         |
     |                    ProofResult ──────────> submit_credential
     |                        |                   (proof_hash, PDA)
     |                        |                         |
     |                  Mesh CORROBORATE ──────> add_witness
     |                  (peer verification)        (attestation)
```

## Building

```bash
cd rust && cargo build
cd rust && cargo test   # 28 tests
```

## Integration

See [INTEGRATION.md](INTEGRATION.md) for instructions on integrating these files
into the `mobile` and `node` repositories.

## Cryptographic Primitives

| Primitive | Crate | Purpose |
|-----------|-------|---------|
| Bulletproofs | `bulletproofs 4` | Aggregate range proofs (no trusted setup) |
| Pedersen Commitments | `curve25519-dalek-ng 4` | Hiding+binding commitments on GPS offsets |
| Merlin Transcripts | `merlin 3` | Fiat-Shamir heuristic for non-interactive proofs |
| SHA-256 | `sha2 0.10` | Position/time commitments, proof hashing |
| Ed25519 | `ed25519-dalek 2` / Android KeyFactory | GPS point signing |

## References

- "Private Proofs of When and Where" — Columbia/MIT, ePrint 2026/136
- "Zero-Knowledge Location Privacy via Floating-Point SNARKs" — TU Munich
- "zkLocus: Authenticated Private Geolocation" — Recursive zkSNARKs
- "OLP Protocol" — Bulletproofs + BLS location verification
