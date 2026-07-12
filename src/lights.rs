use crate::color;
use crate::sequences;
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
        jitter: sequences::Sequence,
    },
    Spot {
        /// Normalized direction the cone points, away from `position`.
        direction: tuple::Vector,
        /// Half-angle (radians) of the inner cone of full intensity.
        cone_angle: f64,
        /// Half-angle (radians) beyond which the light contributes nothing.
        fade_angle: f64,
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
            LightKind::Point | LightKind::Spot { .. } => 1,
            LightKind::Area { usteps, vsteps, .. } => usteps * vsteps,
        }
    }

    pub fn set_jitter(&mut self, jitter_by: sequences::Sequence) {
        if let LightKind::Area { jitter, .. } = &mut self.kind {
            *jitter = jitter_by;
        }
    }

    /// The fraction of this light's intensity that reaches `point`,
    /// ignoring shadowing: only a spotlight's cone attenuates it.
    pub fn attenuation_at(&self, point: &tuple::Point) -> f64 {
        match &self.kind {
            LightKind::Point | LightKind::Area { .. } => 1.0,
            LightKind::Spot {
                direction,
                cone_angle,
                fade_angle,
            } => {
                let to_point = tuple::normalize(&(*point - self.position));
                // Rounding can push the dot product of two unit vectors
                // just past ±1, where `acos` returns NaN.
                let angle = tuple::dot(&to_point, direction).clamp(-1.0, 1.0).acos();
                if angle <= *cone_angle {
                    1.0
                } else if angle >= *fade_angle {
                    0.0
                } else {
                    (fade_angle - angle) / (fade_angle - cone_angle)
                }
            }
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
            jitter: sequences::Sequence::constant(0.5),
        },
    }
}

/// A point light restricted to a cone: full intensity within `cone_angle`
/// of `direction`, fading linearly to nothing at `fade_angle`.
pub fn spot_light(
    position: tuple::Point,
    direction: tuple::Vector,
    cone_angle: f64,
    fade_angle: f64,
    intensity: color::Color,
) -> Light {
    Light {
        position,
        intensity,
        kind: LightKind::Spot {
            direction: tuple::normalize(&direction),
            cone_angle,
            fade_angle,
        },
    }
}

pub fn point_on_light(light: &Light, u: usize, v: usize) -> tuple::Point {
    match &light.kind {
        LightKind::Point | LightKind::Spot { .. } => light.position,
        LightKind::Area {
            corner,
            uvec,
            vvec,
            jitter,
            ..
        } => *corner + *uvec * (u as f64 + jitter.next()) + *vvec * (v as f64 + jitter.next()),
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
        LightKind::Spot { .. } => {
            if world::is_shadowed(world, &light.position, point) {
                0.0
            } else {
                light.attenuation_at(point)
            }
        }
    }
}

#[cfg(test)]
mod lights_tests {
    use std::f64::consts::PI;

