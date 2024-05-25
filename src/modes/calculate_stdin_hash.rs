use std::io::{self, Read};
use std::process::ExitCode;

use super::utils::Mode;
use crate::hashes::{hash::Hash, md5::Md5Hash, sha1::Sha1Hash, sha256::Sha256Hash};
use crate::print_some;

macro_rules! calc_hash {
    ($calculator:ty, $input:tt) => {
        Some(<$calculator>::calc($input))
    };
}

macro_rules! calc_hash_if {
    ($flag:tt, $calculator:ty, $input:tt) => {
        if $flag {
            calc_hash!($calculator, $input)
        } else {
            None
        }
    };
}

pub struct CalculateStdinHashMode {
    pub md5: bool,
    pub sha1: bool,
    pub sha256: bool,
}

impl Mode for CalculateStdinHashMode {
    fn run(&self) -> ExitCode {
        let mut buffer = String::new();
        let mut lock = io::stdin().lock();
        match lock.read_to_string(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                return 255.into();
            }
        };

        let (md5, sha1, sha256) = Self::calc_hash(&mut buffer, self.md5, self.sha1, self.sha256);

        print_some!(md5);
        print_some!(sha1);
        print_some!(sha256);
        println!("-");

        0.into()
    }
}

impl CalculateStdinHashMode {
    fn calc_hash(
        input: &String,
        md5: bool,
        sha1: bool,
        sha256: bool,
    ) -> (Option<String>, Option<String>, Option<String>) {
        if !md5 && !sha1 && !sha256 {
            return (
                calc_hash!(Md5Hash, input),
                calc_hash!(Sha1Hash, input),
                calc_hash!(Sha256Hash, input),
            );
        }

        (
            calc_hash_if!(md5, Md5Hash, input),
            calc_hash_if!(sha1, Sha1Hash, input),
            calc_hash_if!(sha256, Sha256Hash, input),
        )
    }
}
