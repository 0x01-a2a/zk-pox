# ZK-PoX — Zero-Knowledge Proof-of-Experience

**A Privacy-Preserving Verifiable Life Credential Protocol Built on 0x01**

Version 0.1 · March 2026 · 0x01 Protocol Team

---

## 1. Executive Summary

ZK-PoX turns every phone running 0x01 into a **passive, privacy-preserving life credential machine**. Your phone silently records GPS coordinates signed with your SATI cryptographic identity. You can then generate zero-knowledge proofs about WHERE you were and WHEN — without ever revealing exact coordinates, exact times, or your identity.

No cameras. No microphones. No human action. Just GPS + time + cryptography.

The result: a **verifiable, private, self-sovereign life resume** that accumulates automatically. Users interact with their ZeroClaw agent in natural language: *"Prove to my landlord I've lived here for 6 months"* — and the agent handles everything.

**Market**: Decentralized identity is projected at $7.27B in 2026, growing to $35.37B by 2032. Over 60% of enterprises globally are expected to adopt verifiable credentials by end of 2026.

**Competitive position**: Nobody combines ZK location proofs + decentralized agent mesh + on-chain economic stakes + challenge/slashing. 0x01 is uniquely positioned to build this.

---

## 2. The Problem

### 2.1 You Don't Own Your Location History

Google tracks 2+ billion phones. They know where you live, work, eat, sleep, travel. You get nothing for this data. They sell it to advertisers for $200B+/year in revenue.

### 2.2 Proving Real-World Experience is Broken

| Scenario | Current Solution | Problems |
|---|---|---|
| Prove you live somewhere | Utility bill, bank statement | Easy to forge, requires centralized issuer |
| Prove you attended an event | POAP (scan QR code) | Requires manual check-in, can be shared |
| Prove employment history | Reference letter from employer | Employer may refuse, can be fabricated |
| Prove you were NOT somewhere | Nothing | No credible alibi mechanism exists |
| Prove stable lifestyle for insurance | Give full location history to insurer | Total privacy violation |
| Prove identity (anti-Sybil) | Worldcoin iris scan | Invasive biometrics, centralized database |

### 2.3 The Gap

**Existing ZK location projects** (zkLocus, OLP Protocol, ZKLP) solve the cryptography but have no:
- Economic incentive layer (why would anyone generate proofs?)
- Agent infrastructure (who requests/delivers proofs?)
- Reputation system (how do you trust the prover?)
- Anti-spoofing enforcement (what happens if someone fakes GPS?)

**0x01 has all four.** It just doesn't have the ZK circuits yet.

---

## 3. How Users Interact

Users talk to their ZeroClaw agent in natural language. The agent handles proof generation, delivery, and payment autonomously.

### 3.1 User Stories

**Maria (tenant)** — needs to prove residency for visa renewal:
```
Maria:  "Prove to immigration that I've lived at my address for the past 6 months"
Agent:  "I have 4,320 signed GPS points showing you within 200m of [home] 
         between 10pm-7am on 175 out of 180 nights. Generating ZK proof..."
Agent:  "Proof generated (2.1 KB). It proves: 'Device was within 200m of 
         [redacted coordinates] for 175+ nights over 180-day period ending 
         March 2026.' No exact address, times, or identity revealed.
         Shall I submit to the mesh for the immigration verifier agent?"
Maria:  "Yes"
Agent:  "Proof submitted. Verification fee: 0.50 USDC via escrow. 
         Credential attached to your SATI identity as soulbound token."
```

**Jakub (freelancer)** — proving work history without employer reference:
```
Jakub:  "Generate a proof that I commuted to a workplace 5 days/week for 2 years"
Agent:  "Analyzing GPS trajectory history... Found consistent A→B pattern,
         Monday-Friday, 8am-6pm, for 480+ days across 24 months.
         Generating ZK proof..."
Agent:  "Proof ready. It proves: 'Regular 5-day/week commute between two 
         consistent locations for 24 months.' No addresses, routes, or 
         schedule details revealed. Share with potential employer?"
```

