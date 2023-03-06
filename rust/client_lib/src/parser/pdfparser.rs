use std::fs;
use std::io::Write;
use std::path::PathBuf;
use pdf::enc::StreamFilter;

use crate::library_model::Book;
use pdf::error::PdfError;

use pdf::file::File as PdfFile;
use pdf::object::*;


pub fn print_meta(path: &str) -> Result<(), PdfError> {
    let file = PdfFile::open(&path).unwrap();
    if let Some(ref info) = file.trailer.info_dict {
        info.iter()
            .filter(|(_, primitive)| primitive.to_string_lossy().is_ok())
            .for_each(|(key, value)| {
                eprintln!("{:>15}: {}", key, value.to_string_lossy().unwrap());
            });
    }

    if let Some(ref forms) = file.get_root().forms {
        for field in forms.fields.iter() {
            print_field(field, &file);
        }
    }

    Ok(())
}

fn print_field(field: &FieldDictionary, resolve: &impl Resolve) {
    if field.typ == Some(FieldType::Signature) {
        println!("{:?}", field);
    }
    for &kid in field.kids.iter() {
        let child = resolve.get(kid).unwrap();
        print_field(&child, resolve);
    }
}


pub fn parse_pdf(book: Book, thumb_dir : &PathBuf) -> Book{
    let result = (get_cover(&book.path, thumb_dir));
    let handled = match result{
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    };

    book
}

pub fn get_cover(pdf_path: &str, thumb_dir: &PathBuf) -> Result<(), PdfError> {
    let file = PdfFile::open(pdf_path)?;
    let first_page = file.get_page(0)?;
    let resources = first_page.resources()?;
    let mut images: Vec<_> = vec![];

    images.extend(resources.xobjects.iter()
        .map(|(_name, &r)| file.get(r).unwrap())
        .filter(|o| matches!(**o, XObject::Image(_)))
    );

    for (i, o) in images.iter().enumerate() {
        let img = match **o {
            XObject::Image(ref im) => im,
            _ => continue
        };
        let (data, filter) = img.raw_image_data(&file)?;
        let ext = match filter {
            Some(StreamFilter::DCTDecode(_)) => "jpeg",
            Some(StreamFilter::JBIG2Decode) => "jbig2",
            Some(StreamFilter::JPXDecode) => "jp2k",
            _ => continue,
        };

        let fname = format!("extracted_image_{}.{}", i, ext);
        let cover_path = thumb_dir.join("orig.jpg");
        let outputpath = cover_path.to_str().unwrap();
        println!("Writing cover_path: {}" , outputpath);
        let mut file = fs::File::create(cover_path).unwrap();
        file.write_all(&*data).unwrap();
        println!("Wrote file {}", fname);
    }
    Ok(())

}
