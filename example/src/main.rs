use libabmp_rs::hello_world;
use libabmp_rs::abitmap::AbmpBitmapHeader;

fn main() {
    let example_header = AbmpBitmapHeader::default();

    println!("{}", hello_world());
    println!("{:?}", example_header);
}
