use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

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
    #[arg(
        short = 'c',
        long = "check",
        help = "read checksums from the FILEs and check them"
    )]
    filepath: Option<String>,
}

fn main() {
    println!("Hello, world!");

    let args = Args::parse();

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);

    let md5hash = Md5Hash::calc(&buf);
    println!("{}", md5hash);

    let sha1hash = Sha1Hash::calc(&buf);
    println!("{}", sha1hash);

    let sha256hash = Sha256Hash::calc(&buf);
    println!("{}", sha256hash);
}
