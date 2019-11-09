// TODO should the Intersection struct move here
use crate::color;
use crate::lighting;
use crate::ray;
use crate::sphere;
use crate::tuple;
use crate::world;

#[derive(Debug, PartialEq)]
pub struct Computation<'a> {
    pub t: f64,
    pub object: &'a sphere::Sphere,

    pub point: tuple::Tuple,
    pub eyev: tuple::Tuple,
    pub normalv: tuple::Tuple,
    pub inside: bool,
}

pub fn prepare_computations<'a>(
    intersection: &ray::Intersection<'a>,
    ray: &ray::Ray,
) -> Computation<'a> {
    let t = intersection.t;
    let object = intersection.object;
    let point = ray.position(t);
    let eyev = -ray.direction;
    let normalv = object.normal_at(point);
    let inside: bool = tuple::dot(&normalv, &eyev) < 0.0;
    Computation {
        t,
        object,
        point,
        eyev,
        normalv: if inside { -normalv } else { normalv },
        inside,
    }
}

impl<'a> Computation<'a> {
    pub fn shade_hit(&self, world: &world::World) -> color::Color {
        match &world.light {
            Some(world_light) => lighting::lighting(
                &self.object.material,
                world_light,
                &self.point,
                &self.eyev,
                &self.normalv,
            ),
            None => color::black(),
        }
    }
}

#[cfg(test)]
mod intersections_tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::color;
    use crate::intersections;
    use crate::lights;
    use crate::ray;
    use crate::sphere;
    use crate::tuple;
    use crate::world;

    // TODO factor these out into some kind of test utils
    macro_rules! assert_color_approx_eq {
        ($a:expr, $b:expr) => {{
            assert_approx_eq!($a.r, $b.r, 1e-5f64);
            assert_approx_eq!($a.g, $b.g, 1e-5f64);
            assert_approx_eq!($a.b, $b.b, 1e-5f64);
        }};
    }

    #[test]
    fn test_precompute_intersection_state() {
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere::sphere();
        let intersection = ray::intersection(4.0, &shape);

        let computations = intersections::prepare_computations(&intersection, &ray);

        assert_eq!(computations.t, intersection.t);
        assert_eq!(computations.object, intersection.object);
        assert_eq!(computations.point, tuple::point(0.0, 0.0, -1.0));
        assert_eq!(computations.eyev, tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(computations.normalv, tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_prepare_computations_when_the_intersection_occurs_on_the_outside() {
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere::sphere();
        let intersection = ray::intersection(4.0, &shape);

        let computations = intersections::prepare_computations(&intersection, &ray);

        assert_eq!(computations.inside, false);
    }

    #[test]
    fn test_prepare_computations_when_the_intersection_occurs_on_the_inside() {
        let ray = ray::ray(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let shape = sphere::sphere();
        let intersection = ray::intersection(1.0, &shape);

        let computations = intersections::prepare_computations(&intersection, &ray);

        assert_eq!(computations.point, tuple::point(0.0, 0.0, 1.0));
        assert_eq!(computations.eyev, tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(computations.inside, true);
        // Normal is inverted
        assert_eq!(computations.normalv, tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_shading_an_intersection() {
        let world = world::default_world();
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let shape = &world.shapes[0];
        let intersection = ray::intersection(4.0, &shape);

        let computations = intersections::prepare_computations(&intersection, &ray);
        let colour = computations.shade_hit(&world);

        assert_color_approx_eq!(colour, color::color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_shading_an_intersection_from_inside() {
        let mut world = world::default_world();
        world.light = Some(lights::point_light(
            tuple::point(0.0, 0.25, 0.0),
            color::white(),
        ));
        let ray = ray::ray(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let shape = &world.shapes[1];
        let intersection = ray::intersection(0.5, &shape);

        let computations = intersections::prepare_computations(&intersection, &ray);
        let colour = computations.shade_hit(&world);

        assert_color_approx_eq!(colour, color::color(0.90498, 0.90498, 0.90498));
    }
}
