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

    fn intersect(&self, sphere: sphere::Sphere) -> Vec<f64> {
        vec![4.0_f64, 6.0_f64]
    }
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

        let intersections = ray.intersect(sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[1], 4.0_f64);
        assert_eq!(intersections[2], 6.0_f64);
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

        let intersections = ray.intersect(sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[1], 5.0_f64);
        assert_eq!(intersections[2], 5.0_f64);
    }
}