**Anna (driver)** — lower insurance premiums:
```
Anna:   "Show my insurer I have low-risk driving patterns"
Agent:  "Analyzing 12-month GPS data... Pattern: home by 10pm 90% of nights,
         no high-risk zones visited, average 12km daily range.
         Generating ZK proof of stable-lifestyle pattern..."
Agent:  "Proof proves: 'Low mobility variance, no high-risk zone visits, 
         consistent nighttime home presence.' 
         Your insurer's agent is offering 15% premium reduction for 
         verified low-risk credential. Accept?"
```

### 3.2 Agent-to-Agent Flow

When a business or institution needs verification, their agent queries the mesh:

```
Insurer Agent → 0x01 Mesh:  "ADVERTISE: Need proof-of-stable-lifestyle,
                              budget 2 USDC, reputation > 80 required"
                              
Anna's Agent ← 0x01 Mesh:   "Task matches your capability. Auto-accept?"
Anna's Agent → 0x01 Mesh:   "PROPOSE: I can provide this proof, 1.50 USDC"
Insurer Agent → Anna:        "ACCEPT" → Escrow locks 1.50 USDC
Anna's Agent:                 Generates ZK proof from local GPS data
Anna's Agent → Insurer:      "DELIVER: [2.1KB ZK proof]"
Insurer Agent:                Verifies proof on-chain → "APPROVE"
Escrow:                       Releases 1.50 USDC to Anna
Behavior-log:                 Records interaction, updates reputation
```

---

## 4. Architecture

### 4.1 System Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     USER'S PHONE                            │
│                                                             │
│  ┌──────────┐    ┌──────────────┐    ┌──────────────────┐   │
│  │   GPS    │───▶│  Signed GPS  │───▶│   ZK Proof       │   │
│  │ (passive)│    │  Local Store │    │   Generator      │   │
│  └──────────┘    │  (encrypted) │    │   (Bulletproofs)  │   │
│                  └──────────────┘    └────────┬─────────┘   │
│                                              │              │
│  ┌──────────────────────────────────┐        │              │
│  │         ZeroClaw Brain           │◀───────┘              │
│  │  (natural language interface)    │                        │
│  │  "Prove I lived here 6 months"  │                        │
│  └──────────┬───────────────────────┘                        │
│             │                                                │
│  ┌──────────▼───────────────────────┐                        │
│  │       0x01 Node (zerox1-node)    │                        │
│  │  • Mesh communication            │                        │
│  │  • Escrow management             │                        │
│  │  • Reputation tracking           │                        │
│  └──────────┬───────────────────────┘                        │
└─────────────┼───────────────────────────────────────────────┘
              │
              │  P2P Mesh (libp2p gossipsub + Kademlia DHT)
              │
┌─────────────▼───────────────────────────────────────────────┐
│                    0x01 MESH NETWORK                         │
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────────┐   │
│  │ Verifier     │  │ Corroborating│  │ Requester        │   │
│  │ Agents       │  │ Witnesses    │  │ Agents           │   │
│  │ (validate    │  │ (anti-spoof  │  │ (insurance,      │   │
│  │  ZK proofs)  │  │  nearby GPS) │  │  employers, etc) │   │
│  └──────────────┘  └──────────────┘  └──────────────────┘   │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           │  On-chain settlement
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                      SOLANA                                  │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐  ┌────────────────────┐   │
│  │ Behavior    │  │   Escrow    │  │  SATI Identity     │   │
│  │ Log         │  │   (USDC)   │  │  (Token-2022 NFT)  │   │
│  │ + ZK proof  │  │             │  │  + Soulbound       │   │
│  │   entries   │  │             │  │    Credentials     │   │
│  └─────────────┘  └─────────────┘  └────────────────────┘   │
│                                                              │
│  ┌─────────────┐  ┌─────────────┐                           │
│  │ Stake-Lock  │  │  Challenge  │                           │
│  │ (10 USDC    │  │  (dispute   │                           │
│  │  slashable) │  │   → slash)  │                           │
│  └─────────────┘  └─────────────┘                           │
└──────────────────────────────────────────────────────────────┘
```

### 4.2 Data Flow

```
Phase 1: PASSIVE COLLECTION (continuous, no user action)
──────────────────────────────────────────────────────────

  GPS Sensor ──(every 5 min)──▶ SignedGPSPoint {
                                   lat: f64,           // exact (NEVER leaves device)
                                   lng: f64,           // exact (NEVER leaves device)
                                   timestamp: i64,     // unix seconds
                                   accuracy_m: f32,    // GPS accuracy in meters
                                   signature: [u8;64], // Ed25519 sign(lat|lng|ts, SATI_key)
                                }
                                   │
                                   ▼
                                Local encrypted SQLite DB
                                (AES-256, key in Android Keystore)


