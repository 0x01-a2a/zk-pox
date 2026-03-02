use sha2::{Digest, Sha256};

/// Commit to a GPS position without revealing it.
/// Returns `SHA-256(lat_le_bytes || lng_le_bytes || salt)`.
pub fn position_commitment(lat: f64, lng: f64, salt: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(lat.to_le_bytes());
    hasher.update(lng.to_le_bytes());
    hasher.update(salt);
    hasher.finalize().into()
}

/// Commit to a timestamp without revealing it.
/// Returns `SHA-256(timestamp_le_bytes || salt)`.
pub fn time_commitment(timestamp: i64, salt: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(timestamp.to_le_bytes());
    hasher.update(salt);
    hasher.finalize().into()
}

/// Hash the public inputs for compact on-chain storage.
/// Returns `SHA-256(center_hash || radius || time_window_days || min_count || count_proven)`.
pub fn public_inputs_hash(
    center_hash: &[u8; 32],
    radius_m: u32,
    time_window_days: u32,
    min_count: u32,
    count_proven: u32,
) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(center_hash);
    hasher.update(radius_m.to_le_bytes());
    hasher.update(time_window_days.to_le_bytes());
    hasher.update(min_count.to_le_bytes());
    hasher.update(count_proven.to_le_bytes());
    hasher.finalize().into()
}

/// Hash a completed proof for on-chain reference.
/// Returns `SHA-256(proof_bytes)`.
pub fn proof_hash(proof_bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(proof_bytes);
    hasher.finalize().into()
}

/// The message that gets signed when recording a GPS point.
/// `SHA-256(lat_le_bytes || lng_le_bytes || timestamp_le_bytes)`
pub fn gps_point_message(lat: f64, lng: f64, timestamp: i64) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(lat.to_le_bytes());
    hasher.update(lng.to_le_bytes());
    hasher.update(timestamp.to_le_bytes());
    hasher.finalize().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_commitment_deterministic() {
        let salt = [0xABu8; 32];
        let c1 = position_commitment(52.2297, 21.0122, &salt);
        let c2 = position_commitment(52.2297, 21.0122, &salt);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_position_commitment_different_salt() {
        let salt1 = [0xABu8; 32];
        let salt2 = [0xCDu8; 32];
        let c1 = position_commitment(52.2297, 21.0122, &salt1);
        let c2 = position_commitment(52.2297, 21.0122, &salt2);
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_position_commitment_different_location() {
        let salt = [0xABu8; 32];
        let c1 = position_commitment(52.2297, 21.0122, &salt);
        let c2 = position_commitment(48.8566, 2.3522, &salt);
        assert_ne!(c1, c2);
    }

    #[test]
    fn test_time_commitment_deterministic() {
        let salt = [0x11u8; 32];
        let c1 = time_commitment(1_740_000_000, &salt);
        let c2 = time_commitment(1_740_000_000, &salt);
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_gps_point_message() {
        let msg = gps_point_message(52.2297, 21.0122, 1_740_000_000);
        assert_eq!(msg.len(), 32);
    }

    #[test]
    fn test_proof_hash() {
        let h = proof_hash(b"some proof data");
        assert_eq!(h.len(), 32);
    }
}
