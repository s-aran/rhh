use std::rc::Rc;

use rusqlite::{Connection, Result};

use super::model::Model;

#[derive(Debug)]
pub struct Md5HashTable {
    pub id: Option<i64>,
    pub file_id: i64,
    pub hash: String,
}

impl Model for Md5HashTable {
    fn create(connection: &Connection) {
        static SQL: &str = r#"CREATE TABLE IF NOT EXISTS md5_hash_table (
            id INTEGER PRIMARY KEY,
            file_id INTEGER NOT NULL UNIQUE,
            hash BLOB NOT NULL,
            FOREIGN KEY (file_id) REFERENCES files (id)
            UNIQUE(file_id, hash)
        );
        "#;

        connection.execute(SQL, []).unwrap();
    }

    fn get(connection: &Connection, id: i64) -> Self {
        static SQL: &str = "SELECT * FROM md5_hash_table WHERE id = ?";

        let mut stmt = connection.prepare(SQL).unwrap();
        let mut rows = stmt.query(&[&id]).unwrap();
        let row = rows.next().unwrap().unwrap();

        let id = row.get(0).unwrap();
        let file_id = row.get(1).unwrap();
        let hash = row.get(2).unwrap();

        Self {
            id: Some(id),
            file_id,
            hash,
        }
    }

    fn all(connection: &Connection) -> Vec<Rc<Self>> {
        static SQL: &str = "SELECT * FROM md5_hash_table";

        let mut stmt = connection.prepare(SQL).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut result = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let id = row.get(0).unwrap();
            let file_id = row.get(1).unwrap();
            let hash = row.get(2).unwrap();
            result.push(Rc::new(Self {
                id: Some(id),
                file_id,
                hash,
            }))
        }

        result
    }

    fn insert(&self, connection: &Connection) -> i64 {
        static INSERT_SQL: &str = r#"
        INSERT INTO md5_hash_table (file_id, hash)
        VALUES (?, ?)
    "#;

        let mut stmt = connection.prepare(INSERT_SQL).unwrap();
        match stmt.execute([&format!("{}", self.file_id), &self.hash]) {
            Ok(_) => return connection.last_insert_rowid(),
            Err(e) => {
                eprintln!("FileTable: {}", e);
            }
        };

        -1
    }

    fn update(&self, connection: &Connection) -> i64 {
        static UPDATE_SQL: &str = r#"
        UPDATE md5_hash_table
        SET file_id = ?, hash = ?
        WHERE id = ?
        "#;

        let mut stmt = connection.prepare(UPDATE_SQL).unwrap();
        stmt.execute([
            &format!("{}", self.file_id),
            &self.hash,
            &format!("{}", self.id.unwrap()),
        ])
        .unwrap();

        self.id.unwrap()
    }

    fn delete(&self, connection: &Connection) {
        static DELETE_SQL: &str = "DELETE FROM md5_hash_table WHERE id = ?";

        let mut stmt = connection.prepare(DELETE_SQL).unwrap();
        stmt.execute([self.id.unwrap()]).unwrap();
    }
}
