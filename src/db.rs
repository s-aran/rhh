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

pub fn create_database(conn: &mut Connection, file_list: &Vec<PathBuf>) {
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

    {
        let tx = conn.transaction().unwrap();
        for path in file_list.iter() {
            let file_id = insert_files(&tx, &path);
            println!("{}", path.display());

            let mut file = match File::open(&path) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };
            let mut buffer = vec![];
            file.read_to_end(&mut buffer).expect("Failed to read file");
            insert_md5_hash_table(&tx, file_id, &buffer);
            insert_sha1_hash_table(&tx, file_id, &buffer);
            insert_sha256_hash_table(&tx, file_id, &buffer);
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

fn insert_md5_hash_table(conn: &Connection, file_id: i64, buffer: &Vec<u8>) -> i64 {
    let hash = Md5Hash::calc_bytes(buffer);
    let h = Md5HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}

fn insert_sha1_hash_table(conn: &Connection, file_id: i64, buffer: &Vec<u8>) -> i64 {
    let hash = Sha1Hash::calc_bytes(buffer);
    let h = Sha1HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}

fn insert_sha256_hash_table(conn: &Connection, file_id: i64, buffer: &Vec<u8>) -> i64 {
    let hash = Sha256Hash::calc_bytes(buffer);
    let h = Sha256HashTable {
        id: None,
        file_id,
        hash,
    };

    h.insert(conn)
}
