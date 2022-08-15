use std::fs::File;
use std::io::Read;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn main() {
    println!("Hello, world!");

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    let mut md5 = Md5::new();
    md5.input(buf.as_slice());
    println!("{}", md5.result_str());
}
