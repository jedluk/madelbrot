use std::fmt;
use std::mem::replace;

#[path = "./file_headers.rs"]
mod file_headers;

pub struct Bitmap {
    width: i32,
    height: i32,
    pixels: Box<Vec<u8>>,

}

impl Bitmap {
    pub fn new(width: Option<i32>, height: Option<i32>) -> Bitmap {
        let width: i32 = width.unwrap_or(0);
        let height: i32 = height.unwrap_or(0);
        let pixels = Box::new(vec![0u8; (3 * width * height) as usize]);

        Bitmap {
            width,
            height,
            pixels,
        }
    }

    fn setPixel(&self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        // let pix = self.pixels;
        // replace()
    }

    fn write(&self, name: &String) -> bool {
        let fileHeader = file_headers::BitmapInfoHeader::from_defaults(self.width, self.height);

        true
    }
}

impl fmt::Display for Bitmap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "height:{} width: {}", self.height, self.width)
    }
}