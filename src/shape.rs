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
    Cylinder {
        minimum: f64,
        maximum: f64,
        closed: bool,
    },
    Cone {
        minimum: f64,
        maximum: f64,
        closed: bool,
    },
    Group {
        children: Vec<Shape>,
    },
    Triangle {
        p1: tuple::Point,
        p2: tuple::Point,
        p3: tuple::Point,
        e1: tuple::Vector,
        e2: tuple::Vector,
        normal: tuple::Vector,
    },
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

    pub fn default_cylinder() -> Shape {
        return Shape::cylinder(f64::NEG_INFINITY, f64::INFINITY, false);
    }

    pub fn cylinder(minimum: f64, maximum: f64, closed: bool) -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Cylinder {
                minimum,
                maximum,
                closed,
            },
        };
    }

    pub fn default_cone() -> Shape {
        return Shape::cone(f64::NEG_INFINITY, f64::INFINITY, false);
    }

    pub fn cone(minimum: f64, maximum: f64, closed: bool) -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Cone {
                minimum,
                maximum,
                closed,
            },
        };
    }

    pub fn default_group() -> Shape {
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Group {
                children: Vec::new(),
            },
        };
    }

    pub fn add_child(&mut self, child: Shape) {
        match &mut self.shape_type {
            ShapeType::Group { children } => children.push(child),
            _ => panic!("only groups can contain children"),
        }
    }

    pub fn triangle(p1: tuple::Point, p2: tuple::Point, p3: tuple::Point) -> Shape {
        // The edge vectors and normal never vary across the surface, so
        // they are computed once here rather than on every intersection.
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let normal = tuple::normalize(&tuple::cross(&e2, &e1));
        return Shape {
            transform: matrix::Matrix4::IDENTITY,
            material: material::material(),
            shape_type: ShapeType::Triangle {
                p1,
                p2,
                p3,
                e1,
                e2,
                normal,
            },
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

    fn cylinder_local_normal_at(
        &self,
        object_point: tuple::Point,
        minimum: f64,
        maximum: f64,
    ) -> tuple::Vector {
        // The square of the distance from the y axis; a point within one
        // unit of the axis and within EPSILON of an extent is on an end cap.
        let distance = object_point.x.powf(2.0) + object_point.z.powf(2.0);

        if distance < 1.0 && object_point.y >= maximum - EPSILON {
            return tuple::Vector::new(0.0, 1.0, 0.0);
        }
        if distance < 1.0 && object_point.y <= minimum + EPSILON {
            return tuple::Vector::new(0.0, -1.0, 0.0);
        }
        return tuple::Vector::new(object_point.x, 0.0, object_point.z);
    }

    fn cone_local_normal_at(
        &self,
        object_point: tuple::Point,
        minimum: f64,
        maximum: f64,
    ) -> tuple::Vector {
        // Same end cap check as the cylinder, except the cap radius is the
        // absolute value of the extent's y rather than 1.
        let distance = object_point.x.powf(2.0) + object_point.z.powf(2.0);

        if distance < maximum.powf(2.0) && object_point.y >= maximum - EPSILON {
            return tuple::Vector::new(0.0, 1.0, 0.0);
        }
        if distance < minimum.powf(2.0) && object_point.y <= minimum + EPSILON {
            return tuple::Vector::new(0.0, -1.0, 0.0);
        }

        // The walls slope at 45 degrees, so the normal leans away from the
        // y axis by the point's distance from it, downward on the upper
        // half. At the tip this degenerates to a zero vector.
        let mut y = distance.sqrt();
        if object_point.y > 0.0 {
            y = -y;
        }
        return tuple::Vector::new(object_point.x, y, object_point.z);
    }

    pub(crate) fn local_normal_at(&self, object_point: tuple::Point) -> tuple::Vector {
        match self.shape_type {
            ShapeType::Sphere => self.sphere_local_normal_at(object_point),
            ShapeType::Plane => self.plane_local_normal_at(object_point),
            ShapeType::Cube => self.cube_local_normal_at(object_point),
            ShapeType::Cylinder {
                minimum, maximum, ..
            } => self.cylinder_local_normal_at(object_point, minimum, maximum),
            ShapeType::Cone {
                minimum, maximum, ..
            } => self.cone_local_normal_at(object_point, minimum, maximum),
            // A group has no surface of its own; normals are always computed
            // on the concrete child shape the ray actually hit.
            ShapeType::Group { .. } => panic!("groups do not have a local normal"),
            ShapeType::Triangle { normal, .. } => normal,
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
            ShapeType::Cylinder {
                minimum,
                maximum,
                closed,
            } => self.cylinder_local_intersect(local_ray, minimum, maximum, closed),
            ShapeType::Cone {
                minimum,
                maximum,
                closed,
            } => self.cone_local_intersect(local_ray, minimum, maximum, closed),
            ShapeType::Group { ref children } => self.group_local_intersect(children, local_ray),
            ShapeType::Triangle { p1, e1, e2, .. } => {
                self.triangle_local_intersect(local_ray, p1, e1, e2)
            }
        }
    }

    fn triangle_local_intersect(
        &self,
        local_ray: ray::Ray,
        p1: tuple::Point,
        e1: tuple::Vector,
        e2: tuple::Vector,
    ) -> Vec<intersection::Intersection<'_>> {
        // Möller–Trumbore: solve origin + t*dir = p1 + u*e1 + v*e2 for
        // (t, u, v) via Cramer's rule, where u and v are barycentric
        // coordinates that must stay within the triangle.
        let dir_cross_e2 = tuple::cross(&local_ray.direction, &e2);
        let det = tuple::dot(&e1, &dir_cross_e2);

        // A determinant of zero means the ray is parallel to the triangle.
        if det.abs() < EPSILON {
            return vec![];
        }
        let f = 1.0 / det;

        // A u outside [0, 1] means the ray passes beyond the p1-p3 edge.
        let p1_to_origin = local_ray.origin - p1;
        let u = f * tuple::dot(&p1_to_origin, &dir_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return vec![];
        }

        // A negative v or u + v beyond 1 means the ray passes beyond the
        // p1-p2 or p2-p3 edge respectively.
        let origin_cross_e1 = tuple::cross(&p1_to_origin, &e1);
        let v = f * tuple::dot(&local_ray.direction, &origin_cross_e1);
        if v < 0.0 || (u + v) > 1.0 {
            return vec![];
        }

        let t = f * tuple::dot(&e2, &origin_cross_e1);
        return vec![intersection::intersection(t, self)];
    }

    fn group_local_intersect<'a>(
        &'a self,
        children: &'a [Shape],
        local_ray: ray::Ray,
    ) -> Vec<intersection::Intersection<'a>> {
        let mut intersections: Vec<intersection::Intersection<'a>> = children
            .iter()
            .flat_map(|child| child.intersect(&local_ray))
            .collect();

        // Each child reports a transform relative to this group, so prepend
        // the group's own transform to walk the accumulated matrix one level
        // closer to world space.
        for intersection in intersections.iter_mut() {
            intersection.world_transform = self.transform * intersection.world_transform;
        }

        intersections.sort_unstable_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        return intersections;
    }

    fn cone_local_intersect(
        &self,
        local_ray: ray::Ray,
        minimum: f64,
        maximum: f64,
        closed: bool,
    ) -> Vec<intersection::Intersection<'_>> {
        let mut intersections = vec![];

        // The cylinder's coefficients with the y terms subtracted in.
        let a = local_ray.direction.x.powf(2.0) - local_ray.direction.y.powf(2.0)
            + local_ray.direction.z.powf(2.0);
        let b = 2.0 * local_ray.origin.x * local_ray.direction.x
            - 2.0 * local_ray.origin.y * local_ray.direction.y
            + 2.0 * local_ray.origin.z * local_ray.direction.z;
        let c = local_ray.origin.x.powf(2.0) - local_ray.origin.y.powf(2.0)
            + local_ray.origin.z.powf(2.0);

        if a.abs() < EPSILON {
            // The ray is parallel to one half of the cone, but unless `b`
            // is also zero it still strikes the other half once.
            if b.abs() >= EPSILON {
                let t = -c / (2.0 * b);
                let y = local_ray.origin.y + t * local_ray.direction.y;
                if minimum < y && y < maximum {
                    intersections.push(intersection::intersection(t, self));
                }
            }
        } else {
            let discriminant = b.powf(2.0) - 4.0 * a * c;

            if discriminant >= 0.0 {
                let mut t0 = (-b - discriminant.sqrt()) / (2.0 * a);
                let mut t1 = (-b + discriminant.sqrt()) / (2.0 * a);
                if t0 > t1 {
                    std::mem::swap(&mut t0, &mut t1);
                }

                // The minimum and maximum bounds are exclusive.
                let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
                if minimum < y0 && y0 < maximum {
                    intersections.push(intersection::intersection(t0, self));
                }
                let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
                if minimum < y1 && y1 < maximum {
                    intersections.push(intersection::intersection(t1, self));
                }
            }
        }

        // A cone's cap radius equals the absolute value of the extent's y.
        self.intersect_caps(
            &local_ray,
            minimum,
            maximum,
            closed,
            minimum.abs(),
            maximum.abs(),
            &mut intersections,
        );

        return intersections;
    }

    fn cylinder_local_intersect(
        &self,
        local_ray: ray::Ray,
        minimum: f64,
        maximum: f64,
        closed: bool,
    ) -> Vec<intersection::Intersection<'_>> {
        let mut intersections = vec![];

        let a = local_ray.direction.x.powf(2.0) + local_ray.direction.z.powf(2.0);

        // When `a` is zero the ray is parallel to the y axis and cannot hit
        // the walls, but it may still pass through the end caps.
        if a.abs() >= EPSILON {
            let b = 2.0 * local_ray.origin.x * local_ray.direction.x
                + 2.0 * local_ray.origin.z * local_ray.direction.z;
            let c = local_ray.origin.x.powf(2.0) + local_ray.origin.z.powf(2.0) - 1.0;

            let discriminant = b.powf(2.0) - 4.0 * a * c;
            if discriminant < 0.0 {
                return vec![];
            }

            let mut t0 = (-b - discriminant.sqrt()) / (2.0 * a);
            let mut t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            // The minimum and maximum bounds are exclusive.
            let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
            if minimum < y0 && y0 < maximum {
                intersections.push(intersection::intersection(t0, self));
            }
            let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
            if minimum < y1 && y1 < maximum {
                intersections.push(intersection::intersection(t1, self));
            }
        }

        // A cylinder's caps both have its unit radius.
        self.intersect_caps(
            &local_ray,
            minimum,
            maximum,
            closed,
            1.0,
            1.0,
            &mut intersections,
        );

        return intersections;
    }

    fn intersect_caps<'a>(
        &'a self,
        local_ray: &ray::Ray,
        minimum: f64,
        maximum: f64,
        closed: bool,
        minimum_radius: f64,
        maximum_radius: f64,
        intersections: &mut Vec<intersection::Intersection<'a>>,
    ) {
        // Caps only matter if the shape is closed and the ray isn't
        // parallel to them.
        if !closed || local_ray.direction.y.abs() < EPSILON {
            return;
        }

        // Intersect the ray with the planes at y=minimum and y=maximum, and
        // keep each hit that lands within that cap's radius.
        let t_lower = (minimum - local_ray.origin.y) / local_ray.direction.y;
        if check_cap(local_ray, t_lower, minimum_radius) {
            intersections.push(intersection::intersection(t_lower, self));
        }
        let t_upper = (maximum - local_ray.origin.y) / local_ray.direction.y;
        if check_cap(local_ray, t_upper, maximum_radius) {
            intersections.push(intersection::intersection(t_upper, self));
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

// Check whether the point where the ray crosses a cap's plane at `t` lies
// within the cap's radius of the y axis.
fn check_cap(ray: &ray::Ray, t: f64, radius: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.z;
    return x.powf(2.0) + z.powf(2.0) <= radius.powf(2.0);
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
mod cylinder_tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::ray;
    use crate::shape;
    use crate::tuple;

    #[test]
    fn test_the_default_minimum_and_maximum_for_a_cylinder() {
        let cylinder = shape::Shape::default_cylinder();

        match &cylinder.shape_type {
            shape::ShapeType::Cylinder {
                minimum, maximum, ..
            } => {
                assert_eq!(*minimum, f64::NEG_INFINITY);
                assert_eq!(*maximum, f64::INFINITY);
            }
            _ => panic!("expected a cylinder"),
        }
    }

    #[test]
    fn test_the_default_closed_value_for_a_cylinder() {
        let cylinder = shape::Shape::default_cylinder();

        match &cylinder.shape_type {
            shape::ShapeType::Cylinder { closed, .. } => {
                assert_eq!(*closed, false);
            }
            _ => panic!("expected a cylinder"),
        }
    }

    #[test]
    fn test_a_ray_misses_a_cylinder() {
        // The first ray is on the surface pointing along the walls, the
        // second is inside pointing along the axis, and the third is outside
        // and askew from all axes.
        let examples = [
            (
                tuple::Point::new(1.0, 0.0, 0.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 0.0, 0.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(1.0, 1.0, 1.0),
            ),
        ];

        for (origin, direction) in examples {
            let cylinder = shape::Shape::default_cylinder();
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cylinder.local_intersect(ray);

            assert_eq!(intersections.len(), 0, "ray from {:?}", origin);
        }
    }

    #[test]
    fn test_a_ray_strikes_a_cylinder() {
        // A tangent hit still produces two intersections (mirroring how
        // sphere tangents work), then a perpendicular hit through the
        // middle, then a skewed hit.
        let examples = [
            (
                tuple::Point::new(1.0, 0.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                tuple::Point::new(0.5, 0.0, -5.0),
                tuple::Vector::new(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];

        for (origin, direction, t0, t1) in examples {
            let cylinder = shape::Shape::default_cylinder();
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cylinder.local_intersect(ray);

            assert_eq!(intersections.len(), 2, "ray from {:?}", origin);
            assert_approx_eq!(intersections[0].t, t0, 1e-5f64);
            assert_approx_eq!(intersections[1].t, t1, 1e-5f64);
        }
    }

    #[test]
    fn test_normal_vector_on_a_cylinder() {
        // One point on each of the +x, -x, +z and -z sides; y has no effect.
        let examples = [
            (
                tuple::Point::new(1.0, 0.0, 0.0),
                tuple::Vector::new(1.0, 0.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 5.0, -1.0),
                tuple::Vector::new(0.0, 0.0, -1.0),
            ),
            (
                tuple::Point::new(0.0, -2.0, 1.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
            ),
            (
                tuple::Point::new(-1.0, 1.0, 0.0),
                tuple::Vector::new(-1.0, 0.0, 0.0),
            ),
        ];

        for (point, expected) in examples {
            let cylinder = shape::Shape::default_cylinder();

            let normal = cylinder.local_normal_at(point);

            assert_eq!(expected, normal, "point {:?}", point);
        }
    }

    #[test]
    fn test_intersecting_a_constrained_cylinder() {
        // In order: a ray escaping diagonally from inside; rays passing
        // above and below the truncated section; rays hitting exactly the
        // (exclusive) maximum and minimum bounds; and a ray through the
        // middle.
        let examples = [
            (
                tuple::Point::new(0.0, 1.5, 0.0),
                tuple::Vector::new(0.1, 1.0, 0.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 3.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 2.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 1.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 1.5, -2.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                2,
            ),
        ];

        for (origin, direction, count) in examples {
            let cylinder = shape::Shape::cylinder(1.0, 2.0, false);
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cylinder.local_intersect(ray);

            assert_eq!(intersections.len(), count, "ray from {:?}", origin);
        }
    }

    #[test]
    fn test_intersecting_the_caps_of_a_closed_cylinder() {
        // The first ray passes down the axis through both caps. The second
        // and fourth enter through a cap and exit through the wall. The
        // corner cases exit exactly where the far cap meets the wall, and
        // must still produce only two intersections.
        let examples = [
            (
                tuple::Point::new(0.0, 3.0, 0.0),
                tuple::Vector::new(0.0, -1.0, 0.0),
                2,
            ),
            (
                tuple::Point::new(0.0, 3.0, -2.0),
                tuple::Vector::new(0.0, -1.0, 2.0),
                2,
            ),
            // corner case
            (
                tuple::Point::new(0.0, 4.0, -2.0),
                tuple::Vector::new(0.0, -1.0, 1.0),
                2,
            ),
            (
                tuple::Point::new(0.0, 0.0, -2.0),
                tuple::Vector::new(0.0, 1.0, 2.0),
                2,
            ),
            // corner case
            (
                tuple::Point::new(0.0, -1.0, -2.0),
                tuple::Vector::new(0.0, 1.0, 1.0),
                2,
            ),
        ];

        for (origin, direction, count) in examples {
            let cylinder = shape::Shape::cylinder(1.0, 2.0, true);
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cylinder.local_intersect(ray);

            assert_eq!(intersections.len(), count, "ray from {:?}", origin);
        }
    }

    #[test]
    fn test_the_normal_vector_on_a_cylinders_end_caps() {
        // Three points on the bottom cap, then three on the top cap.
        let examples = [
            (
                tuple::Point::new(0.0, 1.0, 0.0),
                tuple::Vector::new(0.0, -1.0, 0.0),
            ),
            (
                tuple::Point::new(0.5, 1.0, 0.0),
                tuple::Vector::new(0.0, -1.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 1.0, 0.5),
                tuple::Vector::new(0.0, -1.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 2.0, 0.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
            (
                tuple::Point::new(0.5, 2.0, 0.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
            (
                tuple::Point::new(0.0, 2.0, 0.5),
                tuple::Vector::new(0.0, 1.0, 0.0),
            ),
        ];

        for (point, expected) in examples {
            let cylinder = shape::Shape::cylinder(1.0, 2.0, true);

            let normal = cylinder.local_normal_at(point);

            assert_eq!(expected, normal, "point {:?}", point);
        }
    }
}

#[cfg(test)]
mod cone_tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::ray;
    use crate::shape;
    use crate::tuple;

    #[test]
    fn test_intersecting_a_cone_with_a_ray() {
        // A head-on hit at the cone's tip, a diagonal hit, and a skewed ray
        // that hits one half going in and the other half much later.
        let examples = [
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                tuple::Point::new(1.0, 1.0, -5.0),
                tuple::Vector::new(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];

        for (origin, direction, t0, t1) in examples {
            let cone = shape::Shape::default_cone();
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cone.local_intersect(ray);

            assert_eq!(intersections.len(), 2, "ray from {:?}", origin);
            assert_approx_eq!(intersections[0].t, t0, 1e-5f64);
            assert_approx_eq!(intersections[1].t, t1, 1e-5f64);
        }
    }

    #[test]
    fn test_intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        // A ray parallel to one half of the cone (`a` is zero, `b` is not)
        // still strikes the other half at a single point.
        let cone = shape::Shape::default_cone();
        let direction = tuple::normalize(&tuple::Vector::new(0.0, 1.0, 1.0));
        let ray = ray::ray(tuple::Point::new(0.0, 0.0, -1.0), direction);

        let intersections = cone.local_intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_approx_eq!(intersections[0].t, 0.35355, 1e-5f64);
    }

    #[test]
    fn test_intersecting_a_cones_end_caps() {
        // The first ray runs between the halves without touching them. The
        // second enters through a cap and exits through a wall. The third
        // runs up the y axis: through the lower cap, both walls (which meet
        // at the origin), and the upper cap.
        let examples = [
            (
                tuple::Point::new(0.0, 0.0, -5.0),
                tuple::Vector::new(0.0, 1.0, 0.0),
                0,
            ),
            (
                tuple::Point::new(0.0, 0.0, -0.25),
                tuple::Vector::new(0.0, 1.0, 1.0),
                2,
            ),
            (
                tuple::Point::new(0.0, 0.0, -0.25),
                tuple::Vector::new(0.0, 1.0, 0.0),
                4,
            ),
        ];

        for (origin, direction, count) in examples {
            let cone = shape::Shape::cone(-0.5, 0.5, true);
            let ray = ray::ray(origin, tuple::normalize(&direction));

            let intersections = cone.local_intersect(ray);

            assert_eq!(intersections.len(), count, "ray from {:?}", origin);
        }
    }

    #[test]
    fn test_computing_the_normal_vector_on_a_cone() {
        // Local (un-normalized) normals: degenerate at the tip, and leaning
        // away from the y axis at 45 degrees on the walls.
        let examples = [
            (
                tuple::Point::new(0.0, 0.0, 0.0),
                tuple::Vector::new(0.0, 0.0, 0.0),
            ),
            (
                tuple::Point::new(1.0, 1.0, 1.0),
                tuple::Vector::new(1.0, -(2.0_f64.sqrt()), 1.0),
            ),
            (
                tuple::Point::new(-1.0, -1.0, 0.0),
                tuple::Vector::new(-1.0, 1.0, 0.0),
            ),
        ];

        for (point, expected) in examples {
            let cone = shape::Shape::default_cone();

            let normal = cone.local_normal_at(point);

            assert_eq!(expected, normal, "point {:?}", point);
        }
    }
}

#[cfg(test)]
mod group_tests {
    use crate::matrix;
    use crate::ray;
    use crate::shape;
    use crate::transformation::Transform;
    use crate::tuple;

    fn children(group: &shape::Shape) -> &Vec<shape::Shape> {
        match &group.shape_type {
            shape::ShapeType::Group { children } => children,
            _ => panic!("expected a group"),
        }
    }

    #[test]
    fn test_creating_a_new_group() {
        let group = shape::Shape::default_group();

        assert_eq!(group.transform, matrix::Matrix4::IDENTITY);
        assert_eq!(children(&group).len(), 0);
    }

    #[test]
    fn test_adding_a_child_to_a_group() {
        let mut group = shape::Shape::default_group();

        group.add_child(shape::Shape::default_sphere());

        assert_eq!(children(&group).len(), 1);
        assert_eq!(children(&group)[0], shape::Shape::default_sphere());
    }

    #[test]
    fn test_intersecting_a_ray_with_an_empty_group() {
        let group = shape::Shape::default_group();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = group.local_intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_intersecting_a_ray_with_a_nonempty_group() {
        // The first sphere sits at the origin, the second in front of it,
        // and the third off to the side where the ray misses it. The
        // intersections come back sorted by t, so the nearer sphere's pair
        // appears first.
        let mut group = shape::Shape::default_group();
        let sphere1 = shape::Shape::default_sphere();
        let mut sphere2 = shape::Shape::default_sphere();
        sphere2.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 0.0, -3.0));
        let mut sphere3 = shape::Shape::default_sphere();
        sphere3.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(5.0, 0.0, 0.0));
        group.add_child(sphere1);
        group.add_child(sphere2);
        group.add_child(sphere3);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = group.local_intersect(ray);

        let children = children(&group);
        assert_eq!(intersections.len(), 4);
        assert_eq!(intersections[0].object, &children[1]);
        assert_eq!(intersections[1].object, &children[1]);
        assert_eq!(intersections[2].object, &children[0]);
        assert_eq!(intersections[3].object, &children[0]);
    }

    #[test]
    fn test_intersecting_a_transformed_group() {
        // The group's scaling moves the sphere's world position to
        // (10, 0, 0), where the ray hits it. This works without any special
        // handling as long as the group's local_intersect calls the
        // children's intersect (not local_intersect), so each child applies
        // its own transform too.
        let mut group = shape::Shape::default_group();
        group.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(2.0, 2.0, 2.0));
        let mut sphere = shape::Shape::default_sphere();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(5.0, 0.0, 0.0));
        group.add_child(sphere);
        let ray = ray::ray(
            tuple::Point::new(10.0, 0.0, -10.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = group.intersect(&ray);

        assert_eq!(intersections.len(), 2);
    }
}

#[cfg(test)]
mod triangle_tests {
    use crate::ray;
    use crate::shape;
    use crate::tuple;

    // The book's test triangle: apex at (0, 1, 0), base corners at
    // (-1, 0, 0) and (1, 0, 0), all in the z=0 plane.
    fn triangle() -> shape::Shape {
        shape::Shape::triangle(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Point::new(-1.0, 0.0, 0.0),
            tuple::Point::new(1.0, 0.0, 0.0),
        )
    }

    #[test]
    fn test_constructing_a_triangle() {
        // The edge vectors e1 = p2 - p1 and e2 = p3 - p1 and the normal
        // (normalize(cross(e2, e1))) are the same everywhere on the
        // triangle, so the constructor precomputes them once.
        let p1 = tuple::Point::new(0.0, 1.0, 0.0);
        let p2 = tuple::Point::new(-1.0, 0.0, 0.0);
        let p3 = tuple::Point::new(1.0, 0.0, 0.0);

        let triangle = shape::Shape::triangle(p1, p2, p3);

        match &triangle.shape_type {
            shape::ShapeType::Triangle {
                p1: point1,
                p2: point2,
                p3: point3,
                e1,
                e2,
                normal,
            } => {
                assert_eq!(*point1, p1);
                assert_eq!(*point2, p2);
                assert_eq!(*point3, p3);
                assert_eq!(*e1, tuple::Vector::new(-1.0, -1.0, 0.0));
                assert_eq!(*e2, tuple::Vector::new(1.0, -1.0, 0.0));
                assert_eq!(*normal, tuple::Vector::new(0.0, 0.0, -1.0));
            }
            _ => panic!("expected a triangle"),
        }
    }

    #[test]
    fn test_finding_the_normal_on_a_triangle() {
        // Every point on a flat triangle shares the precomputed normal.
        let triangle = triangle();

        let expected = tuple::Vector::new(0.0, 0.0, -1.0);
        assert_eq!(
            triangle.local_normal_at(tuple::Point::new(0.0, 0.5, 0.0)),
            expected,
        );
        assert_eq!(
            triangle.local_normal_at(tuple::Point::new(-0.5, 0.75, 0.0)),
            expected,
        );
        assert_eq!(
            triangle.local_normal_at(tuple::Point::new(0.5, 0.25, 0.0)),
            expected,
        );
    }

    #[test]
    fn test_intersecting_a_ray_parallel_to_the_triangle() {
        // The ray points along y, parallel to the triangle's plane, so the
        // determinant in the intersection algorithm is zero.
        let triangle = triangle();
        let ray = ray::ray(
            tuple::Point::new(0.0, -1.0, -2.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );

        let intersections = triangle.local_intersect(ray);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_a_ray_misses_the_triangles_edges() {
        // One ray aimed past each edge of the triangle. The first misses
        // beyond the p1-p3 edge (caught by the u check); the other two miss
        // beyond the p1-p2 and p2-p3 edges (caught by the v checks).
        let examples = [
            ("p1-p3", tuple::Point::new(1.0, 1.0, -2.0)),
            ("p1-p2", tuple::Point::new(-1.0, 1.0, -2.0)),
            ("p2-p3", tuple::Point::new(0.0, -1.0, -2.0)),
        ];

        for (name, origin) in examples {
            let triangle = triangle();
            let ray = ray::ray(origin, tuple::Vector::new(0.0, 0.0, 1.0));

            let intersections = triangle.local_intersect(ray);

            assert_eq!(intersections.len(), 0, "case `{}`", name);
        }
    }

    #[test]
    fn test_a_ray_strikes_a_triangle() {
        // Unlike the other solids, a ray crosses a triangle at most once.
        let triangle = triangle();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.5, -2.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = triangle.local_intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_eq!(intersections[0].t, 2.0);
    }
}

#[cfg(test)]
mod smooth_triangle_tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::assert_tuple_approx_eq;
    use crate::intersection;
    use crate::ray;
    use crate::shape;
    use crate::tuple;

    // The book's background triangle: the same corners as the flat test
    // triangle, but with a vertex normal at each corner pointing up at the
    // apex and outward along x at the base corners.
    fn smooth_triangle() -> shape::Shape {
        shape::Shape::smooth_triangle(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Point::new(-1.0, 0.0, 0.0),
            tuple::Point::new(1.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
            tuple::Vector::new(-1.0, 0.0, 0.0),
            tuple::Vector::new(1.0, 0.0, 0.0),
        )
    }

    #[test]
    fn test_constructing_a_smooth_triangle() {
        // A smooth triangle stores a normal for each corner alongside the
        // corner points themselves.
        let triangle = smooth_triangle();

        match &triangle.shape_type {
            shape::ShapeType::SmoothTriangle {
                p1,
                p2,
                p3,
                n1,
                n2,
                n3,
                ..
            } => {
                assert_eq!(*p1, tuple::Point::new(0.0, 1.0, 0.0));
                assert_eq!(*p2, tuple::Point::new(-1.0, 0.0, 0.0));
                assert_eq!(*p3, tuple::Point::new(1.0, 0.0, 0.0));
                assert_eq!(*n1, tuple::Vector::new(0.0, 1.0, 0.0));
                assert_eq!(*n2, tuple::Vector::new(-1.0, 0.0, 0.0));
                assert_eq!(*n3, tuple::Vector::new(1.0, 0.0, 0.0));
            }
            _ => panic!("expected a smooth triangle"),
        }
    }

    #[test]
    fn test_an_intersection_with_a_smooth_triangle_stores_u_and_v() {
        // The barycentric u/v that Möller-Trumbore computes identify where
        // on the triangle the hit landed; they must be preserved on the
        // intersection so the normal can be interpolated later.
        let triangle = smooth_triangle();
        let ray = ray::ray(
            tuple::Point::new(-0.2, 0.3, -2.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = triangle.local_intersect(ray);

        assert_eq!(intersections.len(), 1);
        assert_approx_eq!(intersections[0].u, 0.45, 1e-5f64);
        assert_approx_eq!(intersections[0].v, 0.25, 1e-5f64);
    }

    #[test]
    fn test_a_smooth_triangle_interpolates_the_normal_from_u_and_v() {
        // The queried point is deliberately the origin: only the hit's u
        // and v drive the result, blending the three vertex normals with
        // the barycentric weights n2*u + n3*v + n1*(1 - u - v).
        let triangle = smooth_triangle();
        let hit = intersection::intersection_with_uv(1.0, &triangle, 0.45, 0.25);

        let normal = hit.normal_at(tuple::Point::new(0.0, 0.0, 0.0));

        assert_tuple_approx_eq!(tuple::Vector::new(-0.5547, 0.83205, 0.0), normal);
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
