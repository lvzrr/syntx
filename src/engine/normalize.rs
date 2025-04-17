///! Normalization module (engine/normalize.rs)
///!
use xxhash_rust::xxh3::xxh3_128;

pub fn normalize(name: &[u8], brace_level: usize) -> u64 {
    let mut buffer = Vec::with_capacity(name.len() + 8);
    buffer.extend_from_slice(name);
    buffer.extend_from_slice(&(brace_level as u64).to_le_bytes());

    let hash128 = xxh3_128(&buffer);
    (hash128 >> 64) as u64 ^ (hash128 as u64)
}
