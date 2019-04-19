use std::ops::{Index, Mul};

use crate::tuple;

#[derive(Debug, PartialEq)]
pub struct Matrix4 {
    m: [[f64; 4]; 4],
}

pub fn matrix4(
    m: (
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
        (f64, f64, f64, f64),
    ),
) -> Matrix4 {
    Matrix4 {
        m: [
            [(m.0).0, (m.0).1, (m.0).2, (m.0).3],
            [(m.1).0, (m.1).1, (m.1).2, (m.1).3],
            [(m.2).0, (m.2).1, (m.2).2, (m.2).3],
            [(m.3).0, (m.3).1, (m.3).2, (m.3).3],
        ],
    }
}

impl Index<(u64, u64)> for Matrix4 {
    type Output = f64;

    fn index(&self, key: (u64, u64)) -> &Self::Output {
        &self.m[key.0 as usize][key.1 as usize]
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = matrix4((
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
        ));

        for x in 0..4 {
            for y in 0..4 {
                result.m[x][y] = self.m[x][0] * other.m[0][y]
                    + self.m[x][1] * other.m[1][y]
                    + self.m[x][2] * other.m[2][y]
                    + self.m[x][3] * other.m[3][y];
                ;
            }
        }
        result
    }
}

impl Mul<tuple::Tuple> for Matrix4 {
    type Output = tuple::Tuple;

    fn mul(self, other: tuple::Tuple) -> tuple::Tuple {
        let mut result = tuple::Tuple {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };

        result.x = self.m[0][0] * other.x
            + self.m[0][1] * other.y
            + self.m[0][2] * other.z
            + self.m[0][3] * other.w;
        result.y = self.m[1][0] * other.x
            + self.m[1][1] * other.y
            + self.m[1][2] * other.z
            + self.m[1][3] * other.w;
        result.z = self.m[2][0] * other.x
            + self.m[2][1] * other.y
            + self.m[2][2] * other.z
            + self.m[2][3] * other.w;
        result.w = self.m[3][0] * other.x
            + self.m[3][1] * other.y
            + self.m[3][2] * other.z
            + self.m[3][3] * other.w;
        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix3 {
    m: [[f64; 3]; 3],
}

pub fn matrix3(m: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))) -> Matrix3 {
    Matrix3 {
        m: [
            [(m.0).0, (m.0).1, (m.0).2],
            [(m.1).0, (m.1).1, (m.1).2],
            [(m.2).0, (m.2).1, (m.2).2],
        ],
    }
}

impl Index<(u64, u64)> for Matrix3 {
    type Output = f64;

    fn index(&self, key: (u64, u64)) -> &Self::Output {
        &self.m[key.0 as usize][key.1 as usize]
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix2 {
    m: [[f64; 2]; 2],
}

pub fn matrix2(m: ((f64, f64), (f64, f64))) -> Matrix2 {
    Matrix2 {
        m: [[(m.0).0, (m.0).1], [(m.1).0, (m.1).1]],
    }
}

impl Index<(u64, u64)> for Matrix2 {
    type Output = f64;

    fn index(&self, key: (u64, u64)) -> &Self::Output {
        &self.m[key.0 as usize][key.1 as usize]
    }
}

#[cfg(test)]
mod tuple_tests {
    use crate::matrix;
    use crate::tuple;

    #[test]
    fn test_constructor_4_by_4() {
        let matrix1 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (5.5, 6.5, 7.5, 8.5),
            (9.0, 10.0, 11.0, 12.0),
            (13.5, 14.5, 15.5, 16.5),
        ));
        assert_eq!(matrix1[(0, 0)], 1.0);
        assert_eq!(matrix1[(0, 1)], 2.0);
        assert_eq!(matrix1[(0, 2)], 3.0);
        assert_eq!(matrix1[(0, 3)], 4.0);

        assert_eq!(matrix1[(1, 0)], 5.5);
        assert_eq!(matrix1[(1, 1)], 6.5);
        assert_eq!(matrix1[(1, 2)], 7.5);
        assert_eq!(matrix1[(1, 3)], 8.5);

        assert_eq!(matrix1[(2, 0)], 9.0);
        assert_eq!(matrix1[(2, 1)], 10.0);
        assert_eq!(matrix1[(2, 2)], 11.0);
        assert_eq!(matrix1[(2, 3)], 12.0);

        assert_eq!(matrix1[(3, 0)], 13.5);
        assert_eq!(matrix1[(3, 1)], 14.5);
        assert_eq!(matrix1[(3, 2)], 15.5);
        assert_eq!(matrix1[(3, 3)], 16.5);
    }

