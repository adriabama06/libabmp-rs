use libabmp_rs::hello_world;
use libabmp_rs::abitmap::{AbmpBitmapHeader, AbmpBitmap};

fn main() {
    let example_header = AbmpBitmapHeader::default();

    println!("{}", hello_world());
    println!("{:?}", example_header);

    
    
    
    let mut sample_1 = AbmpBitmap::default();

    sample_1.file_read_file("../samples/square.bmp".to_string()).unwrap();

    println!("{:?}", sample_1);

    // TODO: Try to draw before store

    sample_1.file_write_file("square_edit.bmp".to_string()).unwrap();

    
    
    
    let example_bitmap = AbmpBitmap::create(6, 4);

    example_bitmap.file_write_file("test.bmp".to_string()).unwrap();

    // TODO: Try to draw




    let mut sample_2 = AbmpBitmap::default();

    sample_2.file_read_file("../samples/twoofpadding.bmp".to_string()).unwrap();

    println!("{:?}", sample_2);

    sample_2.file_write_file("copy_twoofpadding.bmp".to_string()).unwrap();
}
