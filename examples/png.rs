use png::text_metadata::{ITXtChunk, ZTXtChunk};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;
use immo::error::ImageError;
use immo::png::Png;

const BLACK: (u8, u8, u8, u8) = (0, 0, 0, 255);
const WHITE: (u8, u8, u8, u8) = (255,255,255,255);
const LIGHT_AQUA: (u8, u8, u8, u8) = (170,222,240,255);
const DARK_AQUA: (u8, u8, u8, u8) = (0,140,180,255);
const AQUA: (u8, u8, u8, u8) = (0,222,255,255);

fn main() -> Result<(),ImageError>{
    let start = Instant::now();
    let mut png = Png::new(16,16);

    png.point(4,3,BLACK);
    png.point(5,3,BLACK);
    png.point(6,3,BLACK);
    png.point(7,3,BLACK);
    png.point(8,3,BLACK);
    png.point(9,3,BLACK);
    png.point(10,3,BLACK);
    png.point(11,3,BLACK);

    png.point(3,4,BLACK);
    png.point(4,4,WHITE);
    png.point(5,4,LIGHT_AQUA);
    png.point(6,4,WHITE);
    png.point(7,4,AQUA);
    png.point(8,4,AQUA);
    png.point(9,4,DARK_AQUA);
    png.point(10,4,LIGHT_AQUA);
    png.point(11,4,AQUA);
    png.point(12,4,BLACK);

    png.point(2,5,BLACK);
    png.point(3,5,WHITE);
    png.point(4,5,LIGHT_AQUA);
    png.point(5,5,WHITE);
    png.point(6,5,AQUA);
    png.point(7,5,AQUA);
    png.point(8,5,LIGHT_AQUA);
    png.point(9,5,LIGHT_AQUA);
    png.point(10,5,DARK_AQUA);
    png.point(11,5,LIGHT_AQUA);
    png.point(12,5,DARK_AQUA);
    png.point(13,5,BLACK);

    png.point(1,6,BLACK);
    png.point(2,6,WHITE);
    png.point(3,6,LIGHT_AQUA);
    png.point(4,6,WHITE);
    png.point(5,6,AQUA);
    png.point(6,6,WHITE);
    png.point(7,6,WHITE);
    png.point(8,6,WHITE);
    png.point(9,6,WHITE);
    png.point(10,6,LIGHT_AQUA);
    png.point(11,6,DARK_AQUA);
    png.point(12,6,LIGHT_AQUA);
    png.point(13,6,AQUA);
    png.point(14,6,BLACK);

    png.point(0,7,BLACK);
    png.point(1,7,DARK_AQUA);
    png.point(2,7,DARK_AQUA);
    png.point(3,7,DARK_AQUA);
    png.point(4,7,DARK_AQUA);
    png.point(5,7,AQUA);
    png.point(6,7,AQUA);
    png.point(7,7,AQUA);
    png.point(8,7,AQUA);
    png.point(9,7,DARK_AQUA);
    png.point(10,7,DARK_AQUA);
    png.point(11,7,DARK_AQUA);
    png.point(12,7,DARK_AQUA);
    png.point(13,7,DARK_AQUA);
    png.point(14,7,DARK_AQUA);
    png.point(15,7,BLACK);

    png.point(1,8,BLACK);
    png.point(2,8,AQUA);
    png.point(3,8,LIGHT_AQUA);
    png.point(4,8,LIGHT_AQUA);
    png.point(5,8,AQUA);
    png.point(6,8,LIGHT_AQUA);
    png.point(7,8,LIGHT_AQUA);
    png.point(8,8,WHITE);
    png.point(9,8,AQUA);
    png.point(10,8,WHITE);
    png.point(11,8,DARK_AQUA);
    png.point(12,8,LIGHT_AQUA);
    png.point(13,8,DARK_AQUA);
    png.point(14,8,BLACK);

    png.point(2,9,BLACK);
    png.point(3,9,LIGHT_AQUA);
    png.point(4,9,LIGHT_AQUA);
    png.point(5,9,LIGHT_AQUA);
    png.point(6,9,LIGHT_AQUA);
    png.point(7,9,AQUA);
    png.point(8,9,LIGHT_AQUA);
    png.point(9,9,AQUA);
    png.point(10,9,AQUA);
    png.point(11,9,DARK_AQUA);
    png.point(12,9,DARK_AQUA);
    png.point(13,9,BLACK);

    png.point(3,10,BLACK);


    let path = String::from("sample.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 16,16); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(png.as_slice()).unwrap();

    let end = start.elapsed();
    println!("{}.{:03} second elapsed.", end.as_secs(), end.subsec_nanos() / 1_000_000);

    Ok(())
}