use crate::color;
use crate::tuple;
use crate::world;

pub enum LightKind {
    Point,
    Area {
        corner: tuple::Point,
        uvec: tuple::Vector,
        vvec: tuple::Vector,
        usteps: usize,
        vsteps: usize,
    },
}

pub struct Light {
    pub position: tuple::Point,
    pub intensity: color::Color,
    pub kind: LightKind,
}

impl Light {
    pub fn samples(&self) -> usize {
        match self.kind {
            LightKind::Point => 1,
            LightKind::Area { usteps, vsteps, .. } => usteps * vsteps,
        }
    }
}

pub fn point_light(position: tuple::Point, intensity: color::Color) -> Light {
    Light {
        position,
        intensity,
        kind: LightKind::Point,
    }
}

pub fn area_light(
    corner: tuple::Point,
    full_uvec: tuple::Vector,
    usteps: usize,
    full_vvec: tuple::Vector,
    vsteps: usize,
    intensity: color::Color,
) -> Light {
    let uvec = full_uvec / usteps as f64;
    let vvec = full_vvec / vsteps as f64;
    let position = corner + full_uvec * 0.5 + full_vvec * 0.5;
    Light {
        position,
        intensity,
        kind: LightKind::Area {
            corner,
            uvec,
            vvec,
            usteps,
            vsteps,
        },
    }
}

pub fn point_on_light(light: &Light, u: usize, v: usize) -> tuple::Point {
    match light.kind {
        LightKind::Point => light.position,
        LightKind::Area {
            corner, uvec, vvec, ..
        } => corner + uvec * (u as f64 + 0.5) + vvec * (v as f64 + 0.5),
    }
}

pub fn intensity_at(light: &Light, point: &tuple::Point, world: &world::World) -> f64 {
    match light.kind {
        LightKind::Point => {
            if world::is_shadowed(world, &light.position, point) {
                0.0
            } else {
                1.0
            }
        }
        LightKind::Area { usteps, vsteps, .. } => {
            let mut total = 0.0;
            for v in 0..vsteps {
                for u in 0..usteps {
                    let light_position = point_on_light(light, u, v);
                    if !world::is_shadowed(world, &light_position, point) {
                        total += 1.0;
                    }
                }
            }
            total / light.samples() as f64
        }
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

    #[test]
    fn test_creating_an_area_light() {
        let corner = tuple::Point::new(0.0, 0.0, 0.0);
        let v1 = tuple::Vector::new(2.0, 0.0, 0.0);
        let v2 = tuple::Vector::new(0.0, 0.0, 1.0);

        let light = lights::area_light(corner, v1, 4, v2, 2, color::white());

        match light.kind {
            lights::LightKind::Area {
                corner,
                uvec,
                vvec,
                usteps,
                vsteps,
            } => {
                assert_eq!(corner, tuple::Point::new(0.0, 0.0, 0.0));
                assert_eq!(uvec, tuple::Vector::new(0.5, 0.0, 0.0));
                assert_eq!(usteps, 4);
                assert_eq!(vvec, tuple::Vector::new(0.0, 0.0, 0.5));
                assert_eq!(vsteps, 2);
            }
            lights::LightKind::Point => panic!("expected an area light"),
        }
        assert_eq!(light.samples(), 8);
        assert_eq!(light.position, tuple::Point::new(1.0, 0.0, 0.5));
    }

    #[test]
    fn test_finding_a_single_point_on_an_area_light() {
        let corner = tuple::Point::new(0.0, 0.0, 0.0);
        let v1 = tuple::Vector::new(2.0, 0.0, 0.0);
        let v2 = tuple::Vector::new(0.0, 0.0, 1.0);
        let light = lights::area_light(corner, v1, 4, v2, 2, color::white());

        let cases = [
            (0, 0, tuple::Point::new(0.25, 0.0, 0.25)),
            (1, 0, tuple::Point::new(0.75, 0.0, 0.25)),
            (0, 1, tuple::Point::new(0.25, 0.0, 0.75)),
            (2, 0, tuple::Point::new(1.25, 0.0, 0.25)),
            (3, 1, tuple::Point::new(1.75, 0.0, 0.75)),
        ];

        for (u, v, expected) in cases {
            assert_eq!(lights::point_on_light(&light, u, v), expected);
        }
    }

    #[test]
    fn test_the_area_light_intensity_function() {
        let w = world::default_world();
        let corner = tuple::Point::new(-0.5, -0.5, -5.0);
        let v1 = tuple::Vector::new(1.0, 0.0, 0.0);
        let v2 = tuple::Vector::new(0.0, 1.0, 0.0);
        let light = lights::area_light(corner, v1, 2, v2, 2, color::white());

        let cases = [
            (tuple::Point::new(0.0, 0.0, 2.0), 0.0),
            (tuple::Point::new(1.0, -1.0, 2.0), 0.25),
            (tuple::Point::new(1.5, 0.0, 2.0), 0.5),
            (tuple::Point::new(1.25, 1.25, 3.0), 0.75),
            (tuple::Point::new(0.0, 0.0, -2.0), 1.0),
        ];

        for (point, expected) in cases {
            let got = lights::intensity_at(&light, &point, &w);
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
