use std::{fs::File, io::Read, path::Path};

use super::{md5::Md5Hash, sha1::Sha1Hash, sha256::Sha256Hash};

pub trait Hash {
    fn calc(value: impl Into<String>) -> String {
        Self::calc_bytes(value.into().as_bytes())
    }
    async fn acalc(value: impl Into<String>) -> String {
        Self::calc(value)
    }

    fn calc_bytes(bytes: &[u8]) -> String;
    async fn acalc_bytes(bytes: &[u8]) -> String {
        Self::calc_bytes(bytes)
    }

    fn calc_from_file(file: &mut File) -> String {
        let mut buf = String::new();
        let _ = file.read_to_string(&mut buf);
        Self::calc(buf)
    }

    async fn acalc_from_file(file: &mut File) -> String {
        Self::calc_from_file(file)
    }

    fn calc_from_path(path: &Path) -> String {
        let mut file = File::open(path).unwrap();
        Self::calc_from_file(&mut file)
    }

    async fn acalc_from_path(path: &Path) -> String {
        Self::calc_from_path(path)
    }

    fn get_hash_length() -> usize;
}

pub struct ChecksumFileUtils;

impl ChecksumFileUtils {
    const DELIMITER: &'static str = "  ";

    pub fn check(checksum_filepath: &Path, ignore_missing: bool) {
        let hash_filename_map = match Self::parse_checksum_file(checksum_filepath) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("{}", e);
                return;
            }
        };

        for (hash, filename) in hash_filename_map {
            match Self::check_hash(&hash, &filename, ignore_missing) {
                Ok(r) => {
                    println!("{}: {}", filename, r);
                }
                Err(e) => {
                    panic!("{}", e);
                }
            };
        }
    }

    fn parse_checksum_file(checksum_file: &Path) -> Result<Vec<(String, String)>, String> {
        let mut checksum_file = match File::open(checksum_file) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("{}", e));
            }
        };
        let mut buf = String::new();
        let _ = checksum_file.read_to_string(&mut buf);

        let delimiter_length = Self::DELIMITER.len();

        // key ... checksum
        // value ... filename
        let mut checksum_filename_vec = Vec::<(String, String)>::new();
        for line in buf.lines() {
            let space_pos = match line.find(Self::DELIMITER) {
                Some(n) => n,
                None => {
                    return Err(format!("delimiter not found"));
                }
            };

            let hash = line[0..space_pos].to_owned();
            let filename = line[space_pos + delimiter_length..line.len()].to_owned();

            checksum_filename_vec.push((hash, filename));
        }

        Ok(checksum_filename_vec)
    }

    fn check_hash(
        hash: &String,
        filename: &String,
        ignore_missing: bool,
    ) -> Result<&'static str, String> {
        let path = Path::new(&filename);
        if !path.exists() {
            if ignore_missing {
                return Ok("Not found");
            }
            return Err(format!("{} not found", filename));
        }

        let matched = if hash.len() == Md5Hash::get_hash_length() {
            hash == &Md5Hash::calc_from_path(&path)
        } else if hash.len() == Sha1Hash::get_hash_length() {
            hash == &Sha1Hash::calc_from_path(&path)
        } else if hash.len() == Sha256Hash::get_hash_length() {
            hash == &Sha256Hash::calc_from_path(&path)
        } else {
            return Err(format!("invalid hash length: {}  {}", hash, filename));
        };

        Ok(if matched { "OK" } else { "FAILED" })
    }
}
