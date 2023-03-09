use png::text_metadata::{ITXtChunk, ZTXtChunk};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;
use immo::error::ImageError;
use immo::png::Png;

const RED: (u8, u8, u8, u8) = (255, 0, 0, 255);
const ORANGE: (u8, u8, u8, u8) = (255,150,0,255);
const YELLOW: (u8, u8, u8, u8) = (255,240,0,255);
const GREEN: (u8, u8, u8, u8) = (0,135,0,255);
const BLUE: (u8, u8, u8, u8) = (0,145,255,255);
const NAVY: (u8, u8, u8, u8) = (0,100,190,255);
const PURPLE: (u8, u8, u8, u8) = (145,0,130,255);

fn main() -> Result<(),ImageError>{
    println!("This process takes approximately one minute for a Debug build.");
    let start = Instant::now();
    let mut png = Png::new(1024,1024);

    let color = [RED,ORANGE,YELLOW,GREEN,BLUE,NAVY,PURPLE];
    let mut index = 0;

    let mut size = 1024;
    let mut pos = 0;

    for i in 0..256 {
        if index == 6 {
            index = 0;
        }
        png.draw_rectangle(pos,pos,size,size,1,color[index])?;
        size -= 4;
        pos += 2;
        index += 1;
    }

    let path = String::from("optical.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 1024,1024); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();


    writer.write_image_data(png.as_slice()).unwrap();

    let end = start.elapsed();
    println!("{}.{:03} second elapsed.", end.as_secs(), end.subsec_nanos() / 1_000_000);

    Ok(())
}