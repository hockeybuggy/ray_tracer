use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z, w: 1.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z, w: 0.0 }
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        return *self - *normal * 2.0 * dot(&self, &normal);
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

pub fn magnitude(v: &Vector) -> f64 {
    (v.x * v.x + v.y * v.y + v.z * v.z).sqrt()
}

pub fn normalize(v: &Vector) -> Vector {
    let mag = magnitude(v);
    Vector {
        x: v.x / mag,
        y: v.y / mag,
        z: v.z / mag,
        w: v.w / mag,
    }
}

pub fn dot(a: &Vector, b: &Vector) -> f64 {
    a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
}

pub fn cross(a: &Vector, b: &Vector) -> Vector {
    return Vector::new(
        a.y * b.z - a.z * b.y,
        a.z * b.x - a.x * b.z,
        a.x * b.y - a.y * b.x,
    );
}

#[cfg(test)]
mod tuple_tests {
    use crate::assert_tuple_approx_eq;
    use crate::tuple;

    #[test]
    fn test_point_constructor_creates_point_with_w_1() {
        let point = tuple::Point::new(4.0, -4.0, 3.0);

        let expected_point = tuple::Point {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(point, expected_point);
    }

    #[test]
    fn test_vector_constructor_creates_tuples_with_w_0() {
        let vector = tuple::Vector::new(4.0, -4.0, 3.0);

        let expected_vector = tuple::Vector {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(vector, expected_vector);
    }

    #[test]
    fn test_vectors_and_vectors_can_be_added() {
        let point = tuple::Vector {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 0.0,
        };
        let vector = tuple::Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let expected_vector = tuple::Vector {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 0.0,
        };
        assert_eq!(point + vector, expected_vector);
    }

    #[test]
    fn test_vectors_and_points_can_be_added() {
        let point = tuple::Point {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let vector = tuple::Vector {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let expected_point = tuple::Point {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
        assert_eq!(point + vector, expected_point);
    }

    #[test]
    fn test_subtracting_two_points_becomes_a_vector() {
        let point1 = tuple::Point::new(3.0, 2.0, 1.0);
        let point2 = tuple::Point::new(5.0, 6.0, 7.0);

        let expected_vector = tuple::Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(point1 - point2, expected_vector);
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point_becomes_another_point() {
        let point1 = tuple::Point::new(3.0, 2.0, 1.0);
        let vector = tuple::Vector::new(5.0, 6.0, 7.0);

        let expected_point = tuple::Point::new(-2.0, -4.0, -6.0);
        assert_eq!(point1 - vector, expected_point);
    }

    #[test]
    fn test_subtracting_two_vectors() {
        let vector1 = tuple::Vector::new(3.0, 2.0, 1.0);
        let vector2 = tuple::Vector::new(5.0, 6.0, 7.0);

        let expected_vector = tuple::Vector::new(-2.0, -4.0, -6.0);
        assert_eq!(vector1 - vector2, expected_vector);
    }

    #[test]
    fn test_negation_of_vectors() {
        let vector1 = tuple::Vector {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_vector = tuple::Vector {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq!(-vector1, expected_vector);
    }

    #[test]
    fn test_mutiplication_by_a_scalar_of_a_vector() {
        let tuple1 = tuple::Vector {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_vector = tuple::Vector {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(tuple1 * 3.5, expected_vector);
    }

    #[test]
    fn test_division_by_a_scalar_of_a_vector() {
        let tuple1 = tuple::Vector {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_vector = tuple::Vector {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(tuple1 / 2.0, expected_vector);
    }

    #[test]
    fn test_magnitude_unit_vectors() {
        let vectorx = tuple::Vector::new(1.0, 0.0, 0.0);
        let vectory = tuple::Vector::new(0.0, 1.0, 0.0);
        let vectorz = tuple::Vector::new(0.0, 0.0, 1.0);
        assert_eq!(tuple::magnitude(&vectorx), 1.0);
        assert_eq!(tuple::magnitude(&vectory), 1.0);
        assert_eq!(tuple::magnitude(&vectorz), 1.0);
    }

    #[test]
    fn test_magnitude_positive_nonunit() {
        let vector1 = tuple::Vector::new(1.0, 2.0, 3.0);
        assert_eq!(tuple::magnitude(&vector1), 14.0_f64.sqrt());
    }

    #[test]
    fn test_magnitude_negitive_nonunit() {
        let vector1 = tuple::Vector::new(-1.0, -2.0, -3.0);
        assert_eq!(tuple::magnitude(&vector1), 14.0_f64.sqrt());
    }

    #[test]
    fn test_normalize_simple_vector() {
        let vector1 = tuple::Vector::new(4.0, 0.0, 0.0);
        assert_eq!(
            tuple::normalize(&vector1),
            tuple::Vector::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn test_normalize_non_simple_vector() {
        let vector1 = tuple::Vector::new(1.0, 2.0, 3.0);
        let expected_vector = tuple::Vector::new(
            1.0 / 14_f64.sqrt(),
            2.0 / 14_f64.sqrt(),
            3.0 / 14_f64.sqrt(),
        );
        assert_eq!(tuple::normalize(&vector1), expected_vector);
    }

    #[test]
    fn test_magnitude_of_normialized_vector_is_1() {
        let vector1 = tuple::Vector::new(1.0, 2.0, 3.0);
        assert_eq!(tuple::magnitude(&tuple::normalize(&vector1)), 1.0);
    }

    #[test]
    fn test_dot_product_of_two_vectors() {
        let vector1 = tuple::Vector::new(1.0, 2.0, 3.0);
        let vector2 = tuple::Vector::new(2.0, 3.0, 4.0);
        assert_eq!(tuple::dot(&vector1, &vector2), 20.0);
    }

    #[test]
    fn test_cross_product_of_two_vectors() {
        let vector1 = tuple::Vector::new(1.0, 2.0, 3.0);
        let vector2 = tuple::Vector::new(2.0, 3.0, 4.0);
        assert_eq!(
            tuple::cross(&vector1, &vector2),
            tuple::Vector::new(-1.0, 2.0, -1.0)
        );
        assert_eq!(
            tuple::cross(&vector2, &vector1),
            tuple::Vector::new(1.0, -2.0, 1.0)
        );
    }

    #[test]
    fn test_reflecting_a_vector_at_45_degrees() {
        let vector = tuple::Vector::new(1.0, -1.0, 0.0);
        let normal = tuple::Vector::new(0.0, 1.0, 0.0);

        let reflected = vector.reflect(&normal);

        let expected = tuple::Vector::new(1.0, 1.0, 0.0);
        assert_eq!(expected, reflected);
    }

    #[test]
    fn test_reflecting_a_vector_off_a_slanted_surface() {
        let vector = tuple::Vector::new(0.0, -1.0, 0.0);
        let normal = tuple::Vector::new(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let reflected = vector.reflect(&normal);

        let expected = tuple::Vector::new(1.0, 0.0, 0.0);
        assert_tuple_approx_eq!(expected, reflected);
    }
}
