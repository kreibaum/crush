use clap_derive::Parser;
use image::{codecs::jpeg::JpegEncoder, GenericImageView};
use std::{fs::File, io::Write};

const MAX_DIMENSION: u32 = 2000;

#[derive(Parser)]
#[clap(name = "crush", version = "1.0", author = "Your Name")]
struct Opts {
    /// Target size in bytes (e.g., 100k, 1M)
    #[clap(short, default_value = "200000")]
    target_size: String,

    /// Input image file path
    #[clap(required = true)]
    input_file: String,

    /// Output image file path
    #[clap(short, default_value = "output.jpg")]
    output_file: String,
}

fn main() {
    // Load the image
    let img = image::open("examples/screenshot_of_puzzle_shark.png").unwrap();

    let resized_img = scale_down_image(img, MAX_DIMENSION);

    let size_target = 200_000;

    let buffer = encode_jpeg_for_size_target(resized_img, size_target);

    write_result_file(buffer);
}

/// Encode the image as a JPEG with a target file size.
/// This starts at 90% quality and does binary search on the 0..100 quality range.
/// The resulting image may be slightly larger than the target size.
fn encode_jpeg_for_size_target(resized_img: image::DynamicImage, size_target: usize) -> Vec<u8> {
    let mut max_quality = 100;
    let mut min_quality = 0;
    // First, we try compressing the image with a quality of 90
    let mut test_quality = 90;
    let mut buffer = Vec::new();

    while max_quality - min_quality > 1 {
        buffer = encode_jpeg(resized_img.clone(), test_quality);
        let test_size = buffer.len();
        println!(
            "Image file size is: {} at {}% quality.",
            test_size, test_quality
        );

        // If the image is too large, we need to reduce the quality
        if test_size > size_target {
            max_quality = test_quality;
        } else {
            min_quality = test_quality;
        }
        test_quality = (max_quality + min_quality) / 2;
    }
    buffer
}

/// Scale down the image while preserving aspect ratio.
fn scale_down_image(img: image::DynamicImage, max_dimension: u32) -> image::DynamicImage {
    // Get the original dimensions
    let (original_width, original_height) = img.dimensions();

    // Calculate the new dimensions while preserving aspect ratio
    let max_width = max_dimension;
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
    img.resize(new_width, new_height, image::imageops::FilterType::Lanczos3)
}

/// In-Memory JPEG Encoding with desired quality.
fn encode_jpeg(resized_img: image::DynamicImage, quality: u8) -> Vec<u8> {
    // Create a new in-memory buffer
    let mut buffer = Vec::new();

    // Create a new JpegEncoder with the desired quality (1-100)
    let mut encoder = JpegEncoder::new_with_quality(&mut buffer, quality);

    // Convert the resized image to a buffer of RGB pixels
    let rgb_image = resized_img.to_rgb8();
    let (width, height) = rgb_image.dimensions();
    let rgb_pixels = rgb_image.into_raw();

    // Encode the RGB pixels using the JpegEncoder
    encoder
        .encode(&rgb_pixels, width, height, image::ExtendedColorType::Rgb8)
        .unwrap();

    buffer
}

/// Write the result to a file after we got the desired size.
fn write_result_file(buffer: Vec<u8>) {
    // Create the "output" directory if it doesn't exist
    std::fs::create_dir_all("output").unwrap();

    // Create a new file named "output/output.jpg"
    let mut file = File::create("output/output.jpg").unwrap();

    // Write the contents of the buffer to the file
    file.write_all(&buffer).unwrap();

    println!("Resized image saved to output/output.jpg");
}
