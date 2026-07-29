use crate::abitmap::{self, AbmpBitmap, AbmpBitmapHeader};

use std::{fs::File, io::{self, Write}};

impl AbmpBitmapHeader {
    pub fn write_header_to_memory(&mut self, data: &mut Vec<u8>) -> Result<(), abitmap::Error> {
        if &self.signature != b"BM" {
            return Err(abitmap::Error::IsNotABmpFile);
        }

        if self.width * self.height * 3 + (self.width % 4) * self.height != self.imagesize && self.imagesize != 0
        {
            return Err(abitmap::Error::BmpDataIsCorrupted);
        }

        data[0..2].copy_from_slice(&self.signature);
        data[2..6].copy_from_slice(&self.filesize.to_le_bytes());
        data[6..10].copy_from_slice(&self.reserved.to_le_bytes());
        data[10..14].copy_from_slice(&self.dataoffset.to_le_bytes());

        data[14..18].copy_from_slice(&self.size.to_le_bytes());
        data[18..22].copy_from_slice(&self.width.to_le_bytes());
        data[22..26].copy_from_slice(&self.height.to_le_bytes());
        data[26..28].copy_from_slice(&self.planes.to_le_bytes());
        data[28..30].copy_from_slice(&self.bits_per_pixel.to_le_bytes());
        data[30..34].copy_from_slice(&self.compression.to_le_bytes());
        data[34..38].copy_from_slice(&self.imagesize.to_le_bytes());
        data[38..42].copy_from_slice(&self.x_pixels_per_m.to_le_bytes());
        data[42..46].copy_from_slice(&self.y_pixels_per_m.to_le_bytes());
        data[46..50].copy_from_slice(&self.colors_used.to_le_bytes());
        data[50..54].copy_from_slice(&self.important_colors.to_le_bytes());

        Ok(())
    }
}

impl AbmpBitmap {
    pub fn write_pixeldata_to_memory(&mut self, data: &mut Vec<u8>) -> Result<(), abitmap::Error> {
        let start: usize = self.header.dataoffset as usize;
        let end: usize = start + self.header.imagesize as usize;

        if self.pixel_data.len() != self.header.imagesize as usize {
            return Err(abitmap::Error::SizeMismatch);
        }
        if end > data.len() {
            return Err(abitmap::Error::OutOfBounds);
        }

        data[start..end].copy_from_slice(&self.pixel_data);

        Ok(())
    }

    pub fn write_bmp_to_file_memory(&mut self, file: &mut File) -> io::Result<()> {
        let mut file_data: Vec<u8> = vec![0; (self.header.dataoffset + self.header.imagesize) as usize];

        self.header.write_header_to_memory(&mut file_data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;

        self.write_pixeldata_to_memory(&mut file_data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;

        file.write_all(&file_data)?;

        Ok(())
    }

    pub fn write_bmp_to_filepath_using_memory(&mut self, path: String) -> io::Result<()>
    {
        let mut file = File::create(path)?;

        self.write_bmp_to_file_memory(&mut file)?;

        Ok(())
    }
}
