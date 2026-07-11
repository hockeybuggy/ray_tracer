use crate::color;
use crate::tuple;

#[derive(Debug, PartialEq)]
pub enum UvPattern {
    Checkers {
        width: u32,
        height: u32,
        a: color::Color,
        b: color::Color,
    },
}

impl UvPattern {
    pub fn checkers(width: u32, height: u32, a: color::Color, b: color::Color) -> UvPattern {
        return UvPattern::Checkers {
            width,
            height,
            a,
            b,
        };
    }

    pub fn uv_pattern_at(&self, u: f64, v: f64) -> color::Color {
        match self {
            UvPattern::Checkers {
                width,
                height,
                a,
                b,
            } => {
                let u2 = (u * *width as f64).floor() as i64;
                let v2 = (v * *height as f64).floor() as i64;
                if (u2 + v2) % 2 == 0 { *a } else { *b }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UvMap {
    Spherical,
}

impl UvMap {
    pub fn map(&self, point: &tuple::Point) -> (f64, f64) {
        match self {
            UvMap::Spherical => spherical_map(point),
        }
    }
}

/// Unwraps a point on a unit sphere (centered at the origin) onto a 2D
/// (u, v) square: u increases counter-clockwise around the equator as
/// seen from above, v goes 0 at the south pole to 1 at the north pole.
pub fn spherical_map(point: &tuple::Point) -> (f64, f64) {
    // Azimuthal angle: -π < theta <= π, increasing clockwise from above.
    let theta = point.x.atan2(point.z);

    // The point's distance from the origin is the sphere's radius.
    let radius = (point.x * point.x + point.y * point.y + point.z * point.z).sqrt();

    // Polar angle: 0 <= phi <= π, from the north pole down.
    let phi = (point.y / radius).acos();

    // -0.5 < raw_u <= 0.5; flip so u increases counter-clockwise.
    let raw_u = theta / (2.0 * std::f64::consts::PI);
    let u = 1.0 - (raw_u + 0.5);

    // Flip phi so v is 0 at the south pole and 1 at the north pole.
    let v = 1.0 - phi / std::f64::consts::PI;

    return (u, v);
}

#[cfg(test)]
mod uv_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::tuple;
    use crate::uv;

    // Scenario Outline: Checker pattern in 2D
    #[test]
    fn test_checker_pattern_in_2d() {
        let checkers = uv::UvPattern::checkers(2, 2, color::black(), color::white());

        let cases = [
            (0.0, 0.0, color::black()),
            (0.5, 0.0, color::white()),
            (0.0, 0.5, color::white()),
            (0.5, 0.5, color::black()),
            (1.0, 1.0, color::black()),
        ];
        for (u, v, expected) in cases {
            assert_color_approx_eq!(checkers.uv_pattern_at(u, v), expected);
        }
    }

    // Scenario Outline: Using a spherical mapping on a 3D point
    #[test]
    fn test_spherical_mapping_on_a_3d_point() {
        let sqrt2_over_2 = 2.0_f64.sqrt() / 2.0;
        let cases = [
            (tuple::Point::new(0.0, 0.0, -1.0), 0.0, 0.5),
            (tuple::Point::new(1.0, 0.0, 0.0), 0.25, 0.5),
            (tuple::Point::new(0.0, 0.0, 1.0), 0.5, 0.5),
            (tuple::Point::new(-1.0, 0.0, 0.0), 0.75, 0.5),
            (tuple::Point::new(0.0, 1.0, 0.0), 0.5, 1.0),
            (tuple::Point::new(0.0, -1.0, 0.0), 0.5, 0.0),
            (
                tuple::Point::new(sqrt2_over_2, sqrt2_over_2, 0.0),
                0.25,
                0.75,
            ),
        ];
        for (point, expected_u, expected_v) in cases {
            let (u, v) = uv::spherical_map(&point);
            assert!(
                (u - expected_u).abs() < 1e-5,
                "u for {:?}: {} != {}",
                point,
                u,
                expected_u
            );
            assert!(
                (v - expected_v).abs() < 1e-5,
                "v for {:?}: {} != {}",
                point,
                v,
                expected_v
            );
        }
    }
}
