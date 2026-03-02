//! ZK-PoX mesh integration — CORROBORATE protocol.
//!
//! Handles peer-to-peer corroboration of ZK proofs over the 0x01 pubsub mesh.
//! When an agent wants to submit a credential on-chain, it first broadcasts
//! a CORROBORATE_REQUEST to nearby peers. Each peer verifies the proof
//! independently and responds with a signed CORROBORATE_RESPONSE.
//!
//! Integration:
//!   Copy to node/crates/zerox1-node/src/zkpox.rs
//!   Add `pub mod zkpox;` to node/crates/zerox1-node/src/lib.rs
//!   See node-patch.md for message handling additions.

use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------
// Message Types
// -------------------------------------------------------------------------

/// Mesh message type identifier for ZK-PoX corroboration.
pub const MSG_TYPE_CORROBORATE_REQUEST: u8 = 0x30;
pub const MSG_TYPE_CORROBORATE_RESPONSE: u8 = 0x31;

/// A request for nearby mesh peers to verify and attest to a ZK proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorroborateRequest {
    /// Unique request identifier.
    pub request_id: [u8; 32],
    /// The requesting agent's public key.
    pub agent_pubkey: [u8; 32],
    /// Serialized ProofResult (JSON bytes).
    pub proof_result_json: Vec<u8>,
    /// The credential_id that will be used in submit_credential.
    pub credential_id: [u8; 32],
    /// Unix timestamp of the request.
    pub timestamp: i64,
    /// Signature of the request by the agent.
    pub signature: [u8; 64],
}

/// A response from a mesh peer attesting to a verified ZK proof.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorroborateResponse {
    /// Must match the request_id from CorroborateRequest.
    pub request_id: [u8; 32],
    /// The witnessing peer's public key.
    pub witness_pubkey: [u8; 32],
    /// Whether the proof verified successfully.
    pub verified: bool,
    /// Unix timestamp of the response.
    pub timestamp: i64,
    /// Ed25519 signature of (request_id || verified || timestamp).
    pub signature: [u8; 64],
}

// -------------------------------------------------------------------------
// Processing
// -------------------------------------------------------------------------

/// Result of processing a corroboration request.
#[derive(Debug)]
pub enum CorroborateAction {
    /// Proof verified — respond with attestation.
    Attest(CorroborateResponse),
    /// Proof invalid — respond with rejection.
    Reject(CorroborateResponse),
    /// Request is malformed or from an untrusted peer — ignore.
    Ignore,
}

/// Process an incoming CORROBORATE_REQUEST.
///
/// 1. Deserialize the ProofResult from the request.
/// 2. Verify the ZK proof using zkpox-core::verifier.
/// 3. If valid, construct a signed CORROBORATE_RESPONSE.
///
/// In production, this would call into the zkpox-core verifier.
/// This stub demonstrates the protocol flow.
pub fn handle_corroborate_request(
    request: &CorroborateRequest,
    our_pubkey: &[u8; 32],
    _our_signing_key: &[u8; 64],
) -> CorroborateAction {
    // Parse the proof result
    let proof_str = match std::str::from_utf8(&request.proof_result_json) {
        Ok(s) => s,
        Err(_) => return CorroborateAction::Ignore,
    };

    // In production: deserialize and verify with zkpox_core::verifier::verify_proof()
    // For now, we check structural validity.
    let _proof_json: serde_json::Value = match serde_json::from_str(proof_str) {
        Ok(v) => v,
        Err(_) => return CorroborateAction::Ignore,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    // Reject requests older than 5 minutes
    if (now - request.timestamp).abs() > 300 {
        return CorroborateAction::Ignore;
    }

    // TODO: Verify the ZK proof using zkpox_core::verifier::verify_proof()
    // For now, assume verification passes if the JSON is well-formed.
    let verified = true;

    // TODO: Sign (request_id || verified || timestamp) with our_signing_key
    let signature = [0u8; 64]; // Placeholder — real implementation uses Ed25519

    let response = CorroborateResponse {
        request_id: request.request_id,
        witness_pubkey: *our_pubkey,
        verified,
        timestamp: now,
        signature,
    };

    if verified {
        CorroborateAction::Attest(response)
    } else {
        CorroborateAction::Reject(response)
    }
}

/// Collect corroboration responses and decide when to submit on-chain.
///
/// Returns true when we have enough attestations to submit the credential.
pub fn has_enough_witnesses(responses: &[CorroborateResponse], min_witnesses: usize) -> bool {
    let valid_count = responses.iter().filter(|r| r.verified).count();
    valid_count >= min_witnesses
}

// -------------------------------------------------------------------------
// Credential Submission Helper
// -------------------------------------------------------------------------

/// Build the instruction data for the on-chain submit_credential call.
///
/// In production, this would construct a Solana transaction using the
/// anchor client library and submit it via RPC.
#[derive(Debug, Clone)]
pub struct CredentialSubmission {
    pub credential_id: [u8; 32],
    pub claim_type: u8,
    pub proof_hash: [u8; 32],
    pub public_inputs_hash: [u8; 32],
    pub witness_pubkeys: Vec<[u8; 32]>,
}

pub fn prepare_submission(
    credential_id: [u8; 32],
    claim_type: u8,
    proof_hash: [u8; 32],
    public_inputs_hash: [u8; 32],
    responses: &[CorroborateResponse],
) -> CredentialSubmission {
    let witness_pubkeys: Vec<[u8; 32]> = responses
        .iter()
        .filter(|r| r.verified)
        .map(|r| r.witness_pubkey)
        .collect();

    CredentialSubmission {
        credential_id,
        claim_type,
        proof_hash,
        public_inputs_hash,
        witness_pubkeys,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_enough_witnesses() {
        let responses = vec![
            CorroborateResponse {
                request_id: [0u8; 32],
                witness_pubkey: [1u8; 32],
                verified: true,
                timestamp: 1000,
                signature: [0u8; 64],
            },
            CorroborateResponse {
                request_id: [0u8; 32],
                witness_pubkey: [2u8; 32],
                verified: true,
                timestamp: 1001,
                signature: [0u8; 64],
            },
            CorroborateResponse {
                request_id: [0u8; 32],
                witness_pubkey: [3u8; 32],
                verified: false,
                timestamp: 1002,
                signature: [0u8; 64],
            },
        ];

        assert!(has_enough_witnesses(&responses, 2));
        assert!(!has_enough_witnesses(&responses, 3));
    }

    #[test]
    fn test_prepare_submission() {
        let responses = vec![
            CorroborateResponse {
                request_id: [0u8; 32],
                witness_pubkey: [1u8; 32],
                verified: true,
                timestamp: 1000,
                signature: [0u8; 64],
            },
            CorroborateResponse {
                request_id: [0u8; 32],
                witness_pubkey: [2u8; 32],
                verified: false,
                timestamp: 1001,
                signature: [0u8; 64],
            },
        ];

        let sub = prepare_submission(
            [0xAA; 32],
            0, // Residency
            [0xBB; 32],
            [0xCC; 32],
            &responses,
        );

        assert_eq!(sub.witness_pubkeys.len(), 1);
        assert_eq!(sub.witness_pubkeys[0], [1u8; 32]);
    }
}
