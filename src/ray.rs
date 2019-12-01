use crate::intersection;
use crate::matrix;
use crate::matrix::Inverse;
use crate::shape::Shape;
use crate::sphere;
use crate::tuple;
use crate::world;

#[derive(Debug)]
pub struct Ray {
    pub origin: tuple::Point,
    pub direction: tuple::Vector,
}

pub fn ray(origin: tuple::Point, direction: tuple::Vector) -> Ray {
    return Ray { origin, direction };
}

impl Ray {
    pub fn position(&self, t: f64) -> tuple::Point {
        self.origin + self.direction * t
    }

    pub fn intersect<'a>(&'a self, sphere: &'a sphere::Sphere) -> Vec<intersection::Intersection> {
        let transformed_ray = self.transform(&sphere.transformation_matrix().inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin - tuple::Point::new(0.0, 0.0, 0.0);

        let a = tuple::dot(&transformed_ray.direction, &transformed_ray.direction);
        let b = 2.0 * tuple::dot(&transformed_ray.direction, &sphere_to_ray);
        let c = tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminanant = b.powf(2.0) - 4.0 * a * c;

        let mut intersections = Vec::new();

        if discriminanant < 0.0 {
            return intersections;
        }

        let t1 = (-b - discriminanant.sqrt()) / (2.0 * a);
        intersections.push(intersection::intersection(t1, &sphere));
        let t2 = (-b + discriminanant.sqrt()) / (2.0 * a);
        intersections.push(intersection::intersection(t2, &sphere));

        return intersections;
    }

    pub fn intersect_world<'a>(
        &'a self,
        world: &'a world::World,
    ) -> Vec<intersection::Intersection> {
        let mut intersections: Vec<intersection::Intersection> = world
            .shapes
            .iter()
            .flat_map(|shape| self.intersect(&shape))
            .collect();
        intersections.sort_unstable_by(|x, y| x.t.partial_cmp(&y.t).unwrap());
        return intersections;
    }

    pub fn transform(&self, matrix: &matrix::Matrix4) -> Ray {
        Ray {
            origin: *matrix * self.origin,
            direction: *matrix * self.direction,
        }
    }
}

pub fn hit<'a>(
    intersections: &'a Vec<intersection::Intersection>,
) -> Option<&'a intersection::Intersection<'a>> {
    intersections
        .iter()
        .filter(|inter| inter.t.is_sign_positive())
        .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap()) // Not sure that this is a runtime safe unwrap
}

#[cfg(test)]
mod ray_tests {
    use crate::matrix;
    use crate::ray;
    use crate::shape::Shape;
    use crate::sphere;
    use crate::transformation::Transform;
    use crate::tuple;

    #[test]
    fn test_create_and_query_a_ray() {
        let origin = tuple::Point::new(1.0, 2.0, 3.0);
        let direction = tuple::Vector::new(4.0, 5.0, 6.0);

        let ray = ray::ray(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn test_compute_a_point_from_a_distance() {
        let ray = ray::ray(
            tuple::Point::new(2.0, 3.0, 4.0),
            tuple::Vector::new(1.0, 0.0, 0.0),
        );

        assert_eq!(ray.position(0.0), tuple::Point::new(2.0, 3.0, 4.0));
        assert_eq!(ray.position(1.0), tuple::Point::new(3.0, 3.0, 4.0));
        assert_eq!(ray.position(-1.0), tuple::Point::new(1.0, 3.0, 4.0));
        assert_eq!(ray.position(2.5), tuple::Point::new(4.5, 3.0, 4.0));
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
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = sphere::Sphere::default();

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

        let ray = ray::ray(
            tuple::Point::new(0.0, 1.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = sphere::Sphere::default();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 5.0_f64);
        assert_eq!(intersections[1].t, 5.0_f64);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 2.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = sphere::Sphere::default();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 0);
    }

    #[test]
    fn test_ray_originates_from_inside_a_sphere() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = sphere::Sphere::default();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -1.0_f64);
        assert_eq!(intersections[1].t, 1.0_f64);
    }

    #[test]
    fn test_ray_is_in_front_of_a_sphere() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let sphere = sphere::Sphere::default();

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, -6.0_f64);
        assert_eq!(intersections[1].t, -4.0_f64);
    }

    #[test]
    fn test_translating_a_ray() {
        let ray = ray::ray(
            tuple::Point::new(1.0, 2.0, 3.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        let tmatrix = matrix::Matrix4::IDENTITY.translation(3.0, 4.0, 5.0);

        let transformed_ray = ray.transform(&tmatrix);

        assert_eq!(transformed_ray.origin, tuple::Point::new(4.0, 6.0, 8.0));
        assert_eq!(transformed_ray.direction, tuple::Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn test_scaling_a_ray() {
        let ray = ray::ray(
            tuple::Point::new(1.0, 2.0, 3.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );
        let tmatrix = matrix::Matrix4::IDENTITY.scaling(2.0, 3.0, 4.0);

        let transformed_ray = ray.transform(&tmatrix);

        assert_eq!(transformed_ray.origin, tuple::Point::new(2.0, 6.0, 12.0));
        assert_eq!(transformed_ray.direction, tuple::Vector::new(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_spheres_default_transformation_matrix() {
        let sphere = sphere::Sphere::default();
        assert_eq!(sphere.transformation_matrix(), &matrix::Matrix4::IDENTITY);
    }

    #[test]
    fn test_spheres_can_have_its_transformation_set() {
        let mut sphere = sphere::Sphere::default();

        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(2.0, 3.0, 4.0));

        assert_eq!(
            sphere.transformation_matrix(),
            &matrix::Matrix4::IDENTITY.translation(2.0, 3.0, 4.0)
        );
    }

    #[test]
    fn test_intersecting_a_scaled_sphere_with_a_ray() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let mut sphere = sphere::Sphere::default();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(2.0, 2.0, 2.0));

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 2);
        assert_eq!(intersections[0].t, 3.0);
        assert_eq!(intersections[1].t, 7.0);
    }

    #[test]
    fn test_intersecting_a_translated_sphere_with_a_ray() {
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let mut sphere = sphere::Sphere::default();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(5.0, 0.0, 0.0));

        let intersections = ray.intersect(&sphere);

        assert_eq!(intersections.len(), 0);
    }
}
