use crate::matrix;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    pub transform: matrix::Matrix4,
}

pub fn sphere() -> Sphere {
    return Sphere {
        transform: matrix::Matrix4::IDENTITY,
    };
}
