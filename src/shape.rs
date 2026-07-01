use crate::intersection;
use crate::material;
use crate::matrix;
use crate::matrix::{Inverse, Transpose};
use crate::ray;
use crate::tuple;

const EPSILON: f64 = 1e-5;

#[derive(Debug, PartialEq)]
enum ShapeType {
    Sphere,
    Plane,
    Cube,
}

#[derive(Debug, PartialEq)]
pub struct Shape {
    pub transform: matrix::Matrix4,
    pub material: material::Material,
    shape_type: ShapeType,
}

impl Shape {
    pub fn default_sphere() -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Sphere,
        };
    }

    pub fn default_plane() -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Plane,
        };
    }

    pub fn default_cube() -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Cube,
        };
    }

    pub fn glass_sphere() -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::glass(),
            shape_type: ShapeType::Sphere,
        };
    }

    pub fn transformation_matrix(&self) -> &matrix::Matrix4 {
        &self.transform
    }

    pub fn set_transformation_matrix(&mut self, new_transform: matrix::Matrix4) {
        self.transform = new_transform;
    }

    pub fn intersect(&self, ray: &ray::Ray) -> Vec<intersection::Intersection<'_>> {
        let local_ray = ray.transform(&self.transformation_matrix().inverse().unwrap());
        return self.local_intersect(local_ray);
    }

    fn sphere_local_normal_at(&self, object_point: tuple::Point) -> tuple::Vector {
        object_point - tuple::Point::new(0.0, 0.0, 0.0)
    }

    fn plane_local_normal_at(&self, _object_point: tuple::Point) -> tuple::Vector {
        tuple::Vector::new(0.0, 1.0, 0.0)
    }

    fn cube_local_normal_at(&self, object_point: tuple::Point) -> tuple::Vector {
        let maxc = object_point
            .x
            .abs()
            .max(object_point.y.abs())
            .max(object_point.z.abs());

        if maxc == object_point.x.abs() {
            return tuple::Vector::new(object_point.x, 0.0, 0.0);
        } else if maxc == object_point.y.abs() {
            return tuple::Vector::new(0.0, object_point.y, 0.0);
        }
        return tuple::Vector::new(0.0, 0.0, object_point.z);
    }

    fn local_normal_at(&self, object_point: tuple::Point) -> tuple::Vector {
        match self.shape_type {
            ShapeType::Sphere => self.sphere_local_normal_at(object_point),
            ShapeType::Plane => self.plane_local_normal_at(object_point),
            ShapeType::Cube => self.cube_local_normal_at(object_point),
        }
    }

    pub fn normal_at(&self, world_point: tuple::Point) -> tuple::Vector {
        let transform_inverse = self.transform.inverse().unwrap();
        let object_point = transform_inverse * world_point;
        let object_normal = self.local_normal_at(object_point);
        let mut world_normal = transform_inverse.transpose() * object_normal;
        // This is sorta a cheat to skip finding the submatrix.
        world_normal.w = 0.0;
        return tuple::normalize(&world_normal);
    }

    pub fn local_intersect(&self, local_ray: ray::Ray) -> Vec<intersection::Intersection<'_>> {
        match self.shape_type {
            ShapeType::Sphere => self.sphere_local_intersect(local_ray),
            ShapeType::Plane => self.plane_local_intersect(local_ray),
            ShapeType::Cube => self.cube_local_intersect(local_ray),
        }
    }

    fn cube_local_intersect(&self, local_ray: ray::Ray) -> Vec<intersection::Intersection<'_>> {
        let (xtmin, xtmax) = check_axis(local_ray.origin.x, local_ray.direction.x);
        let (ytmin, ytmax) = check_axis(local_ray.origin.y, local_ray.direction.y);
        let (ztmin, ztmax) = check_axis(local_ray.origin.z, local_ray.direction.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return vec![];
        }

        return vec![
            intersection::intersection(tmin, self),
            intersection::intersection(tmax, self),
        ];
    }

    fn sphere_local_intersect(&self, local_ray: ray::Ray) -> Vec<intersection::Intersection<'_>> {
        let sphere_to_ray = local_ray.origin - tuple::Point::new(0.0, 0.0, 0.0);

        let a = tuple::dot(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * tuple::dot(&local_ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminanant = b.powf(2.0) - 4.0 * a * c;

        if discriminanant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminanant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminanant.sqrt()) / (2.0 * a);
        return vec![
            intersection::intersection(t1, self),
            intersection::intersection(t2, self),
        ];
    }

    fn plane_local_intersect(&self, local_ray: ray::Ray) -> Vec<intersection::Intersection<'_>> {
        if local_ray.direction.y.abs() < EPSILON {
            return vec![];
        }
        let t = -local_ray.origin.y / local_ray.direction.y;

        return vec![intersection::intersection(t, self)];
    }
}

