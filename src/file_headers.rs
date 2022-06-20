use packed_struct::prelude::*;

pub struct BitmapInfoHeader {
    header_size: i32,
    width: i32,
    height: i32,
    bits_per_pixel: i16,
    compression: i32,
    data_size: i32,
    horizontal_resolution: i32,
    vertical_resolution: i32,
    colors: i32,
    important_colors: i32,
}

impl Default for BitmapInfoHeader {
    fn default() -> BitmapInfoHeader {
        BitmapInfoHeader {
            header_size: 40,
            width: 1280,
            height: 720,
            bits_per_pixel: 24,
            compression: 0,
            data_size: 0,
            horizontal_resolution: 2400,
            vertical_resolution: 2400,
            colors: 0,
            important_colors: 0,
        }
    }
}

impl BitmapInfoHeader {
    pub fn from_defaults(width: i32, height: i32) -> BitmapInfoHeader {
        BitmapInfoHeader {
            width,
            height,
            ..Default::default()
        }
    }
}

pub struct BitmapFileHeader {
    header: [char; 2],
    file_size: i32,
    reserved: i32,
    data_offset: i32,
}

impl BitmapFileHeader {
    pub fn new(file_size: i32, data_offset: i32) -> BitmapFileHeader {
        BitmapFileHeader {
            header: ['B', 'M'],
            file_size,
            data_offset,
            reserved: 0,
        }
    }
}