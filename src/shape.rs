use crate::tuple;

pub trait Shape {
    fn default() -> Self;
    fn normal_at(&self, world_point: tuple::Point) -> tuple::Vector;
}
