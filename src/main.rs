use glob::glob;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::path::{self, Path, PathBuf};
use std::thread::{self, available_parallelism};
use std::{fs::File, future::Future, io::Read};
use tokio::task::futures;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;

use clap::{arg, command, Parser};
use rusqlite::{Connection, Result};

mod db;

#[derive(Debug)]
enum HashAlgorithm {
    Md5 = 1,
    Sha1 = 2,
    Sha256 = 3,
}

// #[derive(Debug)]
// struct FileHash {
//     full_path: String,
//     algorithm: HashAlgorithm,
//     hash: Vec<u8>,
// }

#[derive(Debug)]
struct Files {
    id: u64,
    full_path: String,
    file_name: String,
}

#[derive(Debug)]
struct Md5HashTable {
    file_id: u64,
    hash: Vec<u8>,
}

#[derive(Debug)]
struct Sha1HashTable {
    file_id: u64,
    hash: Vec<u8>,
}

#[derive(Debug)]
struct Sha256HashTable {
    file_id: u64,
    hash: Vec<u8>,
}

trait Hash {
    fn calc(value: impl Into<String>) -> String;

    fn calc_from_file(file: &mut File) -> String {
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);
        Self::calc(buf)
    }

    fn calc_from_path(path: &Path) -> String {
        let mut file = File::open(path).unwrap();
        Self::calc_from_file(&mut file)
    }
}

struct Md5Hash {}

impl Hash for Md5Hash {
    fn calc(value: impl Into<String>) -> String {
        let mut md5 = Md5::new();
        md5.input(value.into().as_bytes());
        md5.result_str()
    }
}

struct Sha1Hash {}

impl Hash for Sha1Hash {
    fn calc(value: impl Into<String>) -> String {
        let mut sha1 = Sha1::new();
        sha1.input(value.into().as_bytes());
        sha1.result_str()
    }
}

struct Sha256Hash {}

impl Hash for Sha256Hash {
    fn calc(value: impl Into<String>) -> String {
        let mut sha256 = Sha256::new();
        sha256.input(value.into().as_bytes());
        sha256.result_str()
    }
}

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
    filepath: Option<String>,
}

async fn async_calc_md5(value: impl Into<String>) -> String {
    Md5Hash::calc(value)
}

async fn async_calc_sha1(value: impl Into<String>) -> String {
    Sha1Hash::calc(value)
}

async fn async_calc_sha256(value: impl Into<String>) -> String {
    Sha256Hash::calc(value)
}

async fn async_calc_md5_from_file(file: &mut File) -> impl Future<Output = String> {
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    async_calc_md5(buf)
}

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

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let md5hash = async_calc_md5(&buf).await;
        println!("{}", md5hash);

        let sha1hash = async_calc_sha1(&buf).await;
        println!("{}", sha1hash);

        let sha256hash = async_calc_sha256(&buf).await;
        println!("{}", sha256hash);

        if args.file.is_none() {
            return;
        }

        for f in args.file.unwrap() {
            let mut file = File::open(&f).unwrap();
            let md5hash = async_calc_md5_from_file(&mut file).await;
            println!("{}  {}", md5hash.await, &f);
        }
    });

    let cpus = available_parallelism().unwrap().get();
    println!("number of CPUs: {}", cpus);

    // glob_with_recursive("./*", &mut |p| {
    //     let mut f = File::open(p).unwrap();
    //     let mut buf = String::new();
    //     let _ = f.read_to_string(&mut buf);
    //     println!(
    //         "{}    {}    {}",
    //         p.display(),
    //         Md5Hash::calc(&buf),
    //         Sha1Hash::calc(&buf)
    //     );
    // });

    let db_path = Path::new("hash_table.db");
    let mut conn = Connection::open(&db_path).unwrap();

    println!("con = {:?}", conn);

    let initialize_list = [r#"
        PRAGMA foreign_keys=true
        "#];
    let create_tables = [
        r#"
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            full_path TEXT NOT NULL,
            file_name TEXT NOT NULL
        );
        "#,
        r#"CREATE TABLE IF NOT EXISTS md5_hash_table (
            id INTEGER PRIMARY KEY,
            file_id INTEGER NOT NULL UNIQUE,
            hash BLOB NOT NULL,
            FOREIGN KEY (file_id) REFERENCES files (id)
        );
        "#,
        r#"CREATE TABLE IF NOT EXISTS sha256_hash_table (
            id INTEGER PRIMARY KEY,
            file_id INTEGER NOT NULL UNIQUE,
            hash BLOB NOT NULL,
            FOREIGN KEY (file_id) REFERENCES files (id)
        );
        "#,
    ];

    for sql in initialize_list {
        conn.execute(sql, []).unwrap();
    }

    for sql in create_tables {
        conn.execute(sql, []).unwrap();
    }

    let mut file_list: Vec<PathBuf> = vec![];
    glob_with_recursive("./*", &mut |p| {
        file_list.push(p.clone());
    });

    let tx = conn.transaction().unwrap();
    for file in file_list.iter() {
        let file_id = insert_files(&tx, &file);

        insert_md5_hash_table(&tx, file_id, &file);
        insert_sha256_hash_table(&tx, file_id, &file);
    }
    tx.commit();
}

fn insert_files(conn: &Connection, path: &PathBuf) -> i64 
{
    static INSERT_SQL: &str = r#"
        INSERT INTO files (full_path, file_name)
        VALUES (?, ?)
    "#;

    let file_name = path.file_name().unwrap().to_str().unwrap();
    let mut stmt = conn.prepare(INSERT_SQL).unwrap();
    stmt.execute([path.to_str().unwrap(), file_name]).unwrap();

    conn.last_insert_rowid()
}

fn insert_md5_hash_table(conn: &Connection, file_id: i64, path: &PathBuf) -> i64 {
    static INSERT_SQL: &str = r#"
        INSERT INTO md5_hash_table (file_id, hash)
        VALUES (?, ?)
    "#;

    let md5_hash = Md5Hash::calc_from_path(&path);
    let mut stmt = conn.prepare(INSERT_SQL).unwrap();
    stmt.execute([file_id.to_string(), md5_hash]).unwrap();

    conn.last_insert_rowid()
}

fn insert_sha256_hash_table(conn: &Connection, file_id: i64, path: &PathBuf) -> i64 {
    static INSERT_SQL: &str = r#"
        INSERT INTO sha256_hash_table (file_id, hash)
        VALUES (?, ?)
    "#;

    let sha256_hash = Sha256Hash::calc_from_path(&path);
    let mut stmt = conn.prepare(INSERT_SQL).unwrap();
    stmt.execute([file_id.to_string(), sha256_hash]).unwrap();

    conn.last_insert_rowid()
}
