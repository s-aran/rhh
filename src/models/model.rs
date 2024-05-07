use std::rc::Rc;

use rusqlite::{Connection, Result};

pub trait Model {
    fn get(connection: &Connection, id: i64) -> Self;
    fn all(connection: &Connection) -> Vec<Rc<Self>>;
    fn create(connection: &Connection) -> Result<usize>;
    fn insert(&self, connection: &Connection) -> i64;
    fn update(&self, connection: &Connection) -> i64;
    fn delete(&self, connection: &Connection);
}
