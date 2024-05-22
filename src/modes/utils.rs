use std::process::ExitCode;

use crate::Args;

use super::unexpected_arguments::UnexpectedArgumentsMode;

pub trait Mode {
    fn run(&self) -> ExitCode;
}

pub fn determine_mode(args: &Args) -> Box<dyn Mode> {
    Box::new(UnexpectedArgumentsMode { args: args.clone() })
}
