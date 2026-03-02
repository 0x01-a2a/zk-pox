# ZK-PoX — Zero-Knowledge Proof-of-Experience

A privacy-preserving verifiable life credential protocol built on the 0x01 agent mesh.

## What It Does

Your phone passively records GPS coordinates signed with your SATI cryptographic identity.
You can then generate **zero-knowledge proofs** about WHERE you were and WHEN — without
revealing exact coordinates, exact times, or your identity.

No cameras. No microphones. No human action. Just GPS + time + cryptography.

## Repository Layout

```
rust/                  Rust crates (core library + mobile JNI bridge)
android/               Kotlin files for the mobile app (GPS logger, database, RN module)
react-native/          React Native screens and hooks
solana/                Anchor program for on-chain credentials
node-integration/      Rust module + patch guides for zerox1-node mesh integration
INTEGRATION.md         Step-by-step guide: where each file goes in the 0x01 repos
```

## Proof Types

| Type | Proves | Use Case |
|------|--------|----------|
| RESIDENCY | "Near location H for N+ nights in period P" | Visa, rental, proof of address |
| COMMUTE | "Traveled A→B, D days/week, for W weeks" | Employment verification, tax |
| ATTENDANCE | "Within R meters of E for T+ hours on date D" | Conference POAPs, check-ins |
| ABSENCE | "NOT within R meters of X during period P" | Legal alibi, geo-exclusion |
| STABILITY | "Location variance below threshold over period P" | Insurance risk scoring |
| TRAVEL | "In N distinct regions during period P" | Travel credentials, nomad proof |

## How Users Interact

Users talk to their ZeroClaw agent in natural language:

```
User:  "Prove to my landlord I've lived here for 6 months"
Agent: "Generating RESIDENCY_PROOF... 175/180 nights confirmed.
        Proof size: 2.1 KB. Submit to mesh?"
```

## Building

```bash
cd rust && cargo build
cd rust && cargo test
```

## Integration

See [INTEGRATION.md](INTEGRATION.md) for instructions on integrating these files
into the `mobile` and `node` repositories.

## References

- "Private Proofs of When and Where" — Columbia/MIT, ePrint 2026/136
- "Zero-Knowledge Location Privacy via Floating-Point SNARKs" — TU Munich
- "zkLocus: Authenticated Private Geolocation" — Recursive zkSNARKs
- "OLP Protocol" — Bulletproofs + BLS location verification
