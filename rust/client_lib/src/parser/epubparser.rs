use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use epub::doc::EpubDoc;
use crate::library_model::ffi::Book;
use uuid::Uuid;
pub fn parse_epub(path: &str, thumb_dir: &PathBuf) -> Book{
    let mut epub = EpubDoc::new(path).unwrap();
    let cover = epub.get_cover().unwrap();
    let uuid = Uuid::new_v4().to_string();
    let thumb_path = thumb_dir.join(&uuid);
    fs::create_dir_all(&thumb_path).unwrap();
    let cover_path = thumb_path.join("orig.jpg");
    let mut file = fs::File::create(cover_path).unwrap();
    file.write_all(&cover.0).unwrap();

    Book {
        uuid,
        name: path.split("/").last().unwrap().to_string(),
        path: path.to_string(),
        title: epub.mdata("title").unwrap().to_string(),

        ..Default::default()
    }
}
