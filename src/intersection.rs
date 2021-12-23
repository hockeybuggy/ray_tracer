use crate::color;
use crate::lighting;
use crate::ray;
use crate::shape;
use crate::tuple;
use crate::world;

const EPSILON: f64 = 1e-5;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a shape::Shape,
}

pub fn intersection(t: f64, object: &shape::Shape) -> Intersection {
    Intersection { t, object }
}

#[derive(Debug, PartialEq)]
pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a shape::Shape,

    pub point: tuple::Point,
    pub eyev: tuple::Vector,
    pub normalv: tuple::Vector,
    pub reflectv: tuple::Vector,
    pub inside: bool,
    pub over_point: tuple::Point,

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
    let normalv = object.normal_at(point);
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
        point,
        eyev,
        normalv: maybe_inverted_normalv,
        reflectv,
        inside,
        over_point: point + maybe_inverted_normalv * EPSILON,
        n1,
        n2,
    }
}

impl<'a> Computation<'a> {
    pub fn shade_hit(&self, world: &world::World, remaining: usize) -> color::Color {
        let has_light = &world.light.is_some();
        if !has_light {
            return color::black();
        }
        let shadowed = world::is_shadowed(&world, &self.over_point);
        let surface = lighting::lighting(
            &self.object.material,
            &self.object,
            &world.light.as_ref().unwrap(),
            &self.point,
            &self.eyev,
            &self.normalv,
            shadowed,
        );
        let reflected = world.reflected_color(&self, remaining);
        return surface + reflected;
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

        let intersections = ray.intersect(&sphere);

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
        world.light = Some(lights::point_light(
            tuple::Point::new(0.0, 0.25, 0.0),
            color::white(),
        ));
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
}
