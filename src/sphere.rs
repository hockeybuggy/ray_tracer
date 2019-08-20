use crate::matrix;
use crate::matrix::{Inverse, Transpose};
use crate::tuple;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: matrix::Matrix4,
}

pub fn sphere() -> Sphere {
    return Sphere {
        transform: matrix::Matrix4::IDENTITY,
    };
}

impl Sphere {
    fn normal_at(&self, world_point: tuple::Tuple) -> tuple::Tuple {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = transform_inverse * world_point;
        let object_normal = object_point - tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = transform_inverse.transpose() * object_normal;
        // This is sorta a cheat to skip finding the submatrix.
        world_normal.w = 0.0;
        return tuple::normalize(&world_normal);
    }
}

#[cfg(test)]
mod sphere_tests {
    use assert_approx_eq::assert_approx_eq;

    // TODO factor these out into some kind of test utils
    macro_rules! assert_tuple_approx_eq {
        ($a:expr, $b:expr) => {{
            assert_approx_eq!($a.x, $b.x, 1e-5f64);
            assert_approx_eq!($a.y, $b.y, 1e-5f64);
            assert_approx_eq!($a.z, $b.z, 1e-5f64);
            assert_approx_eq!($a.w, $b.w, 1e-5f64);
        }};
    }

    use crate::matrix;
    use crate::sphere;
    use crate::transformation::Transform;
    use crate::tuple;

    #[test]
    fn test_normal_on_a_sphere_on_the_x() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(1.0, 0.0, 0.0));

        let expected = tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_y() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(0.0, 1.0, 0.0));

        let expected = tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_z() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(0.0, 0.0, 1.0));

        let expected = tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        let expected = tuple::vector(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_is_nornalized_vector() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(tuple::normalize(&normal), normal);
    }

    #[test]
    fn test_normal_on_a_translated_sphere() {
        let mut sphere = sphere::sphere();
        sphere.transform = matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0);

        let normal = sphere.normal_at(tuple::point(0.0, 1.707107, -0.707107));

        let expected = tuple::vector(0.0, 0.707107, -0.707107);
        assert_tuple_approx_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_transformed_sphere() {
        let mut sphere = sphere::sphere();
        sphere.transform = matrix::Matrix4::IDENTITY
            .rotation_x(std::f64::consts::PI / 5.0)
            .scaling(1.0, 0.5, 1.0);

        let normal = sphere.normal_at(tuple::point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        let expected = tuple::vector(0.0, 0.97014, -0.24254);
        assert_tuple_approx_eq!(expected, normal);
    }
}
