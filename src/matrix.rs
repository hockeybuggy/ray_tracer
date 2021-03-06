use std::ops::{Index, Mul};

use crate::tuple;

pub trait Transpose {
    fn transpose(&self) -> Self;
}

trait Submatrix {
    type Submatrix;
    fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Self::Submatrix;
}

trait Determinant {
    fn determinant(&self) -> f64;
}

trait Minor {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64;
}

trait Cofactor {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64;
}

pub trait Inverse: Sized {
    fn is_invertible(&self) -> bool;
    fn inverse(&self) -> Option<Self>;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    m: [[f64; 4]; 4],
}

impl Matrix4 {
    const SIZE: usize = 4;
    pub const IDENTITY: Self = Self {
        m: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ],
    };

    pub fn new(
        m: (
            (f64, f64, f64, f64),
            (f64, f64, f64, f64),
            (f64, f64, f64, f64),
            (f64, f64, f64, f64),
        ),
    ) -> Self {
        Self {
            m: [
                [(m.0).0, (m.0).1, (m.0).2, (m.0).3],
                [(m.1).0, (m.1).1, (m.1).2, (m.1).3],
                [(m.2).0, (m.2).1, (m.2).2, (m.2).3],
                [(m.3).0, (m.3).1, (m.3).2, (m.3).3],
            ],
        }
    }

    pub fn empty() -> Self {
        Self::new((
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
        ))
    }
}

impl Index<(usize, usize)> for Matrix4 {
    type Output = f64;

    fn index(&self, key: (usize, usize)) -> &Self::Output {
        &self.m[key.0][key.1]
    }
}

impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = Self::empty();

        for x in 0..4 {
            for y in 0..4 {
                result.m[x][y] = self[(x, 0)] * other[(0, y)]
                    + self[(x, 1)] * other[(1, y)]
                    + self[(x, 2)] * other[(2, y)]
                    + self[(x, 3)] * other[(3, y)];
            }
        }
        result
    }
}

fn mul_matrix4_by_tuple(matrix: &Matrix4, x: f64, y: f64, z: f64, w: f64) -> (f64, f64, f64, f64) {
    let nx = matrix[(0, 0)] * x + matrix[(0, 1)] * y + matrix[(0, 2)] * z + matrix[(0, 3)] * w;
    let ny = matrix[(1, 0)] * x + matrix[(1, 1)] * y + matrix[(1, 2)] * z + matrix[(1, 3)] * w;
    let nz = matrix[(2, 0)] * x + matrix[(2, 1)] * y + matrix[(2, 2)] * z + matrix[(2, 3)] * w;
    let nw = matrix[(3, 0)] * x + matrix[(3, 1)] * y + matrix[(3, 2)] * z + matrix[(3, 3)] * w;
    (nx, ny, nz, nw)
}

impl Mul<tuple::Point> for Matrix4 {
    type Output = tuple::Point;

    fn mul(self, other: tuple::Point) -> tuple::Point {
        let mut result = tuple::Point::new(0.0, 0.0, 0.0);
        let (x, y, z, w) = mul_matrix4_by_tuple(&self, other.x, other.y, other.z, other.w);
        result.x = x;
        result.y = y;
        result.z = z;
        result.w = w;
        return result;
    }
}

impl Mul<tuple::Vector> for Matrix4 {
    type Output = tuple::Vector;

    fn mul(self, other: tuple::Vector) -> tuple::Vector {
        let mut result = tuple::Vector::new(0.0, 0.0, 0.0);
        let (x, y, z, w) = mul_matrix4_by_tuple(&self, other.x, other.y, other.z, other.w);
        result.x = x;
        result.y = y;
        result.z = z;
        result.w = w;
        return result;
    }
}

impl Transpose for Matrix4 {
    fn transpose(&self) -> Self {
        Self::new((
            (self[(0, 0)], self[(1, 0)], self[(2, 0)], self[(3, 0)]),
            (self[(0, 1)], self[(1, 1)], self[(2, 1)], self[(3, 1)]),
            (self[(0, 2)], self[(1, 2)], self[(2, 2)], self[(3, 2)]),
            (self[(0, 3)], self[(1, 3)], self[(2, 3)], self[(3, 3)]),
        ))
    }
}

