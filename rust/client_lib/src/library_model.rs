#![feature(once_cell)]

use std::{env, fs};
use std::collections::HashMap;
use std::path::{Path};
use std::sync::Mutex;
use std::thread;

use once_cell::sync::Lazy;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use library_db::LibraryDB;

use crate::constant;
use crate::db::library_db;
use crate::library_model::ffi::*;
use crate::parser::parse_book;

pub const COVER_WIDTHS: [u32; 4] = [64, 128, 256, 512];

/// Holds the connection to the database, the path to the library and the uuid identifier of the library
pub struct LibraryDBModel {
	pub uuid: String,
	pub path: String,
	pub db_conn: Mutex<LibraryDB>,
}

static mut LIBRARY_DB_CONNS: Lazy<HashMap<String, LibraryDBModel>> = Lazy::new(HashMap::new);

/// Creates a new library with the given UUID at the specified path. This function creates a directory
/// with the UUID as its name in the `DATA_DIR` constant, and creates a subdirectory called "thumb" within
/// the library directory. It then calls `open_library` with the UUID and path arguments to initialize the
/// library, and `scan_library` with the UUID argument to scan the library for new items.
/// # Arguments
/// * `uuid` - The uuid of the library
/// * `path` - The path to the library
/// # Example
/// ```
/// create_library("1234", "/home/user/Books");
/// ```
pub fn create_library(uuid: &str, path: &str) {
	let library_dir = constant::DATA_DIR.join(uuid);
	fs::create_dir_all(&library_dir).unwrap();
	let thumb_dir = library_dir.join("thumb");
	fs::create_dir_all(thumb_dir).unwrap();
	open_library(uuid, path);
	scan_library(String::from(uuid));
}

/// Opens the library with the given UUID and path, creating a new connection to the library's
/// database and storing it in the global `LIBRARY_DB_CONNS` map. This function also sets the
/// `RUST_BACKTRACE` environment variable to enable full backtraces in case of errors.
///
/// # Arguments
///
/// * `uuid` - A string slice representing the UUID of the library to open.
/// * `path` - A string slice representing the path to the library to open.
///
/// # Safety
///
/// This function uses `unsafe` code to insert the library's database connection into the
/// `LIBRARY_DB_CONNS` global map. This is safe as long as the map is only accessed in a
/// thread-safe manner.
///
/// # Example
///
/// ```
/// open_library("12345", "/path/to/my/library");
/// ```
pub fn open_library(uuid: &str, path: &str) {
	env::set_var("RUST_BACKTRACE", "1");
	env::set_var("RUST_BACKTRACE", "full");
	let library = LibraryDBModel {
		uuid: String::from(uuid),
		path: String::from(path),
		db_conn: Mutex::new(LibraryDB::open(uuid)),
	};
	unsafe { LIBRARY_DB_CONNS.insert(String::from(uuid), library); }
}

/// Scans the library with the given UUID for new items, using the library's database connection
/// stored in the global `LIBRARY_DB_CONNS` map. This function spawns a new thread to perform the
/// scanning process.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library to scan.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// scan_library(String::from("12345"));
/// ```
pub fn scan_library(model_uuid: String) {
	let model = unsafe { LIBRARY_DB_CONNS.get(&*model_uuid).unwrap() };
	thread::spawn(|| {scan_library_aux(model, &model.path, 0)});
}


fn scan_library_aux(db_model: &LibraryDBModel, path: &String, parent_folder_id: u32) {
	let mut dirs: Vec<Dir> = Vec::new();
	let mut books: Vec<Book> = Vec::new();
	println!("Scanning {path}");
	for file in fs::read_dir(path).unwrap() {
		let file = file.unwrap();
		let path_buf = file.path();
		let path = path_buf.as_path();

		if file.path().is_dir() {
			dirs.push(create_folder(path, parent_folder_id));
		} else {
			let thumb_dir = constant::DATA_DIR.join(&db_model.uuid).join("thumb");
			let mut book = parse_book(path.to_str().unwrap(), thumb_dir.clone());

			println!("Book: {:?}", book);
			book.folder_id = parent_folder_id;
			books.push(book);
		}
	}
	//Add folders and books to db and get the folders with ids
	dirs = db_model.add_folder_contents_to_db(&dirs, &books, parent_folder_id);
	//Iterate over the found fol
	//dirs.iter().map(|folder|
	//	self.scan_library_aux(&*format!("{}/{}", path, folder.name), folder.id)).collect()
}

impl LibraryDBModel {
	fn add_folder_contents_to_db(&self, folders: &Vec<Dir>, books: &Vec<Book>, parent_folder_id: u32) -> Vec<Dir> {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.add_folders(folders);
		match db_conn.add_books(books) {
			Ok(result) => result,
			Err(err) => {
				println!("Error adding books: {}", err);
				0
			}
		};
		db_conn.get_folders(parent_folder_id)
	}
}

/// Returns a vector of `Book` objects representing the books contained in the folder with the
/// given ID in the library with the specified UUID. This function uses the library's database
/// connection stored in the global `LIBRARY_DB_CONNS` map to retrieve the books.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library to retrieve books from.
/// * `folder_id` - An `u32` representing the ID of the folder to retrieve books from.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// let books = get_books(String::from("12345"), 0);
///
/// for book in books {
///     println!("{}", book.title);
/// }
/// ```
pub fn get_books(model_uuid: String, folder_id: u32) -> Vec<Book> {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let db_conn = model.db_conn.lock().unwrap();
	db_conn.get_books_by_folder(folder_id)
}

