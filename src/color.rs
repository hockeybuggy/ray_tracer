use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

pub fn color(r: f64, g: f64, b: f64) -> Color {
    Color { r, g, b }
}
pub fn black() -> Color {
    color(0.0, 0.0, 0.0)
}

pub fn white() -> Color {
    color(1.0, 1.0, 1.0)
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod color_tests {
    use crate::color;

    #[test]
    fn test_color_constructor() {
        let color1 = color::color(-0.5, 0.4, 1.7);

        assert_eq!(color1.r, -0.5);
        assert_eq!(color1.g, 0.4);
        assert_eq!(color1.b, 1.7);
    }

    #[test]
    fn test_colors_can_be_added() {
        let color1 = color::color(0.9, 0.6, 0.75);
        let color2 = color::color(0.7, 0.1, 0.25);

        let expected_color = color::color(1.6, 0.7, 1.0);
        assert_eq!(color1 + color2, expected_color);
    }

    #[test]
    fn test_colors_can_be_subtracted() {
        let color1 = color::color(0.9, 0.6, 0.75);
        let color2 = color::color(0.7, 0.1, 0.25);

        let expected_color = color::color(0.9 - 0.7, 0.5, 0.5);
        assert_eq!(color1 - color2, expected_color);
    }

    #[test]
    fn test_colors_can_be_muliplied_by_a_scalar() {
        let color1 = color::color(0.2, 0.3, 0.4);

        let expected_color = color::color(0.4, 0.6, 0.8);
        assert_eq!(color1 * 2.0, expected_color);
    }

    #[test]
    fn test_colors_can_be_muliplied_with_each_other() {
        let color1 = color::color(1.0, 0.2, 0.4);
        let color2 = color::color(0.9, 1.0, 0.2);

        let expected_color = color::color(0.9, 0.2, 0.4 * 0.2);
        assert_eq!(color1 * color2, expected_color);
    }

    #[test]
    fn test_black_returns_a_black_color() {
        assert_eq!(color::color(0.0, 0.0, 0.0), color::black());
    }
}