impl Cofactor for Matrix4 {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        let minor = self.minor(row_to_exclude, col_to_exclude);
        if (row_to_exclude + col_to_exclude) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Determinant for Matrix4 {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for x in 0..4 {
            determinant = determinant + self[(0, x)] * self.cofactor(0, x);
        }
        determinant
    }
}

impl Inverse for Matrix4 {
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Option<Matrix4> {
        if !self.is_invertible() {
            return None;
        }

        let mut inverse = Matrix4::empty();

        let determinant = self.determinant();
        for x in 0..4 {
            for y in 0..4 {
                let c = self.cofactor(y, x);
                inverse.m[x as usize][y as usize] = c / determinant;
            }
        }

        Some(inverse)
    }
}

impl Submatrix for Matrix4 {
    type Submatrix = Matrix3;

    fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Self::Submatrix {
        let mut result = Self::Submatrix::empty();
        let mut curr_row = 0;
        let mut curr_col = 0;
        for x in 0..Self::SIZE {
            if x == row_to_exclude {
                continue;
            }
            for y in 0..Self::SIZE {
                if y == col_to_exclude {
                    continue;
                }
                result.m[curr_row][curr_col] = self.m[x as usize][y as usize];
                curr_col = curr_col + 1;
            }
            curr_col = 0;
            curr_row = curr_row + 1;
        }
        result
    }
}

impl Minor for Matrix4 {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        self.submatrix(row_to_exclude, col_to_exclude).determinant()
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix3 {
    m: [[f64; 3]; 3],
}

impl Matrix3 {
    const SIZE: usize = 3;

    pub fn new(m: ((f64, f64, f64), (f64, f64, f64), (f64, f64, f64))) -> Self {
        Self {
            m: [
                [(m.0).0, (m.0).1, (m.0).2],
                [(m.1).0, (m.1).1, (m.1).2],
                [(m.2).0, (m.2).1, (m.2).2],
            ],
        }
    }

    fn empty() -> Self {
        Self::new(((0.0, 0.0, 0.0), (0.0, 0.0, 0.0), (0.0, 0.0, 0.0)))
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f64;

    fn index(&self, key: (usize, usize)) -> &Self::Output {
        &self.m[key.0][key.1]
    }
}

impl Submatrix for Matrix3 {
    type Submatrix = Matrix2;

    fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Self::Submatrix {
        let mut result = Self::Submatrix::empty();
        let mut curr_row = 0;
        let mut curr_col = 0;
        for x in 0..Self::SIZE {
            if x == row_to_exclude {
                continue;
            }
            for y in 0..Self::SIZE {
                if y == col_to_exclude {
                    continue;
                }
                result.m[curr_row][curr_col] = self[(x, y)];
                curr_col = curr_col + 1;
            }
            curr_col = 0;
            curr_row = curr_row + 1;
        }
        result
    }
}

impl Minor for Matrix3 {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        self.submatrix(row_to_exclude, col_to_exclude).determinant()
    }
}

impl Cofactor for Matrix3 {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        let minor = self.minor(row_to_exclude, col_to_exclude);
        if (row_to_exclude + col_to_exclude) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Determinant for Matrix3 {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for x in 0..3 {
            determinant = determinant + self[(0, x)] * self.cofactor(0, x);
        }
        determinant
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix2 {
    m: [[f64; 2]; 2],
}

impl Matrix2 {
    pub fn new(m: ((f64, f64), (f64, f64))) -> Self {
        Self {
            m: [[(m.0).0, (m.0).1], [(m.1).0, (m.1).1]],
        }
    }

    fn empty() -> Self {
        Self::new(((0.0, 0.0), (0.0, 0.0)))
    }
}

impl Index<(u64, u64)> for Matrix2 {
    type Output = f64;

    fn index(&self, key: (u64, u64)) -> &Self::Output {
        &self.m[key.0 as usize][key.1 as usize]
    }
}

impl Determinant for Matrix2 {
    fn determinant(&self) -> f64 {
        self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::assert_matrix_approx_eq;
    use crate::matrix;
    use crate::matrix::{Cofactor, Determinant, Inverse, Minor, Submatrix, Transpose};
    use crate::tuple;

    #[test]
    fn test_constructor_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
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
        let matrix1 = matrix::Matrix3::new(((1.0, 2.0, 3.0), (5.5, 6.5, 7.5), (9.0, 10.0, 11.0)));
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
        let matrix1 = matrix::Matrix2::new(((1.0, 2.0), (5.5, 6.5)));
        assert_eq!(matrix1[(0, 0)], 1.0);
        assert_eq!(matrix1[(0, 1)], 2.0);

        assert_eq!(matrix1[(1, 0)], 5.5);
        assert_eq!(matrix1[(1, 1)], 6.5);
    }

    #[test]
    fn test_4_by_4_eq() {
        let matrix1 = matrix::Matrix4::new((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::Matrix4::new((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_4_by_4_ne() {
        let matrix1 = matrix::Matrix4::new((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::Matrix4::new((
            (2.0, 3.0, 4.0, 5.0),
            (6.0, 7.0, 8.0, 9.0),
            (8.0, 7.0, 6.0, 5.0),
            (4.0, 3.0, 2.0, 1.0),
        ));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_3_by_3_eq() {
        let matrix1 = matrix::Matrix3::new(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));
        let matrix2 = matrix::Matrix3::new(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_3_by_3_ne() {
        let matrix1 = matrix::Matrix3::new(((1.0, 2.0, 3.0), (5.0, 6.0, 7.0), (9.0, 8.0, 7.0)));
        let matrix2 = matrix::Matrix3::new(((2.0, 3.0, 4.0), (6.0, 7.0, 8.0), (8.0, 7.0, 6.0)));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_2_by_2_eq() {
        let matrix1 = matrix::Matrix2::new(((1.0, 2.0), (5.0, 6.0)));
        let matrix2 = matrix::Matrix2::new(((1.0, 2.0), (5.0, 6.0)));

        assert_eq!(matrix1, matrix2);
    }

    #[test]
    fn test_2_by_2_ne() {
        let matrix1 = matrix::Matrix2::new(((1.0, 2.0), (5.0, 6.0)));
        let matrix2 = matrix::Matrix2::new(((2.0, 3.0), (6.0, 7.0)));

        assert_ne!(matrix1, matrix2);
    }

    #[test]
    fn test_matrix_multiplication() {
        let matrix1 = matrix::Matrix4::new((
            (1.0, 2.0, 3.0, 4.0),
            (5.0, 6.0, 7.0, 8.0),
            (9.0, 8.0, 7.0, 6.0),
            (5.0, 4.0, 3.0, 2.0),
        ));
        let matrix2 = matrix::Matrix4::new((
            (-2.0, 1.0, 2.0, 3.0),
            (3.0, 2.0, 1.0, -1.0),
            (4.0, 3.0, 6.0, 5.0),
            (1.0, 2.0, 7.0, 8.0),
        ));

        let matrix3 = matrix1 * matrix2;

        assert_eq!(
            matrix3,
            matrix::Matrix4::new((
                (20.0, 22.0, 50.0, 48.0),
                (44.0, 54.0, 114.0, 108.0),
                (40.0, 58.0, 110.0, 102.0),
                (16.0, 26.0, 46.0, 42.0),
            ))
        );
    }

    #[test]
    fn test_tuple_multiplication() {
        let matrix1 = matrix::Matrix4::new((
            (1.0, 2.0, 3.0, 4.0),
            (2.0, 4.0, 4.0, 2.0),
            (8.0, 6.0, 4.0, 1.0),
            (0.0, 0.0, 0.0, 1.0),
        ));
        let tuple1 = tuple::Point {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        let result: tuple::Point = matrix1 * tuple1;

        assert_eq!(
            result,
            tuple::Point {
                x: 18.0,
                y: 24.0,
                z: 33.0,
                w: 1.0,
            }
        );
    }

    #[test]
    fn test_identity_matrix() {
        let matrix1 = matrix::Matrix4::new((
            (0.0, 1.0, 2.0, 3.0),
            (1.0, 2.0, 4.0, 8.0),
            (2.0, 4.0, 8.0, 16.0),
            (4.0, 8.0, 16.0, 32.0),
        ));

        let result = matrix1 * matrix::Matrix4::IDENTITY;

        assert_eq!(
            result,
            matrix::Matrix4::new((
                (0.0, 1.0, 2.0, 3.0),
                (1.0, 2.0, 4.0, 8.0),
                (2.0, 4.0, 8.0, 16.0),
                (4.0, 8.0, 16.0, 32.0),
            ))
        );
    }

    #[test]
    fn test_transpose_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (0.0, 9.0, 3.0, 0.0),
            (9.0, 8.0, 0.0, 8.0),
            (1.0, 8.0, 5.0, 3.0),
            (0.0, 0.0, 5.0, 8.0),
        ));

        let result = matrix1.transpose();

        assert_eq!(
            result,
            matrix::Matrix4::new((
                (0.0, 9.0, 1.0, 0.0),
                (9.0, 8.0, 8.0, 0.0),
                (3.0, 0.0, 5.0, 5.0),
                (0.0, 8.0, 3.0, 8.0),
            ))
        );
    }

    #[test]
    fn test_transpose_identity_matrix() {
        assert_eq!(
            matrix::Matrix4::IDENTITY.transpose(),
            matrix::Matrix4::IDENTITY,
        )
    }

    #[test]
    fn test_determinant_of_2_by_2() {
        let matrix1 = matrix::Matrix2::new(((1.0, 5.0), (-3.0, 2.0)));

        assert_eq!(17.0, matrix1.determinant());
    }

    #[test]
    fn test_submatrix_of_3_by_3() {
        let matrix1 = matrix::Matrix3::new(((1.0, 5.0, 0.0), (-3.0, 2.0, 7.0), (0.0, 6.0, -3.0)));
        let expected = matrix::Matrix2::new(((-3.0, 2.0), (0.0, 6.0)));

        assert_eq!(expected, matrix1.submatrix(0, 2));
    }

    #[test]
    fn test_submatrix_of_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (-6.0, 1.0, 1.0, 6.0),
            (-8.0, 5.0, 8.0, 6.0),
            (-1.0, 0.0, 8.0, 2.0),
            (-7.0, 1.0, -1.0, 1.0),
        ));
        let expected =
            matrix::Matrix3::new(((-6.0, 1.0, 6.0), (-8.0, 8.0, 6.0), (-7.0, -1.0, 1.0)));

        assert_eq!(expected, matrix1.submatrix(2, 1));
    }

    #[test]
    fn test_minor_of_3_by_3() {
        let matrix1 = matrix::Matrix3::new(((3.0, 5.0, 0.0), (2.0, -1.0, -7.0), (6.0, -1.0, 5.0)));
        let matrix2 = matrix1.submatrix(1, 0);

        assert_eq!(25.0, matrix2.determinant());
        assert_eq!(25.0, matrix1.minor(1, 0));
    }

    #[test]
    fn test_confactor_of_a_3_by_3() {
        let matrix1 = matrix::Matrix3::new(((3.0, 5.0, 0.0), (2.0, -1.0, -7.0), (6.0, -1.0, 5.0)));

        assert_eq!(-12.0, matrix1.minor(0, 0));
        assert_eq!(-12.0, matrix1.cofactor(0, 0));
        assert_eq!(25.0, matrix1.minor(1, 0));
        assert_eq!(-25.0, matrix1.cofactor(1, 0));
    }

    #[test]
    fn test_cofactor_of_a_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (-2.0, -8.0, 3.0, 5.0),
            (-3.0, 1.0, 7.0, 3.0),
            (1.0, 2.0, -9.0, 6.0),
            (-6.0, 7.0, 7.0, -9.0),
        ));

        assert_eq!(690.0, matrix1.cofactor(0, 0));
        assert_eq!(447.0, matrix1.cofactor(0, 1));
        assert_eq!(210.0, matrix1.cofactor(0, 2));
        assert_eq!(51.0, matrix1.cofactor(0, 3));
    }

    #[test]
    fn test_determinant_of_a_3_by_3() {
        let matrix1 = matrix::Matrix3::new(((1.0, 2.0, 6.0), (-5.0, 8.0, -4.0), (2.0, 6.0, 4.0)));

        assert_eq!(56.0, matrix1.cofactor(0, 0));
        assert_eq!(12.0, matrix1.cofactor(0, 1));
        assert_eq!(-46.0, matrix1.cofactor(0, 2));
        assert_eq!(-196.0, matrix1.determinant());
    }

    #[test]
    fn test_determinant_of_a_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (-2.0, -8.0, 3.0, 5.0),
            (-3.0, 1.0, 7.0, 3.0),
            (1.0, 2.0, -9.0, 6.0),
            (-6.0, 7.0, 7.0, -9.0),
        ));

        assert_eq!(-4071.0, matrix1.determinant());
    }

    #[test]
    fn test_is_invertible_of_invertible_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (6.0, 4.0, 4.0, 4.0),
            (5.0, 5.0, 7.0, 6.0),
            (4.0, -9.0, 3.0, -7.0),
            (9.0, 1.0, 7.0, -6.0),
        ));

