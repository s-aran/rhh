use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;
use crypto::sha2::Sha256;

fn main() {
    println!("Hello, world!");

    let args = env::args().collect();

    let handlers = HashMap::from([("f", |value: &str| {
        println!("f");
        value == "test"
    })]);

    

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    let mut md5 = Md5::new();
    md5.input(buf.as_slice());
    println!("{}", md5.result_str());

    let mut sha1 = Sha1::new();
    sha1.input(buf.as_slice());
    println!("{}", sha1.result_str());

    let mut sha256 = Sha256::new();
    sha256.input(buf.as_slice());
    println!("{}", sha256.result_str());
}
