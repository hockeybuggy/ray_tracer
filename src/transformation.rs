use crate::matrix;

fn translation(x: f64, y: f64, z: f64) -> matrix::Matrix4 {
    matrix::Matrix4::new((
        (1.0, 0.0, 0.0, x),
        (0.0, 1.0, 0.0, y),
        (0.0, 0.0, 1.0, z),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

fn scaling(x: f64, y: f64, z: f64) -> matrix::Matrix4 {
    matrix::Matrix4::new((
        (x, 0.0, 0.0, 0.0),
        (0.0, y, 0.0, 0.0),
        (0.0, 0.0, z, 0.0),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

#[cfg(test)]
mod transformation_tests {
    use crate::transformation;
    use crate::tuple;

    #[test]
    fn test_simple_translation_matrix() {
        let translation_matrix = transformation::translation(5.0, -3.0, 2.0);
        let point = tuple::point(-3.0, 4.0, 5.0);

        let expected = tuple::point(2.0, 1.0, 7.0);
        assert_eq!(translation_matrix * point, expected);
    }

    #[test]
    fn test_inverse_translation_matrix() {
        use crate::matrix::Inverse;
        let translation_matrix = transformation::translation(5.0, -3.0, 2.0)
            .inverse()
            .unwrap();
        let point = tuple::point(-3.0, 4.0, 5.0);

        let expected = tuple::point(-8.0, 7.0, 3.0);
        assert_eq!(translation_matrix * point, expected);
    }

    #[test]
    fn test_translation_does_not_affect_vectors() {
        let translation_matrix = transformation::translation(5.0, -3.0, 2.0);
        let vector = tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(translation_matrix * vector, vector);
    }

    #[test]
    fn test_scaling_applied_to_a_point() {
        let scaling_matrix = transformation::scaling(2.0, 3.0, 4.0);
        let point = tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(scaling_matrix * point, tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_applied_to_a_vector() {
        let scaling_matrix = transformation::scaling(2.0, 3.0, 4.0);
        let vector = tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(scaling_matrix * vector, tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_with_the_inverse() {
        use crate::matrix::Inverse;
        let scaling_matrix = transformation::scaling(2.0, 3.0, 4.0);
        let vector = tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(
            scaling_matrix.inverse().unwrap() * vector,
            tuple::vector(-2.0, 2.0, 2.0)
        );
    }

    #[test]
    fn test_scaling_to_achieve_reflection() {
        let scaling_matrix = transformation::scaling(-1.0, 1.0, 1.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_eq!(scaling_matrix * point, tuple::point(-2.0, 3.0, 4.0));
    }
}
