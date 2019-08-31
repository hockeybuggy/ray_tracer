use crate::matrix;
use crate::matrix::Inverse;
use crate::sphere;
use crate::tuple;

#[derive(Debug)]
pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}

pub fn ray(origin: tuple::Tuple, direction: tuple::Tuple) -> Ray {
    return Ray { origin, direction };
}

impl Ray {
    pub fn position(&self, t: f64) -> tuple::Tuple {
        self.origin + self.direction * t
    }

    pub fn intersect<'a>(&'a self, sphere: &'a sphere::Sphere) -> Vec<Intersection> {
        let transformed_ray = self.transform(&sphere.transform.inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin - tuple::point(0.0, 0.0, 0.0);

        let a = tuple::dot(&transformed_ray.direction, &transformed_ray.direction);
        let b = 2.0 * tuple::dot(&transformed_ray.direction, &sphere_to_ray);
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

    fn transform(&self, matrix: &matrix::Matrix4) -> Ray {
        Ray {
            origin: *matrix * self.origin,
            direction: *matrix * self.direction,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a sphere::Sphere,
}

fn intersection(t: f64, object: &sphere::Sphere) -> Intersection {
    Intersection { t, object }
}

pub fn hit<'a>(intersections: &'a Vec<Intersection>) -> Option<&'a Intersection<'a>> {
    intersections
        .iter()
        .filter(|inter| inter.t.is_sign_positive())
        .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap()) // Not sure that this is a runtime safe unwrap
}

#[cfg(test)]
mod ray_tests {
    use crate::matrix;
    use crate::ray;
    use crate::sphere;
    use crate::transformation;
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

    #[test]
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
        assert_eq!(intersections[0].t, 4.0_f64);
        assert_eq!(intersections[1].t, 6.0_f64);
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

    #[test]
    fn test_hit_all_intersections_positive_t() {
        let sphere = sphere::sphere();
        let intersection1 = ray::intersection(1.0, &sphere);
        let intersection2 = ray::intersection(2.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        let expected = ray::intersection(1.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_hit_some_intersections_have_negitive_t() {
        let sphere = sphere::sphere();
        let intersection1 = ray::intersection(-1.0, &sphere);
        let intersection2 = ray::intersection(1.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        let expected = ray::intersection(1.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_hit_all_intersections_negitive_t() {
        let sphere = sphere::sphere();
        let intersection1 = ray::intersection(-2.0, &sphere);
        let intersection2 = ray::intersection(-1.0, &sphere);
        let intersections = vec![intersection1, intersection2];

        let hit = ray::hit(&intersections);

        assert_eq!(hit, None);
    }

    #[test]
    fn test_hit_is_always_the_lowest() {
        let sphere = sphere::sphere();
        let intersection1 = ray::intersection(5.0, &sphere);
        let intersection2 = ray::intersection(7.0, &sphere);
        let intersection3 = ray::intersection(-3.0, &sphere);
        let intersection4 = ray::intersection(2.0, &sphere);
        let intersections = vec![intersection1, intersection2, intersection3, intersection4];

        let hit = ray::hit(&intersections);

        let expected = ray::intersection(2.0, &sphere);
        assert_eq!(hit.unwrap(), &expected);
    }

    #[test]
    fn test_translating_a_ray() {
        let ray = ray::ray(tuple::point(1.0, 2.0, 3.0), tuple::vector(0.0, 1.0, 0.0));
        let tmatrix = transformation::translation(3.0, 4.0, 5.0);

        let transformed_ray = ray.transform(&tmatrix);

        assert_eq!(transformed_ray.origin, tuple::point(4.0, 6.0, 8.0));
        assert_eq!(transformed_ray.direction, tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let ray = ray::ray(tuple::point(1.0, 2.0, 3.0), tuple::vector(0.0, 1.0, 0.0));
        let tmatrix = transformation::scaling(2.0, 3.0, 4.0);

        let transformed_ray = ray.transform(&tmatrix);

        assert_eq!(transformed_ray.origin, tuple::point(2.0, 6.0, 12.0));
        assert_eq!(transformed_ray.direction, tuple::vector(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_spheres_default_transformation_matrix() {
        let sphere = sphere::sphere();
        assert_eq!(sphere.transform, matrix::Matrix4::IDENTITY);
    }

    #[test]
    fn test_spheres_can_have_its_transformation_set() {
        let mut sphere = sphere::sphere();

        sphere.transform = transformation::translation(2.0, 3.0, 4.0);

        assert_eq!(sphere.transform, transformation::translation(2.0, 3.0, 4.0));
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = sphere::sphere();
        sphere.transform = transformation::scaling(2.0, 2.0, 2.0);

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));

        let mut sphere = sphere::sphere();
        sphere.transform = transformation::translation(5.0, 0.0, 0.0);

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 0);
    }
}
