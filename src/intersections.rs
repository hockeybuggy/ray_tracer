// TODO should the Intersection struct move here
use crate::ray;
use crate::sphere;
use crate::tuple;

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

#[cfg(test)]
mod intersections_tests {
    use crate::intersections;
    use crate::ray;
    use crate::sphere;
    use crate::tuple;

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
}
