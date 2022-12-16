use std::io::Cursor;

use clipboard_win::{formats, get_clipboard, raw, set_clipboard, Clipboard};
use image::io::Reader as ImageReader;
use image::ImageFormat;

fn main() {
    // let text = "my sample ><";

    println!("start");
    let bmp: Vec<u8> = get_clipboard(formats::Bitmap).expect("bmp");
    let mut bmp2 = bmp.clone();
    bmp2.resize(20, 0);
    println!("{:?}", bmp2);

    let img2 = ImageReader::new(Cursor::new(bmp.as_slice()))
        .with_guessed_format()
        .expect("not an image")
        .decode()
        .expect("couldn't decode");

    let mut c = Cursor::new(Vec::<u8>::new());
    img2.write_to(&mut c, ImageFormat::Bmp)
        .expect("faild write vec");

    let out = c.into_inner();
    let mut out2 = out.clone();
    out2.resize(20, 0);
    println!("{:?}", out2);

    set_clipboard(formats::Bitmap, out.as_slice()).expect("write");
    println!("end");
}
