use glob::glob;
use std::path::{Path, PathBuf};
use std::thread::available_parallelism;
use std::{fs::File, future::Future, io::Read};

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;

use clap::{arg, command, Parser};

trait Hash {
    fn calc(value: impl Into<String>) -> String;
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

fn glob_with_recursive(pattern: &str) {
    glob(pattern)
        .expect("Failed to read glob pattern")
        .for_each(|entry| match entry {
            Ok(path) => {
                if path.is_dir() {
                    glob_with_recursive(&format!("{}/*", path.display()));
                } else {
                    println!("{}", path.display());
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

    glob_with_recursive("./*");
}
