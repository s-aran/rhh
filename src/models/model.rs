use std::rc::Rc;

use rusqlite::Connection;

pub trait Model {
    fn create(connection: &Connection);

    fn get(connection: &Connection, id: i64) -> Self;
    fn all(connection: &Connection) -> Vec<Rc<Self>>;

    fn insert(&self, connection: &Connection) -> i64;
    fn update(&self, connection: &Connection) -> i64;
    fn delete(&self, connection: &Connection);
}
