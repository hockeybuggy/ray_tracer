use crate::color;
use crate::intersection;
use crate::lights;
use crate::matrix;
use crate::ray;
use crate::shape;
use crate::transformation::Transform;
use crate::tuple;

pub struct World {
    // TODO Adding multiple light sources can be done by changing this to a vector of lights.
    pub light: Option<lights::Light>,
    pub shapes: Vec<shape::Shape>,
}

pub fn world() -> World {
    World {
        light: None,
        shapes: Vec::new(),
    }
}

impl World {
    pub fn color_at(&self, ray: &ray::Ray, remaining: usize) -> color::Color {
        let intersections = ray.intersect_world(&self);
        let hit = ray::hit(&intersections);
        if hit.is_none() {
            return color::black();
        }
        let computations = intersection::prepare_computations(&hit.unwrap(), &ray);
        return computations.shade_hit(&self, remaining - 1);
    }

    pub fn reflected_color(
        &self,
        computations: &intersection::Computation,
        remaining: usize,
    ) -> color::Color {
        if remaining == 0 {
            return color::black();
        }
        if computations.object.material.reflective == 0.0 {
            return color::black();
        }

        let reflected_ray = ray::ray(computations.over_point, computations.reflectv);
        let color = self.color_at(&reflected_ray, remaining);

        return color * computations.object.material.reflective;
    }
}

pub fn default_world() -> World {
    let white_point_light =
        lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
    let mut lime_sphere = shape::Shape::default_sphere();
    lime_sphere.material.color = color::color(0.8, 1.0, 0.6);
    lime_sphere.material.diffuse = 0.7;
    lime_sphere.material.specular = 0.2;
    let mut small_sphere = shape::Shape::default_sphere();
    small_sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.5, 0.5, 0.5));
    let shapes = vec![lime_sphere, small_sphere];
    World {
        light: Some(white_point_light),
        shapes: shapes,
    }
}

pub fn is_shadowed(world: &World, point: &tuple::Point) -> bool {
    // TODO this unwrap doesn't feel safe
    let v = world.light.as_ref().unwrap().position - *point;
    let distance = tuple::magnitude(&v);
    let direction = tuple::normalize(&v);

    let ray = ray::ray(*point, direction);
    let intersections = ray.intersect_world(&world);

    let hit = ray::hit(&intersections);
    if hit.is_some() && hit.unwrap().t < distance {
        return true;
    }
    false
}

#[cfg(test)]
mod world_tests {
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
    fn empty_world() {
        let world = world::world();

        assert_eq!(world.light.is_some(), false);
        assert_eq!(world.shapes.len(), 0);
    }