        assert_eq!(-2120.0, matrix1.determinant());
        assert_eq!(true, matrix1.is_invertible());
    }

    #[test]
    fn test_is_invertible_of_non_invertible_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (-4.0, 2.0, -2.0, -3.0),
            (9.0, 6.0, 2.0, 6.0),
            (0.0, -5.0, 1.0, -5.0),
            (0.0, 0.0, 0.0, 0.0),
        ));

        assert_eq!(0.0, matrix1.determinant());
        assert_eq!(false, matrix1.is_invertible());
    }

    #[test]
    fn test_inverse_of_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (-5.0, 2.0, 6.0, -8.0),
            (1.0, -5.0, 1.0, 8.0),
            (7.0, 7.0, -6.0, -7.0),
            (1.0, -3.0, 7.0, 4.0),
        ));

        let inverse = matrix1.inverse().unwrap();

        assert_eq!(532.0, matrix1.determinant());
        assert_eq!(-160.0, matrix1.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, inverse.m[3][2]);

        assert_eq!(105.0, matrix1.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, inverse.m[2][3]);
        let expected = matrix::Matrix4::new((
            (0.21805, 0.45113, 0.24060, -0.04511),
            (-0.80827, -1.45677, -0.44361, 0.52068),
            (-0.0789475, -0.22368, -0.05263, 0.19737),
            (-0.52256, -0.81391, -0.30075, 0.30639),
        ));
        assert_matrix_approx_eq!(expected, inverse);
    }

    #[test]
    fn test_inverse_of_another_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (8.0, -5.0, 9.0, 2.0),
            (7.0, 5.0, 6.0, 1.0),
            (-6.0, 0.0, 9.0, 6.0),
            (-3.0, 0.0, -9.0, -4.0),
        ));

        let inverse = matrix1.inverse().unwrap();

        let expected = matrix::Matrix4::new((
            (-0.15385, -0.15385, -0.28205, -0.53846),
            (-0.07692, 0.12308, 0.02564, 0.03077),
            (0.35897, 0.35897, 0.43590, 0.92308),
            (-0.6923, -0.6923, -0.76923, -1.92308),
        ));
        assert_matrix_approx_eq!(expected, inverse);
    }

    #[test]
    fn test_inverse_of_a_third_4_by_4() {
        let matrix1 = matrix::Matrix4::new((
            (9.0, 3.0, 0.0, 9.0),
            (-5.0, -2.0, -6.0, -3.0),
            (-4.0, 9.0, 6.0, 4.0),
            (-7.0, 6.0, 6.0, 2.0),
        ));

        let inverse = matrix1.inverse().unwrap();

        let expected = matrix::Matrix4::new((
            (-0.04074, -0.07778, 0.14444, -0.22222),
            (-0.07778, 0.03333, 0.36667, -0.33333),
            (-0.02901, -0.14630, -0.10926, 0.12963),
            (0.17778, 0.06667, -0.26667, 0.33333),
        ));
        assert_matrix_approx_eq!(expected, inverse);
    }

    #[test]
    fn test_non_invertable_returns_error() {
        let matrix1 = matrix::Matrix4::new((
            (-4.0, 2.0, -2.0, -3.0),
            (9.0, 6.0, 2.0, 6.0),
            (0.0, -5.0, 1.0, -5.0),
            (0.0, 0.0, 0.0, 0.0),
        ));
        let uninvertable_error = matrix1.inverse();

        assert_eq!(uninvertable_error, None);
    }

    #[test]
    fn test_multiply_a_product_by_its_inverse() {
        let matrix1 = matrix::Matrix4::new((
            (3.0, -9.0, 7.0, 3.0),
            (3.0, -8.0, 2.0, -9.0),
            (-4.0, 4.0, 4.0, 1.0),
            (-6.0, 5.0, -1.0, 1.0),
        ));
        let matrix2 = matrix::Matrix4::new((
            (8.0, 2.0, 2.0, 2.0),
            (3.0, -1.0, 7.0, 0.0),
            (7.0, 0.0, 5.0, 4.0),
            (6.0, -2.0, 0.0, 5.0),
        ));

        let product = matrix1 * matrix2;
        let product_times_inverse = product * matrix2.inverse().unwrap();
        assert_matrix_approx_eq!(product_times_inverse, matrix1);
    }
}
