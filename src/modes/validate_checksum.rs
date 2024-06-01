use std::{path::PathBuf, process::ExitCode};

use crate::ChecksumFileUtils;

use super::utils::Mode;

pub struct ValidateChecksumMode {
    pub checksum_filepath: PathBuf,
    pub ignore_missing: bool,
}

impl Mode for ValidateChecksumMode {
    fn run(&self) -> ExitCode {
        let file_path = self.checksum_filepath.as_path();
        match ChecksumFileUtils::check(file_path, self.ignore_missing) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                println!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}