// Find where the ray crosses the pair of parallel planes at -1 and +1 on one
// axis. A zero direction divides to +/- infinity, which f64 handles natively,
// so no EPSILON special case is needed.
fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin = (-1.0 - origin) / direction;
    let tmax = (1.0 - origin) / direction;

    if tmin > tmax {
        return (tmax, tmin);
    }
    return (tmin, tmax);
}

#[cfg(test)]
mod sphere_tests {
    use crate::assert_tuple_approx_eq;
    use crate::material;
    use crate::matrix;
    use crate::shape;
    use crate::transformation::Transform;
    use crate::tuple;

    #[test]
    fn test_normal_on_a_sphere_on_the_x() {
        let sphere = shape::Shape::default_sphere();

        let normal = sphere.normal_at(tuple::Point::new(1.0, 0.0, 0.0));

        let expected = tuple::Vector::new(1.0, 0.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_y() {
        let sphere = shape::Shape::default_sphere();

        let normal = sphere.normal_at(tuple::Point::new(0.0, 1.0, 0.0));

        let expected = tuple::Vector::new(0.0, 1.0, 0.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_on_the_z() {
        let sphere = shape::Shape::default_sphere();

        let normal = sphere.normal_at(tuple::Point::new(0.0, 0.0, 1.0));

        let expected = tuple::Vector::new(0.0, 0.0, 1.0);
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_sphere_at_a_nonaxial_point() {
        let sphere = shape::Shape::default_sphere();

        let normal = sphere.normal_at(tuple::Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        let expected = tuple::Vector::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        );
        assert_eq!(expected, normal);
    }

    #[test]
    fn test_normal_is_nornalized_vector() {
        let sphere = shape::Shape::default_sphere();

        let normal = sphere.normal_at(tuple::Point::new(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(tuple::normalize(&normal), normal);
    }

    #[test]
    fn test_normal_on_a_translated_sphere() {
        let mut sphere = shape::Shape::default_sphere();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0));

        let normal = sphere.normal_at(tuple::Point::new(0.0, 1.707107, -0.707107));

        let expected = tuple::Vector::new(0.0, 0.707107, -0.707107);
        assert_tuple_approx_eq!(expected, normal);
    }

    #[test]
    fn test_normal_on_a_transformed_sphere() {
        let mut sphere = shape::Shape::default_sphere();
        sphere.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_x(std::f64::consts::PI / 5.0)
                .scaling(1.0, 0.5, 1.0),
        );

        let normal = sphere.normal_at(tuple::Point::new(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        let expected = tuple::Vector::new(0.0, 0.97014, -0.24254);
        assert_tuple_approx_eq!(expected, normal);
    }

    #[test]
    fn test_sphere_has_a_default_material() {
        let sphere = shape::Shape::default_sphere();
        assert_eq!(sphere.material, material::material());
    }

    #[test]
    fn test_spheres_material_can_be_set() {
        let mut sphere = shape::Shape::default_sphere();
        let mut material1 = material::material();
        material1.ambient = 1.0;

        sphere.material = material1;

        assert_eq!(sphere.material.ambient, 1.0);
    }
}

#[cfg(test)]
mod cube_tests {
    use crate::ray;
    use crate::shape;
    use crate::tuple;

    #[test]
    fn test_a_ray_intersects_a_cube() {
        // One ray aimed at each face of the cube, plus one starting inside it.
        let examples = [
            (
                "+x",
                tuple::Point::new(5.0, 0.5, 0.0),
                tuple::Vector::new(-1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "-x",
                tuple::Point::new(-5.0, 0.5, 0.0),
                tuple::Vector::new(1.0, 0.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "+y",
                tuple::Point::new(0.5, 5.0, 0.0),
                tuple::Vector::new(0.0, -1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "-y",
                tuple::Point::new(0.5, -5.0, 0.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
                4.0,
                6.0,
            ),
            (
                "+z",
                tuple::Point::new(0.5, 0.0, 5.0),
                tuple::Vector::new(0.0, 0.0, -1.0),
                4.0,
                6.0,
            ),
            (
                "-z",
                tuple::Point::new(0.5, 0.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                "inside",
                tuple::Point::new(0.0, 0.5, 0.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                -1.0,
                1.0,
            ),
        ];

        for (name, origin, direction, t1, t2) in examples {
            let cube = shape::Shape::default_cube();
            let ray = ray::ray(origin, direction);

            let intersections = cube.local_intersect(ray);

            assert_eq!(intersections.len(), 2, "case `{}`", name);
            assert_eq!(intersections[0].t, t1, "case `{}`", name);
            assert_eq!(intersections[1].t, t2, "case `{}`", name);
        }
    }

    #[test]
    fn test_a_ray_misses_a_cube() {
        // The first three rays point diagonally away from the cube; the rest
        // run parallel to a face but past the cube.
        let examples = [
            (
                tuple::Point::new(-2.0, 0.0, 0.0),
                tuple::Vector::new(0.2673, 0.5345, 0.8018),
            ),
            (
                tuple::Point::new(0.0, -2.0, 0.0),
                tuple::Vector::new(0.8018, 0.2673, 0.5345),
            ),
            (
                tuple::Point::new(0.0, 0.0, -2.0),
                tuple::Vector::new(0.5345, 0.8018, 0.2673),
            ),
            (
                tuple::Point::new(2.0, 0.0, 2.0),
                tuple::Vector::new(0.0, 0.0, -1.0),
            ),
            (
                tuple::Point::new(0.0, 2.0, 2.0),
                tuple::Vector::new(0.0, -1.0, 0.0),
            ),
            (
                tuple::Point::new(2.0, 2.0, 0.0),
                tuple::Vector::new(-1.0, 0.0, 0.0),
            ),
        ];

        for (origin, direction) in examples {
            let cube = shape::Shape::default_cube();
            let ray = ray::ray(origin, direction);

            let intersections = cube.local_intersect(ray);

            assert_eq!(intersections.len(), 0, "ray from {:?}", origin);
        }
    }

    #[test]
    fn test_the_normal_on_the_surface_of_a_cube() {
        // The last two cases are corners, which are treated as being on the
        // +x or -x face.
        let examples = [
            (
                tuple::Point::new(1.0, 0.5, -0.8),
                tuple::Vector::new(1.0, 0.0, 0.0),
            ),
            (
                tuple::Point::new(-1.0, -0.2, 0.9),
                tuple::Vector::new(-1.0, 0.0, 0.0),
            ),
            (
                tuple::Point::new(-0.4, 1.0, -0.1),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
            (
                tuple::Point::new(0.3, -1.0, -0.7),
                tuple::Vector::new(0.0, -1.0, 0.0),
            ),
            (
                tuple::Point::new(-0.6, 0.3, 1.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
            ),
            (
                tuple::Point::new(0.4, 0.4, -1.0),
                tuple::Vector::new(0.0, 0.0, -1.0),
            ),
            (
                tuple::Point::new(1.0, 1.0, 1.0),
                tuple::Vector::new(1.0, 0.0, 0.0),
            ),
            (
                tuple::Point::new(-1.0, -1.0, -1.0),
                tuple::Vector::new(-1.0, 0.0, 0.0),
            ),
        ];

        for (point, expected) in examples {
            let cube = shape::Shape::default_cube();

            let normal = cube.local_normal_at(point);

            assert_eq!(expected, normal, "point {:?}", point);
        }
    }
}

#[cfg(test)]
mod plane_tests {
    use crate::ray;
    use crate::shape;
    use crate::tuple;

    #[test]
    fn test_normal_on_a_plane_is_constant() {
        let plane = shape::Shape::default_plane();

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

    #[test]
    fn test_intersect_with_a_ray_parallel_to_the_plane() {
        let plane = shape::Shape::default_plane();
        let ray = ray::ray(
            tuple::Point::new(0.0, 10.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = plane.local_intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_with_a_coplaner_ray() {
        let plane = shape::Shape::default_plane();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = plane.local_intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersect_with_a_plane_from_above() {
        let plane = shape::Shape::default_plane();
        let ray = ray::ray(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Vector::new(0.0, -1.0, 0.0),
        );

        let intersections = plane.local_intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, &plane);
    }

    #[test]
    fn test_intersect_with_a_plane_from_below() {
        let plane = shape::Shape::default_plane();
        let ray = ray::ray(
            tuple::Point::new(0.0, -1.0, 0.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );

        let intersections = plane.local_intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 1.0);
        assert_eq!(intersections[0].object, &plane);
    }
}
