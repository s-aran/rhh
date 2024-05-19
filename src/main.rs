use glob::glob;
use hashes::hash::Hash;
use hashes::md5::Md5Hash;
use hashes::sha1::Sha1Hash;
use hashes::sha256::Sha256Hash;
use models::model::Model;
use std::path::{Path, PathBuf};
use std::thread::available_parallelism;
use std::{fs::File, io::Read};

use clap::{arg, command, Parser};
use rusqlite::Connection;

mod db;
mod models;

use models::file_table::FileTable;
use models::md5_hash_table::Md5HashTable;
use models::sha1_hash_table::Sha1HashTable;
use models::sha256_hash_table::Sha256HashTable;

use crate::hashes::hash::ChecksumFileUtils;

mod hashes;

#[derive(Parser, Debug)]
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

fn validate_database_arguments(args: &Args) -> Result<(bool, bool), String> {
    let initialize = args.initialize_database;
    let update = args.update_database;

    if initialize && update {
        return Err(format!("invalid option: --initialize-db with --update-db"));
    }

    Ok((initialize, update))
}

fn includes_checksum_filename(filepath: &Path, lower_pattern: impl Into<String>) -> bool {
    let filename_os_str = match filepath.file_name() {
        Some(n) => n,
        None => {
            return false;
        }
    };

    filename_os_str
        .to_string_lossy()
        .to_lowercase()
        .find(&lower_pattern.into())
        .is_some()
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    // passed checksum file
    if args.checksum_filepath.is_some() {
        let checksum_filepath_string = args.checksum_filepath.unwrap();
        let checksum_filepath = Path::new(&checksum_filepath_string);
        ChecksumFileUtils::check(checksum_filepath, true);
        return;
    }

    match initialize_database(&args) {
        Ok(processed) => {
            if processed {
                return;
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

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
}

fn initialize_database(args: &Args) -> Result<bool, String> {
    let (initialize, update) = match validate_database_arguments(&args) {
        Ok(flags) => flags,
        Err(s) => {
            eprintln!("{}", s);
            return Err(s);
        }
    };

    if !(initialize || update) {
        return Ok(false);
    }

    let db_path = Path::new("hash_table.db");
    if initialize {
        match std::fs::remove_file(&db_path) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    };

    let mut conn = Connection::open(&db_path).unwrap();
    create_database(&mut conn);

    Ok(true)
}

fn create_database(conn: &mut Connection) {
    println!("con = {:?}", conn);

    let initialize_list = [r#"
        PRAGMA foreign_keys=true
        "#];

    {
        let tx = conn.transaction().unwrap();
        for sql in initialize_list {
            tx.execute(sql, []).unwrap();
        }

        FileTable::create(&tx);
        Md5HashTable::create(&tx);
        Sha1HashTable::create(&tx);
        Sha256HashTable::create(&tx);

        match tx.commit() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("transaction commit failed. {:?}", e);
                return;
            }
        }
    }

    let mut file_list: Vec<PathBuf> = vec![];
    glob_with_recursive("./*", &mut |p| {
        file_list.push(p.clone());
    });

    {
        let tx = conn.transaction().unwrap();
        for file in file_list.iter() {
            let file_id = insert_files(&tx, &file);

            insert_md5_hash_table(&tx, file_id, &file);
            insert_sha1_hash_table(&tx, file_id, &file);
            insert_sha256_hash_table(&tx, file_id, &file);
        }

        match tx.commit() {
            Ok(()) => {}
            Err(e) => {
                eprintln!("transaction commit failed. {:?}", e);
                return;
            }
        }
    }
}

fn insert_files(conn: &Connection, path: &PathBuf) -> i64 {
    let file_name = path.file_name().unwrap().to_str().unwrap();
    let f = FileTable {
        id: None,
        full_path: path.to_str().unwrap().to_string(),
        file_name: file_name.to_string(),
    };

    f.insert(conn)
}

fn insert_md5_hash_table(conn: &Connection, file_id: i64, path: &PathBuf) -> i64 {
    let hash = Md5Hash::calc_from_path(&path);
    let h = Md5HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}

fn insert_sha1_hash_table(conn: &Connection, file_id: i64, path: &PathBuf) -> i64 {
    let hash = Sha1Hash::calc_from_path(&path);
    let h = Sha1HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}

fn insert_sha256_hash_table(conn: &Connection, file_id: i64, path: &PathBuf) -> i64 {
    let hash = Sha256Hash::calc_from_path(&path);
    let h = Sha256HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}
