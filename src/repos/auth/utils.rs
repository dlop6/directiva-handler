use sha2::{Digest, Sha256};
use hex;

pub fn hashing_composite_key(first_arg: &str, second_arg: &str) -> String {
    let mut hasher = Sha256::new();
    let combined = format!("{}:{}", first_arg, second_arg);
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
