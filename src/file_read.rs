use crate::abitmap::{ABMP_HEADER_SIZE, AbmpBitmap, AbmpBitmapHeader};

use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

impl AbmpBitmapHeader {
    pub fn read_header_from_file(&mut self, file: &mut File) -> io::Result<()> {
        let mut buf = [0u8; ABMP_HEADER_SIZE as usize];

        file.read_exact(&mut buf)?;

        let u32_at = |offset: usize| -> u32 {
            u32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
        };
        let u16_at = |offset: usize| -> u16 {
            u16::from_le_bytes(buf[offset..offset + 2].try_into().unwrap())
        };

        self.signature = [buf[0], buf[1]];
        self.filesize = u32_at(2);
        self.reserved = u32_at(6);
        self.dataoffset = u32_at(10);

        self.size = u32_at(14);
        self.width = u32_at(18);
        self.height = u32_at(22);
        self.planes = u16_at(26);
        self.bits_per_pixel = u16_at(28);
        self.compression = u32_at(30);
        self.imagesize = u32_at(34);
        self.x_pixels_per_m = u32_at(38);
        self.y_pixels_per_m = u32_at(42);
        self.colors_used = u32_at(46);
        self.important_colors = u32_at(50);

        if &self.signature != b"BM" {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invaild file, found {} and expected 'BM'", std::str::from_utf8(&self.signature).unwrap()),
            ));
        }

        if self.width * self.height * 3 + (self.width % 4) * self.height != self.imagesize && self.imagesize != 0
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "A: This is not a BMP file, B: The file is wrong."
            ));
        }

        Ok(())
    }
}

impl AbmpBitmap {
    pub fn read_pixeldata_from_file(&mut self, file: &mut File) -> io::Result<()> {
        if self.header.compression != 0 {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Compression is not supported"
            ));
        }

        if self.header.bits_per_pixel <= 8 {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Low bits per pixel is not supported"
            ));
        }

        self.pixel_data = vec![0; self.header.imagesize as usize];

        file.seek_relative(self.header.dataoffset as i64)?;

        file.read_exact(&mut self.pixel_data)?;

        Ok(())
    }

    pub fn read_bmp_from_file_direct(&mut self, file: &mut File) -> io::Result<()> {
        let file_start = file.stream_position()?;

        let file_size = file.seek(SeekFrom::End(0))? - file_start;

        if file_size < ABMP_HEADER_SIZE as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size is lower than header size",
            ));
        }

        file.seek(SeekFrom::Start(file_start))?;
        self.header.read_header_from_file(file)?;

        file.seek(SeekFrom::Start(file_start))?;
        self.read_pixeldata_from_file(file)?;

        Ok(())
    }

    pub fn read_bmp_from_filepath_using_directread(&mut self, path: String) -> io::Result<()>
    {
        let mut file = File::open(path)?;

        self.read_bmp_from_file_direct(&mut file)?;

        Ok(())
    }
}
