use std::rc::Rc;

use rusqlite::Connection;

use super::model::Model;

#[derive(Debug)]
pub struct FileTable {
    pub id: Option<i64>,
    pub full_path: String,
    pub file_name: String,
}

impl Model for FileTable {
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

    fn create(&self, connection: &Connection) -> i64 {
        static INSERT_SQL: &str = r#"
            INSERT INTO files (full_path, file_name)
            VALUES (?,?)
        "#;

        let mut stmt = connection.prepare(INSERT_SQL).unwrap();
        stmt.execute([&self.full_path, &self.file_name]).unwrap();

        connection.last_insert_rowid()
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
