use image::{io::Reader, ImageError};
use std::{ffi::OsStr, io::Cursor, path::Path};

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

/// Can the path trigger a thumbnail to be created.
pub fn can_have_thumbnail(path: &str) -> bool {
    let extension = Path::new(&path)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or("");

    IMAGE_EXTS
        .into_iter()
        .any(|ext| ext.eq(&extension.to_uppercase()))
}
