//! JNI bridge: exposes zkpox-core to Kotlin on Android.
//!
//! This crate compiles to a `.so` shared library that gets loaded by
//! `ZkPoxModule.kt` via `System.loadLibrary("zkpox_mobile")`.
//!
//! Integration:
//!   Cross-compile with cargo-ndk for aarch64-linux-android and
//!   armv7-linux-androideabi, then place the .so in
//!   mobile/android/app/src/main/jniLibs/{arm64-v8a,armeabi-v7a}/

use zkpox_core::types::*;
use zkpox_core::prover;
use zkpox_core::verifier;

/// Generate a ZK-PoX proof from JSON-serialized inputs.
///
/// # Arguments
/// * `points_json` — JSON array of `SignedGPSPoint`
/// * `request_json` — JSON object `ProofRequest`
///
/// # Returns
/// JSON string of `ProofResult`, or an error string prefixed with "ERROR:"
///
/// Called from Kotlin:
/// ```kotlin
/// val resultJson = generateProofNative(pointsJson, requestJson)
/// ```
#[no_mangle]
pub extern "C" fn generate_proof_json(
    points_json: *const u8,
    points_len: usize,
    request_json: *const u8,
    request_len: usize,
    out_buf: *mut u8,
    out_buf_len: usize,
) -> i32 {
    let result = std::panic::catch_unwind(|| {
        let points_slice = unsafe { std::slice::from_raw_parts(points_json, points_len) };
        let request_slice = unsafe { std::slice::from_raw_parts(request_json, request_len) };

        let points_str = match std::str::from_utf8(points_slice) {
            Ok(s) => s,
            Err(_) => return write_output(out_buf, out_buf_len, b"ERROR:Invalid UTF-8 in points"),
        };

        let request_str = match std::str::from_utf8(request_slice) {
            Ok(s) => s,
            Err(_) => return write_output(out_buf, out_buf_len, b"ERROR:Invalid UTF-8 in request"),
        };

        let points: Vec<SignedGPSPoint> = match serde_json::from_str(points_str) {
            Ok(p) => p,
            Err(e) => {
                let msg = format!("ERROR:Failed to parse points: {e}");
                return write_output(out_buf, out_buf_len, msg.as_bytes());
            }
        };

        let request: ProofRequest = match serde_json::from_str(request_str) {
            Ok(r) => r,
            Err(e) => {
                let msg = format!("ERROR:Failed to parse request: {e}");
                return write_output(out_buf, out_buf_len, msg.as_bytes());
            }
        };

        match prover::generate_proof(&points, &request) {
            Ok(result) => {
                let json = serde_json::to_string(&result).unwrap_or_else(|e| {
                    format!("ERROR:Serialization failed: {e}")
                });
                write_output(out_buf, out_buf_len, json.as_bytes())
            }
            Err(e) => {
                let msg = format!("ERROR:{e}");
                write_output(out_buf, out_buf_len, msg.as_bytes())
            }
        }
    });

    match result {
        Ok(n) => n,
        Err(_) => write_output(out_buf, out_buf_len, b"ERROR:Panic during proof generation"),
    }
}

/// Verify a ZK-PoX proof from JSON-serialized input.
///
/// Returns 1 for valid, 0 for invalid, -1 for error.
#[no_mangle]
pub extern "C" fn verify_proof_json(
    result_json: *const u8,
    result_len: usize,
) -> i32 {
    let outcome = std::panic::catch_unwind(|| {
        let slice = unsafe { std::slice::from_raw_parts(result_json, result_len) };
        let s = match std::str::from_utf8(slice) {
            Ok(s) => s,
            Err(_) => return -1,
        };

        let proof_result: ProofResult = match serde_json::from_str(s) {
            Ok(r) => r,
            Err(_) => return -1,
        };

        match verifier::verify_proof(&proof_result) {
            Ok(()) => 1,
            Err(_) => 0,
        }
    });

    outcome.unwrap_or(-1)
}

fn write_output(buf: *mut u8, buf_len: usize, data: &[u8]) -> i32 {
    let len = data.len().min(buf_len);
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), buf, len);
    }
    len as i32
}
