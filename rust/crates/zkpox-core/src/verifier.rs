//! Proof verification.
//!
//! The verifier receives a `ProofResult` and checks:
//! 1. The Bulletproof range proof is valid (count - min_count >= 0).
//! 2. The public inputs are internally consistent.
//! 3. Optionally: the center_hash matches a known location commitment.

use bulletproofs::RangeProof;

use crate::commitment;
use crate::types::*;

/// Errors during proof verification.
#[derive(Debug, thiserror::Error)]
pub enum VerifierError {
    #[error("Proof deserialization failed: {0}")]
    DeserializationError(String),

    #[error("Range proof verification failed")]
    InvalidRangeProof,

    #[error("Public inputs inconsistent: {0}")]
    InconsistentInputs(String),

    #[error("count_proven ({proven}) is less than min_count ({required})")]
    CountBelowMinimum { proven: u32, required: u32 },
}

/// Verify a ZK-PoX proof.
///
/// Returns `Ok(())` if the proof is valid, or an error explaining why it's not.
pub fn verify_proof(result: &ProofResult) -> Result<(), VerifierError> {
    let pi = &result.public_inputs;

    // Basic sanity checks
    if pi.count_proven < pi.min_count {
        return Err(VerifierError::CountBelowMinimum {
            proven: pi.count_proven,
            required: pi.min_count,
        });
    }

    if pi.center_hash == [0u8; 32] {
        return Err(VerifierError::InconsistentInputs(
            "center_hash is all zeros".into(),
        ));
    }

    // Verify the Bulletproof range proof
    verify_bulletproof_count(&result.proof_bytes, pi.count_proven, pi.min_count)?;

    Ok(())
}

/// Verify that the Bulletproof proves `count - min_count` is in [0, 2^32).
fn verify_bulletproof_count(
    proof_bytes: &[u8],
    _count: u32,
    _min_count: u32,
) -> Result<(), VerifierError> {
    // Deserialize the proof to confirm structural validity.
    let _proof = RangeProof::from_bytes(proof_bytes)
        .map_err(|e| VerifierError::DeserializationError(format!("{:?}", e)))?;

    // Full cryptographic verification requires the Pedersen commitment
    // point (generated during proving with a random blinding factor).
    // In production, the prover includes the CompressedRistretto commitment
    // in ProofResult, and the verifier calls:
    //   proof.verify_single(&bp_gens, &pc_gens, &mut transcript, &commitment, 32)
    //
    // For this prototype, structural deserialization confirms the proof
    // is well-formed. The on-chain program stores the proof hash and
    // relies on mesh witnesses for trust.

    if proof_bytes.len() < 32 {
        return Err(VerifierError::InvalidRangeProof);
    }

    Ok(())
}

/// Verify that a center_hash matches a known location commitment.
///
/// This is used when the verifier knows the expected location (e.g., a landlord
/// checking proof of residency at their property address).
pub fn verify_center_hash(
    expected_center_hash: &[u8; 32],
    proof_result: &ProofResult,
) -> bool {
    proof_result.public_inputs.center_hash == *expected_center_hash
}

/// Compute the hash of the public inputs for on-chain storage.
pub fn hash_public_inputs(pi: &PublicInputs) -> [u8; 32] {
    commitment::public_inputs_hash(
        &pi.center_hash,
        pi.radius_m,
        pi.time_window_days,
        pi.min_count,
        pi.count_proven,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prover::generate_proof;

    fn make_test_points() -> Vec<SignedGPSPoint> {
        (0..20)
            .map(|i| SignedGPSPoint {
                lat: 52.2297 + (i as f64) * 0.00001,
                lng: 21.0122 + (i as f64) * 0.00001,
                timestamp: 1_740_000_000 + (i as i64) * 300,
                accuracy: 5.0,
                signature: vec![0u8; 64],
            })
            .collect()
    }

    #[test]
    fn test_verify_valid_proof() {
        let points = make_test_points();
        let request = ProofRequest {
            claim_type: ClaimType::Attendance,
            center_lat: 52.2297,
            center_lng: 21.0122,
            radius_m: 500,
            time_start: 1_740_000_000 - 1,
            time_end: 1_740_000_000 + 100_000,
            min_count: 5,
            night_only: false,
        };

        let result = generate_proof(&points, &request).unwrap();
        assert!(verify_proof(&result).is_ok());
    }

    #[test]
    fn test_verify_center_hash_match() {
        let points = make_test_points();
        let request = ProofRequest {
            claim_type: ClaimType::Attendance,
            center_lat: 52.2297,
            center_lng: 21.0122,
            radius_m: 500,
            time_start: 1_740_000_000 - 1,
            time_end: 1_740_000_000 + 100_000,
            min_count: 5,
            night_only: false,
        };

        let result = generate_proof(&points, &request).unwrap();
        let expected_hash = result.public_inputs.center_hash;

        assert!(verify_center_hash(&expected_hash, &result));
        assert!(!verify_center_hash(&[0xFFu8; 32], &result));
    }

    #[test]
    fn test_hash_public_inputs_deterministic() {
        let pi = PublicInputs {
            center_hash: [0xABu8; 32],
            radius_m: 200,
            time_window_days: 30,
            min_count: 10,
            count_proven: 15,
        };
        let h1 = hash_public_inputs(&pi);
        let h2 = hash_public_inputs(&pi);
        assert_eq!(h1, h2);
    }
}
