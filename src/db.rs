use crate::hashes;
use crate::hashes::hash::Hash;
use crate::models::model::Model;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use rusqlite::{Connection, Error, ErrorCode};

use crate::models::file_table::FileTable;
use crate::models::md5_hash_table::Md5HashTable;
use crate::models::sha1_hash_table::Sha1HashTable;
use crate::models::sha256_hash_table::Sha256HashTable;
use hashes::md5::Md5Hash;
use hashes::sha1::Sha1Hash;
use hashes::sha256::Sha256Hash;
use rayon::prelude::*;

pub static HASH_TABLE_FILENAME: &str = "hash_table.db";

pub fn is_sqlite_error_constraint_violation(e: &Error) -> bool {
    match e.sqlite_error() {
        Some(e) => e.code == ErrorCode::ConstraintViolation,
        None => false,
    }
}

pub fn get_id_by_file_id(
    connection: &Connection,
    table_name: impl Into<String> + std::fmt::Display,
    file_id: i64,
) -> i64 {
    let sql = format!(
        r#"
            SELECT id FROM {} WHERE file_id=?
        "#,
        table_name
    );

    let mut stmt = connection.prepare(&sql).unwrap();
    let mut rows = stmt.query([file_id]).unwrap();

    let row = rows.next().unwrap().unwrap();
    let id = row.get(0).unwrap();

    id
}

pub fn create_tables(conn: &mut Connection) {
    let initialize_list = [r#"
        PRAGMA foreign_keys=true
    "#];

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

fn process(path: &PathBuf) -> (String, String, String) {
    let mut file = match File::open(&path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e);
            return ("".to_owned(), "".to_owned(), "".to_owned());
        }
    };

    println!("{}", path.display());
    let mut buffer = vec![];
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let md5 = Md5Hash::calc_bytes(&buffer);
    let sha1 = Sha1Hash::calc_bytes(&buffer);
    let sha256 = Sha256Hash::calc_bytes(&buffer);

    (md5, sha1, sha256)
}

pub fn insert_tables(conn: &Connection, path: &PathBuf, md5: String, sha1: String, sha256: String) {
    let f = FileTable {
        id: None,
        full_path: path.to_string_lossy().to_string(),
        file_name: path.file_name().unwrap().to_string_lossy().to_string(),
    };

    let file_id = f.insert(conn);

    Md5HashTable {
        id: None,
        file_id,
        hash: md5,
    }
    .insert(conn);

    Sha1HashTable {
        id: None,
        file_id,
        hash: sha1,
    }
    .insert(conn);

    Sha256HashTable {
        id: None,
        file_id,
        hash: sha256,
    }
    .insert(conn);
}

pub fn create_database(conn: &mut Connection, file_list: &Vec<PathBuf>) {
    let hashes: Vec<(&PathBuf, String, String, String)> = file_list
        .par_iter()
        .map(|f| {
            let (md5, sha1, sha256) = process(&f);
            return (f, md5, sha1, sha256);
        })
        .collect();

    create_tables(conn);

    let tx = conn.transaction().unwrap();
    hashes.iter().for_each(|e| {
        let (f, md5, sha1, sha256) = e;
        insert_tables(
            &tx,
            &f,
            md5.to_string(),
            sha1.to_string(),
            sha256.to_string(),
        );
    });

    match tx.commit() {
        Ok(()) => {}
        Err(e) => {
            eprintln!("transaction commit failed. {:?}", e);
            return;
        }
    }
}
