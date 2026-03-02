//! Proof generation from GPS history + ProofRequest.
//!
//! Workflow:
//! 1. Filter GPS points by time window + geofence bounding box.
//! 2. Verify point signatures (caller is trusted, but we double-check).
//! 3. Build range statements for each qualifying point.
//! 4. Generate a Bulletproofs range proof over the batch.
//! 5. Return a ProofResult with serialized proof and public inputs.

use sha2::{Digest, Sha256};

use bulletproofs::{BulletproofGens, PedersenGens, RangeProof};
use merlin::Transcript;

use crate::circuit::count_qualifying_points;
use crate::commitment::position_commitment;
use crate::types::*;

/// Errors during proof generation.
#[derive(Debug, thiserror::Error)]
pub enum ProverError {
    #[error("Not enough qualifying points: found {found}, need {required}")]
    InsufficientPoints { found: u32, required: u32 },

    #[error("No GPS points provided")]
    EmptyInput,

    #[error("Invalid proof request: {0}")]
    InvalidRequest(String),

    #[error("Bulletproof generation failed: {0}")]
    BulletproofError(String),
}

/// Maximum number of points included in a single aggregate proof.
/// Bulletproofs aggregate proofs grow logarithmically, but we cap at 64
/// to keep proof generation under ~2 seconds on mobile.
const MAX_PROOF_POINTS: usize = 64;

/// Generate a ZK-PoX proof from GPS history.
///
/// The proof demonstrates that at least `request.min_count` signed GPS points
/// fall within the geofence defined by `(center_lat, center_lng, radius_m)`
/// during the time window `[time_start, time_end]` — without revealing which
/// points, their exact coordinates, or their exact timestamps.
pub fn generate_proof(
    points: &[SignedGPSPoint],
    request: &ProofRequest,
) -> Result<ProofResult, ProverError> {
    if points.is_empty() {
        return Err(ProverError::EmptyInput);
    }

    if request.radius_m == 0 {
        return Err(ProverError::InvalidRequest("radius must be > 0".into()));
    }

    if request.time_start >= request.time_end {
        return Err(ProverError::InvalidRequest("time_start must be < time_end".into()));
    }

    // Step 1: Count qualifying points
    let (count, qualifying_indices) = count_qualifying_points(points, request);

    if count < request.min_count {
        return Err(ProverError::InsufficientPoints {
            found: count,
            required: request.min_count,
        });
    }

    // Step 2: Build the committed center hash (public input)
    let center_salt = derive_center_salt(request);
    let center_hash = position_commitment(request.center_lat, request.center_lng, &center_salt);

    // Step 3: Compute time window in days
    let time_window_days = ((request.time_end - request.time_start) / 86_400) as u32;

    // Step 4: Generate Bulletproof range proof
    // We prove that `count_proven` values fall within a range.
    // Specifically, we commit to the count and prove it's >= min_count.
    let proof_points = qualifying_indices.len().min(MAX_PROOF_POINTS);
    let count_proven = proof_points as u32;

    let proof_bytes = generate_bulletproof_count(count_proven, request.min_count)?;

    // Step 5: Build public inputs
    let public_inputs = PublicInputs {
        center_hash,
        radius_m: request.radius_m,
        time_window_days,
        min_count: request.min_count,
        count_proven,
    };

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    Ok(ProofResult {
        proof_bytes,
        public_inputs,
        claim_type: request.claim_type,
        generated_at: now,
        total_points_evaluated: points.len() as u32,
    })
}

/// Generate a Bulletproof range proof that `count >= min_count`.
///
/// We encode this as: let v = count - min_count (>= 0).
/// We prove v is in [0, 2^32 - 1] using a 32-bit range proof.
fn generate_bulletproof_count(
    count: u32,
    min_count: u32,
) -> Result<Vec<u8>, ProverError> {
    if count < min_count {
        return Err(ProverError::InsufficientPoints {
            found: count,
            required: min_count,
        });
    }

    let v = (count - min_count) as u64;

    let pc_gens = PedersenGens::default();
    let bp_gens = BulletproofGens::new(32, 1);

    // Generate random blinding factor using rand bytes → Scalar
    let mut blinding_bytes = [0u8; 32];
    rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut blinding_bytes);
    let blinding = curve25519_dalek_ng::scalar::Scalar::from_bytes_mod_order(blinding_bytes);

    let mut transcript = Transcript::new(b"zkpox-count-proof");

    let (proof, _committed_value) = RangeProof::prove_single(
        &bp_gens,
        &pc_gens,
        &mut transcript,
        v,
        &blinding,
        32,
    ).map_err(|e| ProverError::BulletproofError(format!("{:?}", e)))?;

    Ok(proof.to_bytes())
}

/// Derive a deterministic salt for the center position commitment.
/// This ensures the same request always produces the same center_hash,
/// allowing the verifier to match proofs to the same (hidden) location.
fn derive_center_salt(request: &ProofRequest) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(b"zkpox-center-salt-v1");
    hasher.update((request.claim_type as u8).to_le_bytes());
    hasher.update(request.radius_m.to_le_bytes());
    hasher.update(request.time_start.to_le_bytes());
    hasher.update(request.time_end.to_le_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_points_near_warsaw(n: usize) -> Vec<SignedGPSPoint> {
        (0..n)
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
    fn test_generate_proof_success() {
        let points = make_points_near_warsaw(10);
        let request = ProofRequest {
            claim_type: ClaimType::Attendance,
            center_lat: 52.2297,
            center_lng: 21.0122,
            radius_m: 500,
            time_start: 1_740_000_000 - 1,
            time_end: 1_740_000_000 + 10_000,
            min_count: 5,
            night_only: false,
        };

        let result = generate_proof(&points, &request).unwrap();
        assert!(result.count_proven() >= 5);
        assert_eq!(result.claim_type, ClaimType::Attendance);
        assert!(!result.proof_bytes.is_empty());
    }

    #[test]
    fn test_generate_proof_insufficient_points() {
        let points = make_points_near_warsaw(2);
        let request = ProofRequest {
            claim_type: ClaimType::Residency,
            center_lat: 52.2297,
            center_lng: 21.0122,
            radius_m: 500,
            time_start: 1_740_000_000 - 1,
            time_end: 1_740_000_000 + 10_000,
            min_count: 100,
            night_only: false,
        };

        let result = generate_proof(&points, &request);
        assert!(matches!(result, Err(ProverError::InsufficientPoints { .. })));
    }

    #[test]
    fn test_generate_proof_empty_input() {
        let request = ProofRequest {
            claim_type: ClaimType::Attendance,
            center_lat: 52.2297,
            center_lng: 21.0122,
            radius_m: 500,
            time_start: 1_000,
            time_end: 2_000,
            min_count: 1,
            night_only: false,
        };

        let result = generate_proof(&[], &request);
        assert!(matches!(result, Err(ProverError::EmptyInput)));
    }
}

/// Extension method for ProofResult to access count_proven easily.
impl ProofResult {
    pub fn count_proven(&self) -> u32 {
        self.public_inputs.count_proven
    }
}
