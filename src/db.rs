use std::path::Path;

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
    fn open_db_file(path: &Path) -> Result<Self, rusqlite::Error> {
        todo!()
    }

    fn open_db_in_memory() -> Result<Self, rusqlite::Error> {
        todo!()
    }

    fn execute(self, sql: impl Into<String>) -> Result<(), rusqlite::Error> {
        todo!()
    }
}
