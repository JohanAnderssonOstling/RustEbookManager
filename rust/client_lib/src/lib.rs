
extern crate core;
#[allow(dead_code)]
#[allow(warnings)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(unused_assignments)]
#[allow(unused_parens)]
mod db;
pub mod constant;
mod library_model;
mod parser;

use std::fs;
use db::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::ffi::*;

lazy_static!{
    static ref HOME_DB_CONN: Mutex<home_db::HomeDb> = Mutex::new(home_db::HomeDb::new());
    static ref LIBRARY_DB_CONNS: Vec<Mutex<library_db::LibraryDB>> = Vec::new();
}

pub fn add_library(name: &str, path: &str) -> Library{
    let home_db = HOME_DB_CONN.lock().unwrap();
    let local_path = path.replace("file://", "");
    let library = home_db.add_library(name, local_path.as_str());

    let library_dir = constant::DATA_DIR.join(&library.uuid);
    fs::create_dir_all(library_dir).unwrap();

    let library_model = library_model::open_library(&library.uuid, &library.path);
    library_model[0].scan_library();

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

#[cxx::bridge]
mod ffi {
    pub struct Library {
        pub uuid: String,
        pub name: String,
        pub path: String,
    }

    extern "Rust" {
        fn add_library(name: &str, path: &str) -> Library;
        fn get_library_list() -> Vec<Library>;
        fn delete_library(library_uuid: &str);
    }
}
