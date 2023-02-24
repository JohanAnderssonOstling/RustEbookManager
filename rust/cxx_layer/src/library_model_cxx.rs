use crate::library_model_cxx::ffi::*;

pub fn open_library(uuid: &str, path: &str) {

}

pub fn scan_library(model_uuid: String) {

}
pub fn get_books(model_uuid: String, folder_id: u32) -> Vec<Book> {
	vec![]
}
pub fn get_folders(model_uuid: String, parent_id: u32) -> Vec<Dir> {
	vec![]
}
pub fn get_cover_path(model_uuid: String, book_uuid: &str) -> String {
	String::from("")
}

pub fn set_book_location(model_uuid: String, book_uuid: &str, location: &str, percentage: u32) {

}

pub fn get_book_location(model_uuid: String, book_uuid: &str) -> ReadPosition {
	ReadPosition{
		location: String::from(""),
		percentage: 0,
	}
}

pub fn get_cover_widths() -> Vec<u32> {
	vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]
}


#[cxx::bridge]
pub mod ffi {
	pub struct Book{
		uuid: String,
		title: String,
	}
	pub struct Dir{
		uuid: String,
		name: String,
	}
	pub struct ReadPosition{
		pub(crate) location: String,
		pub(crate) percentage: u32,
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
