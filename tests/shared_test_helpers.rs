use std::path::Path;

use ray_tracer::canvas;

pub fn write_image_to_file(
    canvas: &canvas::Canvas,
    output_file_path: &str,
) -> Result<(), image::error::ImageError> {
    let image = canvas.canvas_to_image();
    let path = Path::new(output_file_path);
    image.save(path)
}

pub fn read_image_from_fixture_file(
    fixture_name: &str,
) -> Result<image::RgbImage, image::error::ImageError> {
    let path_name = format!("tests/fixtures/{}.png", fixture_name);
    println!("{}", &path_name);
    let path = Path::new(&path_name);
    let image: image::RgbImage = image::open(path).unwrap().to_rgb();
    return Ok(image);
}
