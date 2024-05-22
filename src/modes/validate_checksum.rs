use std::process::ExitCode;

use crate::Args;

use super::utils::Mode;

pub struct ChecksumFileMode {
    pub args: Args,
}

impl Mode for ChecksumFileMode {
    fn run(&self) -> ExitCode {
        0.into()
    }
}