    #[test]
    fn default_world_properties() {
        let world = world::default_world();

        // There is a white point light in the world.
        let expected_light =
            lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
        assert_eq!(world.light.is_some(), true);
        let world_light = world.light.unwrap();
        assert_eq!(world_light.position, expected_light.position);
        assert_eq!(world_light.intensity, expected_light.intensity);
        // There are two spheres
        assert_eq!(world.shapes.len(), 2);
        // One is a different color
        let mut expected_s1 = shape::Shape::default_sphere();
        expected_s1.material.color = color::color(0.8, 1.0, 0.6);
        expected_s1.material.diffuse = 0.7;
        expected_s1.material.specular = 0.2;
        let first_shape = &world.shapes[0];
        assert_eq!(first_shape.material.color, expected_s1.material.color);
        // One is a different size
        let mut expected_s2 = shape::Shape::default_sphere();
        expected_s2.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.5, 0.5, 0.5));

        let second_shape = &world.shapes[1];
        assert_eq!(
            second_shape.transformation_matrix(),
            expected_s2.transformation_matrix()
        );
    }

    #[test]
    fn default_world_intersected_with_a_ray() {
        let world = world::default_world();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let intersections = ray.intersect_world(&world);

        assert_eq!(intersections.len(), 4);
        // Note that these are sorted by `.t`
        assert_eq!(intersections[0].t, 4.0_f64);
        assert_eq!(intersections[1].t, 4.5_f64);
        assert_eq!(intersections[2].t, 5.5_f64);
        assert_eq!(intersections[3].t, 6.0_f64);
    }

    #[test]
    fn color_at_when_a_ray_misses() {
        let world = world::default_world();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );

        let color = world.color_at(&ray, 10);

        assert_color_approx_eq!(color, color::black());
    }

    #[test]
    fn color_at_when_a_ray_hits() {
        let world = world::default_world();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );

        let color = world.color_at(&ray, 10);

        assert_color_approx_eq!(color, color::color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_at_with_an_intersection_behind_the_ray() {
        let mut world = world::default_world();
        {
            let outer = world.shapes.get_mut(0);
            outer.unwrap().material.ambient = 1.0;
            let inner = world.shapes.get_mut(1);
            inner.unwrap().material.ambient = 1.0;
        }
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.075),
            tuple::Vector::new(0.0, 0.0, -1.0),
        );

        let color = world.color_at(&ray, 10);

        let inner = world.shapes.get_mut(1);
        assert_color_approx_eq!(color, inner.as_ref().unwrap().material.color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = world::default_world();
        let point = tuple::Point::new(0.0, 10.0, 0.0);

        assert_eq!(world::is_shadowed(&world, &point), false);
    }

    #[test]
    fn there_is_a_shadow_when_an_object_is_between_the_point_and_the_light() {
        let world = world::default_world();
        let point = tuple::Point::new(10.0, -10.0, 10.0);

        assert_eq!(world::is_shadowed(&world, &point), true);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let world = world::default_world();
        let point = tuple::Point::new(-20.0, 20.0, -20.0);

        assert_eq!(world::is_shadowed(&world, &point), false);
    }

    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let world = world::default_world();
        let point = tuple::Point::new(-2.0, 2.0, -2.0);

        assert_eq!(world::is_shadowed(&world, &point), false);
    }

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut world = world::world();
        let light_position = tuple::Point::new(0.0, 0.0, -10.0);
        let light_color = color::color(1.0, 1.0, 1.0);
        world.light = Some(lights::point_light(light_position, light_color));
        let sphere1 = shape::Shape::default_sphere();
        world.shapes.push(sphere1);
        let mut sphere2 = shape::Shape::default_sphere();
        sphere2.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 0.0, 10.0));
        world.shapes.push(sphere2);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let intersection = intersection::intersection(4.0, &world.shapes[1]);

        let computations = intersection::prepare_computations(&intersection, &ray);
        let color = computations.shade_hit(&world, 10);
        let expected_color = color::color(0.1, 0.1, 0.1);
        assert_color_approx_eq!(color, expected_color);
    }

    #[test]
    fn reflected_color_for_a_nonreflective_material() {
        let mut world = world::default_world();
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        let mut sphere = shape::Shape::default_sphere();
        sphere.material.ambient = 1.0;
        world.shapes.push(sphere);
        let intersection = intersection::intersection(1.0, &world.shapes[2]);

        let computations = intersection::prepare_computations(&intersection, &ray);
        let color = world.reflected_color(&computations, 10);

        let expected_color = color::color(0.0, 0.0, 0.0);
        assert_color_approx_eq!(color, expected_color);
    }

    #[test]
    fn reflected_color_for_a_reflective_material() {
        let mut world = world::default_world();
        let mut plane = shape::Shape::default_plane();
        plane.material.reflective = 0.5;
        plane.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, -1.0, 0.0));
        world.shapes.push(plane);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -3.0),
            tuple::Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let intersection = intersection::intersection(2.0_f64.sqrt(), &world.shapes[2]);

        let computations = intersection::prepare_computations(&intersection, &ray);
        let color = world.reflected_color(&computations, 10);

        let expected_color = color::color(0.19033, 0.23791, 0.14274);
        assert_color_approx_eq!(color, expected_color);
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut world = world::default_world();
        let mut plane = shape::Shape::default_plane();
        plane.material.reflective = 0.5;
        plane.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, -1.0, 0.0));
        world.shapes.push(plane);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -3.0),
            tuple::Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let intersection = intersection::intersection(2.0_f64.sqrt(), &world.shapes[2]);

        let computations = intersection::prepare_computations(&intersection, &ray);
        let color = computations.shade_hit(&world, 10);

        let expected_color = color::color(0.87676, 0.92435, 0.82918);
        assert_color_approx_eq!(color, expected_color);
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        // Prevent two surfaces reflecting at one another from bouncing their rays back and fourth forever.
        let mut world = world::world();
        let light_position = tuple::Point::new(0.0, 0.0, -10.0);
        let light_color = color::color(1.0, 1.0, 1.0);
        world.light = Some(lights::point_light(light_position, light_color));
        let mut upper_plane = shape::Shape::default_plane();
        upper_plane.material.reflective = 1.0;
        upper_plane
            .set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, -1.0, 0.0));
        world.shapes.push(upper_plane);
        let mut lower_plane = shape::Shape::default_plane();
        lower_plane.material.reflective = 1.0;
        lower_plane.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0));
        world.shapes.push(lower_plane);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, 1.0, 0.0),
        );

        // Without an addtional contraint light will bounce between these two planes indefinitly.
        // Our assertion is really: does this call terminate.
        let color = world.color_at(&ray, 10);

        // We don't really care about the color here. See note above.
        let expected_color = color::color(1.0, 1.0, 1.0);
        assert_color_approx_eq!(color, expected_color);
    }

    #[test]
    fn reflected_color_at_the_maximum_recursive_depth() {
        let mut world = world::default_world();
        let mut plane = shape::Shape::default_plane();
        plane.material.reflective = 0.5;
        plane.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, -1.0, 0.0));
        world.shapes.push(plane);
        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -3.0),
            tuple::Vector::new(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let intersection = intersection::intersection(2.0_f64.sqrt(), &world.shapes[2]);

        let computations = intersection::prepare_computations(&intersection, &ray);
        // Note that there are zero remaing light bounces
        let color = world.reflected_color(&computations, 0);

        let expected_color = color::color(0.0, 0.0, 0.0);
        assert_color_approx_eq!(color, expected_color);
    }
}
