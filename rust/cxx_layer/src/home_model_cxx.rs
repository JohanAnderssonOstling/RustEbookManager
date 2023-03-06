use crate::home_model_cxx::ffi::*;

pub fn add_library(name: &str, path: &str) -> Library {
	Library {
		name: String::from(name),
		uuid: String::from(""),
		path: String::from(""),
	}
}
pub fn get_library_list() -> Vec<Library> {
	vec![]
}
pub fn delete_library(library_uuid: &str) {

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
