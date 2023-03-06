use client_lib::library_model;
use crate::library_model_cxx::ffi::*;


pub fn open_library(uuid: &str, path: &str) {
	library_model::open_library(uuid, path);
}

pub fn scan_library(model_uuid: String) {
	library_model::scan_library(model_uuid);
}
pub fn get_books(model_uuid: String, folder_id: u32) -> Vec<Book> {
	let books = library_model::get_books(model_uuid, folder_id);
	let mut result = Vec::new();
	for book in books {
		let book = Book {
			uuid: book.uuid,
			name: book.name,
			path: book.path,
			read_location: book.read_location,
			read_percentage: book.read_percentage,
			title: book.title,
			isbn: book.isbn,
			folder_id: book.folder_id,
		};
		result.push(book);
	}
	result

}
pub fn get_folders(model_uuid: String, parent_id: u32) -> Vec<Dir> {
	let dirs = library_model::get_folders(model_uuid, parent_id);
	let mut result = Vec::new();
	for dir in dirs {
		let dir = Dir {
			id: dir.id,
			name: dir.name,
			parent_id: dir.parent_id,
		};
		result.push(dir);
	}
	result
}
pub fn get_cover_path(model_uuid: String, book_uuid: &str) -> String {
	library_model::get_cover_path(model_uuid, book_uuid)
}

pub fn set_book_location(model_uuid: String, book_uuid: &str, location: &str, percentage: u32) {
	library_model::set_book_location(model_uuid, book_uuid, location, percentage);
}

pub fn get_book_location(model_uuid: String, book_uuid: &str) -> ReadPosition {
	let read_position = library_model::get_book_location(model_uuid, book_uuid);
	ReadPosition {
		read_location: read_position.read_location,
		read_percentage: read_position.read_percentage,
	}
}

pub fn get_cover_widths() -> Vec<u32> {
	library_model::get_cover_widths()
}


#[cxx::bridge]
pub mod ffi {
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
		//type LibraryDBModel;
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
