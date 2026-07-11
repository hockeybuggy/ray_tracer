use crate::color;

use image::{ImageBuffer, RgbImage};

#[derive(Debug, PartialEq)]
pub struct Canvas {
    pub width: u32,
    pub height: u32,
    grid: Vec<Vec<color::Color>>,
}

pub fn canvas(width: u32, height: u32) -> Canvas {
    let mut grid = Vec::with_capacity(height as usize);
    for _y in 0..height {
        let mut row = Vec::with_capacity(width as usize);
        for _x in 0..width {
            row.push(color::color(0.0, 0.0, 0.0));
        }
        grid.push(row);
    }

    Canvas {
        width,
        height,
        grid,
    }
}

impl Canvas {
    pub fn write_pixel(&mut self, x: u32, y: u32, value: color::Color) {
        self.grid[y as usize][x as usize] = value;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> &color::Color {
        &self.grid[y as usize][x as usize]
    }

    pub fn canvas_to_image(&self) -> RgbImage {
        let mut img: RgbImage = ImageBuffer::new(self.width, self.height);

        let clamp = |x: f64| 255.0_f64.min(x * 255.0).max(0.0) as u8;
        let (width, height) = img.dimensions();
        for x in 0..width {
            for y in 0..height {
                let canvas_color = self.pixel_at(x, y);
                let pixel = image::Rgb([
                    clamp(canvas_color.r),
                    clamp(canvas_color.g),
                    clamp(canvas_color.b),
                ]);
                img.put_pixel(x, y, pixel);
            }
        }

        return img;
    }
}

/// Parses a plain-text ("P3") PPM file into a canvas. Comment lines
/// (starting with `#`) are ignored, and RGB triples may span lines.
pub fn canvas_from_ppm(ppm: &str) -> Result<Canvas, String> {
    let mut tokens = ppm
        .lines()
        .map(|line| line.split('#').next().unwrap())
        .flat_map(str::split_whitespace);

    match tokens.next() {
        Some("P3") => {}
        other => return Err(format!("expected PPM magic number \"P3\", got {:?}", other)),
    }

    let width = next_number(&mut tokens, "width")? as u32;
    let height = next_number(&mut tokens, "height")? as u32;
    let scale = next_number(&mut tokens, "scale")?;

    let mut result = canvas(width, height);
    for y in 0..height {
        for x in 0..width {
            let r = next_number(&mut tokens, "red")? / scale;
            let g = next_number(&mut tokens, "green")? / scale;
            let b = next_number(&mut tokens, "blue")? / scale;
            result.write_pixel(x, y, color::color(r, g, b));
        }
    }

    return Ok(result);
}

fn next_number<'a>(
    tokens: &mut impl Iterator<Item = &'a str>,
    describing: &str,
) -> Result<f64, String> {
    match tokens.next() {
        Some(token) => token
            .parse::<f64>()
            .map_err(|_| format!("expected a number for {}, got {:?}", describing, token)),
        None => Err(format!("ran out of input looking for {}", describing)),
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::assert_color_approx_eq;
    use crate::canvas;
    use crate::color;

    #[test]
    fn test_canvas_constructor_sets_height_and_width() {
        let canvas1 = canvas::canvas(10, 20);

        assert_eq!(canvas1.width, 10);
        assert_eq!(canvas1.height, 20);
    }

    #[test]
    fn test_canvas_constructor_sets_all_pixels_to_black() {
        let canvas1 = canvas::canvas(4, 6);

        for x in 0..3 {
            for y in 0..5 {
                assert_eq!(&color::color(0.0, 0.0, 0.0), canvas1.pixel_at(x, y));
            }
        }
    }

    #[test]
    fn test_pixels_can_be_written_to_a_canvas() {
        let mut canvas1 = canvas::canvas(10, 20);
        let color1 = color::color(1.0, 0.0, 0.0);

        canvas1.write_pixel(2, 3, color1);

        let set_value: &color::Color = canvas1.pixel_at(2, 3);
        assert_eq!(&color::color(1.0, 0.0, 0.0), set_value);
    }

    #[test]
    fn test_canvas_to_image_single_color() {
        let mut canvas1 = canvas::canvas(10, 2);
        for y in 0..canvas1.height {
            for x in 0..canvas1.width {
                canvas1.write_pixel(x, y, color::color(1.0, 0.8, 0.6));
            }
        }

        let result = canvas1.canvas_to_image();

        let expected_image =
            image::ImageBuffer::from_fn(10, 2, |_x, _y| image::Rgb([255, 204, 153]));
        assert_eq!(expected_image, result);
    }

    #[test]
    fn test_canvas_to_image_check_edges_are_included() {
        let mut canvas1 = canvas::canvas(5, 3);
        let color1 = color::color(1.5, 0.0, 0.0);
        canvas1.write_pixel(0, 0, color1);
        let color2 = color::color(0.0, 0.5, 0.0);
        canvas1.write_pixel(2, 1, color2);
        let color3 = color::color(-0.5, 0.0, 1.0);
        canvas1.write_pixel(4, 2, color3);

        let result = canvas1.canvas_to_image();

        let mut expected_image = image::ImageBuffer::new(5, 3);
        expected_image.put_pixel(0, 0, image::Rgb([255, 0, 0]));
        expected_image.put_pixel(2, 1, image::Rgb([0, 127, 0]));
        expected_image.put_pixel(4, 2, image::Rgb([0, 0, 255]));
        assert_eq!(expected_image, result);
    }

    // Scenario: Reading a file with the wrong magic number
    #[test]
    fn test_reading_a_file_with_the_wrong_magic_number() {
        let ppm = "P32\n1 1\n255\n0 0 0\n";

        assert!(canvas::canvas_from_ppm(ppm).is_err());
    }

    // Scenario: Reading a PPM returns a canvas of the right size
    #[test]
    fn test_reading_a_ppm_returns_a_canvas_of_the_right_size() {
        let ppm = "P3\n\
                   10 2\n\
                   255\n\
                   0 0 0  0 0 0  0 0 0  0 0 0  0 0 0\n\
                   0 0 0  0 0 0  0 0 0  0 0 0  0 0 0\n\
                   0 0 0  0 0 0  0 0 0  0 0 0  0 0 0\n\
                   0 0 0  0 0 0  0 0 0  0 0 0  0 0 0\n";

        let canvas = canvas::canvas_from_ppm(ppm).unwrap();

        assert_eq!(canvas.width, 10);
        assert_eq!(canvas.height, 2);
    }

    // Scenario Outline: Reading pixel data from a PPM file
    #[test]
    fn test_reading_pixel_data_from_a_ppm_file() {
        let ppm = "P3\n\
                   4 3\n\
                   255\n\
                   255 127 0  0 127 255  127 255 0  255 255 255\n\
                   0 0 0  255 0 0  0 255 0  0 0 255\n\
                   255 255 0  0 255 255  255 0 255  127 127 127\n";

        let canvas = canvas::canvas_from_ppm(ppm).unwrap();

        // 127/255 is ~0.498; write the expected values as exact fractions
        // because the crate's approx tolerance (1e-5) is tighter than the
        // book's rounded 0.498.
        let f = |n: f64| n / 255.0;
        let cases = [
            (0, 0, color::color(1.0, f(127.0), 0.0)),
            (1, 0, color::color(0.0, f(127.0), 1.0)),
            (2, 0, color::color(f(127.0), 1.0, 0.0)),
            (3, 0, color::color(1.0, 1.0, 1.0)),
            (0, 1, color::color(0.0, 0.0, 0.0)),
            (1, 1, color::color(1.0, 0.0, 0.0)),
            (2, 1, color::color(0.0, 1.0, 0.0)),
            (3, 1, color::color(0.0, 0.0, 1.0)),
            (0, 2, color::color(1.0, 1.0, 0.0)),
            (1, 2, color::color(0.0, 1.0, 1.0)),
            (2, 2, color::color(1.0, 0.0, 1.0)),
            (3, 2, color::color(f(127.0), f(127.0), f(127.0))),
        ];
        for (x, y, expected) in cases {
            assert_color_approx_eq!(*canvas.pixel_at(x, y), expected);
        }
    }

    // Scenario: PPM parsing ignores comment lines
    #[test]
    fn test_ppm_parsing_ignores_comment_lines() {
        let ppm = "P3\n\
                   # this is a comment\n\
                   2 1\n\
                   # this, too\n\
                   255\n\
                   # another comment\n\
                   255 255 255\n\
                   # oh, no, comments in the pixel data!\n\
                   255 0 255\n";

        let canvas = canvas::canvas_from_ppm(ppm).unwrap();

        assert_color_approx_eq!(*canvas.pixel_at(0, 0), color::color(1.0, 1.0, 1.0));
        assert_color_approx_eq!(*canvas.pixel_at(1, 0), color::color(1.0, 0.0, 1.0));
    }

    // Scenario: PPM parsing allows an RGB triple to span lines
    #[test]
    fn test_ppm_parsing_allows_an_rgb_triple_to_span_lines() {
        let ppm = "P3\n1 1\n255\n51\n153\n\n204\n";

        let canvas = canvas::canvas_from_ppm(ppm).unwrap();

        assert_color_approx_eq!(*canvas.pixel_at(0, 0), color::color(0.2, 0.6, 0.8));
    }

    // Scenario: PPM parsing respects the scale setting
    #[test]
    fn test_ppm_parsing_respects_the_scale_setting() {
        let ppm = "P3\n\
                   2 2\n\
                   100\n\
                   100 100 100  50 50 50\n\
                   75 50 25  0 0 0\n";

        let canvas = canvas::canvas_from_ppm(ppm).unwrap();

        assert_color_approx_eq!(*canvas.pixel_at(0, 1), color::color(0.75, 0.5, 0.25));
    }
}
