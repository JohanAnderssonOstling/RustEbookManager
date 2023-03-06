use crate::home_model_cxx::ffi::*;
use client_lib::home_model;
pub fn add_library(name: &str, path: &str) -> Library {
	unsafe { std::mem::transmute(home_model::add_library(name, path)) }
}
pub fn get_library_list() -> Vec<Library> {
	unsafe { std::mem::transmute(home_model::get_library_list()) }
}
pub fn delete_library(library_uuid: &str) {
	home_model::delete_library(library_uuid);
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
