use crate::material;
use crate::matrix;
// use crate::matrix::{Inverse, Transpose};
use crate::ray;
use crate::shape::Shape;
use crate::tuple;

#[derive(Debug, PartialEq)]
pub struct Plane {
    transform: matrix::Matrix4,
    pub material: material::Material,
}

impl Shape for Plane {
    fn default() -> Self {
        return Plane {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
        };
    }

    fn transformation_matrix(&self) -> &matrix::Matrix4 {
        &self.transform
    }

    fn set_transformation_matrix(&mut self, new_transform: matrix::Matrix4) {
        self.transform = new_transform;
    }

    fn normal_at(&self, _world_point: tuple::Point) -> tuple::Vector {
        // TODO this is a stub
        tuple::Vector::new(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, _local_ray: ray::Ray) -> tuple::Point {
        tuple::Point::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod plane_tests {
    // use crate::assert_tuple_approx_eq;
    // use crate::material;
    // use crate::matrix;
    use crate::plane;
    use crate::shape::Shape;
    // use crate::transformation::Transform;
    use crate::tuple;

    #[test]
    fn test_normal_on_a_plane_is_constant() {
        let plane = plane::Plane::default();

        assert_eq!(
            plane.normal_at(tuple::Point::new(0.0, 0.0, 0.0)),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        assert_eq!(
            plane.normal_at(tuple::Point::new(10.0, 0.0, -10.0)),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        assert_eq!(
            plane.normal_at(tuple::Point::new(-5.0, 0.0, 150.0)),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
    }
}
