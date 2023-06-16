use std::env;
use std::path::Path;
use image::GenericImageView;
use image::Rgba;
use image::io::Reader as ImageReader;
use std::io::Cursor;

//TODO: return a result
fn load_image(image_path: &str) -> Option<image::DynamicImage> {
    // Open the image
    match image::open(&Path::new(image_path)) {
        Ok(img) => Some(img),
        Err(e) => {
            None
        }
    }
}
fn diff1(a : u8, b : u8) -> u16 {
  (if a > b { a - b} else { b - a}) as u16
}

// return a color difference between two pixels
//fn diff(a : [u8;4], b : [u8;4]) -> [u8;4] {
//    [ diff1(a[0], b[0]), diff1(a[1], b[1]), diff1(a[2], b[2]), diff1(a[3], b[3]) ]
//}

// compute the difference between two colors as the absolute difference of each color gun, max score = 4 * 255
fn diff2(a : Rgba<u8>, b : Rgba<u8>) -> u16 {
    diff1(a[0], b[0]) + diff1(a[1], b[1]) + diff1(a[2], b[2]) + diff1(a[3], b[3])
}

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure two image file paths are provided
    if args.len() != 3 {
        println!("Usage: ./image_viewer <image1_path> <image2_path>");
        return;
    }

    let img1 = load_image(&args[1]).unwrap();
    let img2 = load_image(&args[2]).unwrap();

    let rgba1: Vec<Rgba<u8>> = img1.pixels().map(|p| p.2).collect();
    let rgba2: Vec<Rgba<u8>> = img2.pixels().map(|p| p.2).collect();
    let rgbaDelta = Vec::new();
    for (p1,p2) in rgba1.iter().zip(rgba2.iter()) {
        let dpixel = diff2(*p1,*p2) / 4;
        rgbaDelta.push(dpixel);
        rgbaDelta.push(0);
        rgbaDelta.push(255-dpixel);
        rgbaDelta.push(0xFF);
    }
    let out = ImageReader::new(Cursor::new(rgbaDelta)).with_guessed_format()?.decode()?;
    out.save("delta.webp")
}