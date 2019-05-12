use crate::sphere;
use crate::tuple;

struct Ray {
    origin: tuple::Tuple,
    direction: tuple::Tuple,
}

fn ray(origin: tuple::Tuple, direction: tuple::Tuple) -> Ray {
    return Ray { origin, direction };
}

impl Ray {
    fn position(&self, t: f64) -> tuple::Tuple {
        self.origin + self.direction * t
    }

    fn intersect<'a>(&'a self, sphere: &'a sphere::Sphere) -> Vec<Intersection> {
        let sphere_to_ray = self.origin - tuple::point(0.0, 0.0, 0.0);

        let a = tuple::dot(&self.direction, &self.direction);
        let b = 2.0 * tuple::dot(&self.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminanant = b.powf(2.0) - 4.0 * a * c;

        let mut intersections = Vec::new();

        if discriminanant < 0.0 {
            return intersections;
        }

        let t1 = (-b - discriminanant.sqrt()) / (2.0 * a);
        intersections.push(intersection(t1, &sphere));
        let t2 = (-b + discriminanant.sqrt()) / (2.0 * a);
        intersections.push(intersection(t2, &sphere));

        return intersections;
    }
}

struct Intersection<'a> {
    t: f64,
    object: &'a sphere::Sphere,
}

fn intersection(t: f64, object: &sphere::Sphere) -> Intersection {
    Intersection { t, object }
}

#[cfg(test)]
mod intersection_tests {
    use crate::ray;
    use crate::sphere;
}

#[cfg(test)]
mod ray_tests {
    use crate::ray;
    use crate::sphere;
    use crate::tuple;

    #[test]
    fn test_create_and_query_a_ray() {
        let origin = tuple::point(1.0, 2.0, 3.0);
        let direction = tuple::vector(4.0, 5.0, 6.0);

        let ray = ray::ray(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_compute_a_point_from_a_distance() {
        let ray = ray::ray(tuple::point(2.0, 3.0, 4.0), tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(ray.position(0.0), tuple::point(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), tuple::point(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), tuple::point(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), tuple::point(4.5, 3.0, 4.0));
    }

    fn test_ray_intersects_a_sphere_at_two_points() {
        /*
           Sphere at origin, ray along the z
                                        x  x
                                     x        x
             o -------------------- A -------- B ---->
                                    x          x
                                     x        x
                                        x  x
            o: (0, 0, -5)
            A: (0, 0, -1)
            B: (0, 0, 1)
        */
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let sphere = sphere::sphere();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[1].t, 4.0_f64);
        assert_eq!(intersections[2].t, 6.0_f64);
    }

    #[test]
    fn test_ray_intersects_a_sphere_at_a_tangent() {
        /*
           Sphere at origin, ray along a tangent

             o ---------------------------A--------->
                                     x        x
                                    x          x
                                    x          x
                                     x        x
                                        x  x
            o: (0, 0, -5)
            A: (0, 1, 0)
        */

        let ray = ray::ray(tuple::point(0.0, 1.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let sphere = sphere::sphere();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0_f64);
        assert_eq!(intersections[1].t, 5.0_f64);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let ray = ray::ray(tuple::point(0.0, 2.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let sphere = sphere::sphere();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_originates_from_inside_a_sphere() {
        let ray = ray::ray(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let sphere = sphere::sphere();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0_f64);
        assert_eq!(intersections[1].t, 1.0_f64);
    }

    #[test]
    fn test_ray_is_in_front_of_a_sphere() {
        let ray = ray::ray(tuple::point(0.0, 0.0, 5.0), tuple::vector(0.0, 0.0, 1.0));
        let sphere = sphere::sphere();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0_f64);
        assert_eq!(intersections[1].t, -4.0_f64);
    }

    #[test]
    fn test_intersection_encapsulates_t_and_object() {
        let sphere = sphere::sphere();
        let intersection = ray::intersection(3.5, &sphere);

        assert_eq!(intersection.t, 3.5_f64);
        assert_eq!(intersection.object, &sphere);
    }

    #[test]
    fn test_intersections_in_a_vector() {
        let sphere = sphere::sphere();
        let intersection1 = ray::intersection(1.0, &sphere);
        let intersection2 = ray::intersection(2.0, &sphere);

        let intersections = vec![intersection1, intersection2];

        assert_eq!(intersections[0].t, 1.0_f64);
        assert_eq!(intersections[1].t, 2.0_f64);
    }

    #[test]
    fn test_intersections_sets_the_object_in_the_intersection() {
        let sphere = sphere::sphere();
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].object, &sphere);
        assert_eq!(intersections[1].object, &sphere);
    }
}
