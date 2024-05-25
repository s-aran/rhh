use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use crate::{
    hashes::{hash::Hash, md5::Md5Hash, sha1::Sha1Hash, sha256::Sha256Hash},
    print_some,
};

use super::utils::Mode;

macro_rules! calc_hash {
    ($calculator:ty, $path:tt) => {
        Some(<$calculator>::calc_from_path($path))
    };
}

macro_rules! calc_hash_if {
    ($flag:tt, $calculator:ty, $path:tt) => {
        if $flag {
            calc_hash!($calculator, $path)
        } else {
            None
        }
    };
}

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

            let file_hash = (
                file,
                Self::calc_hash(file, self.md5, self.sha1, self.sha256),
            );
            hashes.push(file_hash);
        }

        for (file, hash) in hashes.iter() {
            let (md5, sha1, sha256) = hash;

            print_some!(md5);
            print_some!(sha1);
            print_some!(sha256);
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
                calc_hash!(Md5Hash, file),
                calc_hash!(Sha1Hash, file),
                calc_hash!(Sha256Hash, file),
            );
        }

        (
            calc_hash_if!(md5, Md5Hash, file),
            calc_hash_if!(sha1, Sha1Hash, file),
            calc_hash_if!(sha256, Sha256Hash, file),
        )
    }
}
