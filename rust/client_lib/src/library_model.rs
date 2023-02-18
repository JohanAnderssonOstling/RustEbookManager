use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use image::DynamicImage;

use library_db::LibraryDB;

use crate::constant;
use crate::db::library_db;
use crate::library_model::ffi::*;
use crate::parser::parse_book;

pub struct LibraryDBModel {
	pub uuid: String,
	pub path: String,
	pub db_conn: Mutex<LibraryDB>,
}

pub const COVER_WIDTHS: [u32; 4] = [64, 128, 256, 512];
pub const MAX_HEIGHT_RATION: f32 = 1.6;

pub fn open_library(uuid: &str, path: &str) -> Vec<LibraryDBModel> {
	println!("Opening library: {}", uuid);
	let library = LibraryDBModel {
		uuid: String::from(uuid),
		path: String::from(path),
		db_conn: Mutex::new(LibraryDB::open(uuid)),
	};
	vec![library] //Return vec for C++ compatibility
}

fn create_thumb_dir(uuid: &str) -> PathBuf {
	let thumb_dir = constant::DATA_DIR.join(uuid).join("thumb");
	fs::create_dir_all(&thumb_dir).unwrap();
	thumb_dir
}

impl LibraryDBModel {
	fn add_folder_contents_to_db(&self, folders: &Vec<Dir>, books: &Vec<Book>, parent_folder_id: u32) -> Vec<Dir> {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.add_folders(&folders);
		let add_result = match db_conn.add_books(&books) {
			Ok(result) => result,
			Err(err) => {
				println!("Error adding books: {}", err);
				0
			}
		};
		db_conn.get_folders(parent_folder_id)
	}
	pub fn scan_library(&self) {
		self.scan_library_aux(&self.path, 0);
	}

	fn scan_library_aux(&self, path: &str, parent_folder_id: u32) {
		let mut dirs: Vec<Dir> = Vec::new();
		let mut books: Vec<Book> = Vec::new();
		println!("Scanning {}", path);
		for file in fs::read_dir(path).unwrap() {
			let file = file.unwrap();
			let path_buf = file.path();
			let path = path_buf.as_path();

			if file.path().is_dir() {
				dirs.push(create_folder(path, parent_folder_id));
			} else {
				let thumb_dir = create_thumb_dir(&self.uuid);
				let mut book = parse_book(path.to_str().unwrap(), thumb_dir.clone());
				book.folder_id = parent_folder_id;
				self.create_thumbnails(thumb_dir.join(&book.uuid));
				books.push(book);
			}
		}
		//Add folders and books to db and get the folders with ids
		dirs = self.add_folder_contents_to_db(&dirs, &books, parent_folder_id);
		//Iterate over the found fol
		dirs.iter().map(|folder|
			self.scan_library_aux(&*format!("{}/{}", path, folder.name), folder.id)).collect()
	}

	pub fn create_thumbnails(&self, thumbnail_folder: PathBuf) {
		let orig_img_path: PathBuf = thumbnail_folder.clone().join("orig.jpg");
		let orig_img = match image::open(orig_img_path) {
			Ok(img) => img,
			Err(err) => {
				println!("Error opening image: {}", err);
				return;
			}
		};
		for cover_width in COVER_WIDTHS.iter() {
			let save_path = thumbnail_folder.clone().join(format!("{}.jpg", cover_width));
			let thumbnail = orig_img.thumbnail(*cover_width, ((*cover_width as f32 * MAX_HEIGHT_RATION) as u32));
			thumbnail.save(save_path).unwrap();
		}
	}

	pub fn get_books(&self, folder_id: u32) -> Vec<Book> {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.get_books_by_folder(folder_id)
	}

	pub fn get_folders(&self, parent_id: u32) -> Vec<Dir> {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.get_folders(parent_id)
	}

	pub fn get_cover_path(&self, book_uuid: &str) -> String {
		let cover_path = constant::DATA_DIR.join(&self.uuid).join("thumb").join(book_uuid);
		cover_path.to_str().unwrap().to_string()
	}

	pub fn close(&self) {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.close();
	}

	pub fn set_book_location(&self, book_uuid: &str, location: &str, percentage: u32) {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.set_book_location(book_uuid, location, percentage);
	}

	pub fn get_book_location(&self, book_uuid: &str) -> ReadPosition {
		let db_conn = self.db_conn.lock().unwrap();
		db_conn.get_book_location(book_uuid)
	}
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
	#[derive(Default, Clone)]
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
		fn open_library(uuid: &str, path: &str) -> Vec<LibraryDBModel>;
		fn scan_library(&self);
		fn get_books(&self, folder_id: u32) -> Vec<Book>;
		fn get_folders(&self, parent_id: u32) -> Vec<Dir>;
		fn get_cover_path(&self, book_uuid: &str) -> String;

		fn set_book_location(&self, book_uuid: &str, location: &str, percentage: u32);
		fn get_book_location(&self, book_uuid: &str) -> ReadPosition;
		pub fn get_cover_widths() -> Vec<u32>;
	}
}
