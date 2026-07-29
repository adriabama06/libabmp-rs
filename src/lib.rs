pub mod abitmap;
pub mod get;
pub mod create;
pub mod file_read;
pub mod file_write;
pub mod read;
pub mod write;
pub mod print;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::abitmap::{AbmpBitmap, AbmpBitmapHeader};
    use crate::get::abmp_get_pixel_position_from_top_left;
    use std::env;

    /// Returns the path to the samples directory relative to the crate root.
    fn samples_path(name: &str) -> String {
        format!("samples/{}", name)
    }

    /// Returns a temp file path that is unique per test name.
    fn temp_path(name: &str) -> String {
        let mut path = env::temp_dir();
        path.push(format!("libabmp_rs_test_{}", name));
        path.to_str().unwrap().to_string()
    }

    /// Reads an AbmpBitmap from a file path.
    fn read_bmp(path: &str) -> AbmpBitmap {
        let mut bitmap = AbmpBitmap::default();
        bitmap.file_read_file(path.to_string()).unwrap();
        bitmap
    }

    /// Asserts that a pixel at (x, y) in the bitmap has the expected BGR values.
    fn assert_pixel(bitmap: &AbmpBitmap, x: u32, y: u32, expected_b: u8, expected_g: u8, expected_r: u8) {
        let pos = abmp_get_pixel_position_from_top_left(&bitmap.header, x, y) as usize;
        assert_eq!(bitmap.pixel_data[pos], expected_b, "Blue channel mismatch at ({}, {})", x, y);
        assert_eq!(bitmap.pixel_data[pos + 1], expected_g, "Green channel mismatch at ({}, {})", x, y);
        assert_eq!(bitmap.pixel_data[pos + 2], expected_r, "Red channel mismatch at ({}, {})", x, y);
    }

    // -------------------------------------------------------------------------
    // Test: default_header_creation
    // Description: Verifies that AbmpBitmapHeader::default() produces a
    //              header with all fields set to zero, as expected for a
    //              newly constructed default header.
    // What it does: Creates a default AbmpBitmapHeader and inspects its fields.
    // What is expected: All header fields (signature, filesize, width, height,
    //                   etc.) should be zero. The signature is not "BM", so
    //                   it is not yet a valid BMP header.
    // -------------------------------------------------------------------------
    #[test]
    fn test_default_header_creation() {
        let header = AbmpBitmapHeader::default();

        assert_eq!(header.signature, [0u8, 0], "Signature bytes should be zero for a default header");
        assert_eq!(header.filesize, 0, "File size should be zero for a default header");
        assert_eq!(header.width, 0, "Width should be zero for a default header");
        assert_eq!(header.height, 0, "Height should be zero for a default header");
        assert_eq!(header.bits_per_pixel, 0, "Bits per pixel should be zero for a default header");
        assert_eq!(header.dataoffset, 0, "Data offset should be zero for a default header");
        assert_eq!(header.imagesize, 0, "Image size should be zero for a default header");
    }

    // -------------------------------------------------------------------------
    // Test: read_square_bmp_and_modify_pixel
    // Description: Reads an existing BMP file (square.bmp), modifies a single
    //              pixel at coordinate (2,2) to pure red, writes the result to
    //              a temp file, and then reads it back to verify the change.
    // What it does:
    //   1. Opens samples/square.bmp and deserializes it into an AbmpBitmap.
    //   2. Computes the byte offset of pixel (2,2) using
    //      abmp_get_pixel_position_from_top_left.
    //   3. Sets B=0, G=0, R=255 (pure red in BGR order) at that offset.
    //   4. Writes the modified bitmap to a temp file.
    //   5. Reads the temp file back and asserts pixel (2,2) is pure red.
    // What is expected: The file read succeeds, the pixel at (2,2) is exactly
    //                   (B=0, G=0, R=255), and the written file is a valid
    //                   BMP that can be read back with the same pixel value.
    // -------------------------------------------------------------------------
    #[test]
    fn test_read_square_bmp_and_modify_pixel() {
        let path = samples_path("square.bmp");
        let mut bitmap = AbmpBitmap::default();
        bitmap.file_read_file(path).unwrap();

        // Verify the square.bmp has the expected dimensions (4x4).
        assert_eq!(bitmap.header.width, 4, "square.bmp should be 4 pixels wide");
        assert_eq!(bitmap.header.height, 4, "square.bmp should be 4 pixels tall");
        assert_eq!(bitmap.header.bits_per_pixel, 24, "square.bmp should be 24bpp");

        // Modify pixel at (2,2) to pure red: B=0, G=0, R=255.
        let pos = abmp_get_pixel_position_from_top_left(&bitmap.header, 2, 2) as usize;
        bitmap.pixel_data[pos] = 0;
        bitmap.pixel_data[pos + 1] = 0;
        bitmap.pixel_data[pos + 2] = 255;

        // Write the modified bitmap to a temp file.
        let out_path = temp_path("read_square_modify");
        bitmap.file_write_file(out_path.clone()).unwrap();

        // Read back and verify pixel (2,2) is pure red.
        let result = read_bmp(&out_path);
        assert_pixel(&result, 2, 2, 0, 0, 255);

        // Clean up temp file.
        let _ = std::fs::remove_file(&out_path);
    }

    // -------------------------------------------------------------------------
    // Test: create_bitmap_with_color_gradients
    // Description: Creates a new 6x4 bitmap from scratch using the AbmpBitmap::
    //              create factory function, fills each row with a different color
    //              gradient (blue, green, red), writes it to a temp file, then
    //              reads it back and verifies the pixel data matches.
    // What it does:
    //   1. Creates a new 6x4 AbmpBitmap with AbmpBitmap::create(6, 4).
    //   2. Row 0 (y=0): sets pixels 0-3 with a blue gradient (B decreases).
    //   3. Row 1 (y=1): sets pixels 0-4 with a green gradient (G decreases).
    //   4. Row 2 (y=2): sets pixels 0-5 with a red gradient (R decreases).
    //   5. Writes the bitmap to a temp file and reads it back.
    //   6. Verifies dimensions and checks gradient pixel values at key positions.
    // What is expected: The created bitmap has width=6, height=4, bits_per_pixel=24.
    //                   The written file is valid and can be read back with
    //                   matching header values and pixel gradients intact.
    // -------------------------------------------------------------------------
    #[test]
    fn test_create_bitmap_with_color_gradients() {
        let mut bitmap = AbmpBitmap::create(6, 4);

        // Verify the created bitmap has the correct dimensions.
        assert_eq!(bitmap.header.width, 6, "Created bitmap should be 6 pixels wide");
        assert_eq!(bitmap.header.height, 4, "Created bitmap should be 4 pixels tall");
        assert_eq!(bitmap.header.bits_per_pixel, 24, "Created bitmap should be 24bpp");

        // Row 0 (y=0): Blue gradient - B channel decreases from 255 to 135.
        // G=0, R=0 for all pixels in this row.
        for i in 0..4 {
            let pos = abmp_get_pixel_position_from_top_left(&bitmap.header, i, 0) as usize;
            bitmap.pixel_data[pos] = 0;       // B = 0
            bitmap.pixel_data[pos + 1] = 0;    // G = 0
            bitmap.pixel_data[pos + 2] = (255 - i * 40) as u8; // R = decreasing
        }

        // Row 1 (y=1): Green gradient - G channel decreases from 255 to 95.
        // B=0, R=0 for all pixels in this row.
        for i in 0..5 {
            let pos = abmp_get_pixel_position_from_top_left(&bitmap.header, i, 1) as usize;
            bitmap.pixel_data[pos] = 0;       // B = 0
            bitmap.pixel_data[pos + 1] = (255 - i * 40) as u8; // G = decreasing
            bitmap.pixel_data[pos + 2] = 0;    // R = 0
        }

        // Row 2 (y=2): Blue channel gradient decreasing from 255 to 55.
        // B=decreasing, G=0, R=0 for all pixels in this row.
        for i in 0..6 {
            let pos = abmp_get_pixel_position_from_top_left(&bitmap.header, i, 2) as usize;
            bitmap.pixel_data[pos] = (255 - i * 40) as u8; // B = decreasing
            bitmap.pixel_data[pos + 1] = 0;    // G = 0
            bitmap.pixel_data[pos + 2] = 0;    // R = 0
        }

        // Write the gradient bitmap to a temp file.
        let out_path = temp_path("create_gradient");
        bitmap.file_write_file(out_path.clone()).unwrap();

        // Read back and verify header dimensions.
        let result = read_bmp(&out_path);
        assert_eq!(result.header.width, 6, "Written bitmap should retain width=6");
        assert_eq!(result.header.height, 4, "Written bitmap should retain height=4");

        // Verify gradient pixel values at key positions in the read-back bitmap.
        // Row 0, pixel 0: pure blue (B=255).
        assert_pixel(&result, 0, 0, 0, 0, 255);
        // Row 0, pixel 3: dimmer blue (B=135).
        assert_pixel(&result, 3, 0, 0, 0, 135);
        // Row 1, pixel 0: pure green (G=255).
        assert_pixel(&result, 0, 1, 0, 255, 0);
        // Row 1, pixel 4: dimmer green (G=95).
        assert_pixel(&result, 4, 1, 0, 95, 0);
        // Row 2, pixel 0: pure blue (B=255).
        assert_pixel(&result, 0, 2, 255, 0, 0);
        // Row 2, pixel 5: dimmer blue (B=55).
        assert_pixel(&result, 5, 2, 55, 0, 0);

        // Clean up temp file.
        let _ = std::fs::remove_file(&out_path);
    }

    // -------------------------------------------------------------------------
    // Test: read_twoofpadding_bmp_and_print_pixel
    // Description: Reads the twoofpadding.bmp sample file, uses the `print`
    //              method to draw a single red pixel at coordinate (0,3),
    //              writes the result to a temp file, and reads it back to
    //              verify the pixel was placed correctly.
    // What it does:
    //   1. Opens samples/twoofpadding.bmp (6x4, 24bpp) and deserializes it.
    //   2. Calls bitmap.print(0, 3, 255, 0, 0) to set pixel (0,3) to red.
    //      The print method takes (x, y, R, G, B).
    //   3. Writes the modified bitmap to a temp file.
    //   4. Reads the temp file back and asserts pixel (0,3) is red.
    // What is expected: The print method correctly places a red pixel at (0,3).
    //                   The written file is valid and the pixel value persists
    //                   after a round-trip read.
    // -------------------------------------------------------------------------
    #[test]
    fn test_read_twoofpadding_bmp_and_print_pixel() {
        let path = samples_path("twoofpadding.bmp");
        let mut bitmap = AbmpBitmap::default();
        bitmap.file_read_file(path).unwrap();

        // Verify twoofpadding.bmp has the expected dimensions (6x4).
        assert_eq!(bitmap.header.width, 6, "twoofpadding.bmp should be 6 pixels wide");
        assert_eq!(bitmap.header.height, 4, "twoofpadding.bmp should be 4 pixels tall");

        // Draw a red pixel at position (0, 3) using the print method.
        // print(x, y, R, G, B) sets pixel at (x,y) to the given BGR color.
        bitmap.print(0, 3, 255, 0, 0);

        // Write the modified bitmap to a temp file.
        let out_path = temp_path("twoofpadding_print");
        bitmap.file_write_file(out_path.clone()).unwrap();

        // Read back and verify pixel (0,3) is red (B=0, G=0, R=255).
        let result = read_bmp(&out_path);
        assert_pixel(&result, 0, 3, 0, 0, 255);

        // Clean up temp file.
        let _ = std::fs::remove_file(&out_path);
    }

    // -------------------------------------------------------------------------
    // Test: read_generated_bmp_and_print_diagonal_patterns
    // Description: Reads the generated.bmp sample file (5x5, 24bpp), then uses
    //              the `print` method to draw green pixels along the main diagonal
    //              and blue pixels along the anti-diagonal (skipping the center
    //              pixel at i=2 in both passes). Writes the result to a temp file
    //              and reads it back to verify the pattern pixels are correct.
    // What it does:
    //   1. Opens samples/generated.bmp and deserializes it.
    //   2. Iterates i=0..=4, skipping i=2: prints green (R=0,G=255,B=0) at (i,i).
    //   3. Iterates i=0..=4, skipping i=2: prints blue (R=0,G=0,B=255) at (i,4-i).
    //   4. Writes the modified bitmap to a temp file.
    //   5. Reads it back and verifies the diagonal and anti-diagonal pixel colors.
    // What is expected: Green pixels appear at positions (0,0), (1,1), (3,3), (4,4).
    //                   Blue pixels appear at positions (0,4), (1,3), (3,1), (4,0).
    //                   Position (2,2) is skipped and retains its original value.
    //                   The written file is valid and all pixel values persist
    //                   correctly after a round-trip read.
    // -------------------------------------------------------------------------
    #[test]
    fn test_read_generated_bmp_and_print_diagonal_patterns() {
        let path = samples_path("generated.bmp");
        let mut bitmap = AbmpBitmap::default();
        bitmap.read_file(path).unwrap();

        // Verify generated.bmp has the expected dimensions (5x5).
        assert_eq!(bitmap.header.width, 5, "generated.bmp should be 5 pixels wide");
        assert_eq!(bitmap.header.height, 5, "generated.bmp should be 5 pixels tall");

        // Print green pixels along the main diagonal (x=y), skipping i=2.
        // print(x, y, R, G, B) — here R=0, G=255, B=0 means pure green.
        for i in 0..=4 {
            if i == 2 { continue; }
            bitmap.print(i, i, 0, 255, 0);
        }

        // Print blue pixels along the anti-diagonal (x, 4-x), skipping i=2.
        // print(x, y, R, G, B) — here R=0, G=0, B=255 means pure blue.
        for i in 0..=4 {
            if i == 2 { continue; }
            bitmap.print(i, 4 - i, 0, 0, 255);
        }

        // Write the patterned bitmap to a temp file.
        let out_path = temp_path("generated_pattern");
        bitmap.write_file(out_path.clone()).unwrap();

        // Read back and verify the diagonal and anti-diagonal pixels.
        let result = read_bmp(&out_path);

        // Green pixels on the main diagonal (skipping center i=2).
        assert_pixel(&result, 0, 0, 0, 255, 0);
        assert_pixel(&result, 1, 1, 0, 255, 0);
        assert_pixel(&result, 3, 3, 0, 255, 0);
        assert_pixel(&result, 4, 4, 0, 255, 0);

        // Blue pixels on the anti-diagonal (skipping center i=2).
        assert_pixel(&result, 0, 4, 255, 0, 0);
        assert_pixel(&result, 1, 3, 255, 0, 0);
        assert_pixel(&result, 3, 1, 255, 0, 0);
        assert_pixel(&result, 4, 0, 255, 0, 0);

        // Clean up temp file.
        let _ = std::fs::remove_file(&out_path);
    }
}
