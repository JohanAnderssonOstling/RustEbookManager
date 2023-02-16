
use pdf::enc::StreamFilter;

use crate::library_model::ffi::Book;
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


pub fn parse_pdf(book: Book) -> Book{
    get_cover(&book.path);
    book
}

pub fn get_cover(pdf_path: &str){
    let file = PdfFile::open(pdf_path).unwrap();
    let first_page = file.get_page(0).unwrap();
    let resources = first_page.resources().unwrap();
    let mut images: Vec<_> = vec![];
    images.extend(resources.xobjects.iter().map(|(_name, &r)| file.get(r).unwrap())
        .filter(|o| matches!(**o, XObject::Image(_)))
    );

    for (i, o) in images.iter().enumerate() {
        let img = match **o {
            XObject::Image(ref im) => im,
            _ => continue
        };
        let (data, filter) = img.raw_image_data(&file).unwrap();
        let ext = match filter {
            Some(StreamFilter::DCTDecode(_)) => "jpeg",
            Some(StreamFilter::JBIG2Decode) => "jbig2",
            Some(StreamFilter::JPXDecode) => "jp2k",
            _ => continue,
        };

        let fname = format!("extracted_image_{}.{}", i, ext);

        println!("Wrote file {}", fname);
    }

}
