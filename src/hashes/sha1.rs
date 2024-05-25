use crypto::{digest::Digest, sha1::Sha1};

use crate::hashes::hash::Hash;

pub struct Sha1Hash {}

impl Hash for Sha1Hash {
    fn calc_bytes(bytes: &[u8]) -> String {
        let mut sha1 = Sha1::new();
        sha1.input(bytes);
        sha1.result_str()
    }

    fn get_hash_length() -> usize {
        40
    }
}
