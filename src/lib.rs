// TODO remove dead code exception
#![allow(dead_code)]

pub mod camera;
pub mod canvas;
pub mod color;
pub mod lighting;
pub mod lights;
pub mod material;
pub mod matrix;
pub mod ray;
pub mod sphere;
pub mod transformation;
pub mod tuple;
pub mod world;

mod intersections; // TODO not sure if this should be public
mod test_helpers;
