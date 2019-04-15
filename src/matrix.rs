use std::ops::Index;

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
}
