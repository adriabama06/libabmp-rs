use crate::abitmap::{ABMP_HEADER_SIZE, AbmpBitmap, AbmpBitmapHeader};

use std::{fs::File, io::{self, Seek, SeekFrom, Write}};

impl AbmpBitmapHeader {
    pub fn write_header_to_file(&self, file: &mut File) -> io::Result<()> {
        // Validación básica: firma "BM"
        if &self.signature != b"BM" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invaild file, found {} and expected 'BM'", std::str::from_utf8(&self.signature).unwrap()),
            ));
        }

        if self.width * self.height * 3 + (self.width % 4) * self.height != self.imagesize && self.imagesize != 0 // It is valid to set imagesize = 0 if compression = 0
        {
            // A: This is not a BMP file, B: The file is wrong.
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "A: This is not a BMP file, B: The file is wrong."
            ));
        }

        let mut buf = [0u8; ABMP_HEADER_SIZE as usize];

        buf[0..2].copy_from_slice(&self.signature);
        buf[2..6].copy_from_slice(&self.filesize.to_le_bytes());
        buf[6..10].copy_from_slice(&self.reserved.to_le_bytes());
        buf[10..14].copy_from_slice(&self.dataoffset.to_le_bytes());

        buf[14..18].copy_from_slice(&self.size.to_le_bytes());
        buf[18..22].copy_from_slice(&self.width.to_le_bytes());
        buf[22..26].copy_from_slice(&self.height.to_le_bytes());
        buf[26..28].copy_from_slice(&self.planes.to_le_bytes());
        buf[28..30].copy_from_slice(&self.bits_per_pixel.to_le_bytes());
        buf[30..34].copy_from_slice(&self.compression.to_le_bytes());
        buf[34..38].copy_from_slice(&self.imagesize.to_le_bytes());
        buf[38..42].copy_from_slice(&self.x_pixels_per_m.to_le_bytes());
        buf[42..46].copy_from_slice(&self.y_pixels_per_m.to_le_bytes());
        buf[46..50].copy_from_slice(&self.colors_used.to_le_bytes());
        buf[50..54].copy_from_slice(&self.important_colors.to_le_bytes());

        file.write_all(&buf)?;

        Ok(())
    }
}

impl AbmpBitmap {
    pub fn write_pixeldata_to_file(&self, file: &mut File) -> io::Result<()> {
        file.write_all(&self.pixel_data)?;

        Ok(())
    }

    pub fn write_bmp_to_file_direct(&self, file: &mut File) -> io::Result<()> {
        let file_start = file.stream_position()?;


        self.header.write_header_to_file(file)?;

        file.seek(SeekFrom::Start(file_start + self.header.dataoffset as u64))?;

        self.write_pixeldata_to_file(file)?;

        Ok(())
    }

    pub fn write_bmp_to_filepath_using_directwrite(&self, path: String) -> io::Result<()>
    {
        // Open file
        let mut file = File::create(path)?;

        self.write_bmp_to_file_direct(&mut file)?;

        Ok(())
    }
}
