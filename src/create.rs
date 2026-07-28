use crate::abitmap::{ABMP_HEADER_SIZE, AbmpBitmap};

impl AbmpBitmap {
    pub fn create(width: u32, height: u32) -> Self {
        let mut bitmap: AbmpBitmap = AbmpBitmap::default();

        bitmap.header.signature[0] = 'B' as u8;
        bitmap.header.signature[1] = 'M' as u8;

        bitmap.header.dataoffset = ABMP_HEADER_SIZE;

        bitmap.header.width = width;
        bitmap.header.height = height;


        bitmap.header.bits_per_pixel = 24;
        bitmap.header.planes = 1;
        bitmap.header.size = 40;

        let padding: u32 = width % 4;

        /*   imagesize = width*height*3(colors BGR) + padding * height   */
        bitmap.header.imagesize = width * height * 3 + padding * height;

        bitmap.header.filesize = ABMP_HEADER_SIZE + bitmap.header.imagesize;

        bitmap.pixel_data = vec![255u8; bitmap.header.imagesize as usize];

        return bitmap;
    }
}
