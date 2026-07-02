use crate::color;
use crate::lighting;
use crate::matrix;
use crate::matrix::{Inverse, Transpose};
use crate::ray;
use crate::shape;
use crate::tuple;
use crate::world;

const EPSILON: f64 = 1e-5;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a shape::Shape,

    // The object-to-world transform. This starts as the object's own
    // transform, and each enclosing group prepends its transform as the
    // intersections bubble back up the tree.
    pub world_transform: matrix::Matrix4,
}

pub fn intersection(t: f64, object: &shape::Shape) -> Intersection<'_> {
    Intersection {
        t,
        object,
        world_transform: object.transform,
    }
}

impl<'a> Intersection<'a> {
    pub fn world_to_object(&self, world_point: tuple::Point) -> tuple::Point {
        return self.world_transform.inverse().unwrap() * world_point;
    }

    pub fn normal_to_world(&self, object_normal: tuple::Vector) -> tuple::Vector {
        let mut world_normal = self.world_transform.inverse().unwrap().transpose() * object_normal;
        // This is sorta a cheat to skip finding the submatrix.
        world_normal.w = 0.0;
        return tuple::normalize(&world_normal);
    }

    pub fn normal_at(&self, world_point: tuple::Point) -> tuple::Vector {
        let object_point = self.world_to_object(world_point);
        let object_normal = self.object.local_normal_at(object_point);
        return self.normal_to_world(object_normal);
    }
}

#[derive(Debug, PartialEq)]
pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a shape::Shape,
    pub world_transform: matrix::Matrix4,

    pub point: tuple::Point,
    pub eyev: tuple::Vector,
    pub normalv: tuple::Vector,
    pub reflectv: tuple::Vector,
    pub inside: bool,
    pub over_point: tuple::Point,
    pub under_point: tuple::Point,

    // refactive indices of either side of the ray-object intersection
    pub n1: f64,
    pub n2: f64,
}

pub fn prepare_computations<'a>(
    hit: &Intersection<'a>,
    ray: &ray::Ray,
    intersections: &Vec<&Intersection<'a>>,
) -> Computation<'a> {
    let t = hit.t;
    let object = hit.object;
    let point = ray.position(t);
    let eyev = -ray.direction;
    let normalv = hit.normal_at(point);
    let inside: bool = tuple::dot(&normalv, &eyev) < 0.0;
    let maybe_inverted_normalv = if inside { -normalv } else { normalv };
    let reflectv = ray.direction.reflect(&maybe_inverted_normalv);

    let mut containers: Vec<&shape::Shape> = vec![];
    let mut n1 = 1.0_f64;
    let mut n2 = 1.0_f64;
    for i in intersections.iter() {
        if i == &hit {
            if containers.is_empty() {
                n1 = 1.0;
            } else {
                n1 = containers.last().unwrap().material.refractive_index;
            }
        }

        let index_of_hit_object = containers.iter().position(|&x| x == i.object);
        if index_of_hit_object.is_some() {
            containers.remove(index_of_hit_object.unwrap());
        } else {
            containers.push(i.object);
        }

        if i == &hit {
            if containers.is_empty() {
                n2 = 1.0;
            } else {
                n2 = containers.last().unwrap().material.refractive_index;
            }
        }
    }

    Computation {
        t,
        object,
        world_transform: hit.world_transform,
        point,
        eyev,
        normalv: maybe_inverted_normalv,
        reflectv,
        inside,
        over_point: point + maybe_inverted_normalv * EPSILON,
        under_point: point - maybe_inverted_normalv * EPSILON,
        n1,
        n2,
    }
}

impl<'a> Computation<'a> {
    pub fn shade_hit(&self, world: &world::World, remaining: usize) -> color::Color {
        if world.lights.is_empty() {
            return color::black();
        }
        let mut surface = color::black();
        for light in world.lights.iter() {
            let shadowed = world::is_shadowed(&world, &light, &self.over_point);
            surface = surface
                + lighting::lighting(
                    &self.object.material,
                    &self.world_transform,
                    &light,
                    &self.point,
                    &self.eyev,
                    &self.normalv,
                    shadowed,
                );
        }
        let reflected = world.reflected_color(&self, remaining);
        let refracted = world.refracted_color(&self, remaining);

        if self.object.material.reflective > 0.0 && self.object.material.transparency > 0.0 {
            let reflectance = self.reflectance();
            return surface + reflected * reflectance + refracted * (1.0 - reflectance);
        }

        return surface + reflected + refracted;
    }

