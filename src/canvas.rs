use crate::canvas;
use crate::color;

use std::cmp::min;
use std::fs::File;
use std::io::Write;

pub struct Canvas {
    pub width: u64,
    pub height: u64,
    grid: Vec<Vec<color::Color>>,
}

pub fn canvas(width: u64, height: u64) -> Canvas {
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
    pub fn write_pixel(&mut self, x: u64, y: u64, value: color::Color) {
        self.grid[y as usize][x as usize] = value;
    }

    pub fn pixel_at(&self, x: u64, y: u64) -> &color::Color {
        &self.grid[y as usize][x as usize]
    }

    pub fn canvas_to_ppm(&self, mut output: File) -> Result<(), std::io::Error> {
        write!(output, "P3\n")?;
        write!(output, "{} {}\n", self.width, self.height)?;
        write!(output, "255\n")?;

        let clamp = |x: f64| 255.0_f64.min(x * 255.0).max(0.0);
        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.pixel_at(x, y);
                // println!("{} {}", x, y);
                // println!("{:.*}", 0, clamp(c.g));
                write!(
                    output,
                    "{:.*} {:.*} {:.*} ",
                    0,
                    clamp(c.r),
                    0,
                    clamp(c.g),
                    0,
                    clamp(c.b)
                )?;
            }
            write!(output, "\n")?;
        }
        // write!(output, "\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::canvas;
    use crate::color;

    use std::fs::File;
    use std::io::Read;

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
    fn test_canvas_to_ppm_writes_the_header() -> Result<(), std::io::Error> {
        let mut canvas1 = canvas::canvas(5, 3);
        let path = "output1.ppm";

        let mut output = File::create(path)?;

        canvas1.canvas_to_ppm(output);

        let mut input = File::open(path)?;
        let mut contents = String::new();
        input.read_to_string(&mut contents)?;
        let expected = "\
P3
5 3
255
";
        assert!(contents.contains(expected));
        Ok(())
    }

    #[test]
    fn test_canvas_to_ppm_writes_pixel_data() -> Result<(), std::io::Error> {
        let mut canvas1 = canvas::canvas(5, 3);
        let color1 = color::color(1.5, 0.0, 0.0);
        canvas1.write_pixel(0, 0, color1);
        let color2 = color::color(0.0, 0.5, 0.0);
        canvas1.write_pixel(2, 1, color2);
        let color3 = color::color(-0.5, 0.0, 1.0);
        canvas1.write_pixel(4, 2, color3);
        let path = "output2.ppm";

        let mut output = File::create(path)?;

        canvas1.canvas_to_ppm(output)?;

        let mut input = File::open(path)?;
        let mut contents = String::new();
        input.read_to_string(&mut contents)?;
        let expected = "\
255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
";
        println!("{}", contents);
        assert!(contents.contains(expected));
        Ok(())
    }

}
