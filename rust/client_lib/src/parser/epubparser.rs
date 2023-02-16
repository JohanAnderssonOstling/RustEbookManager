use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use epub::doc::EpubDoc;
use crate::library_model::ffi::Book;
use uuid::Uuid;
pub fn parse_epub(book: Book, thumb_dir: PathBuf) -> Book{
    let mut epub_book = book.clone();
    let mut epub = EpubDoc::new(book.path).unwrap();
    let cover = epub.get_cover().unwrap();
   // let cover_path = thumb_path.join("orig.jpg");
    //let mut file = fs::File::create(cover_path).unwrap();
    //file.write_all(&cover.0).unwrap();

    epub_book.title = epub.mdata("title").unwrap().to_string();
    return epub_book
}
