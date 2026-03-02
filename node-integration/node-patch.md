# Node Patch — Mesh Message Handling for ZK-PoX

## File: `node/crates/zerox1-node/src/node.rs`

### 1. Add import (top of file):

```rust
use crate::zkpox;
```

### 2. Add to `handle_pubsub_message` match arms:

Find the `match msg_type { ... }` block in `handle_pubsub_message`.
Add these arms before the default `_ =>` catch-all:

```rust
zkpox::MSG_TYPE_CORROBORATE_REQUEST => {
    let request: zkpox::CorroborateRequest = match serde_json::from_slice(&payload) {
        Ok(r) => r,
        Err(e) => {
            log::warn!("Invalid CORROBORATE_REQUEST: {e}");
            return;
        }
    };
    match zkpox::handle_corroborate_request(&request, &our_pubkey, &our_signing_key) {
        zkpox::CorroborateAction::Attest(response) => {
            let resp_bytes = serde_json::to_vec(&response).unwrap();
            self.publish(zkpox::MSG_TYPE_CORROBORATE_RESPONSE, &resp_bytes).await;
            log::info!("ZK-PoX: attested credential {:?}", hex::encode(&request.credential_id[..8]));
        }
        zkpox::CorroborateAction::Reject(response) => {
            let resp_bytes = serde_json::to_vec(&response).unwrap();
            self.publish(zkpox::MSG_TYPE_CORROBORATE_RESPONSE, &resp_bytes).await;
            log::warn!("ZK-PoX: rejected credential {:?}", hex::encode(&request.credential_id[..8]));
        }
        zkpox::CorroborateAction::Ignore => {
            log::debug!("ZK-PoX: ignoring malformed corroboration request");
        }
    }
}

zkpox::MSG_TYPE_CORROBORATE_RESPONSE => {
    let response: zkpox::CorroborateResponse = match serde_json::from_slice(&payload) {
        Ok(r) => r,
        Err(e) => {
            log::warn!("Invalid CORROBORATE_RESPONSE: {e}");
            return;
        }
    };
    // Store the response for the pending credential submission.
    // The credential submission logic will check has_enough_witnesses()
    // and submit on-chain when the threshold is met.
    self.pending_corroborations
        .entry(response.request_id)
        .or_insert_with(Vec::new)
        .push(response);
}
```

### 3. Add field to Node struct:

```rust
/// Pending ZK-PoX corroboration responses, keyed by request_id.
pending_corroborations: HashMap<[u8; 32], Vec<zkpox::CorroborateResponse>>,
```

Initialize in `Node::new()`:

```rust
pending_corroborations: HashMap::new(),
```

### 4. Add module declaration to `lib.rs`:

```rust
pub mod zkpox;
```
