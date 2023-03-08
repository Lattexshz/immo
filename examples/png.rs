use png::text_metadata::{ITXtChunk, ZTXtChunk};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use immo::error::ImageError;
use immo::png::Png;

fn main() {
    let mut png = Png::new(10,10);
    png.fill_rectangle(0, 0, 5, 5, (255, 50, 210, 255)).unwrap();
    png.fill_rectangle(5, 0, 5, 5, (0, 0, 0, 255)).unwrap();
    png.fill_rectangle(0, 5, 5, 5, (0, 0, 0, 255)).unwrap();
    png.fill_rectangle(5, 5, 5, 5, (255, 50, 210, 255)).unwrap();

    let path = String::from("sample.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 10,10); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(png.as_slice()).unwrap();
}