    use crate::assert_tuple_approx_eq;
    use crate::color;
    use crate::lights;
    use crate::sequences;
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
                ..
            } => {
                assert_eq!(corner, tuple::Point::new(0.0, 0.0, 0.0));
                assert_eq!(uvec, tuple::Vector::new(0.5, 0.0, 0.0));
                assert_eq!(usteps, 4);
                assert_eq!(vvec, tuple::Vector::new(0.0, 0.0, 0.5));
                assert_eq!(vsteps, 2);
            }
            _ => panic!("expected an area light"),
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

    #[test]
    fn test_finding_a_single_point_on_a_jittered_area_light() {
        let corner = tuple::Point::new(0.0, 0.0, 0.0);
        let v1 = tuple::Vector::new(2.0, 0.0, 0.0);
        let v2 = tuple::Vector::new(0.0, 0.0, 1.0);
        let mut light = lights::area_light(corner, v1, 4, v2, 2, color::white());
        light.set_jitter(sequences::sequence(&[0.3, 0.7]));

        let cases = [
            (0, 0, tuple::Point::new(0.15, 0.0, 0.35)),
            (1, 0, tuple::Point::new(0.65, 0.0, 0.35)),
            (0, 1, tuple::Point::new(0.15, 0.0, 0.85)),
            (2, 0, tuple::Point::new(1.15, 0.0, 0.35)),
            (3, 1, tuple::Point::new(1.65, 0.0, 0.85)),
        ];

        for (u, v, expected) in cases {
            assert_tuple_approx_eq!(lights::point_on_light(&light, u, v), expected);
        }
    }

    #[test]
    fn test_the_area_light_with_jittered_samples() {
        let w = world::default_world();
        let corner = tuple::Point::new(-0.5, -0.5, -5.0);
        let v1 = tuple::Vector::new(1.0, 0.0, 0.0);
        let v2 = tuple::Vector::new(0.0, 1.0, 0.0);

        let cases = [
            (tuple::Point::new(0.0, 0.0, 2.0), 0.0),
            (tuple::Point::new(1.0, -1.0, 2.0), 0.5),
            (tuple::Point::new(1.5, 0.0, 2.0), 0.75),
            (tuple::Point::new(1.25, 1.25, 3.0), 0.75),
            (tuple::Point::new(0.0, 0.0, -2.0), 1.0),
        ];

        // Each row gets a fresh light: intensity_at consumes 4 jitter values
        // per call, so a shared cursor would drift across rows.
        for (point, expected) in cases {
            let mut light = lights::area_light(corner, v1, 2, v2, 2, color::white());
            light.set_jitter(sequences::sequence(&[0.7, 0.3, 0.9, 0.1, 0.5]));

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

    #[test]
    fn test_spot_light_has_position_and_intensity() {
        let intensity = color::color(1.0, 1.0, 1.0);
        let position = tuple::Point::new(0.0, 1.0, 0.0);

        let light = lights::spot_light(
            position,
            tuple::Vector::new(0.0, -1.0, 0.0),
            PI / 6.0,
            PI / 4.0,
            intensity,
        );

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
        assert_eq!(light.samples(), 1);
    }

    #[test]
    fn test_a_point_light_reaches_everywhere_at_full_intensity() {
        let light = lights::point_light(tuple::Point::new(0.0, 0.0, 0.0), color::white());

        let attenuation = light.attenuation_at(&tuple::Point::new(3.0, -2.0, 7.0));

        assert_eq!(attenuation, 1.0);
    }

    // A spotlight one unit up, shining straight down with a 30 degree
    // full-intensity cone that fades out at 45 degrees.
    fn downward_spot_light() -> lights::Light {
        lights::spot_light(
            tuple::Point::new(0.0, 1.0, 0.0),
            tuple::Vector::new(0.0, -1.0, 0.0),
            PI / 6.0,
            PI / 4.0,
            color::white(),
        )
    }

    #[test]
    fn test_a_spot_light_is_full_intensity_inside_its_cone() {
        let light = downward_spot_light();

        // Directly below the light, and slightly off-axis (about 14
        // degrees), both inside the 30 degree cone.
        assert_eq!(light.attenuation_at(&tuple::Point::new(0.0, 0.0, 0.0)), 1.0);
        assert_eq!(
            light.attenuation_at(&tuple::Point::new(0.25, 0.0, 0.0)),
            1.0
        );
    }

    #[test]
    fn test_a_spot_light_is_dark_outside_its_fade_angle() {
        let light = downward_spot_light();

        // About 63 degrees off-axis, past the 45 degree fade angle.
        assert_eq!(light.attenuation_at(&tuple::Point::new(2.0, 0.0, 0.0)), 0.0);
        // Directly behind the light.
        assert_eq!(light.attenuation_at(&tuple::Point::new(0.0, 2.0, 0.0)), 0.0);
    }

    #[test]
    fn test_a_spot_light_fades_between_its_cone_and_fade_angles() {
        // A 30 degree cone fading out at 60 degrees, so a point 45 degrees
        // off-axis is halfway through the fade band.
        let light = lights::spot_light(
            tuple::Point::new(0.0, 0.0, 0.0),
            tuple::Vector::new(0.0, -1.0, 0.0),
            PI / 6.0,
            PI / 3.0,
            color::white(),
        );

        let attenuation = light.attenuation_at(&tuple::Point::new(1.0, -1.0, 0.0));

        assert!((attenuation - 0.5).abs() < 1e-5, "{}", attenuation);
    }

    #[test]
    fn test_spot_lights_evaluate_intensity_at_a_given_point() {
        // The default world's light, restricted to a 30 degree cone aimed
        // at the origin that fades out at 45 degrees.
        let w = world::default_world();
        let light = lights::spot_light(
            tuple::Point::new(-10.0, 10.0, -10.0),
            tuple::Vector::new(10.0, -10.0, 10.0),
            PI / 6.0,
            PI / 4.0,
            color::white(),
        );

        let cases = [
            // Unshadowed and nearly on-axis.
            (tuple::Point::new(0.0, 0.0, -1.0001), 1.0),
            // Unshadowed but far outside the fade angle.
            (tuple::Point::new(0.0, 15.0, 0.0), 0.0),
            // On-axis but shadowed by the sphere at the origin.
            (tuple::Point::new(0.0, 0.0, 1.0001), 0.0),
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
