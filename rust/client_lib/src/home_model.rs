

use crate::db::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::library_model::create_library;

pub struct Library {
	pub uuid: String,
	pub name: String,
	pub path: String,
}

lazy_static!{
    static ref HOME_DB_CONN: Mutex<home_db::HomeDb> = Mutex::new(home_db::HomeDb::new());

}

pub fn add_library(name: &str, path: &str) -> Library{
	let home_db = HOME_DB_CONN.lock().unwrap();
	let local_path = path.replace("file://", "");
	let library = home_db.add_library(name, local_path.as_str());
	create_library(library.uuid.as_str(), library.path.as_str());


	return library
}

pub fn get_library_list() -> Vec<Library>{
	let home_db = HOME_DB_CONN.lock().unwrap();
	let library_list = home_db.get_libraries();
	library_list
}

pub fn delete_library(library_uuid: &str){
	let home_db = HOME_DB_CONN.lock().unwrap();
	home_db.delete_library(library_uuid);
}
