use std::path::Path;

use rusqlite::{Connection, Error, ErrorCode};

use crate::models::file_table::FileTable;

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
