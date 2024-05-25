use std::process::ExitCode;

use crate::Args;
use crate::ChecksumFileUtils;

use super::utils::Mode;

use std::path::Path;

pub struct ValidateChecksumMode {
    pub args: Args,
}

impl Mode for ValidateChecksumMode {
    fn run(&self) -> ExitCode {
        let checksum_filepath_string = self.args.checksum_filepath.as_ref().unwrap();
        let checksum_filepath = Path::new(&checksum_filepath_string);
        ChecksumFileUtils::check(checksum_filepath, true);
        0.into()
    }
}
