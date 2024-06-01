use std::process::ExitCode;

use clap::{arg, command, Parser};

mod db;
mod models;

use crate::hashes::hash::ChecksumFileUtils;

mod hashes;
mod modes;
mod utils;

use crate::modes::utils::determine_mode;

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(help = "FILE")]
    files: Option<Vec<String>>,

    #[arg(long = "md5", default_value = "false", help = "show md5 hash")]
    md5: bool,

    #[arg(long = "sha1", default_value = "false", help = "show sha1 hash")]
    sha1: bool,

    #[arg(long = "sha256", default_value = "false", help = "show sha256 hash")]
    sha256: bool,

    #[arg(
        short = 'c',
        long = "check",
        help = "read checksums from the FILEs and check them"
    )]
    checksum_filepath: Option<String>,

    #[arg(
        long = "ignore-missing",
        alias = "ignore",
        help = "do not fail or report status for missing files"
    )]
    ignore_missing: bool,

    #[arg(
        long = "init-db",
        default_value = "false",
        help = "drop database and insert file and hash records"
    )]
    initialize_database: bool,

    #[arg(
        long = "update-db",
        default_value = "false",
        help = "append file and hash records to database"
    )]
    update_database: bool,

    #[arg(short = 'd', long = "use-db", help = "use hash database")]
    use_db: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let m = determine_mode(&args);
    m.run()
}