Phase 2: PROOF GENERATION (on-demand, triggered by user or agent)
─────────────────────────────────────────────────────────────────

  User: "Prove I lived here 6 months"
           │
           ▼
  ZeroClaw Brain parses intent:
    • claim_type: RESIDENCY
    • center: [user's home GPS]
    • radius: 200m
    • time_window: 180 days
    • min_nights: 150
           │
           ▼
  ZK Circuit (Bulletproofs / SP1):
    Private inputs:  4,320 SignedGPSPoints (NEVER revealed)
    Public inputs:   center_hash, radius, time_window, min_count
    Statement:       "At least 150 of 4,320 points fall within 200m 
                      of committed center, between 10pm-7am, across 
                      180 consecutive days"
    Output:          ZK Proof (2-3 KB) + public inputs
           │
           ▼
  Proof is valid WITHOUT knowing:
    ✗ Exact home address
    ✗ Exact GPS coordinates
    ✗ Exact times of arrival/departure
    ✗ Movement patterns during the day
    ✗ Identity of the prover (until voluntarily linked to SATI)


Phase 3: VERIFICATION & SETTLEMENT (on-chain)
──────────────────────────────────────────────

  Proof ──▶ Verifier Agent (checks ZK math) ──▶ behavior-log on-chain
                                                      │
                                                      ▼
                                               Soulbound Credential
                                               attached to SATI NFT:
                                               {
                                                 type: "RESIDENCY",
                                                 verified: true,
                                                 period: "180 days",
                                                 confidence: "150/180 nights",
                                                 issued: 1740873600,
                                                 proof_hash: "0x..."
                                               }
```

---

## 5. ZK Proof Design

### 5.1 What Is Proven vs. What Is Hidden

| Proven (public) | Hidden (private) |
|---|---|
| "Within X meters of a committed location" | The exact committed location |
| "For N out of M days/nights" | Which specific days/nights |
| "During time window [month range]" | Exact timestamps of each visit |
| "Signed by a valid SATI identity" | Which SATI identity (until linked) |
| "GPS points are internally consistent" | The actual GPS trajectory |

### 5.2 Proof Types

```
RESIDENCY_PROOF
  Proves:    "I was near location H for N+ nights in period P"
  Inputs:    GPS points filtered to 10pm-7am, radius R around H
  Use:       Visa, rental applications, proof of address

COMMUTE_PROOF  
  Proves:    "I traveled between location A and B, D days/week, for W weeks"
  Inputs:    GPS trajectory clustering, weekday filtering
  Use:       Employment verification, tax residency, transport subsidies

ATTENDANCE_PROOF
  Proves:    "I was within R meters of location E for T+ hours on date D"
  Inputs:    GPS points during event window, venue geofence
  Use:       Conference badges, concert POAPs, court-ordered check-ins

ABSENCE_PROOF
  Proves:    "I was NOT within R meters of location X during period P"
  Inputs:    All GPS points in period P, none fall within geofence
  Use:       Legal alibi, restraining order compliance, geo-exclusion

STABILITY_PROOF
  Proves:    "My location variance is below threshold T over period P"
  Inputs:    Statistical analysis of GPS point distribution
  Use:       Insurance risk scoring, creditworthiness for unbanked

TRAVEL_PROOF
  Proves:    "I was in N distinct geographic regions during period P"
  Inputs:    GPS points clustered by country/region boundaries
  Use:       Loyalty programs, travel credentials, nomad verification
```

### 5.3 Cryptographic Foundation

```
Based on published research (February 2026):

1. "Private Proofs of When and Where" (Columbia/MIT, ePrint 2026/136)
   → Position commitments with post-quantum security
   → Complex spatio-temporal assertions without location disclosure

2. "Zero-Knowledge Location Privacy via Floating-Point SNARKs" (TU Munich)
   → IEEE 754 compliant ZK circuits for GPS coordinates
   → 15.9× constraint reduction, 0.26s proof generation
   → Peer proximity verification: 470 verifications/second

3. "zkLocus: Authenticated Private Geolocation" (Recursive zkSNARKs)
   → Constant-size proofs regardless of assertion count
   → Organic rollup: multiple proofs compressed into one
   → Zero-trust model: data never leaves device

4. "Composable Anonymous Proof-of-Location" (IEEE Access)
   → Unforgeable, non-transferable location credentials
   → 94× communication overhead reduction vs. prior work
   → Mobile-compatible proof generation

Implementation approach:
  → Bulletproofs (dalek-cryptography, pure Rust) for range proofs
  → SP1 zkVM (Succinct) for complex assertions if needed
  → Proof size: 2-3 KB per assertion
  → Generation time target: < 1 second on modern smartphone
```

---

## 6. Anti-Spoofing: The Economic Security Layer

GPS can be faked with mock location apps. ZK-PoX addresses this through three layers:

### 6.1 Peer Corroboration (Mesh Witnesses)

```
   Prover's Phone              Nearby 0x01 Phones (witnesses)
   ┌──────────┐               ┌──────────┐  ┌──────────┐
   │ GPS: X,Y │               │ GPS: X',Y'│  │ GPS: X'',Y''│
   │ Time: T  │               │ Time: T   │  │ Time: T      │
   └──────┬───┘               └──────┬────┘  └──────┬───────┘
          │                          │               │
          └──────────┬───────────────┘               │
                     │ Mesh discovery                 │
                     ▼                                │
              "Were any agents near                   │
               location X,Y at time T?"               │
                     │                                │
                     ▼                                ▼
              Witness responses:
              "Yes, I was within 500m"  "Yes, I was within 300m"
              (signed with their SATI keys)
              
              Corroboration score: 2/2 witnesses confirm proximity
              → HIGH confidence (spoofing 3+ devices simultaneously 
                is exponentially harder)
```

Research confirms: crowdsourced GPS corroboration achieves **98.72% spoofing detection rate** with 62ms latency (SSRN 4713184, 2024).

### 6.2 Economic Deterrence (Stake Slashing)

```
  Agent stakes 10 USDC to join the mesh (already implemented in stake-lock)
  
  If GPS spoofing is detected:
    → Any agent can submit a CHALLENGE (already implemented)
    → Challenge resolution checks corroboration evidence
    → Guilty → 10 USDC stake SLASHED (already implemented)
    → Credential REVOKED from SATI NFT
    
  Cost of spoofing: 10 USDC per attempt
  Reward for honest proving: 0.50-2.00 USDC per proof
  
  → Rational agents never spoof (expected loss >> expected gain)
```

### 6.3 Temporal Consistency Analysis

```
  A real phone produces GPS data with natural characteristics:
    • Gradual movement (no teleportation)
    • Signal noise patterns consistent with hardware
    • Realistic velocity between consecutive points
    • Day/night location clustering (home/work pattern)
    
  A spoofed phone has detectable anomalies:
    • Sudden location jumps
    • Unnaturally precise coordinates (no GPS noise)
    • No velocity consistency
    • Statistically impossible patterns
    
  ZeroClaw analyzes GPS history for anomaly detection before 
  generating proofs. Anomalous data → proof generation refused.
```

---

## 7. Real-World Use Cases & Market Sizing

### 7.1 Proof-of-Residency (Immigration & Housing)

**Problem**: 281 million international migrants worldwide need to prove residency. Current methods (utility bills, bank statements) are forgeable, require centralized issuers, and take weeks.

**ZK-PoX solution**: Automatic, unforgeable, private residency proof generated from passive GPS data. No documents, no issuing authority, instant.

**Market**: Immigration services market is $32B globally. Even 1% adoption = $320M.

**User interaction**:
```
User:  "I need proof of residency for my visa application"
Agent: "Generating RESIDENCY_PROOF from your last 6 months of GPS data...
        Proof confirms 175/180 nights at home location. 
        Submitting to immigration verifier agent on mesh.
        Credential issued to your SATI identity. Fee: 0.50 USDC."
```

### 7.2 Employment Verification (Freelancers & Gig Workers)

**Problem**: 1.57 billion informal workers globally cannot prove employment history. No HR department, no pay stubs, no reference letters. Banks won't lend to them. Landlords won't rent to them.

**ZK-PoX solution**: Prove consistent commute patterns without revealing employer address or schedule. "I went to the same workplace 5 days/week for 2 years" is provable from GPS alone.

**Market**: Global gig economy $556B by 2027. Credit scoring for unbanked: 2 billion people.

**User interaction**:
```
User:  "Generate work history proof for bank loan application"
Agent: "Found 24-month consistent weekday commute pattern.
        COMMUTE_PROOF generated: regular 5-day/week presence at 
        a consistent work location for 24 months.
        Share with bank's verification agent? Fee: 1.00 USDC."
```

### 7.3 Insurance Risk Scoring (Auto & Home)

**Problem**: Insurers want behavioral data to price risk. Users don't want to share their full location history with a corporation. Current telematics (Progressive Snapshot, etc.) require invasive tracking devices.

**ZK-PoX solution**: Prove low-risk behavior patterns without revealing any specific locations. "I'm home by 10pm most nights, low daily mileage, no high-risk areas" — provable without showing WHERE home is.

**Market**: Global insurance $7.1T. Telematics-based insurance $126B by 2030.

**User interaction**:
```
User:  "Get me a better insurance rate"
Agent: "Analyzing 12-month GPS patterns... Your risk profile:
        • Home by 10pm: 92% of nights
        • Daily range: avg 11km (low)
        • High-risk zone visits: 0
        Generating STABILITY_PROOF...
        Insurer's agent offers 18% premium reduction for 
        verified low-risk credential. Accept?"
```

### 7.4 Event Attendance (Conferences, Concerts, Compliance)

**Problem**: POAPs require QR code scanning (manual, easy to share). Court-ordered check-ins require ankle monitors (invasive, expensive). Conference attendance verification requires sign-in sheets (forgeable).

**ZK-PoX solution**: Automatic proof of presence at a specific location during a specific time window. No check-in, no wristband, no QR code. Phone in your pocket does it.

**Market**: Global events industry $1.5T. Electronic monitoring $4.8B.

**User interaction**:
```
User:  "Get me the attendance NFT for today's ETH conference"
Agent: "You've been within 100m of the venue for 7 hours today.
        Generating ATTENDANCE_PROOF...
        Credential minted as soulbound token on your SATI NFT.
        No fee — event organizer covers verification cost."
```

### 7.5 Anti-Sybil Identity (The Worldcoin Alternative)

**Problem**: Worldcoin scans your iris to prove you're human. This is invasive, requires specialized hardware (Orbs), centralizes biometric data, and has been banned in multiple countries.

**ZK-PoX solution**: Prove you're a real person with a real phone in a real location by demonstrating consistent, long-term, physically plausible GPS patterns. No biometrics. No special hardware. Just time + location + cryptography.

**Proof**: "This SATI identity has produced 12 months of physically consistent GPS data with natural noise patterns, realistic velocity, day/night cycles, and peer corroboration from 50+ unique witnesses."

A bot farm cannot produce this. GPS spoofing for 12 months continuously, with corroborating witnesses, while maintaining natural patterns, is economically and technically infeasible.

**Market**: Digital identity verification $18.6B by 2027.

---

## 8. Competitive Landscape

```
┌────────────────┬───────────┬──────────┬──────────┬──────────┬──────────┐
│                │ ZK-PoX    │ Worldcoin│ POAP     │ zkLocus  │ Helium/  │
│                │ (0x01)    │          │          │          │ GEODNET  │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Passive        │ ✅ GPS    │ ✗ Iris   │ ✗ QR     │ ✗ Manual │ ✅ Radio │
│ (no action)    │ auto      │ scan     │ scan     │ submit   │ auto     │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Privacy        │ ✅ ZK     │ ✗ Bio-   │ ✗ Public │ ✅ ZK    │ ✗ Raw    │
│ preserving     │ proofs    │ metrics  │ on-chain │ proofs   │ GPS data │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Economic       │ ✅ USDC   │ ✗ WLD    │ ✗ None   │ ✗ None   │ ✅ Token │
│ incentives     │ escrow    │ token    │          │          │ rewards  │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Anti-spoofing  │ ✅ Peer   │ ✅ Iris  │ ✗ Share- │ ✗ None   │ ✅ Infra │
│ enforcement    │ + slash   │ unique   │ able     │          │ based    │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Agent-to-agent │ ✅ Mesh   │ ✗ No     │ ✗ No     │ ✗ No     │ ✗ No    │
│ marketplace    │ + escrow  │          │          │          │          │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Reputation     │ ✅ On-    │ ✗ None   │ ✅ Count │ ✗ None   │ ✗ None  │
│ system         │ chain     │          │ badges   │          │          │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ Challenge /    │ ✅ Stake  │ ✗ None   │ ✗ None   │ ✗ None   │ ✗ None  │
│ dispute        │ slashing  │          │          │          │          │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ On-chain       │ ✅ Soul-  │ ✅ World │ ✅ NFT   │ ✅ Mina  │ ✗ None  │
│ credentials    │ bound     │ ID       │          │          │          │
├────────────────┼───────────┼──────────┼──────────┼──────────┼──────────┤
│ No special     │ ✅ Any    │ ✗ Orb    │ ✅ Any   │ ✅ Any   │ ✗ Hot-  │
│ hardware       │ phone     │ needed   │ phone    │ phone    │ spot    │
└────────────────┴───────────┴──────────┴──────────┴──────────┴──────────┘
```

**ZK-PoX is the only system that combines ALL of**: passive collection, ZK privacy, economic incentives, anti-spoofing with slashing, agent-to-agent marketplace, on-chain reputation, and soulbound credentials — using nothing but a standard smartphone.

---

## 9. What Already Exists in 0x01 vs. What Needs to Be Built

### 9.1 Already Built (v0.2.3)

| Component | Status | Location |
|---|---|---|
| P2P mesh (libp2p) | ✅ Production | `zerox1-node/src/node.rs` |
| SATI identity (Token-2022 NFT) | ✅ Production | `agent-ownership/` |
| USDC escrow (lock → approve → settle) | ✅ Production | `escrow/src/lib.rs` |
| Auto-stake on startup | ✅ Production | `node.rs::ensure_stake_and_lease()` |
| Auto-lease on startup | ✅ Production | `node.rs::ensure_stake_and_lease()` |
| Auto-escrow lock on ACCEPT | ✅ Production | `node.rs::handle_outbound()` |
| Notary assignment | ✅ Production | `node.rs::try_assign_notary()` |
| Behavior log (on-chain) | ✅ Production | `behavior-log/src/lib.rs` |
| Challenge + slash mechanism | ✅ Production | `challenge/src/lib.rs` |
| Reputation (aggregator) | ✅ Production | `zerox1-aggregator/` |
| Mobile app (React Native) | ✅ Production | `mobile/` |
| Foreground service (24/7) | ✅ Production | `NodeService.kt` |
| PhoneBridge (GPS endpoint) | ✅ Production | `PhoneBridgeServer.kt` |
| ZeroClaw agent brain | ✅ Production | `zeroclaw` binary |
| SDK (TypeScript) | ✅ Production | `@zerox1/sdk` |
| Devnet/Mainnet switching | ✅ Production | `constants.rs` + `#[cfg]` |
| EVM gateway (Base → Solana USDC) | ✅ Production | `evm-gateway/` |

### 9.2 Built in ZK-PoX Prototype (this repo)

| Component | Status | Location | Lines |
|---|---|---|---|
| GPS background logger | ✅ Done | `zk-pox/android/GpsLogger.kt` | ~130 |
| Ed25519 GPS point signing | ✅ Done | `GpsLogger.kt` (PKCS#8 wrapping, API 33+ native, HMAC fallback) | incl. |
| Encrypted local GPS database | ✅ Done | `zk-pox/android/GpsDatabase.kt` | ~190 |
| ZK circuit (Bulletproofs range proofs) | ✅ Done | `zk-pox/rust/crates/zkpox-core/src/{circuit,prover,verifier}.rs` | ~550 |
| Aggregate Pedersen commitments | ✅ Done | `prover.rs` — committed GPS coordinate offsets, power-of-two padding | incl. |
| Cryptographic verification | ✅ Done | `verifier.rs` — full Bulletproofs verify_multiple round-trip | incl. |
| Anti-spoofing module | ✅ Done | `zk-pox/rust/crates/zkpox-core/src/antispoof.rs` | ~200 |
| JNI bridge (Android native) | ✅ Done | `zk-pox/rust/crates/zkpox-mobile/src/lib.rs` (jni 0.21 crate) | ~130 |
| On-chain credential (Anchor) | ✅ Done | `zk-pox/solana/src/lib.rs` — v2 with commitments_hash, count_proven | ~230 |
| Corroboration protocol | ✅ Done | `zk-pox/node-integration/zkpox.rs` | ~200 |
| React Native module (Kotlin) | ✅ Done | `zk-pox/android/ZkPoxModule.kt` | ~180 |
| React Native UI | ✅ Done | `zk-pox/react-native/Credentials.tsx` | ~340 |
| React Native hook + types | ✅ Done | `zk-pox/react-native/{useZkPox.ts,ZkPoxModule.ts}` | ~170 |
| Integration guide | ✅ Done | `zk-pox/INTEGRATION.md` | ~290 |
| 28 Rust tests passing | ✅ Done | circuit, commitment, prover, verifier, antispoof | — |

**Total prototype code: ~2,600+ lines across Kotlin, Rust, Anchor, React Native, TypeScript.**

### 9.3 Still Needs to Be Built

| Component | Effort | Description |
|---|---|---|
| Ed25519 signature verification in prover | ~50 lines Rust | Verify GPS point signatures before including them in proofs (currently trusted) |
| Temporal range proofs | ~150 lines Rust | Prove timestamp falls within a time window without revealing exact time |
| Multi-region travel proofs | ~200 lines Rust | Prove visits to N distinct geofence regions |
| Recursive proof compression | ~300 lines Rust | Combine multiple Bulletproofs into a single compact proof |
| ZeroClaw proof intents | ~100 lines config | Natural language → proof type mapping in ZeroClaw TOML capability declarations |
| Anchor tests (TypeScript) | ~200 lines TS | Mocha tests for submit_credential, add_witness, revoke_credential |
| CI/CD pipeline | ~50 lines YAML | GitHub Actions: `cargo ndk` cross-compilation for arm64-v8a + armeabi-v7a |
| Security audit of ZK circuits | External | Professional audit of Bulletproofs usage, commitment scheme, anti-spoofing |
| Partnership integrations | ~500 lines API | Insurance claim verification API, HR/employment verification API |

Everything else — mesh networking, escrow, reputation, challenge, staking, mobile service — is already shipped in 0x01 v0.2.3.

---

## 10. Roadmap

### Phase 1: Foundation — COMPLETE

- [x] GPS background logger with `FusedLocationProviderClient` (`GpsLogger.kt`)
- [x] Ed25519 GPS point signing (PKCS#8 wrapping, API 33+, HMAC fallback)
- [x] Encrypted local GPS database with time-range queries (`GpsDatabase.kt`)
- [x] Bulletproofs ZK range proof circuit — aggregate proofs on committed GPS coordinate offsets
- [x] Pedersen commitments on lat/lng offsets from geofence bounding box
- [x] Full cryptographic verification (prove/verify round-trip, tamper detection)
- [x] SHA-256 position/time commitments, proof hashing, public inputs hashing
- [x] 28 Rust tests passing (circuit, commitment, prover, verifier, antispoof)

### Phase 2: Anti-Spoofing & Mobile Bridge — COMPLETE

- [x] Anti-spoofing module: teleportation detection, impossible velocity analysis, zero-noise mock GPS detection
- [x] Suspicion scoring with configurable thresholds (Clean / Suspicious / LikelySpoofed)
- [x] Anti-spoof gate in JNI bridge — blocks proof generation on spoofed GPS data
- [x] JNI bridge with `jni` crate — proper `Java_world_zerox1_node_ZkPoxModule_*` function signatures
- [x] Three JNI endpoints: `generateProofNative`, `verifyProofNative`, `analyzeSpoofRiskNative`
- [x] React Native Kotlin module (`ZkPoxModule.kt`) with coroutine-based async methods
- [x] React Native TypeScript types (`SpoofAnalysis`, `ProofRequest`, `GpsStats`)
- [x] React Native hook (`useZkPox`) with full state management for stats, proofs, spoof analysis
- [x] Credentials UI screen with GPS stats, integrity analysis, claim selection, proof result display

### Phase 3: On-Chain Integration — COMPLETE (prototype)

- [x] Solana Anchor program: `submit_credential` with `commitments_hash` + `count_proven` (v2 schema)
- [x] `add_witness` instruction for mesh peer attestations (up to 8 witnesses)
- [x] `revoke_credential` instruction (agent-only)
- [x] PDA-based soulbound credential tied to agent identity
- [x] `CORROBORATE_REQUEST/RESPONSE` mesh message type (`node-integration/zkpox.rs`)
- [x] Integration guides: `NodeService.patch`, `constants-patch.md`, `node-patch.md`
- [x] Full integration documentation (`INTEGRATION.md`) with file map

### Phase 4: Hardening & Production — TODO

- [ ] Ed25519 signature verification in prover (currently GPS signatures are trusted)
- [ ] Temporal range proofs (prove timestamp within window without revealing it)
- [ ] Multi-region travel proofs (prove N distinct geofence visits)
- [ ] Recursive proof compression (batch multiple Bulletproofs → single proof)
- [ ] ZeroClaw natural language → proof type mapping (TOML capability declarations)
- [ ] Anchor TypeScript tests for submit_credential, add_witness, revoke_credential
- [ ] CI/CD pipeline: GitHub Actions with `cargo ndk` for arm64-v8a + armeabi-v7a
- [ ] Benchmark proof generation time on actual Android devices (target: < 2s)
- [ ] Challenge extension for GPS spoofing disputes (stake slashing for fake proofs)
- [ ] Agent-to-agent proof marketplace (ADVERTISE → DELIVER flow via mesh)

### Phase 5: Scale & Partnerships — FUTURE

- [ ] Partnership integrations: insurance claim verification API, HR/employment verification API
- [ ] Recursive SNARKs for ultra-compact proofs (Groth16 or Halo2)
- [ ] Cross-chain credential bridging (Solana → EVM via bridge-sdk)
- [ ] Professional security audit of ZK circuits and commitment scheme
- [ ] Mainnet deployment and real-world pilot

---

## 11. Why Only 0x01 Can Build This

The reason nobody has built ZK-PoX isn't that the cryptography doesn't exist — it does (zkLocus, ZKLP, OLP Protocol all published working ZK location proofs).

The reason is that ZK location proofs ALONE are worthless without:

1. **Why would anyone generate proofs?** → 0x01 has USDC escrow payments
2. **Who requests and delivers proofs?** → 0x01 has agent-to-agent mesh
3. **How do you trust the prover?** → 0x01 has on-chain reputation
4. **What if someone fakes GPS?** → 0x01 has challenge + stake slashing
5. **Where does the proof live?** → 0x01 has SATI NFT + behavior-log
6. **What runs 24/7 to collect GPS?** → 0x01 has mobile foreground service
7. **How does the user interact?** → 0x01 has ZeroClaw (natural language AI)

No other project has all seven. That's the moat.

---

## 12. References

1. "Private Proofs of When and Where" — Columbia University & MIT, ePrint 2026/136
2. "Zero-Knowledge Location Privacy via Accurate Floating-Point SNARKs" — TU Munich, ePrint 2024/1842
3. "zkLocus: Authenticated Private Geolocation Off & On-Chain" — Recursive zkSNARKs, zklocus.dev
4. "Composable Anonymous Proof-of-Location" — IEEE Access, 2023
5. "OLP Protocol: Privacy-Preserving Location Verification" — Bulletproofs + BLS, olp-protocol.org
6. "All in One: Improving GPS Accuracy and Security Via Crowdsourcing" — SSRN 4713184, 2024
7. "ERC-7812: ZK Identity Registry" — Ethereum Standards, 2024
8. "ERC-8033: Agent Council Oracles" — Ethereum Standards, 2025
9. "Decentralized Identity Market Size" — 360iResearch, 2026 ($7.27B → $35.37B by 2032)
10. "The Agent Economy: A Blockchain-Based Foundation" — arXiv 2602.14219, February 2026
