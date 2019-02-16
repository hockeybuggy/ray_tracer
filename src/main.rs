// TODO remove dead code exception
#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, PartialEq)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Tuple {
    fn is_point(&self) -> bool {
        return self.w == 1.0;
    }

    fn is_vector(&self) -> bool {
        return self.w == 0.0;
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
        }
    }
}

fn point(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 1.0 }
}

fn vector(x: f64, y: f64, z: f64) -> Tuple {
    Tuple { x, y, z, w: 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_tuple_with_w_1_is_a_point() {
        let point = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 1.0,
        };

        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);

        assert_eq!(point.is_point(), true);
        assert_eq!(point.is_vector(), false);
    }

    #[test]
    fn test_a_tuple_with_w_0_is_a_vector() {
        let vector = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: 0.0,
        };

        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);

        assert_eq!(vector.is_point(), false);
        assert_eq!(vector.is_vector(), true);
    }

    #[test]
    fn test_point_creates_tuples_with_w_1() {
        let point = point(4.0, -4.0, 3.0);

        let expected_tuple = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 1.0,
        };
        assert_eq!(point, expected_tuple);
    }

    #[test]
    fn test_vector_creates_tuples_with_w_0() {
        let vector = vector(4.0, -4.0, 3.0);

        let expected_tuple = Tuple {
            x: 4.0,
            y: -4.0,
            z: 3.0,
            w: 0.0,
        };
        assert_eq!(vector, expected_tuple);
    }

    #[test]
    fn test_tuples_can_be_added() {
        let tuple1 = Tuple {
            x: 3.0,
            y: -2.0,
            z: 5.0,
            w: 1.0,
        };
        let tuple2 = Tuple {
            x: -2.0,
            y: 3.0,
            z: 1.0,
            w: 0.0,
        };

        let expected_tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 6.0,
            w: 1.0,
        };
        assert_eq!(tuple1 + tuple2, expected_tuple);
    }

    #[test]
    fn test_subtracting_two_points_becomes_a_vector() {
        let point1 = point(3.0, 2.0, 1.0);
        let point2 = point(5.0, 6.0, 7.0);

        let expected_vector = vector(-2.0, -4.0, -6.0);
        assert_eq!(point1 - point2, expected_vector);
    }

    #[test]
    fn test_subtracting_a_vector_from_a_point_becomes_another_point() {
        let point1 = point(3.0, 2.0, 1.0);
        let vector = vector(5.0, 6.0, 7.0);

        let expected_point = point(-2.0, -4.0, -6.0);
        assert_eq!(point1 - vector, expected_point);
    }

    fn test_subtracting_two_vectors() {
        let vector1 = vector(3.0, 2.0, 1.0);
        let vector2 = vector(5.0, 6.0, 7.0);

        let expected_vector = vector(-2.0, -4.0, -6.0);
        assert_eq!(vector1 - vector2, expected_vector);
    }

    #[test]
    fn test_negation_of_tuples() {
        let tuple1 = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_tuple = Tuple {
            x: -1.0,
            y: 2.0,
            z: -3.0,
            w: 4.0,
        };
        assert_eq!(-tuple1, expected_tuple);
    }

    #[test]
    fn test_mutiplication_by_a_scalar() {
        let tuple1 = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_tuple = Tuple {
            x: 3.5,
            y: -7.0,
            z: 10.5,
            w: -14.0,
        };
        assert_eq!(tuple1 * 3.5, expected_tuple);
    }

    #[test]
    fn test_devision_by_a_scalar() {
        let tuple1 = Tuple {
            x: 1.0,
            y: -2.0,
            z: 3.0,
            w: -4.0,
        };

        let expected_tuple = Tuple {
            x: 0.5,
            y: -1.0,
            z: 1.5,
            w: -2.0,
        };
        assert_eq!(tuple1 / 2.0, expected_tuple);
    }
}
