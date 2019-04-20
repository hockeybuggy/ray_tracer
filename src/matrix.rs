use std::ops::{Index, Mul};

use crate::tuple;

trait Transposeable {
    fn transpose(&self) -> Self;
}

trait Submatrixable {
    type Submatrix;
    fn submatrix(&self, row_to_exclude: usize, col_to_exclude: usize) -> Self::Submatrix;
}

trait Determinable {
    fn determinant(&self) -> f64;
}

trait Minorable {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64;
}

trait Cofactorable {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4 {
    m: [[f64; 4]; 4],
}

impl Matrix4 {
    const SIZE: usize = 4;

    fn new(
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

    fn empty() -> Self {
        Self::new((
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
            (0.0, 0.0, 0.0, 0.0),
        ))
    }
}

const IDENTITY_MATRIX: Matrix4 = Matrix4 {
    m: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

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

impl Transposeable for Matrix4 {
    fn transpose(&self) -> Self {
        Self::new((
            (self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]),
            (self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]),
            (self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]),
            (self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]),
        ))
    }
}

impl Determinable for Matrix2 {
    fn determinant(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1] * self.m[1][0]
    }
}

impl Submatrixable for Matrix3 {
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
                result.m[curr_row][curr_col] = self.m[x as usize][y as usize];
                curr_col = curr_col + 1;
            }
            curr_col = 0;
            curr_row = curr_row + 1;
        }
        result
    }
}

impl Submatrixable for Matrix4 {
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

impl Minorable for Matrix3 {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        self.submatrix(row_to_exclude, col_to_exclude).determinant()
    }
}

impl Minorable for Matrix4 {
    fn minor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        self.submatrix(row_to_exclude, col_to_exclude).determinant()
    }
}

impl Cofactorable for Matrix3 {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        let minor = self.minor(row_to_exclude, col_to_exclude);
        if (row_to_exclude + col_to_exclude) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Determinable for Matrix3 {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for x in 0..3 {
            determinant = determinant + self.m[0][x] * self.cofactor(0, x);
        }
        determinant
    }
}

impl Cofactorable for Matrix4 {
    fn cofactor(&self, row_to_exclude: usize, col_to_exclude: usize) -> f64 {
        let minor = self.minor(row_to_exclude, col_to_exclude);
        if (row_to_exclude + col_to_exclude) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }
}

impl Determinable for Matrix4 {
    fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        for x in 0..4 {
            determinant = determinant + self.m[0][x] * self.cofactor(0, x);
        }
        determinant
    }
}

fn is_invertible4(input: &Matrix4) -> bool {
    input.determinant() != 0.0
}

fn inverse4(input: &Matrix4) -> Result<Matrix4, &'static str> {
    if !is_invertible4(&input) {
        return Err("uninvertable_error");
    }

    let mut inverse = Matrix4::empty();

    let determinant = input.determinant();
    for x in 0..4 {
        for y in 0..4 {
            let c = input.cofactor(y, x);
            inverse.m[x as usize][y as usize] = c / determinant;
        }
    }

    Ok(inverse)
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

#[derive(Debug, PartialEq)]
pub struct Matrix2 {
    m: [[f64; 2]; 2],
}

impl Matrix2 {
    const SIZE: usize = 2;

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

#[cfg(test)]
mod matrix_tests {
    use std::error::Error;

    use assert_approx_eq::assert_approx_eq;

    use crate::matrix;
    use crate::matrix::{Cofactorable, Determinable, Minorable, Submatrixable, Transposeable};
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

    #[test]
    fn test_identity_matrix() {
        let matrix1 = matrix::Matrix4::new((
            (0.0, 1.0, 2.0, 3.0),
            (1.0, 2.0, 4.0, 8.0),
            (2.0, 4.0, 8.0, 16.0),
            (4.0, 8.0, 16.0, 32.0),
        ));

        let result = matrix1 * matrix::IDENTITY_MATRIX;

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
        assert_eq!(matrix::IDENTITY_MATRIX.transpose(), matrix::IDENTITY_MATRIX,)
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
        assert_eq!(true, matrix::is_invertible4(&matrix1));
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
        assert_eq!(false, matrix::is_invertible4(&matrix1));
    }

