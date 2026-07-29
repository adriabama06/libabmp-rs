use crate::abitmap::{ABMP_HEADER_SIZE, AbmpBitmap, AbmpBitmapHeader};

impl AbmpBitmap {
    pub fn create(width: u32, height: u32) -> Self {
        let mut header: AbmpBitmapHeader = AbmpBitmapHeader::new();

        header.signature[0] = 'B' as u8;
        header.signature[1] = 'M' as u8;

        header.dataoffset = ABMP_HEADER_SIZE;

        header.width = width;
        header.height = height;


        header.bits_per_pixel = 24;
        header.planes = 1;
        header.size = 40;

        let padding: u32 = width % 4;

        /*   imagesize = width*height*3(colors BGR) + padding * height   */
        header.imagesize = width * height * 3 + padding * height;

        header.filesize = ABMP_HEADER_SIZE + header.imagesize;

        AbmpBitmap::new(header)
    }
}
