use std::process::ExitCode;

use crate::Args;

use super::utils::Mode;

pub struct UseDatabaseMode {
    pub args: Args,
}

impl Mode for UseDatabaseMode {
    fn run(&self) -> ExitCode {
        0.into()
    }
}

