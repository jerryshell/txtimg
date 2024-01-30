use flate2::{read::ZlibDecoder, write::ZlibEncoder};
use image::{DynamicImage, Rgb, RgbImage};
use std::io::{Read, Write};

fn compress_text(input: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut encoder = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(input.as_bytes())?;
    encoder.finish()
}

fn decompress_text(compressed_data: &[u8]) -> Result<String, std::io::Error> {
    let mut decoder = ZlibDecoder::new(compressed_data);
    let mut decompressed_data = String::new();
    decoder.read_to_string(&mut decompressed_data)?;
    Ok(decompressed_data)
}

pub fn text_to_image(original_text: &str, output_path: &str) {
    let mut pixel_data: Vec<u8> = compress_text(original_text).expect("Compression failed");

    // alignment
    let remainder = pixel_data.len() % 3;
    let alignment = (3 - remainder) % 3;
    if remainder != 0 {
        pixel_data.resize(pixel_data.len() + alignment, 0);
    }

    // build image_buffer
    let image_width = pixel_data.len() as u32 / 3 + 1;
    let mut image_buffer = RgbImage::new(image_width, 1);
    for (pixel_index, pixel_value_array) in pixel_data.chunks(3).enumerate() {
        let pixel_index = pixel_index as u32;
        let pixel = Rgb([
            pixel_value_array[0],
            pixel_value_array[1],
            pixel_value_array[2],
        ]);
        image_buffer.put_pixel(pixel_index, 0, pixel);
    }

    // alignment pixel
    image_buffer.put_pixel(image_width - 1, 0, Rgb([0, 0, alignment as u8]));

    // save image
    let save_result = DynamicImage::ImageRgb8(image_buffer).save(output_path);
    if let Err(e) = save_result {
        eprintln!("Error saving image: {:?}", e);
    }
}

pub fn image_to_text(image_path: &str) -> String {
    let img = image::open(image_path).expect("Failed to open image");
    let img_bytes = img.as_bytes();
    let alignment = img_bytes.last().expect("Failed to get last byte");

    let data = &img_bytes[0..img_bytes.len() - 3 - *alignment as usize];
    decompress_text(data).expect("Decompression failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let original_text = "Hello Rust! 你好世界 🦀";
        let output_path = "output.png";
        text_to_image(original_text, output_path);
        let text = image_to_text(output_path);
        assert_eq!(text, original_text);

        let original_text = "Hello Rust! 你好，世界 🦀";
        text_to_image(original_text, output_path);
        let text = image_to_text(output_path);
        assert_eq!(text, original_text);
    }
}
