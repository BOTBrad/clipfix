use std::io::Cursor;

use clipboard_win::{formats, get_clipboard, set_clipboard};
use image::io::Reader as ImageReader;
use image::ImageFormat;

fn main() {
    if let Err(e) = convert() {
        println!("{:?}", e);
    }
}

#[derive(Debug, Clone)]
enum Error {
    Clipboard(String),
    Image(&'static str),
}

fn convert() -> Result<(), Error> {
    let bmp: Vec<u8> =
        get_clipboard(formats::Bitmap)
        .map_err(|e| Error::Clipboard(e.to_string()))?;

    let dynamic = ImageReader::new(Cursor::new(bmp.as_slice()))
        .with_guessed_format()
        .map_err(|_| Error::Image("dynamic decode failed"))?;

    let img2 = dynamic.decode().expect("couldn't decode");

    let mut c = Cursor::new(Vec::<u8>::new());
    img2.write_to(&mut c, ImageFormat::Bmp)
        .expect("faild write vec");

    let out = c.into_inner();

    set_clipboard(formats::Bitmap, out.as_slice()).expect("write");

    Ok(())
}
