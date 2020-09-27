use crate::color;

use image::{ImageBuffer, RgbImage};

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

        let clamp = |x: f64| (255.0_f64.min(x * 255.0).max(0.0) as u8);
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

#[cfg(test)]
mod canvas_tests {
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
}
