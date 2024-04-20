use image::{codecs::jpeg::JpegEncoder, GenericImageView};
use std::{fs::File, io::Write};

fn main() {
    // Load the image
    let img = image::open("examples/screenshot_of_puzzle_shark.png").unwrap();

    // Get the original dimensions
    let (original_width, original_height) = img.dimensions();

    // Calculate the new dimensions while preserving aspect ratio
    let max_width = 2000;
    let max_height = 2000;
    let aspect_ratio = original_width as f32 / original_height as f32;

    let (new_width, new_height) = if original_width > max_width || original_height > max_height {
        if aspect_ratio > 1.0 {
            (max_width, (max_width as f32 / aspect_ratio) as u32)
        } else {
            ((max_height as f32 * aspect_ratio) as u32, max_height)
        }
    } else {
        (original_width, original_height)
    };

    // Resize the image
    let resized_img = img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3);

    // Create a new in-memory buffer
    let mut buffer = Vec::new();

    // Create a new JpegEncoder with the desired quality (1-100)
    let quality = 90;
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

    // Convert the resized image to a buffer of RGBA pixels
    let rgb_image = resized_img.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let rgb_pixels = rgb_image.into_raw();

    // Encode the RGB pixels using the JpegEncoder
    encoder
        .encode(&rgb_pixels, width, height, image::ExtendedColorType::Rgb8)
        .unwrap();

    // Create the "output" directory if it doesn't exist
    std::fs::create_dir_all("output").unwrap();

    // Create a new file named "output/output.jpg"
    let mut file = File::create("output/output.jpg").unwrap();

    // Write the contents of the buffer to the file
    file.write_all(&buffer).unwrap();
    println!("Resized image saved to output/output.jpg");
}
