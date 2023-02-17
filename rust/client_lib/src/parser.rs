
use uuid::Uuid;
use crate::library_model::ffi::Book;
use std::path::PathBuf;

pub mod epubparser;
mod pdfparser;

pub fn parse_book(path: &str, thumb_dir: PathBuf) -> Book{
    let uuid = Uuid::new_v4().to_string();
    let file_name = path.split("/").last().unwrap().to_string();
    let file_extension = file_name.split(".").last().unwrap().to_string();
    let mut book =  Book{
        uuid,
        name: file_name,
        path: path.to_string(),
        read_location: "0".to_string(),
        ..Default::default()
    };
    match file_extension.as_str() {
        "epub" => epubparser::parse_epub(book, thumb_dir),
        "pdf" => pdfparser::parse_pdf(book, thumb_dir),
        _ => book,
    }
}