use crate::matrix;
use crate::matrix::Inverse;
use crate::ray;
use crate::tuple;

pub trait Shape {
    /// Construct a 'default' instance of the Shape
    fn default() -> Self;

    /// Get a reference to the Shape's transformation matrix
    fn get_transform(&self) -> &matrix::Matrix4;

    /// Get a mutable reference to the Shape's transformation matrix
    fn get_mut_transform(&mut self) -> &mut matrix::Matrix4;

    fn normal_at(&self, world_point: tuple::Point) -> tuple::Vector;

    fn intersect(&self, ray: ray::Ray) -> tuple::Point {
        let local_ray = ray.transform(&self.get_transform().inverse().unwrap());
        return self.local_intersect(local_ray);
    }

    fn local_intersect(&self, local_ray: ray::Ray) -> tuple::Point;
}
