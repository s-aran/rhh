use std::rc::Rc;

use rusqlite::Connection;

use crate::db::is_sqlite_error_constraint_violation;

use super::model::Model;

#[derive(Debug)]
pub struct FileTable {
    pub id: Option<i64>,
    pub full_path: String,
    pub file_name: String,
}

impl Model for FileTable {
    fn create(connection: &Connection) {
        static SQL: &str = r#"
        CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY,
            full_path TEXT NOT NULL UNIQUE,
            file_name TEXT NOT NULL
        );
        "#;

        connection.execute(SQL, []).unwrap();
    }

    fn get(connection: &Connection, id: i64) -> Self {
        static SQL: &str = "SELECT * FROM files WHERE id = ?";

        let mut stmt = connection.prepare(SQL).unwrap();
        let mut rows = stmt.query(&[&id]).unwrap();
        let row = rows.next().unwrap().unwrap();

        let full_path = row.get(1).unwrap();
        let file_name = row.get(2).unwrap();

        Self {
            id: Some(id),
            full_path,
            file_name,
        }
    }

    fn all(connection: &Connection) -> Vec<Rc<Self>> {
        static SQL: &str = "SELECT * FROM files";

        let mut stmt = connection.prepare(SQL).unwrap();
        let mut rows = stmt.query([]).unwrap();
        let mut files = Vec::new();
        while let Some(row) = rows.next().unwrap() {
            let id = row.get(0).unwrap();
            let full_path = row.get(1).unwrap();
            let file_name = row.get(2).unwrap();
            files.push(Rc::new(Self {
                id: Some(id),
                full_path,
                file_name,
            }))
        }

        files
    }

    fn insert(&self, connection: &Connection) -> i64 {
        static INSERT_SQL: &str = r#"
            INSERT INTO files (full_path, file_name)
            VALUES (?,?)
        "#;

        let mut stmt = connection.prepare(INSERT_SQL).unwrap();
        match stmt.execute([&self.full_path, &self.file_name]) {
            Ok(_) => connection.last_insert_rowid(),
            Err(e) => {
                if is_sqlite_error_constraint_violation(&e) {
                    let duplicated_id = Self::get_id_by_full_path(connection, &self.full_path);
                    return duplicated_id;
                }

                eprintln!("FileTable insert failed. {:?}", e);
                -1
            }
        }
    }

    fn update(&self, connection: &Connection) -> i64 {
        static UPDATE_SQL: &str = r#"
            UPDATE files SET full_path=?, file_name=? WHERE id=?
        "#;

        let mut stmt = connection.prepare(UPDATE_SQL).unwrap();
        stmt.execute([
            &self.full_path,
            &self.file_name,
            &format!("{}", self.id.unwrap()),
        ])
        .unwrap();

        self.id.unwrap()
    }

    fn delete(&self, connection: &Connection) {
        static DELETE_SQL: &str = r#"
            DELETE FROM files WHERE id=?
        "#;

        let mut stmt = connection.prepare(DELETE_SQL).unwrap();
        stmt.execute([self.id.unwrap()]).unwrap();
    }
}

impl FileTable {
    pub fn get_id_by_full_path(connection: &Connection, full_path: impl Into<String>) -> i64 {
        static SQL: &str = r#"
            SELECT id FROM files WHERE full_path=?
        "#;

        let mut stmt = connection.prepare(SQL).unwrap();
        let mut rows = stmt.query(&[&full_path.into()]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let id = row.get(0).unwrap();

        id
    }
}
