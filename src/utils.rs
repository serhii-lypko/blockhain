use std::time::{SystemTime, UNIX_EPOCH};
use tiny_keccak::{Hasher, Sha3};

pub fn create_hash(bytes: &[u8]) -> [u8; 32] {
    let mut hasher = Sha3::v256();
    hasher.update(bytes);

    let mut output: [u8; 32] = [0u8; 32];
    hasher.finalize(&mut output);

    output
}

pub fn get_current_system_time() -> u64 {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    timestamp
}
