use crate::matrix;
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
    fn normal_at(&self, point: tuple::Tuple) -> tuple::Tuple {
        point
    }
}

#[cfg(test)]
mod sphere_tests {
    // use assert_approx_eq::assert_approx_eq;

    use crate::sphere;
    use crate::tuple;

    #[test]
    fn test_normal_on_a_sphere_on_the_x() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(1.0, 0.0, 0.0));

        let expected = tuple::point(1.0, 0.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_y() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(0.0, 1.0, 0.0));

        let expected = tuple::point(0.0, 1.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_z() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(0.0, 0.0, 1.0));

        let expected = tuple::point(0.0, 0.0, 1.0);
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

        let expected = tuple::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_is_mornalized_vector() {
        let sphere = sphere::sphere();

        let normal = sphere.normal_at(tuple::point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(tuple::normalize(&normal), normal);
    }
}
