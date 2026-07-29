use libabmp_rs::{abitmap::{AbmpBitmap, AbmpBitmapHeader}, get::abmp_get_pixel_position_from_top_left};

fn main() {
    let example_header = AbmpBitmapHeader::default();

    println!("Example header: {:?}", example_header);
    



    
    let mut sample_1 = AbmpBitmap::default();

    sample_1.file_read_file("../samples/square.bmp".to_string()).unwrap();

    println!("{:?}", sample_1);

    // BGR => (0,0,255) => PURE RED
    sample_1.pixel_data[abmp_get_pixel_position_from_top_left(&sample_1.header, 2, 2) as usize] = 0;
    sample_1.pixel_data[(abmp_get_pixel_position_from_top_left(&sample_1.header, 2, 2) + 1) as usize] = 0;
    sample_1.pixel_data[(abmp_get_pixel_position_from_top_left(&sample_1.header, 2, 2) + 2) as usize] = 255;

    sample_1.file_write_file("square_edit.bmp".to_string()).unwrap();

    
    

    
    let mut example_bitmap = AbmpBitmap::create(6, 4);

    for i in 0..4 {
        example_bitmap.pixel_data[abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 0) as usize] = 0;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 0) + 1) as usize] = 0;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 0) + 2) as usize] = (255 - i*40) as u8;
    }
    for i in 0..5 {
        example_bitmap.pixel_data[abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 1) as usize] = 0;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 1) + 1) as usize] = (255 - i*40) as u8;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 1) + 2) as usize] = 0;
    }
    for i in 0..6 {
        example_bitmap.pixel_data[abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 2) as usize] = (255 - i*40) as u8;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 2) + 1) as usize] = 0;
        example_bitmap.pixel_data[(abmp_get_pixel_position_from_top_left(&example_bitmap.header, i, 2) + 2) as usize] = 0;
    }

    example_bitmap.file_write_file("test.bmp".to_string()).unwrap();





    let mut sample_2 = AbmpBitmap::default();

    sample_2.file_read_file("../samples/twoofpadding.bmp".to_string()).unwrap();

    println!("{:?}", sample_2);

    sample_2.file_write_file("copy_twoofpadding.bmp".to_string()).unwrap();





    let mut sample_3 = AbmpBitmap::default();

    sample_3.read_file("../samples/generated.bmp".to_string()).unwrap();

    println!("{:?}", sample_3);

    sample_3.write_file("copy_generated.bmp".to_string()).unwrap();
}
