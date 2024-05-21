use crate::hashes;
use crate::hashes::hash::Hash;
use crate::models::model::Model;
use std::path::{Path, PathBuf};

use rusqlite::{Connection, Error, ErrorCode};

use crate::models::file_table::FileTable;
use crate::models::md5_hash_table::Md5HashTable;
use crate::models::sha1_hash_table::Sha1HashTable;
use crate::models::sha256_hash_table::Sha256HashTable;
use hashes::md5::Md5Hash;
use hashes::sha1::Sha1Hash;
use hashes::sha256::Sha256Hash;

trait DbBase<S> {
    fn open_db_file(path: &Path) -> Result<S, rusqlite::Error>;
    fn open_db_in_memory() -> Result<S, rusqlite::Error>;
    fn execute(self, sql: impl Into<String>) -> Result<(), rusqlite::Error>;
}

struct DbSqlite3 {
    connection: rusqlite::Connection,
}

impl DbBase<Self> for DbSqlite3 {
    fn open_db_file(path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open(&path);
        match conn {
            Ok(connection) => Ok(DbSqlite3 { connection }),
            Err(e) => Err(e),
        }
    }

    fn open_db_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = rusqlite::Connection::open_in_memory();
        match conn {
            Ok(connection) => Ok(DbSqlite3 { connection }),
            Err(e) => Err(e),
        }
    }

    fn execute(self, sql: impl Into<String>) -> Result<(), rusqlite::Error> {
        self.connection.execute(sql.into().as_str(), [])?;
        Ok(())
    }
}

struct HashDbFile {}

impl DbBase<Self> for HashDbFile {
    fn open_db_file(_path: &Path) -> Result<Self, rusqlite::Error> {
        todo!()
    }

    fn open_db_in_memory() -> Result<Self, rusqlite::Error> {
        todo!()
    }

    fn execute(self, _sql: impl Into<String>) -> Result<(), rusqlite::Error> {
        todo!()
    }
}

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

pub fn initialize_database(initialize: bool, update: bool) -> Result<Connection, String> {
    let db_path = Path::new("hash_table.db");
    if initialize {
        match std::fs::remove_file(&db_path) {
            Ok(_) => {}
            Err(e) => {
                return Err(format!("{}", e));
            }
        }
    };

    let conn = Connection::open(&db_path).unwrap();
    Ok(conn)
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
