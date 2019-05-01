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
}

#[cfg(test)]
mod ray_tests {
    use crate::ray;
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
}
