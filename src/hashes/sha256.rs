use crypto::{digest::Digest, sha2::Sha256};

use crate::hashes::hash::Hash;

pub struct Sha256Hash {}

impl Hash for Sha256Hash {
    fn calc(value: impl Into<String>) -> String {
        let mut sha256 = Sha256::new();
        sha256.input(value.into().as_bytes());
        sha256.result_str()
    }

    fn get_hash_length() -> usize {
        64
    }
}