    #[test]
    fn test_constructor_3_by_3() {
        let matrix1 = matrix::matrix3(((1.0, 2.0, 3.0), (5.5, 6.5, 7.5), (9.0, 10.0, 11.0)));
        assert_eq!(matrix1[(0, 0)], 1.0);
        assert_eq!(matrix1[(0, 1)], 2.0);
        assert_eq!(matrix1[(0, 2)], 3.0);

        assert_eq!(matrix1[(1, 0)], 5.5);
        assert_eq!(matrix1[(1, 1)], 6.5);
        assert_eq!(matrix1[(1, 2)], 7.5);

        assert_eq!(matrix1[(2, 0)], 9.0);
        assert_eq!(matrix1[(2, 1)], 10.0);
        assert_eq!(matrix1[(2, 2)], 11.0);
    }

    #[test]
    fn test_constructor_2_by_2() {
        let matrix1 = matrix::matrix2(((1.0, 2.0), (5.5, 6.5)));
        assert_eq!(matrix1[(0, 0)], 1.0);
        assert_eq!(matrix1[(0, 1)], 2.0);

        assert_eq!(matrix1[(1, 0)], 5.5);
        assert_eq!(matrix1[(1, 1)], 6.5);
    }

    #[test]
    fn test_4_by_4_eq() {
        let matrix1 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_4_by_4_ne() {
        let matrix1 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::matrix4((
            (2.0, 3.0, 4.0, 5.0),
            (6.0, 7.0, 8.0, 9.0),
            (8.0, 7.0, 6.0, 5.0),
            (4.0, 3.0, 2.0, 1.0),
        ));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_3_by_3_eq() {
        let matrix1 = matrix::matrix3(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));
        let matrix2 = matrix::matrix3(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_3_by_3_ne() {
        let matrix1 = matrix::matrix3(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));
        let matrix2 = matrix::matrix3(((2.0, 3.0, 4.0), (6.0, 7.0, 8.0), (8.0, 7.0, 6.0)));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_2_by_2_eq() {
        let matrix1 = matrix::matrix2(((1.0, 2.0), (5.0, 6.0)));
        let matrix2 = matrix::matrix2(((1.0, 2.0), (5.0, 6.0)));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_2_by_2_ne() {
        let matrix1 = matrix::matrix2(((1.0, 2.0), (5.0, 6.0)));
        let matrix2 = matrix::matrix2(((2.0, 3.0), (6.0, 7.0)));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_matrix_multiplication() {
        let matrix1 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::matrix4((
            (-2.0, 1.0, 2.0, 3.0),
            (3.0, 2.0, 1.0, -1.0),
            (4.0, 3.0, 6.0, 5.0),
            (1.0, 2.0, 7.0, 8.0),
        ));

        let matrix3 = matrix1 * matrix2;

        assert_eq!(
            matrix3,
            matrix::matrix4((
                (20.0, 22.0, 50.0, 48.0),
                (44.0, 54.0, 114.0, 108.0),
                (40.0, 58.0, 110.0, 102.0),
                (16.0, 26.0, 46.0, 42.0),
            ))
        );
    }

    #[test]
    fn test_tuple_multiplication() {
        let matrix1 = matrix::matrix4((
            (1.0, 2.0, 3.0, 4.0),
            (2.0, 4.0, 4.0, 2.0),
            (8.0, 6.0, 4.0, 1.0),
            (0.0, 0.0, 0.0, 1.0),
        ));
        let tuple1 = tuple::Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        let result: tuple::Tuple = matrix1 * tuple1;

        assert_eq!(
            result,
            tuple::Tuple {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0,
            }
        );
    }
}