/// Returns a vector of `Dir` objects representing the folders contained within the folder with
/// the given ID in the library with the specified UUID. This function uses the library's database
/// connection stored in the global `LIBRARY_DB_CONNS` map to retrieve the folders.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library to retrieve folders from.
/// * `parent_id` - An `u32` representing the ID of the parent folder to retrieve folders from.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// let folders = get_folders(String::from("12345"), 0);
/// ```
pub fn get_folders(model_uuid: String, parent_id: u32) -> Vec<Dir> {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let db_conn = model.db_conn.lock().unwrap();
	db_conn.get_folders(parent_id)
}

/// Returns a `String` representing the path to the cover image for the book with the given UUID in
/// the library with the specified UUID. The cover image is expected to be stored in the `thumb`
/// directory within the library's data directory.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library to retrieve the cover image
///                  from.
/// * `book_uuid` - A `&str` representing the UUID of the book to retrieve the cover image for.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// let cover_path = get_cover_path(String::from("12345"), "67890");
/// ```
pub fn get_cover_path(model_uuid: String, book_uuid: &str) -> String {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let cover_path = constant::DATA_DIR.join(&model.uuid).join("thumb").join(book_uuid);
	cover_path.to_str().unwrap().to_string()
}

/// Closes the database connection for the library with the specified UUID.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library to close the database
///                  connection for.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// close(String::from("12345"));
/// ```
pub fn close(model_uuid: String) {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let db_conn = model.db_conn.lock().unwrap();
	db_conn.close();
}

/// Sets the location and percentage read for a book with the specified UUID in the library with
/// the specified UUID.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library that contains the book to set
///                  the location and percentage read for.
/// * `book_uuid` - A string slice representing the UUID of the book to set the location and
///                 percentage read for.
/// * `location` - A string slice representing the read location (page) of the book.
/// * `percentage` - An unsigned 32-bit integer representing the percentage of the book that has
///                  been read.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Example
///
/// ```
/// set_book_location(String::from("12345"), "67890", "/path/to/book", 75);
/// ```
pub fn set_book_location(model_uuid: String, book_uuid: &str, location: &str, percentage: u32) {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let db_conn = model.db_conn.lock().unwrap();
	db_conn.set_book_location(book_uuid, location, percentage);
}

/// Retrieves the read location and percentage read for a book with the specified UUID in the library
/// with the specified UUID.
///
/// # Arguments
///
/// * `model_uuid` - A `String` representing the UUID of the library that contains the book to get
///                  the location and percentage read for.
/// * `book_uuid` - A string slice representing the UUID of the book to get the location and
///                 percentage read for.
///
/// # Safety
///
/// This function uses `unsafe` code to access the global `LIBRARY_DB_CONNS` map. This is safe
/// as long as the map is only accessed in a thread-safe manner.
///
/// # Returns
///
/// A `ReadPosition` struct containing the location and percentage read for the specified book.
///
/// # Example
///
/// ```
/// let read_position: ReadPosition = get_book_location(String::from("12345"), "67890");
/// println!("The current read location is at {} and has been read to {}%.", read_position.location, read_position.percentage);
/// ```
pub fn get_book_location(model_uuid: String, book_uuid: &str) -> ReadPosition {
	let model = unsafe { LIBRARY_DB_CONNS.get(&model_uuid).unwrap() };
	let db_conn = model.db_conn.lock().unwrap();
	db_conn.get_book_location(book_uuid)
}

pub fn create_folder(path: &Path, folder_id: u32) -> Dir {
	let folder = Dir {
		name: String::from(path.file_name().unwrap().to_str().unwrap()),
		parent_id: folder_id,
		id: 0,
	};
	folder
}

pub fn get_cover_widths() -> Vec<u32> {
	COVER_WIDTHS.to_vec()
}

#[cxx::bridge]
pub mod ffi {
	#[derive(Default, Clone, Debug)]
	pub struct Book {
		pub uuid: String,
		pub name: String,
		pub path: String,
		pub read_location: String,
		pub read_percentage: u32,
		pub title: String,
		pub isbn: String,
		pub folder_id: u32,
	}

	pub struct Dir {
		pub id: u32,
		pub name: String,
		pub parent_id: u32,
	}

	pub struct ReadPosition {
		pub read_location: String,
		pub read_percentage: u32,
	}

	extern "Rust" {
		type LibraryDBModel;
		fn open_library(uuid: &str, path: &str);
		fn scan_library(model_uuid: String);
		fn get_books(model_uuid: String, folder_id: u32) -> Vec<Book>;
		fn get_folders(model_uuid: String, parent_id: u32) -> Vec<Dir>;
		fn get_cover_path(model_uuid: String, book_uuid: &str) -> String;

		fn set_book_location(model_uuid: String, book_uuid: &str, location: &str, percentage: u32);
		fn get_book_location(model_uuid: String, book_uuid: &str) -> ReadPosition;
		pub fn get_cover_widths() -> Vec<u32>;
	}
}
