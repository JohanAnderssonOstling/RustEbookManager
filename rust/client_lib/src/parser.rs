use std::{fs, thread};
use std::path::PathBuf;
use uuid::Uuid;
use crate::library_model::Book;

pub const COVER_WIDTHS: [u32; 4] = [64, 128, 256, 512];
pub const MAX_HEIGHT_RATION: f32 = 1.6;
pub mod epubparser;
mod pdfparser;

pub fn parse_book(path: &str, thumb_dir: PathBuf) -> Book{
	println!("Parsing book: {}", path);
    let uuid = Uuid::new_v4().to_string();
    let file_name = path.split('/').last().unwrap().to_string();
    let file_extension = file_name.split('.').last().unwrap().to_string();
    let book_thumb_dir = thumb_dir.join(uuid.clone());
    fs::create_dir_all(&book_thumb_dir).unwrap();
    let book =  Book{
        uuid,
        name: file_name,
        path: path.to_string(),
        read_location: "0".to_string(),
        ..Default::default()
    };
    let parsed_book = match file_extension.as_str() {
        "epub" => (epubparser::parse_epub(book, &book_thumb_dir)),
        "pdf" => (pdfparser::parse_pdf(book, &book_thumb_dir)),
        _ => book,
    };
	thread::spawn(|| {
		create_thumbnails(book_thumb_dir);
	});
	parsed_book
}
pub fn create_thumbnails(thumbnail_folder: PathBuf) {
	let orig_img_path: PathBuf = thumbnail_folder.clone().join("orig.jpg");
	println!("Creating thumbnails for: {:?}", orig_img_path);
	let orig_img = match image::open(orig_img_path) {
		Ok(img) => img,
		Err(err) => {
			println!("Error opening image: {}", err);
			return;
		}
	};
	for cover_width in COVER_WIDTHS.iter() {
		let save_path = thumbnail_folder.clone().join(format!("{}.jpg", cover_width));
		let thumbnail = orig_img.thumbnail(*cover_width, (*cover_width as f32 * MAX_HEIGHT_RATION) as u32);
		thumbnail.save(save_path).unwrap();
	}
}
