use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

use rusqlite::Connection;

use crate::{
    db::{create_database, HASH_TABLE_FILENAME},
    utils::glob_with_recursive,
};

use super::utils::Mode;

pub struct UpdateDatabaseMode {}

impl Mode for UpdateDatabaseMode {
    fn run(&self) -> ExitCode {
        let db_path = Path::new(HASH_TABLE_FILENAME);

        let mut file_list: Vec<PathBuf> = vec![];
        glob_with_recursive("./*", &mut |p| {
            file_list.push(p.clone());
        });

        let mut connection = Connection::open(&db_path).unwrap();
        create_database(&mut connection, &file_list);

        0.into()
    }
}