    pub fn reflectance(&self) -> f64 {
        let mut cos = tuple::dot(&self.eyev, &self.normalv);

        // Check for total internal reflection
        if self.n1 > self.n2 {
            let n_ratio = self.n1 / self.n2;
            let sin2_t = n_ratio.powf(2.0) * (1.0 - cos.powf(2.0));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();

            // When n1 > n2 use cos(theta_t)
            cos = cos_t
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);
        return r0 + (1.0 - r0) * (1.0 - cos).powf(5.0);
    }
}

#[cfg(test)]
mod intersection_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::intersection;
    use crate::lights;
    use crate::matrix;
    use crate::ray;
    use crate::shape;
    use crate::transformation::Transform;
    use crate::tuple;
    use crate::world;

    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_intersection_encapsulates_t_and_object() {
        let sphere = shape::Shape::default_sphere();
        let intersection = intersection::intersection(3.5, &sphere);

        assert_eq!(intersection.t, 3.5_f64);
        assert_eq!(intersection.object, &sphere);
    }

    #[test]
    fn test_intersections_in_a_vector() {
        let sphere = shape::Shape::default_sphere();
        let intersection1 = intersection::intersection(1.0, &sphere);
        let intersection2 = intersection::intersection(2.0, &sphere);

        let intersections = vec![intersection1, intersection2];

        assert_eq!(intersections[0].t, 1.0_f64);
        assert_eq!(intersections[1].t, 2.0_f64);
    }

    #[test]
    fn test_intersections_sets_the_object_in_the_intersection() {
        let sphere = shape::Shape::default_sphere();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = sphere.intersect(&ray);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].object, &sphere);
        assert_eq!(intersections[1].object, &sphere);
    }

    #[test]
    fn test_hit_all_intersections_positive_t() {
        let sphere = shape::Shape::default_sphere();
        let intersection1 = intersection::intersection(1.0, &sphere);
        let intersection2 = intersection::intersection(2.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        let expected = intersection::intersection(1.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_hit_some_intersections_have_negitive_t() {
        let sphere = shape::Shape::default_sphere();
        let intersection1 = intersection::intersection(-1.0, &sphere);
        let intersection2 = intersection::intersection(1.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        let expected = intersection::intersection(1.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_hit_all_intersections_negitive_t() {
        let sphere = shape::Shape::default_sphere();
        let intersection1 = intersection::intersection(-2.0, &sphere);
        let intersection2 = intersection::intersection(-1.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        assert_eq!(hit, None);
    }

    #[test]
    fn test_hit_is_always_the_lowest() {
        let sphere = shape::Shape::default_sphere();
        let intersection1 = intersection::intersection(5.0, &sphere);
        let intersection2 = intersection::intersection(7.0, &sphere);
        let intersection3 = intersection::intersection(-3.0, &sphere);
        let intersection4 = intersection::intersection(2.0, &sphere);
        let intersections = vec![intersection1, intersection2, intersection3, intersection4];

        let hit = ray::hit(&intersections);

        let expected = intersection::intersection(2.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_precompute_intersection_state() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let shape = shape::Shape::default_sphere();
        let intersection = intersection::intersection(4.0, &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);

        assert_eq!(computations.t, intersection.t);
        assert_eq!(computations.object, intersection.object);
        assert_eq!(computations.point, tuple::Point::new(0.0, 0.0, -1.0));
        assert_eq!(computations.eyev, tuple::Vector::new(0.0, 0.0, -1.0));
        assert_eq!(computations.normalv, tuple::Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_prepare_computations_when_the_intersection_occurs_on_the_outside() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let shape = shape::Shape::default_sphere();
        let intersection = intersection::intersection(4.0, &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);

        assert_eq!(computations.inside, false);
    }

    #[test]
    fn test_prepare_computations_when_the_intersection_occurs_on_the_inside() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let shape = shape::Shape::default_sphere();
        let intersection = intersection::intersection(1.0, &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);

        assert_eq!(computations.point, tuple::Point::new(0.0, 0.0, 1.0));
        assert_eq!(computations.eyev, tuple::Vector::new(0.0, 0.0, -1.0));
        assert_eq!(computations.inside, true);
        // Normal is inverted
        assert_eq!(computations.normalv, tuple::Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_shading_an_intersection() {
        let world = world::default_world();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let shape = &world.shapes[0];
        let intersection = intersection::intersection(4.0, &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);
        let colour = computations.shade_hit(&world, 10);

        assert_color_approx_eq!(colour, color::color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_inside() {
        let mut world = world::default_world();
        world.lights = vec![lights::point_light(
            tuple::Point::new(0.0, 0.25, 0.0),
            color::white(),
        )];
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let shape = &world.shapes[1];
        let intersection = intersection::intersection(0.5, &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);
        let colour = computations.shade_hit(&world, 10);

        assert_color_approx_eq!(colour, color::color(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn test_precompute_the_reflection_vector() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 1.0, -1.0),
            tuple::Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let shape = shape::Shape::default_plane();
        let intersection = intersection::intersection(2.0_f64.sqrt(), &shape);

        let computations =
            intersection::prepare_computations(&intersection, &ray, &vec![&intersection]);

        assert_eq!(computations.t, intersection.t);
        assert_eq!(computations.object, intersection.object);
        assert_eq!(
            computations.reflectv,
            tuple::Vector::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
    }

    // There is a diagram for this test on page 205. A, B, and C are glass spheres and a ray is
    // cast through the center of all three. B and C are contained within A and overlap each other
    // slightly. There are 5 points of refraction along the ray enumerated 0 through 5:
    //   0. The ray entering A
    //   1. The ray within A entering B
    //   2. The ray within A and B entering C
    //   3. The ray within A and C exiting B
    //   4. The ray within A exiting C
    //   5. The ray exiting A
    #[test]
    fn test_finding_n1_and_n2_at_various_locations() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -4.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let mut sphere_a = shape::Shape::glass_sphere();
        sphere_a.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(2.0, 2.0, 2.0));
        sphere_a.material.refractive_index = 1.5;
        let mut sphere_b = shape::Shape::glass_sphere();
        sphere_b.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 0.0, -0.25));
        sphere_b.material.refractive_index = 2.0;
        let mut sphere_c = shape::Shape::glass_sphere();
        sphere_c.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 0.0, 0.25));
        sphere_c.material.refractive_index = 2.5;
        let intersections = vec![
            intersection::intersection(2.0, &sphere_a),
            intersection::intersection(2.75, &sphere_b),
            intersection::intersection(3.25, &sphere_c),
            intersection::intersection(4.75, &sphere_b),
            intersection::intersection(5.25, &sphere_c),
            intersection::intersection(6.0, &sphere_a),
        ];
        let xs: Vec<&intersection::Intersection> = intersections.iter().collect();

        //   0. The ray entering A
        let computations_intersection_0 =
            intersection::prepare_computations(&intersections[0], &ray, &xs);
        assert_eq!(computations_intersection_0.n1, 1.0);
        assert_eq!(computations_intersection_0.n2, 1.5);
        //   1. The ray within A entering B
        let computations_intersection_1 =
            intersection::prepare_computations(&intersections[1], &ray, &xs);
        assert_eq!(computations_intersection_1.n1, 1.5);
        assert_eq!(computations_intersection_1.n2, 2.0);
        //   2. The ray within A and B entering C
        let computations_intersection_2 =
            intersection::prepare_computations(&intersections[2], &ray, &xs);
        assert_eq!(computations_intersection_2.n1, 2.0);
        assert_eq!(computations_intersection_2.n2, 2.5);
        //   3. The ray within A and C exiting B
        let computations_intersection_3 =
            intersection::prepare_computations(&intersections[3], &ray, &xs);
        assert_eq!(computations_intersection_3.n1, 2.5);
        assert_eq!(computations_intersection_3.n2, 2.5);
        //   4. The ray within A exiting C
        let computations_intersection_4 =
            intersection::prepare_computations(&intersections[4], &ray, &xs);
        assert_eq!(computations_intersection_4.n1, 2.5);
        assert_eq!(computations_intersection_4.n2, 1.5);
        //   5. The ray exiting A
        let computations_intersection_5 =
            intersection::prepare_computations(&intersections[5], &ray, &xs);
        assert_eq!(computations_intersection_5.n1, 1.5);
        assert_eq!(computations_intersection_5.n2, 1.0);
    }

    #[test]
    fn test_the_under_point_is_offset_below_the_surface() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let mut sphere_a = shape::Shape::glass_sphere();
        sphere_a.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 0.0, 1.0));
        let intersections = vec![intersection::intersection(5.0, &sphere_a)];
        let xs: Vec<&intersection::Intersection> = intersections.iter().collect();

        let computations_intersection =
            intersection::prepare_computations(&intersections[0], &ray, &xs);

        assert!(computations_intersection.under_point.z > intersection::EPSILON / 2.0);
        assert!(computations_intersection.point.z < computations_intersection.under_point.z);
    }

    #[test]
    fn test_the_schlick_approximation_under_total_internal_reflection() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        let sphere = shape::Shape::glass_sphere();
        let intersections = vec![
            intersection::intersection(-2.0_f64.sqrt() / 2.0, &sphere),
            intersection::intersection(2.0_f64.sqrt() / 2.0, &sphere),
        ];
        let xs: Vec<&intersection::Intersection> = intersections.iter().collect();
        let computations_intersection =
            intersection::prepare_computations(&intersections[1], &ray, &xs);

        let reflectance = computations_intersection.reflectance();

        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn test_the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        let sphere = shape::Shape::glass_sphere();
        let intersections = vec![
            intersection::intersection(-1.0, &sphere),
            intersection::intersection(1.0, &sphere),
        ];
        let xs: Vec<&intersection::Intersection> = intersections.iter().collect();
        let computations_intersection =
            intersection::prepare_computations(&intersections[1], &ray, &xs);

        let reflectance = computations_intersection.reflectance();

        assert_approx_eq!(reflectance, 0.04);
    }

    #[test]
    fn test_the_schlick_approximation_with_a_small_angle_and_n2_gt_n1() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.99, -2.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = shape::Shape::glass_sphere();
        let intersections = vec![intersection::intersection(1.8589, &sphere)];
        let xs: Vec<&intersection::Intersection> = intersections.iter().collect();
        let computations_intersection =
            intersection::prepare_computations(&intersections[0], &ray, &xs);

        let reflectance = computations_intersection.reflectance();

        assert_approx_eq!(reflectance, 0.48873);
    }
}

#[cfg(test)]
mod group_intersection_tests {
    use crate::assert_tuple_approx_eq;
    use crate::intersection;
    use crate::matrix;
    use crate::ray;
    use crate::shape;
    use crate::transformation::Transform;
    use crate::tuple;

    // The book's nested hierarchy: an outer group rotated a quarter turn
    // around y, containing a scaled inner group, containing a sphere
    // translated 5 units in x.
    fn nested_groups(scale: (f64, f64, f64)) -> shape::Shape {
        let mut inner = shape::Shape::default_group();
        inner.set_transformation_matrix(
            matrix::Matrix4::IDENTITY.scaling(scale.0, scale.1, scale.2),
        );
        let mut sphere = shape::Shape::default_sphere();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(5.0, 0.0, 0.0));
        inner.add_child(sphere);

        let mut outer = shape::Shape::default_group();
        outer.set_transformation_matrix(
            matrix::Matrix4::IDENTITY.rotation_y(std::f64::consts::PI / 2.0),
        );
        outer.add_child(inner);
        return outer;
    }

    #[test]
    fn test_converting_a_point_from_world_to_object_space() {
        // With uniform scaling the sphere ends up centered at (0, 0, -10)
        // with radius 2, so the ray down the z axis hits it, and the queried
        // point sits on its -x side: one radius left of center, which is
        // (0, 0, -1) in the sphere's own space.
        let group = nested_groups((2.0, 2.0, 2.0));
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -20.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersections = group.intersect(&ray);

        let point = intersections[0].world_to_object(tuple::Point::new(-2.0, 0.0, -10.0));

        assert_tuple_approx_eq!(tuple::Point::new(0.0, 0.0, -1.0), point);
    }

    #[test]
    fn test_converting_a_normal_from_object_to_world_space() {
        // The non-uniform scaling turns the sphere into an ellipsoid
        // centered at (0, 0, -5). The expected vector is the book's
        // (0.2857, 0.4286, -0.8571) in exact form.
        let group = nested_groups((1.0, 2.0, 3.0));
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -10.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersections = group.intersect(&ray);

        let sqrt3_over_3 = 3.0_f64.sqrt() / 3.0;
        let normal = intersections[0].normal_to_world(tuple::Vector::new(
            sqrt3_over_3,
            sqrt3_over_3,
            sqrt3_over_3,
        ));

        assert_tuple_approx_eq!(tuple::Vector::new(2.0 / 7.0, 3.0 / 7.0, -6.0 / 7.0), normal);
    }

    #[test]
    fn test_finding_the_normal_on_a_child_object() {
        // The queried point lies on the ellipsoid's surface. The book
        // rounds both the point and the resulting normal to four decimal
        // places; the expected value here is computed from the rounded
        // point, which is why it differs slightly from the previous test.
        let group = nested_groups((1.0, 2.0, 3.0));
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -10.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersections = group.intersect(&ray);

        let normal = intersections[0].normal_at(tuple::Point::new(1.7321, 1.1547, -5.5774));

        assert_tuple_approx_eq!(tuple::Vector::new(0.28570, 0.42854, -0.85716), normal);
    }

    #[test]
    fn test_preparing_computations_on_a_child_of_transformed_groups() {
        // The ray hits the ellipsoid head on at (0, 0, -6), where the
        // surface faces straight back at the ray. Using only the sphere's
        // own transform here would produce a wildly wrong normal.
        let group = nested_groups((1.0, 2.0, 3.0));
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -10.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersections = group.intersect(&ray);
        let intersection_refs: Vec<&intersection::Intersection> = intersections.iter().collect();

        let computations =
            intersection::prepare_computations(&intersections[0], &ray, &intersection_refs);

        assert_tuple_approx_eq!(tuple::Point::new(0.0, 0.0, -6.0), computations.point);
        assert_tuple_approx_eq!(tuple::Vector::new(0.0, 0.0, -1.0), computations.normalv);
    }
}

#[cfg(test)]
mod uv_intersection_tests {
    use crate::assert_tuple_approx_eq;
    use crate::intersection;
    use crate::ray;
    use crate::shape;
    use crate::tuple;

    #[test]
    fn test_an_intersection_can_encapsulate_u_and_v() {
        // Only triangle intersections carry meaningful u/v values;
        // intersections built the ordinary way leave them at zero.
        let triangle = shape::Shape::triangle(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Point::new(-1.0, 0.0, 0.0),
            tuple::Point::new(1.0, 0.0, 0.0),
        );

        let intersection = intersection::intersection_with_uv(3.5, &triangle, 0.2, 0.4);

        assert_eq!(intersection.u, 0.2);
        assert_eq!(intersection.v, 0.4);
    }

    #[test]
    fn test_preparing_the_normal_on_a_smooth_triangle() {
        // prepare_computations computes the normal via the hit itself, so
        // the smooth triangle's interpolated normal flows through to
        // normalv without any extra plumbing.
        let triangle = shape::Shape::smooth_triangle(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Point::new(-1.0, 0.0, 0.0),
            tuple::Point::new(1.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
            tuple::Vector::new(-1.0, 0.0, 0.0),
            tuple::Vector::new(1.0, 0.0, 0.0),
        );
        let hit = intersection::intersection_with_uv(1.0, &triangle, 0.45, 0.25);
        let ray = ray::ray(
            tuple::Point::new(-0.2, 0.3, -2.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersection_refs = vec![&hit];

        let computations = intersection::prepare_computations(&hit, &ray, &intersection_refs);

        assert_tuple_approx_eq!(
            tuple::Vector::new(-0.5547, 0.83205, 0.0),
            computations.normalv
        );
    }
}
