use crate::matrix;

fn translation(x: f64, y: f64, z: f64) -> matrix::Matrix4 {
    matrix::Matrix4::new((
        (1.0, 0.0, 0.0, x),
        (0.0, 1.0, 0.0, y),
        (0.0, 0.0, 1.0, z),
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
}
