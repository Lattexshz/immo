use png::text_metadata::{ITXtChunk, ZTXtChunk};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use immo::png::Png;

fn main() {
    let mut png = Png::new(10,10);
    png.rectangle(0,0,0,0,(255,255,255,255));
    // png.clear((0,0,0,255));
    // // Y = 0
    // png.point(4,0,(255,255,255,255));
    // png.point(5,0,(255,255,255,255));
    // // Y = 1
    // png.point(3,1,(255,255,255,255));
    // png.point(6,1,(255,255,255,255));
    // // Y = 2
    // png.point(3,2,(255,255,255,255));
    // png.point(6,2,(255,255,255,255));
    // // Y = 3
    // png.point(2,3,(255,255,255,255));
    // png.point(7,3,(255,255,255,255));
    // // Y = 4
    // png.point(2,4,(255,255,255,255));
    // png.point(7,4,(255,255,255,255));
    // // Y = 5
    // png.point(2,5,(255,255,255,255));
    // png.point(3,5,(255,255,255,255));
    // png.point(4,5,(255,255,255,255));
    // png.point(5,5,(255,255,255,255));
    // png.point(6,5,(255,255,255,255));
    // png.point(7,5,(255,255,255,255));
    // // Y = 6
    // png.point(2,6,(255,255,255,255));
    // png.point(7,6,(255,255,255,255));
    // // Y = 7
    // png.point(1,7,(255,255,255,255));
    // png.point(8,7,(255,255,255,255));
    // // Y = 8
    // png.point(1,8,(255,255,255,255));
    // png.point(8,8,(255,255,255,255));
    // // Y = 9
    // png.point(1,9,(255,255,255,255));
    // png.point(8,9,(255,255,255,255));


    let path = String::from("sample.png");
    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, 10,10); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    // Adding text chunks to the header
    encoder
        .add_text_chunk(
            "Testing tEXt".to_string(),
            "This is a tEXt chunk that will appear before the IDAT chunks.".to_string(),
        )
        .unwrap();
    encoder
        .add_ztxt_chunk(
            "Testing zTXt".to_string(),
            "This is a zTXt chunk that is compressed in the png file.".to_string(),
        )
        .unwrap();
    encoder
        .add_itxt_chunk(
            "Testing iTXt".to_string(),
            "iTXt chunks support all of UTF8. Example: हिंदी.".to_string(),
        )
        .unwrap();

    let mut writer = encoder.write_header().unwrap();

    //let data = [255, 0, 0, 255, 0, 0, 0, 255]; // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(png.as_slice()).unwrap(); // Save

    // We can add a tEXt/zTXt/iTXt at any point before the encoder is dropped from scope. These chunks will be at the end of the png file.
    let tail_ztxt_chunk = ZTXtChunk::new(
        "Comment".to_string(),
        "A zTXt chunk after the image data.".to_string(),
    );
    writer.write_text_chunk(&tail_ztxt_chunk).unwrap();

    // The fields of the text chunk are public, so they can be mutated before being written to the file.
    let mut tail_itxt_chunk = ITXtChunk::new("Author".to_string(), "सायंतन खान".to_string());
    tail_itxt_chunk.compressed = true;
    tail_itxt_chunk.language_tag = "hi".to_string();
    tail_itxt_chunk.translated_keyword = "लेखक".to_string();
    writer.write_text_chunk(&tail_itxt_chunk).unwrap();
}