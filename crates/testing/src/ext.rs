//! tmp extentions for testing

use crate::crypto;

/// Retrive the storage key from the given key
pub fn storage_key(service: u32, rkey: &[u8]) -> [u8; 31] {
    let mut input = service.to_le_bytes().to_vec();
    input.extend_from_slice(rkey);
    let hash = crypto::blake2b(&input);

    // construct the final storage key
    let mut key = [0u8; 31];
    key[..8].copy_from_slice(&self::prefix(service, &[255, 255, 255, 255]));
    key[8..].copy_from_slice(&hash[..23]);
    key
}

fn prefix(service: u32, prefix: &[u8; 4]) -> [u8; 8] {
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
