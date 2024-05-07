use std::rc::Rc;

use rusqlite::{Connection, Result};

use super::model::Model;

#[derive(Debug)]
pub struct Sha1HashTable {
    pub id: Option<i64>,
    pub file_id: i64,
    pub hash: String,
}

impl Model for Sha1HashTable {
    fn get(connection: &Connection, id: i64) -> Self {
        static SQL: &str = "SELECT * FROM sha1_hash_table WHERE id = ?";

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
        static SQL: &str = "SELECT * FROM sha1_hash_table";

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

    fn create(connection: &Connection) -> Result<usize> {
        static SQL: &str = r#"CREATE TABLE IF NOT EXISTS sha1_hash_table (
            id INTEGER PRIMARY KEY,
            file_id INTEGER NOT NULL UNIQUE,
            hash BLOB NOT NULL,
            FOREIGN KEY (file_id) REFERENCES files (id)
        );
        "#;

        connection.execute(SQL, [])
    }

    fn insert(&self, connection: &Connection) -> i64 {
        static INSERT_SQL: &str = r#"
        INSERT INTO sha1_hash_table (file_id, hash)
        VALUES (?, ?)
    "#;

        let mut stmt = connection.prepare(INSERT_SQL).unwrap();
        stmt.execute([&format!("{}", self.file_id), &self.hash])
            .unwrap();

        connection.last_insert_rowid()
    }

    fn update(&self, connection: &Connection) -> i64 {
        static UPDATE_SQL: &str = r#"
        UPDATE sha1_hash_table
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
        static DELETE_SQL: &str = "DELETE FROM sha1_hash_table WHERE id = ?";

        let mut stmt = connection.prepare(DELETE_SQL).unwrap();
        stmt.execute([self.id.unwrap()]).unwrap();
    }
}
