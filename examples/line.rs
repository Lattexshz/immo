use immo::error::ImageError;
use immo::png::Png;

use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

const BLACK: (u8, u8, u8, u8) = (0, 0, 0, 255);
const WHITE: (u8, u8, u8, u8) = (255, 255, 255, 255);
const LIGHT_AQUA: (u8, u8, u8, u8) = (170, 222, 240, 255);
const DARK_AQUA: (u8, u8, u8, u8) = (0, 140, 180, 255);
const AQUA: (u8, u8, u8, u8) = (0, 222, 255, 255);

fn main() -> Result<(), ImageError> {
    let start = Instant::now();
    let mut png = Png::new(15, 15);
    png.draw_rectangle(1, 1, 10, 10, 2, BLACK)?;
    //png.line((0,0),(0,14),1,BLACK);

    let path = String::from("line.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 15, 15); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(png.as_slice()).unwrap();

    let end = start.elapsed();
    println!(
        "{}.{:03} second elapsed.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );

    Ok(())
}
