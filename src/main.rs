use glob::glob;
use std::borrow::Borrow;
use std::process::ExitCode;

use std::path::{Path, PathBuf};
use std::thread::available_parallelism;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};

mod db;
mod models;

use crate::db::{create_database, initialize_database};
use crate::hashes::hash::ChecksumFileUtils;

mod hashes;
mod modes;

use crate::modes::utils::{self, determine_mode};

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(help = "FILE")]
    file: Option<Vec<String>>,
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

fn glob_with_recursive<F>(pattern: &str, handler: &mut F)
where
    F: FnMut(&PathBuf) -> (),
{
    glob(pattern)
        .expect("Failed to read glob pattern")
        .for_each(|entry| match entry {
            Ok(path) => {
                if path.is_dir() {
                    glob_with_recursive(&format!("{}/*", path.display()), handler);
                } else {
                    handler(&path);
                }
            }
            Err(e) => println!("{:?}", e),
        });
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

fn main() -> ExitCode {
    let args = Args::parse();

    // passed file
    let a = determine_mode(&args);
    a.run();

    // passed checksum file
    if args.checksum_filepath.is_some() {
        let checksum_filepath_string = args.checksum_filepath.unwrap();
        let checksum_filepath = Path::new(&checksum_filepath_string);
        ChecksumFileUtils::check(checksum_filepath, true);
        return 0.into();
    }

    // db
    let (initialize, update, use_db) = match validate_database_arguments(&args) {
        Ok(flags) => flags,
        Err(s) => {
            eprintln!("{}", s);
            return 0.into();
        }
    };

    // if !(initialize || update) {
    //     return 1.into();
    // }

    let mut connection = match initialize_database(initialize, update) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            return 1.into();
        }
    };

    let mut file_list: Vec<PathBuf> = vec![];
    glob_with_recursive("./*", &mut |p| {
        file_list.push(p.clone());
    });

    create_database(&mut connection, &file_list);

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

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

    let cpus = available_parallelism().unwrap().get();
    println!("number of CPUs: {}", cpus);

    return 0.into();
}
