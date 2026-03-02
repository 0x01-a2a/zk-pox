//! ZK-PoX extension verifier for receiving agents.
//!
//! When an agent receives an ADVERTISE message with extensions.zk_pox,
//! this module extracts the proof and verifies it using zkpox-core.
//! This runs on the RECEIVING side only — the broadcasting node
//! never touches this code.

use serde_json::Value;

/// Result of verifying a ZK-PoX extension from an ADVERTISE message.
#[derive(Debug)]
pub enum ExtensionVerifyResult {
    /// No zk_pox extension present — not a ZK-PoX agent.
    NotPresent,
    /// Extension present but malformed.
    Malformed(String),
    /// Proof verified successfully.
    Valid {
        proof_type: String,
        count_proven: u32,
        time_window_days: u32,
    },
    /// Proof present but cryptographically invalid.
    Invalid(String),
}

/// Check if an ADVERTISE payload contains a ZK-PoX extension.
pub fn has_zkpox_extension(advertise_json: &Value) -> bool {
    advertise_json
        .get("extensions")
        .and_then(|e| e.get("zk_pox"))
        .is_some()
}

/// Extract and verify the ZK-PoX extension from an ADVERTISE payload.
///
/// In production, this calls `zkpox_core::verifier::verify_proof()`
/// on the decoded Bulletproof bytes and commitments. For now,
/// it performs structural validation.
pub fn verify_extension(advertise_json: &Value) -> ExtensionVerifyResult {
    let ext = match advertise_json
        .get("extensions")
        .and_then(|e| e.get("zk_pox"))
    {
        Some(v) => v,
        None => return ExtensionVerifyResult::NotPresent,
    };

    let proof_type = match ext.get("proof_type").and_then(|v| v.as_str()) {
        Some(s) => s.to_string(),
        None => return ExtensionVerifyResult::Malformed("missing proof_type".into()),
    };

    let count_proven = match ext.get("count_proven").and_then(|v| v.as_u64()) {
        Some(n) => n as u32,
        None => return ExtensionVerifyResult::Malformed("missing count_proven".into()),
    };

    let time_window_days = match ext.get("time_window_days").and_then(|v| v.as_u64()) {
        Some(n) => n as u32,
        None => return ExtensionVerifyResult::Malformed("missing time_window_days".into()),
    };

    let _proof_b64 = match ext.get("proof_bytes_b64").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return ExtensionVerifyResult::Malformed("missing proof_bytes_b64".into()),
    };

    let _commitments_b64 = match ext.get("commitments_b64").and_then(|v| v.as_str()) {
        Some(s) => s,
        None => return ExtensionVerifyResult::Malformed("missing commitments_b64".into()),
    };

    // TODO: decode base64, call zkpox_core::verifier::verify_proof()
    // For prototype: structural validation only.

    ExtensionVerifyResult::Valid {
        proof_type,
        count_proven,
        time_window_days,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_no_extension() {
        let msg = json!({ "services": ["delivery"], "geo": { "country": "PL" } });
        assert!(!has_zkpox_extension(&msg));
        assert!(matches!(verify_extension(&msg), ExtensionVerifyResult::NotPresent));
    }

    #[test]
    fn test_valid_extension() {
        let msg = json!({
            "services": ["delivery"],
            "geo": { "country": "PL", "city": "Warsaw" },
            "extensions": {
                "zk_pox": {
                    "proof_type": "RESIDENCY",
                    "radius_m": 2000,
                    "time_window_days": 30,
                    "count_proven": 28,
                    "proof_hash": "abcdef",
                    "proof_bytes_b64": "ZmFrZQ==",
                    "commitments_b64": "ZmFrZQ==",
                    "center_hash": "123456"
                }
            }
        });
        assert!(has_zkpox_extension(&msg));
        match verify_extension(&msg) {
            ExtensionVerifyResult::Valid { proof_type, count_proven, time_window_days } => {
                assert_eq!(proof_type, "RESIDENCY");
                assert_eq!(count_proven, 28);
                assert_eq!(time_window_days, 30);
            }
            other => panic!("expected Valid, got {:?}", other),
        }
    }

    #[test]
    fn test_malformed_extension() {
        let msg = json!({
            "extensions": {
                "zk_pox": {
                    "proof_type": "RESIDENCY"
                    // missing required fields
                }
            }
        });
        assert!(matches!(verify_extension(&msg), ExtensionVerifyResult::Malformed(_)));
    }
}
