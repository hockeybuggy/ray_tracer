use crate::matrix;
use crate::matrix::Inverse;
use crate::ray;
use crate::tuple;

pub trait Shape {
    /// Construct a 'default' instance of the Shape
    fn default() -> Self;

    /// Get a reference to the Shape's transformation matrix
    fn transformation_matrix(&self) -> &matrix::Matrix4;

    fn set_transformation_matrix(&mut self, new_transform: matrix::Matrix4);

    fn normal_at(&self, world_point: tuple::Point) -> tuple::Vector;

    fn intersect(&self, ray: ray::Ray) -> tuple::Point {
        let local_ray = ray.transform(&self.transformation_matrix().inverse().unwrap());
        return self.local_intersect(local_ray);
    }

    fn local_intersect(&self, local_ray: ray::Ray) -> tuple::Point;
}
