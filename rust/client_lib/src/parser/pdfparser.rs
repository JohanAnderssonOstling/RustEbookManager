use std::path::PathBuf;
use uuid::Uuid;
use crate::library_model::ffi::Book;
use pdf::file::File as PdfFile;
use pdf::error::PdfError;
use pdf::object::PdfObj;
pub fn parse_pdf(book: Book) -> Book{
    book
}

fn extract_cover(pdf_path: &str) -> Result<(), PdfError>{
    let pdf_file = PdfFile::open(pdf_path)?;
    let cover_obj = pdf_file.pages()?.pages[0].resources()?.get("XObject")?.clone();
    let cover_data = cover_obj.stream_as::<Vec<u8>>()?;
    // Do something with the extracted cover data here
    Ok(())
}