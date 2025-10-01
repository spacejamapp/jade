//! Storage keys

use service::ServiceId;

const STORAGE_PREFIX: [u8; 4] = [255, 255, 255, 255];

/// Compute the storage key
pub fn storage(service: ServiceId, key: &[u8]) -> [u8; 31] {
    let mut hashed = STORAGE_PREFIX.to_vec();
    hashed.extend_from_slice(key);
    let hash = service::blake2b(hashed.as_slice());

    let mut key = [0u8; 31];
    let mut hashp = [0; 4];
    hashp.copy_from_slice(&hash[..4]);
    key[..8].copy_from_slice(&prefix(service, &hashp));
    key[8..].copy_from_slice(&hash[4..27]);
    key
}

/// Generate a prefix for a storage
///
/// service: [0, 2, 4, 6]
/// prefix: [1, 3, 5, 7]
pub fn prefix(service: u32, prefix: &[u8; 4]) -> [u8; 8] {
    let mut key = [0; 8];
    service
        .to_le_bytes()
        .iter()
        .zip(prefix.iter())
        .enumerate()
        .for_each(|(i, (a, b))| {
            key[i * 2] = *a;
            key[(i + 1) * 2 - 1] = *b;
        });
    key
}
