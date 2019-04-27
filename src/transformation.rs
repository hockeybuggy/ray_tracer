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

fn rotation_x(radians: f64) -> matrix::Matrix4 {
    let r = radians;
    matrix::Matrix4::new((
        (1.0, 0.0, 0.0, 0.0),
        (0.0, r.cos(), -r.sin(), 0.0),
        (0.0, r.sin(), r.cos(), 0.0),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

fn rotation_y(radians: f64) -> matrix::Matrix4 {
    let r = radians;
    matrix::Matrix4::new((
        (r.cos(), 0.0, r.sin(), 0.0),
        (0.0, 1.0, 0.0, 0.0),
        (-r.sin(), 0.0, r.cos(), 0.0),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

fn rotation_z(radians: f64) -> matrix::Matrix4 {
    let r = radians;
    matrix::Matrix4::new((
        (r.cos(), -r.sin(), 0.0, 0.0),
        (r.sin(), r.cos(), 0.0, 0.0),
        (0.0, 0.0, 1.0, 0.0),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

fn shearing(
    x_to_y: f64,
    x_to_z: f64,
    y_to_x: f64,
    y_to_z: f64,
    z_to_x: f64,
    z_to_y: f64,
) -> matrix::Matrix4 {
    // "x_to_y" is shorhand for x in proportion to y, etc.
    matrix::Matrix4::new((
        (1.0, x_to_y, x_to_z, 0.0),
        (y_to_x, 1.0, y_to_z, 0.0),
        (z_to_x, z_to_y, 1.0, 0.0),
        (0.0, 0.0, 0.0, 1.0),
    ))
}

#[cfg(test)]
mod transformation_tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::matrix::Inverse;
    use crate::transformation;
    use crate::tuple;

    macro_rules! assert_tuple_approx_eq {
        ( $a:expr, $b:expr) => {{
            assert_approx_eq!($a.x, $b.x, 1e-5f64);
            assert_approx_eq!($a.y, $b.y, 1e-5f64);
            assert_approx_eq!($a.z, $b.z, 1e-5f64);
            assert_approx_eq!($a.w, $b.w, 1e-5f64);
        }};
    }

    #[test]
    fn test_simple_translation_matrix() {
        let translation_matrix = transformation::translation(5.0, -3.0, 2.0);
        let point = tuple::point(-3.0, 4.0, 5.0);

        let expected = tuple::point(2.0, 1.0, 7.0);
        assert_eq!(translation_matrix * point, expected);
    }

    #[test]
    fn test_inverse_translation_matrix() {
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

    #[test]
    fn test_rotation_x_quarter_turns() {
        let point = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = transformation::rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = transformation::rotation_x(std::f64::consts::PI / 2.0);

        assert_tuple_approx_eq!(
            half_quarter * point,
            tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_tuple_approx_eq!(full_quarter * point, tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_rotation_x_inverse_rotates_in_the_opposite_direction() {
        let point = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = transformation::rotation_x(std::f64::consts::PI / 4.0);
        assert_tuple_approx_eq!(
            half_quarter.inverse().unwrap() * point,
            tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn test_rotation_y_quarter_turns() {
        let point = tuple::point(0.0, 0.0, 1.0);
        let half_quarter = transformation::rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = transformation::rotation_y(std::f64::consts::PI / 2.0);

        assert_tuple_approx_eq!(
            half_quarter * point,
            tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_tuple_approx_eq!(full_quarter * point, tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotation_z_quarter_turns() {
        let point = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = transformation::rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = transformation::rotation_z(std::f64::consts::PI / 2.0);

        assert_tuple_approx_eq!(
            half_quarter * point,
            tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_tuple_approx_eq!(full_quarter * point, tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_y() {
        let shear = transformation::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_x_in_proportion_to_z() {
        let shear = transformation::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_x() {
        let shear = transformation::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_y_in_proportion_to_z() {
        let shear = transformation::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_x() {
        let shear = transformation::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn test_shearing_transformation_moves_z_in_proportion_to_y() {
        let shear = transformation::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = tuple::point(2.0, 3.0, 4.0);

        assert_tuple_approx_eq!(shear * point, tuple::point(2.0, 3.0, 7.0));
    }
}
