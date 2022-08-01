use image::{io::Reader, ImageError};
use std::io::Cursor;

/// Image extensions
pub const IMAGE_EXTS: &'static [&'static str] =
    &["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"];

pub fn get_thumbnail_image(bytes: &Vec<u8>) -> Result<Vec<u8>, ImageError> {
    let mut buf = Vec::new();

    Reader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?
        .thumbnail(500, 500)
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;

    Ok(buf)
}
