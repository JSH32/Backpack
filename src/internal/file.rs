use image::ImageError;
use std::io::Cursor;

/// Image extensions
pub const IMAGE_EXTS: &'static [&'static str] =
    &["PNG", "JPG", "JPEG", "GIF", "WEBP", "JFIF", "PJPEG", "PJP"];

pub fn get_thumbnail_image(bytes: &[u8]) -> Result<Vec<u8>, ImageError> {
    let mut buf = Vec::new();

    image::load_from_memory(&bytes)?
        .thumbnail(500, 500)
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)?;

    Ok(buf)
}
