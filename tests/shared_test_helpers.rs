use std::io::{Read, Seek};
use std::path::Path;

use ray_tracer::canvas;

/// Gets a ppm string (simple image format) by creating a file, writing to it, then reading the contents back and returning the results.
pub fn get_ppm_string_via_file(canvas: &canvas::Canvas, output_file_path: &str) -> String {
    // Borrowed from https://stackoverflow.com/a/47956654
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(output_file_path)
        .unwrap();
    canvas.canvas_to_ppm(&mut output_file).unwrap();
    let mut output_contents = String::new();
    output_file.seek(std::io::SeekFrom::Start(0)).unwrap();
    output_file.read_to_string(&mut output_contents).unwrap();

    return output_contents;
}

pub fn write_image_to_file(
    canvas: &canvas::Canvas,
    output_file_path: &str,
) -> Result<(), image::error::ImageError> {
    let image = canvas.canvas_to_image();
    let path = Path::new(output_file_path);
    image.save(path)
}
