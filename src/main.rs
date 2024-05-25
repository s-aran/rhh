use std::process::ExitCode;

use std::thread::available_parallelism;
use std::{fs::File, io::Read};

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

// async fn async_calc_md5_from_file(file: &mut File) -> impl Future<Output = String> {
//     let mut buf = String::new();
//     let _ = file.read_to_string(&mut buf);
//     async_calc_md5(buf)
// }

fn main() -> ExitCode {
    let args = Args::parse();
    let m = determine_mode(&args);
    m.run()

    // if !(initialize || update) {
    //     return 1.into();
    // }

    // let mut file = File::open("Cargo.lock").unwrap();
    // let mut buf = String::new();
    // let _ = file.read_to_string(&mut buf);

    // let rt = tokio::runtime::Runtime::new().unwrap();

    // rt.block_on(async {
    //     let md5hash = async_calc_md5(&buf).await;
    //     println!("{}", md5hash);

    //     let sha1hash = async_calc_sha1(&buf).await;
    //     println!("{}", sha1hash);

    //     let sha256hash = async_calc_sha256(&buf).await;
    //     println!("{}", sha256hash);

    //     if args.file.is_none() {
    //         return;
    //     }

    //     for f in args.file.unwrap() {
    //         let mut file = File::open(&f).unwrap();
    //         let md5hash = async_calc_md5_from_file(&mut file).await;
    //         println!("{}  {}", md5hash.await, &f);
    //     }
    // });

    // let cpus = available_parallelism().unwrap().get();
    // println!("number of CPUs: {}", cpus);

    // return 0.into();
}
