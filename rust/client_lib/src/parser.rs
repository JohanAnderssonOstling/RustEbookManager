use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use crate::library_model::ffi::Book;

pub mod epubparser;
mod pdfparser;

pub fn parse_book(path: &str, thumb_dir: PathBuf) -> Book{
    let uuid = Uuid::new_v4().to_string();
    let file_name = path.split("/").last().unwrap().to_string();
    let file_extension = file_name.split(".").last().unwrap().to_string();
    let mut book_thumb_dir = thumb_dir.join(uuid.clone());
    fs::create_dir_all(&book_thumb_dir).unwrap();
    let mut book =  Book{
        uuid,
        name: file_name,
        path: path.to_string(),
        ..Default::default()
    };
    match file_extension.as_str() {
        "epub" => epubparser::parse_epub(book, book_thumb_dir),
        "pdf" => pdfparser::parse_pdf(book, book_thumb_dir),
        _ => book,
    }
}