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
    pub inside: bool,
    pub over_point: tuple::Point,
}

pub fn prepare_computations<'a>(
    intersection: &Intersection<'a>,
    ray: &ray::Ray,
) -> Computation<'a> {
    let t = intersection.t;
    let object = intersection.object;
    let point = ray.position(t);
    let eyev = -ray.direction;
    let normalv = object.normal_at(point);
    let inside: bool = tuple::dot(&normalv, &eyev) < 0.0;
    let maybe_inverted_normalv = if inside { -normalv } else { normalv };
    Computation {
        t,
        object,
        point,
        eyev,
        normalv: maybe_inverted_normalv,
        inside,
        over_point: point + maybe_inverted_normalv * EPSILON,
    }
}

impl<'a> Computation<'a> {
    pub fn shade_hit(&self, world: &world::World) -> color::Color {
        let shadowed = world::is_shadowed(&world, &self.over_point);
        match &world.light {
            Some(world_light) => lighting::lighting(
                &self.object.material,
                &self.object,
                world_light,
                &self.point,
                &self.eyev,
                &self.normalv,
                shadowed,
            ),
            None => color::black(),
        }
    }
}

#[cfg(test)]
mod intersection_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::intersection;
    use crate::lights;
    use crate::ray;
    use crate::shape;
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

        let computations = intersection::prepare_computations(&intersection, &ray);

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

        let computations = intersection::prepare_computations(&intersection, &ray);

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

        let computations = intersection::prepare_computations(&intersection, &ray);

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

        let computations = intersection::prepare_computations(&intersection, &ray);
        let colour = computations.shade_hit(&world);

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

        let computations = intersection::prepare_computations(&intersection, &ray);
        let colour = computations.shade_hit(&world);

        assert_color_approx_eq!(colour, color::color(0.90498, 0.90498, 0.90498));
    }
}
