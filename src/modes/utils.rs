use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::str::FromStr;

use crate::Args;

use super::calculate_file_hash::CalculateFileHashMode;
use super::calculate_stdin_hash::CalculateStdinHashMode;
use super::create_db::CreateDatabaseMode;
use super::unexpected_arguments::UnexpectedArgumentsMode;
use super::update_db::UpdateDatabaseMode;
use super::use_db::UseDatabaseMode;
use super::validate_checksum::ValidateChecksumMode;

#[macro_export]
macro_rules! print_some {
    ($hash: tt) => {
        match $hash {
            Some(h) => print!("{}  ", h),
            None => {}
        }
    };
}

pub trait Mode {
    fn run(&self) -> ExitCode;
}

fn validate_database_arguments(args: &Args) -> Result<(bool, bool, bool), String> {
    let initialize = args.initialize_database;
    let update = args.update_database;
    let use_db = args.use_db;

    if initialize && update {
        return Err(format!("invalid option: --initialize-db with --update-db"));
    }

    if use_db && (initialize || update) {
        return Err(format!(
            "invalid option: --use-db with --initialize-db or --update-db"
        ));
    }

    Ok((initialize, update, use_db))
}

pub fn determine_mode(args: &Args) -> Box<dyn Mode> {
    let (initialize, update, use_db) = match validate_database_arguments(&args) {
        Ok(flags) => flags,
        Err(s) => {
            eprintln!("{}", s);
            return Box::new(UnexpectedArgumentsMode { args: args.clone() });
        }
    };

    if initialize {
        return Box::new(CreateDatabaseMode {});
    }

    if update {
        return Box::new(UpdateDatabaseMode {});
    }

    if use_db {
        return Box::new(UseDatabaseMode { args: args.clone() });
    }

    // passed checksum file
    if args.checksum_filepath.is_some() {
        let checksum_filepath = Path::new(match args.checksum_filepath {
            Some(ref p) => p,
            None => {
                return Box::new(UnexpectedArgumentsMode { args: args.clone() });
            }
        });

        return Box::new(ValidateChecksumMode {
            checksum_filepath: PathBuf::from(checksum_filepath),
            ignore_missing: args.ignore_missing,
        });
    }

    if args.files.is_some() {
        let files = args
            .clone()
            .files
            .unwrap()
            .iter()
            .map(|f| PathBuf::from(f))
            .collect();

        return Box::new(CalculateFileHashMode {
            md5: args.md5,
            sha1: args.sha1,
            sha256: args.sha256,
            files,
        });
    }

    Box::new(CalculateStdinHashMode {
        md5: args.md5,
        sha1: args.sha1,
        sha256: args.sha256,
    })

    // Box::new(UnexpectedArgumentsMode { args: args.clone() })
}
