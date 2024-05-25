use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use crypto::md5::Md5;

use crate::hashes::{hash::Hash, md5::Md5Hash, sha1::Sha1Hash, sha256::Sha256Hash};

use super::utils::Mode;

pub struct CalculateFileHashMode {
    pub md5: bool,
    pub sha1: bool,
    pub sha256: bool,
    pub files: Vec<PathBuf>,
}

impl Mode for CalculateFileHashMode {
    fn run(&self) -> ExitCode {
        let mut hashes = vec![];
        for file in self.files.iter() {
            if !file.exists() {
                eprintln!("{} does not exist", file.to_string_lossy());
                return 1.into();
            }

            let a = (
                file,
                Self::calc_hash(file, self.md5, self.sha1, self.sha256),
            );
            hashes.push(a);
        }

        for (file, hash) in hashes.iter() {
            let (md5, sha1, sha256) = hash;

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

            println!("{}", file.to_string_lossy());
        }

        0.into()
    }
}

impl CalculateFileHashMode {
    fn calc_hash(
        file: &Path,
        md5: bool,
        sha1: bool,
        sha256: bool,
    ) -> (Option<String>, Option<String>, Option<String>) {
        if !md5 && !sha1 && !sha256 {
            return (
                Some(Md5Hash::calc_from_path(file)),
                Some(Sha1Hash::calc_from_path(file)),
                Some(Sha256Hash::calc_from_path(file)),
            );
        }

        (
            if md5 {
                Some(Md5Hash::calc_from_path(file))
            } else {
                None
            },
            if sha1 {
                Some(Sha1Hash::calc_from_path(file))
            } else {
                None
            },
            if sha256 {
                Some(Sha256Hash::calc_from_path(file))
            } else {
                None
            },
        )
    }
}
