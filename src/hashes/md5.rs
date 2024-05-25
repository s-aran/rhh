use crypto::{digest::Digest, md5::Md5};

use crate::hashes::hash::Hash;

pub struct Md5Hash {}

impl Hash for Md5Hash {
    fn calc(value: impl Into<String>) -> String {
        Self::calc_bytes(value.into().as_bytes())
    }

    fn calc_bytes(bytes: &[u8]) -> String {
        let mut md5 = Md5::new();
        md5.input(bytes);
        md5.result_str()
    }

    fn get_hash_length() -> usize {
        32
    }
}
