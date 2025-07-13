//! Crypto utilities

/// Compute the BLAKE2b 256-bit hash of a given input.
pub fn blake2b(input: &[u8]) -> [u8; 32] {
    use blake2::{digest::consts::U32, Blake2b, Digest};

    let mut hasher = Blake2b::<U32>::new();
    hasher.update(input);
    hasher.finalize().into()
}
