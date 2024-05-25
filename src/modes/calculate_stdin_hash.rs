use std::io::{self, Read};
use std::process::ExitCode;

use super::utils::Mode;
use crate::hashes::{hash::Hash, md5::Md5Hash, sha1::Sha1Hash, sha256::Sha256Hash};

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

        match md5 {
            Some(md5) => print!("{}  ", md5),
            None => {}
        };

        match sha1 {
            Some(sha1) => print!("{}  ", sha1),
            None => {}
        };

        match sha256 {
            Some(sha256) => print!("{}  ", sha256),
            None => {}
        }

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
                Some(Md5Hash::calc(input)),
                Some(Sha1Hash::calc(input)),
                Some(Sha256Hash::calc(input)),
            );
        }

        (
            if md5 {
                Some(Md5Hash::calc(input))
            } else {
                None
            },
            if sha1 {
                Some(Sha1Hash::calc(input))
            } else {
                None
            },
            if sha256 {
                Some(Sha256Hash::calc(input))
            } else {
                None
            },
        )
    }
}
