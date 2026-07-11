use crate::color;
use crate::tuple;
use crate::world;

pub struct Light {
    pub position: tuple::Point,
    pub intensity: color::Color,
}

pub fn point_light(position: tuple::Point, intensity: color::Color) -> Light {
    Light {
        position,
        intensity,
    }
}

pub fn intensity_at(light: &Light, point: &tuple::Point, world: &world::World) -> f64 {
    if world::is_shadowed(world, &light.position, point) {
        0.0
    } else {
        1.0
    }
}

#[cfg(test)]
mod lights_tests {
    use crate::color;
    use crate::lights;
    use crate::tuple;
    use crate::world;

    #[test]
    fn test_point_light_has_position_an_instensity() {
        let intensity = color::color(1.0, 1.0, 1.0);
        let position = tuple::Point::new(0.0, 0.0, 0.0);

        let light = lights::point_light(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }

    #[test]
    fn test_point_lights_evaluate_intensity_at_a_given_point() {
        let w = world::default_world();
        let light = &w.lights[0];

        let cases = [
            (tuple::Point::new(0.0, 1.0001, 0.0), 1.0),
            (tuple::Point::new(-1.0001, 0.0, 0.0), 1.0),
            (tuple::Point::new(0.0, 0.0, -1.0001), 1.0),
            (tuple::Point::new(0.0, 0.0, 1.0001), 0.0),
            (tuple::Point::new(1.0001, 0.0, 0.0), 0.0),
            (tuple::Point::new(0.0, -1.0001, 0.0), 0.0),
            (tuple::Point::new(0.0, 0.0, 0.0), 0.0),
        ];

        for (point, expected) in cases {
            let got = lights::intensity_at(light, &point, &w);
            assert!(
                (got - expected).abs() < 1e-5,
                "intensity_at({:?}) = {}, want {}",
                point,
                got,
                expected
            );
        }
    }
}