    #[test]
    fn test_inverse_of_4_by_4() -> Result<(), Box<Error>> {
        let matrix1 = matrix::Matrix4::new((
            (-5.0, 2.0, 6.0, -8.0),
            (1.0, -5.0, 1.0, 8.0),
            (7.0, 7.0, -6.0, -7.0),
            (1.0, -3.0, 7.0, 4.0),
        ));

        let inverse = matrix::inverse4(&matrix1)?;

        assert_eq!(532.0, matrix1.determinant());
        assert_eq!(-160.0, matrix1.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, inverse.m[3][2]);

        assert_eq!(105.0, matrix1.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, inverse.m[2][3]);
        let expected = matrix::Matrix4::new((
            (
                0.21804511278195488,
                0.45112781954887216,
                0.24060150375939848,
                -0.045112781954887216,
            ),
            (
                -0.8082706766917294,
                -1.4567669172932332,
                -0.44360902255639095,
                0.5206766917293233,
            ),
            (
                -0.07894736842105263,
                -0.2236842105263158,
                -0.05263157894736842,
                0.19736842105263158,
            ),
            (
                -0.5225563909774437,
                -0.8139097744360902,
                -0.3007518796992481,
                0.30639097744360905,
            ),
        ));
        assert_eq!(expected, inverse);
        Ok(())
    }

    #[test]
    fn test_inverse_of_another_4_by_4() -> Result<(), Box<Error>> {
        let matrix1 = matrix::Matrix4::new((
            (8.0, -5.0, 9.0, 2.0),
            (7.0, 5.0, 6.0, 1.0),
            (-6.0, 0.0, 9.0, 6.0),
            (-3.0, 0.0, -9.0, -4.0),
        ));

        let inverse = matrix::inverse4(&matrix1)?;

        let expected = matrix::Matrix4::new((
            (
                -0.15384615384615385,
                -0.15384615384615385,
                -0.28205128205128205,
                -0.5384615384615384,
            ),
            (
                -0.07692307692307693,
                0.12307692307692308,
                0.02564102564102564,
                0.03076923076923077,
            ),
            (
                0.358974358974359,
                0.358974358974359,
                0.4358974358974359,
                0.9230769230769231,
            ),
            (
                -0.6923076923076923,
                -0.6923076923076923,
                -0.7692307692307693,
                -1.9230769230769231,
            ),
        ));
        assert_eq!(expected, inverse);
        Ok(())
    }

    #[test]
    fn test_inverse_of_a_third_4_by_4() -> Result<(), Box<Error>> {
        let matrix1 = matrix::Matrix4::new((
            (9.0, 3.0, 0.0, 9.0),
            (-5.0, -2.0, -6.0, -3.0),
            (-4.0, 9.0, 6.0, 4.0),
            (-7.0, 6.0, 6.0, 2.0),
        ));

        let inverse = matrix::inverse4(&matrix1)?;

        let expected = matrix::Matrix4::new((
            (
                -0.040740740740740744,
                -0.07777777777777778,
                0.14444444444444443,
                -0.2222222222222222,
            ),
            (
                -0.07777777777777778,
                0.03333333333333333,
                0.36666666666666664,
                -0.3333333333333333,
            ),
            (
                -0.029012345679012345,
                -0.14629629629629629,
                -0.10925925925925926,
                0.12962962962962962,
            ),
            (
                0.17777777777777778,
                0.06666666666666667,
                -0.26666666666666666,
                0.3333333333333333,
            ),
        ));
        assert_eq!(expected, inverse);
        Ok(())
    }

    #[test]
    fn test_non_invertable_returns_error() {
        let matrix1 = matrix::Matrix4::new((
            (-4.0, 2.0, -2.0, -3.0),
            (9.0, 6.0, 2.0, 6.0),
            (0.0, -5.0, 1.0, -5.0),
            (0.0, 0.0, 0.0, 0.0),
        ));
        let uninvertable_error = matrix::inverse4(&matrix1);

        assert_eq!(uninvertable_error, Err("uninvertable_error"));
    }

    #[test]
    fn test_multiply_a_product_by_its_inverse() -> Result<(), Box<Error>> {
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
        let product_times_inverse = product * matrix::inverse4(&matrix2)?;
        for x in 0..4 {
            for y in 0..4 {
                assert_approx_eq!(product_times_inverse[(x, y)], matrix1[(x, y)]);
            }
        }
        Ok(())
    }
}
