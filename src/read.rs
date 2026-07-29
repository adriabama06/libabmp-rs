use crate::abitmap::{self, ABMP_HEADER_SIZE, AbmpBitmap, AbmpBitmapHeader};

use std::{fs::File, io::{self, Read, Seek, SeekFrom}};

impl AbmpBitmapHeader {
    pub fn read_header_from_memory(&mut self, data: &Vec<u8>) -> Result<(), abitmap::Error> {
        if data.len() < ABMP_HEADER_SIZE as usize {
            return Err(abitmap::Error::DataIsSmallerThanHeader);
        }

        let u32_at = |offset: usize| -> u32 {
            u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
        };
        let u16_at = |offset: usize| -> u16 {
            u16::from_le_bytes(data[offset..offset + 2].try_into().unwrap())
        };

        self.signature = [data[0], data[1]];
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
            return Err(abitmap::Error::IsNotABmpFile);
        }

        if self.width * self.height * 3 + (self.width % 4) * self.height != self.imagesize && self.imagesize != 0
        {
            return Err(abitmap::Error::BmpDataIsCorrupted);
        }

        Ok(())
    }
}

impl AbmpBitmap {
    pub fn read_pixeldata_from_memory(&mut self, data: &Vec<u8>) -> Result<(), abitmap::Error> {
        if self.header.compression != 0 {
            return Err(abitmap::Error::CompressionIsNotSupported);
        }

        if self.header.bits_per_pixel <= 8 {
            return Err(abitmap::Error::LowBitsPerPixelIsNotSupported);
        }

        self.pixel_data = vec![0; self.header.imagesize as usize];

        let start: usize = self.header.dataoffset as usize;
        let end: usize = start + self.header.imagesize as usize;

        if end > data.len() {
            return Err(abitmap::Error::DataIsSmallerThanImagesize);
        }

        self.pixel_data.copy_from_slice(&data[start..end]);

        Ok(())
    }

    pub fn read_bmp_from_file_memory(&mut self, file: &mut File) -> io::Result<()> {
        let file_start = file.stream_position()?;

        let file_size = file.seek(SeekFrom::End(0))? - file_start;

        if file_size < ABMP_HEADER_SIZE as u64 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "File size is lower than header size",
            ));
        }

        file.seek(SeekFrom::Start(file_start))?;

        let mut data: Vec<u8> = Vec::new();
        
        file.read_to_end(&mut data)?;

        self.header.read_header_from_memory(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;

        self.read_pixeldata_from_memory(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{:?}", e)))?;

        Ok(())
    }

    pub fn read_bmp_from_filepath_using_memory(&mut self, path: String) -> io::Result<()>
    {
        let mut file = File::open(path)?;

        self.read_bmp_from_file_memory(&mut file)?;

        Ok(())
    }
}
