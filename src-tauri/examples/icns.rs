use std::fs::File;
use std::io::{BufReader, BufWriter};
use tauri_icns::{IconFamily, IconType, Image};

fn main() {
// Load an icon family from an ICNS file.
    let result = File::open("/Users/lishaowen/Downloads/rustrover.icns").unwrap();
    let file = BufReader::new(result);
    let mut icon_family = IconFamily::read(file).unwrap();

    // Extract an icon from the family and save it as a PNG.
    let image = icon_family.get_icon_with_type(IconType::RGBA32_512x512_2x).unwrap();
    image.data();
    let file = BufWriter::new(File::create("../public/rustrover.png").unwrap());
    image.write_png(file).unwrap();
    //
    // // Read in another icon from a PNG file, and add it to the icon family.
    // let file = BufReader::new(File::open("32.png").unwrap());
    // let image = Image::read_png(file).unwrap();
    // icon_family.add_icon(&image).unwrap();
    //
    // // Save the updated icon family to a new ICNS file.
    // let file = BufWriter::new(File::create("16-and-32.icns").unwrap());
    // icon_family.write(file).unwrap();
}