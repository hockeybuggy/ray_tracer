use crate::color;
use crate::lights;
use crate::matrix;
use crate::sphere;
use crate::transformation::Transform;
use crate::tuple;

pub struct World {
    pub light: Option<lights::Light>,
    pub shapes: Vec<sphere::Sphere>,
}

fn world() -> World {
    World {
        light: None,
        shapes: Vec::new(),
    }
}

pub fn default_world() -> World {
    let white_point_light = lights::point_light(tuple::point(-10.0, 10.0, -10.0), color::white());
    let mut lime_sphere = sphere::sphere();
    lime_sphere.material.color = color::color(0.8, 1.0, 0.6);
    lime_sphere.material.diffuse = 0.7;
    lime_sphere.material.specular = 0.2;
    let mut small_sphere = sphere::sphere();
    small_sphere.transform = matrix::Matrix4::IDENTITY.scaling(0.5, 0.5, 0.5);
    let shapes = vec![lime_sphere, small_sphere];
    World {
        light: Some(white_point_light),
        shapes: shapes,
    }
}

#[cfg(test)]
mod world_tests {
    use crate::color;
    use crate::lights;
    use crate::matrix;
    use crate::ray;
    use crate::sphere;
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
        let expected_light = lights::point_light(tuple::point(-10.0, 10.0, -10.0), color::white());
        assert_eq!(world.light.is_some(), true);
        let world_light = world.light.unwrap();
        assert_eq!(world_light.position, expected_light.position);
        assert_eq!(world_light.intensity, expected_light.intensity);
        // There are two spheres
        assert_eq!(world.shapes.len(), 2);
        // One is a different color
        let mut expected_s1 = sphere::sphere();
        expected_s1.material.color = color::color(0.8, 1.0, 0.6);
        expected_s1.material.diffuse = 0.7;
        expected_s1.material.specular = 0.2;
        let first_shape = &world.shapes[0];
        assert_eq!(first_shape.material.color, expected_s1.material.color);
        // One is a different size
        let mut expected_s2 = sphere::sphere();
        expected_s2.transform = matrix::Matrix4::IDENTITY.scaling(0.5, 0.5, 0.5);
        let second_shape = &world.shapes[1];
        assert_eq!(second_shape.transform, expected_s2.transform);
    }

    #[test]
    fn default_world_intersected_with_a_ray() {
        let world = world::default_world();
        let ray = ray::ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));

        let intersections = ray.intersect_world(&world);

        assert_eq!(intersections.len(), 4);
        // Note that these are sorted by `.t`
        assert_eq!(intersections[0].t, 4.0_f64);
        assert_eq!(intersections[1].t, 4.5_f64);
        assert_eq!(intersections[2].t, 5.5_f64);
        assert_eq!(intersections[3].t, 6.0_f64);
    }
}
