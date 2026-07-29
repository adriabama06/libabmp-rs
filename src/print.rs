use crate::{abitmap::AbmpBitmap, get::abmp_get_pixel_position_from_top_left};

impl AbmpBitmap {
    #[allow(non_snake_case)]
    pub fn draw(&mut self, x: u32, y: u32, R: u8, G: u8, B: u8) {
        let pos: usize = (
            abmp_get_pixel_position_from_top_left(&self.header, x % self.header.width, y % self.header.height) % self.header.imagesize
        ) as usize;

        self.pixel_data[pos] = B;
        self.pixel_data[pos + 1] = G;
        self.pixel_data[pos + 2] = R;
    }
}
