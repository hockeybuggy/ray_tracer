use crate::color;

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
}
