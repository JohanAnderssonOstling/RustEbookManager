
use std::fs;
//use sqlx::{Connection, Executor, Pool, Sqlite, SqliteConnection, SqlitePool};

use uuid::Uuid;
use rusqlite::{params, Connection};
use crate::constant;
use crate::ffi::*;
//use uuid::Uuid;

pub(crate) struct HomeDb {
    connection: Connection,
}

impl HomeDb {

    pub fn new() -> Self {
        fs::create_dir_all(constant::DB_DIR.to_str().unwrap()).unwrap();
        let conn_result = Connection::open(constant::HOME_DB_PATH.to_str().unwrap());
        let conn = match conn_result {
            Ok(conn) => conn,
            Err(e) => panic!("not good"),
        };
        let mut home_db = HomeDb {
            connection: conn
        };
        home_db.create_schema();
        home_db
    }

    fn create_schema(&self) {
        let query = "CREATE TABLE IF NOT EXISTS HOME (
        library_uuid TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        path TEXT NOT NULL
        );";
        let result = self.connection.execute(query, []);
    }

    pub fn add_library(&self, name: &str, path: &str) -> Library{
        let query = "INSERT INTO HOME (library_uuid, name, path) VALUES (?, ?, ?)";
        let uuid = Uuid::new_v4().to_string();
        let result = self.connection.execute(query, params![uuid, name, path]).unwrap();
        Library {
            uuid,
            name: name.to_string(),
            path: path.to_string(),
        }
    }

    pub fn delete_library(&self, uuid: &str){
        let query = "DELETE FROM HOME WHERE library_uuid = ?";
        let result = self.connection.execute(query, params![uuid]).unwrap();
    }

    pub fn get_libraries(&self) -> Vec<Library> {
        let query = "SELECT * FROM HOME";
        let mut statement = self.connection.prepare(query).unwrap();
        let mut library_vec: Vec<Library> = Vec::new();

        let library_iter = statement.query_map([], |row| {
            Ok(Library {
                uuid: row.get(0)?,
                name: row.get(1)?,
                path: row.get(2)?,
            })
        }).unwrap();

        for library in library_iter {
            library_vec.push(library.unwrap());
        }
        let hello = "hello";
        return library_vec
    }

}
