pub const ABMP_HEADER_SIZE: u32 = 54;

#[derive(Debug, Default, Clone, Copy)]
pub struct AbmpBitmapHeader {
    pub signature: [u8; 2],
    pub filesize: u32,
    pub reserved: u32,
    pub dataoffset: u32,

    pub size: u32,
    pub width: u32,
    pub height: u32,
    pub planes: u16,
    pub bits_per_pixel: u16,
    pub compression: u32,
    pub imagesize: u32,
    pub y_pixels_per_m: u32,
    pub x_pixels_per_m: u32,
    pub colors_used: u32,
    pub important_colors: u32
}

impl AbmpBitmapHeader {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Default)]
pub struct AbmpBitmap {
    pub header: AbmpBitmapHeader,
    pub color_table: Vec<u8>,
    pub pixel_data: Vec<u8>
}

impl AbmpBitmap {
    pub fn new(header: AbmpBitmapHeader) -> Self {
        let filesize: usize = header.filesize as usize;
        let dataoffset: usize = header.dataoffset as usize;
        AbmpBitmap {
            header,
            color_table: vec![],
            pixel_data: vec![0; filesize.saturating_sub(dataoffset)],
        }
    }
}

#[derive(Debug)]
pub enum Error {
    DataIsSmallerThanHeader,
    IsNotABmpFile,
    BmpDataIsCorrupted,
    CompressionIsNotSupported,
    LowBitsPerPixelIsNotSupported,
    DataIsSmallerThanImagesize
}
