use std::process::ExitCode;

use crate::Args;

use super::utils::Mode;

struct CalculateFileHashMode {
    pub args: Args,
}

impl Mode for CalculateFileHashMode {
    fn run(&self) -> ExitCode {
        0.into()
    }
}
