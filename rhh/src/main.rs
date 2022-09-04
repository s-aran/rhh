use std::fs::File;
use std::io::Read;

use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha1::Sha1;

use crc::Crc;

fn main() {
    println!("Hello, world!");

    let mut file = File::open("Cargo.lock").unwrap();
    let mut buf = Vec::new();
    let _ = file.read_to_end(&mut buf);

    let mut md5 = Md5::new();
    md5.input(buf.as_slice());
    println!("{}", md5.result_str());

    let mut sha1 = Sha1::new();
    sha1.input(buf.as_slice());
    println!("{}", sha1.result_str());

    let crc16 = Crc::<u16>::new(&crc::CRC_16_ISO_IEC_14443_3_A);
    let mut digest16 = crc16.digest();
    digest16.update(buf.as_slice());
    println!("{}", digest16.finalize());

    let crc32 = Crc::<u32>::new(&crc::CRC_32_CKSUM);
    println!("{}", crc32.checksum(buf.as_slice()));
    let mut digest32 = crc32.digest();
    digest32.update(buf.as_slice());
    println!("{}", digest32.finalize());
    println!("{}, {}", buf.len(), buf.last().unwrap());
    println!("{}", &crc::CRC_32_CKSUM.check);
}
