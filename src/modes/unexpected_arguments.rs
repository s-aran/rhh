use std::process::ExitCode;

use crate::Args;

use super::utils::Mode;

pub struct UnexpectedArgumentsMode {
    pub args: Args,
}

impl Mode for UnexpectedArgumentsMode {
    fn run(&self) -> ExitCode {
        255.into()
    }
}
