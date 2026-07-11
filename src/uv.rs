use crate::canvas;
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
    AlignCheck {
        main: color::Color,
        ul: color::Color,
        ur: color::Color,
        bl: color::Color,
        br: color::Color,
    },
    Image {
        canvas: canvas::Canvas,
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

    pub fn align_check(
        main: color::Color,
        ul: color::Color,
        ur: color::Color,
        bl: color::Color,
        br: color::Color,
    ) -> UvPattern {
        return UvPattern::AlignCheck {
            main,
            ul,
            ur,
            bl,
            br,
        };
    }

    pub fn image(canvas: canvas::Canvas) -> UvPattern {
        return UvPattern::Image { canvas };
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
            UvPattern::AlignCheck {
                main,
                ul,
                ur,
                bl,
                br,
            } => {
                // v=0 is the bottom of the square, v=1 is the top.
                if v > 0.8 {
                    if u < 0.2 {
                        return *ul;
                    }
                    if u > 0.8 {
                        return *ur;
                    }
                } else if v < 0.2 {
                    if u < 0.2 {
                        return *bl;
                    }
                    if u > 0.8 {
                        return *br;
                    }
                }
                *main
            }
            UvPattern::Image { canvas } => {
                // Flip v: the pattern's v=0 is the bottom, the canvas's
                // y=0 is the top.
                let v = 1.0 - v;

                // Scale by (dimension - 1) so u=1 / v=1 land on the last
                // pixel instead of one past it.
                let x = u * (canvas.width - 1) as f64;
                let y = v * (canvas.height - 1) as f64;

                *canvas.pixel_at(x.round() as u32, y.round() as u32)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum UvMap {
    Spherical,
    Planar,
    Cylindrical,
}

impl UvMap {
    pub fn map(&self, point: &tuple::Point) -> (f64, f64) {
        match self {
            UvMap::Spherical => spherical_map(point),
            UvMap::Planar => planar_map(point),
            UvMap::Cylindrical => cylindrical_map(point),
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

/// Tiles the x/z plane with unit squares; y is ignored.
pub fn planar_map(point: &tuple::Point) -> (f64, f64) {
    let u = point.x.rem_euclid(1.0);
    let v = point.z.rem_euclid(1.0);

    return (u, v);
}

/// Wraps the pattern around a unit-radius cylinder like a soup-can
/// label; the pattern repeats every whole unit of y.
pub fn cylindrical_map(point: &tuple::Point) -> (f64, f64) {
    let theta = point.x.atan2(point.z);
    let raw_u = theta / (2.0 * std::f64::consts::PI);
    let u = 1.0 - (raw_u + 0.5);

    let v = point.y.rem_euclid(1.0);

    return (u, v);
}

#[derive(Debug, PartialEq)]
pub enum Face {
    Left,
    Right,
    Front,
    Back,
    Up,
    Down,
}

/// The face of the unit cube (corners at (-1,-1,-1) and (1,1,1)) a point
/// lies on: whichever coordinate has the largest absolute value.
pub fn face_from_point(point: &tuple::Point) -> Face {
    let abs_x = point.x.abs();
    let abs_y = point.y.abs();
    let abs_z = point.z.abs();
    let coord = abs_x.max(abs_y).max(abs_z);

    if coord == point.x {
        return Face::Right;
    }
    if coord == -point.x {
        return Face::Left;
    }
    if coord == point.y {
        return Face::Up;
    }
    if coord == -point.y {
        return Face::Down;
    }
    if coord == point.z {
        return Face::Front;
    }
    return Face::Back;
}

// Each face maps its two in-plane coordinates onto (u, v) so that
// adjacent faces share edges without seams. `rem_euclid(2.0)` keeps
// points outside [-1, 1] tiling instead of going negative.

pub fn cube_uv_front(point: &tuple::Point) -> (f64, f64) {
    let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
    let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;
    return (u, v);
}

pub fn cube_uv_back(point: &tuple::Point) -> (f64, f64) {
    let u = (1.0 - point.x).rem_euclid(2.0) / 2.0;
    let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;
    return (u, v);
}

pub fn cube_uv_left(point: &tuple::Point) -> (f64, f64) {
    let u = (point.z + 1.0).rem_euclid(2.0) / 2.0;
    let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;
    return (u, v);
}

pub fn cube_uv_right(point: &tuple::Point) -> (f64, f64) {
    let u = (1.0 - point.z).rem_euclid(2.0) / 2.0;
    let v = (point.y + 1.0).rem_euclid(2.0) / 2.0;
    return (u, v);
}

pub fn cube_uv_up(point: &tuple::Point) -> (f64, f64) {
    let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
    let v = (1.0 - point.z).rem_euclid(2.0) / 2.0;
    return (u, v);
}

pub fn cube_uv_down(point: &tuple::Point) -> (f64, f64) {
    let u = (point.x + 1.0).rem_euclid(2.0) / 2.0;
    let v = (point.z + 1.0).rem_euclid(2.0) / 2.0;
    return (u, v);
}

#[derive(Debug, PartialEq)]
pub struct CubeFaces {
    pub left: UvPattern,
    pub right: UvPattern,
    pub front: UvPattern,
    pub back: UvPattern,
    pub up: UvPattern,
    pub down: UvPattern,
}

#[cfg(test)]
mod uv_tests {
    use crate::assert_color_approx_eq;
    use crate::canvas;
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

    // Scenario Outline: Using a planar mapping on a 3D point
    #[test]
    fn test_planar_mapping_on_a_3d_point() {
        let cases = [
            (tuple::Point::new(0.25, 0.0, 0.5), 0.25, 0.5),
            (tuple::Point::new(0.25, 0.0, -0.25), 0.25, 0.75),
            (tuple::Point::new(0.25, 0.5, -0.25), 0.25, 0.75),
            (tuple::Point::new(1.25, 0.0, 0.5), 0.25, 0.5),
            (tuple::Point::new(0.25, 0.0, -1.75), 0.25, 0.25),
            (tuple::Point::new(1.0, 0.0, -1.0), 0.0, 0.0),
            (tuple::Point::new(0.0, 0.0, 0.0), 0.0, 0.0),
        ];
        for (point, expected_u, expected_v) in cases {
            let (u, v) = uv::planar_map(&point);
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

    // Scenario Outline: Using a cylindrical mapping on a 3D point
    #[test]
    fn test_cylindrical_mapping_on_a_3d_point() {
        let cases = [
            (tuple::Point::new(0.0, 0.0, -1.0), 0.0, 0.0),
            (tuple::Point::new(0.0, 0.5, -1.0), 0.0, 0.5),
            (tuple::Point::new(0.0, 1.0, -1.0), 0.0, 0.0),
            (tuple::Point::new(0.70711, 0.5, -0.70711), 0.125, 0.5),
            (tuple::Point::new(1.0, 0.5, 0.0), 0.25, 0.5),
            (tuple::Point::new(0.70711, 0.5, 0.70711), 0.375, 0.5),
            (tuple::Point::new(0.0, -0.25, 1.0), 0.5, 0.75),
            (tuple::Point::new(-0.70711, 0.5, 0.70711), 0.625, 0.5),
            (tuple::Point::new(-1.0, 1.25, 0.0), 0.75, 0.25),
            (tuple::Point::new(-0.70711, 0.5, -0.70711), 0.875, 0.5),
        ];
        for (point, expected_u, expected_v) in cases {
            let (u, v) = uv::cylindrical_map(&point);
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

    // Scenario Outline: Layout of the "align check" pattern
    #[test]
    fn test_layout_of_the_align_check_pattern() {
        let main = color::color(1.0, 1.0, 1.0);
        let ul = color::color(1.0, 0.0, 0.0);
        let ur = color::color(1.0, 1.0, 0.0);
        let bl = color::color(0.0, 1.0, 0.0);
        let br = color::color(0.0, 1.0, 1.0);
        let pattern = uv::UvPattern::align_check(main, ul, ur, bl, br);

        let cases = [
            (0.5, 0.5, main),
            (0.1, 0.9, ul),
            (0.9, 0.9, ur),
            (0.1, 0.1, bl),
            (0.9, 0.1, br),
        ];
        for (u, v, expected) in cases {
            assert_color_approx_eq!(pattern.uv_pattern_at(u, v), expected);
        }
    }

    // Scenario Outline: Identifying the face of a cube from a point
    #[test]
    fn test_identifying_the_face_of_a_cube_from_a_point() {
        let cases = [
            (tuple::Point::new(-1.0, 0.5, -0.25), uv::Face::Left),
            (tuple::Point::new(1.1, -0.75, 0.8), uv::Face::Right),
            (tuple::Point::new(0.1, 0.6, 0.9), uv::Face::Front),
            (tuple::Point::new(-0.7, 0.0, -2.0), uv::Face::Back),
            (tuple::Point::new(0.5, 1.0, 0.9), uv::Face::Up),
            (tuple::Point::new(-0.2, -1.3, 1.1), uv::Face::Down),
        ];
        for (point, expected) in cases {
            assert_eq!(
                uv::face_from_point(&point),
                expected,
                "face for {:?}",
                point
            );
        }
    }

    // Scenario Outlines: UV mapping each face of a cube
    #[test]
    fn test_uv_mapping_the_faces_of_a_cube() {
        type CubeUvFn = fn(&tuple::Point) -> (f64, f64);
        let cases: [(&str, CubeUvFn, tuple::Point, f64, f64); 12] = [
            (
                "front",
                uv::cube_uv_front,
                tuple::Point::new(-0.5, 0.5, 1.0),
                0.25,
                0.75,
            ),
            (
                "front",
                uv::cube_uv_front,
                tuple::Point::new(0.5, -0.5, 1.0),
                0.75,
                0.25,
            ),
            (
                "back",
                uv::cube_uv_back,
                tuple::Point::new(0.5, 0.5, -1.0),
                0.25,
                0.75,
            ),
            (
                "back",
                uv::cube_uv_back,
                tuple::Point::new(-0.5, -0.5, -1.0),
                0.75,
                0.25,
            ),
            (
                "left",
                uv::cube_uv_left,
                tuple::Point::new(-1.0, 0.5, -0.5),
                0.25,
                0.75,
            ),
            (
                "left",
                uv::cube_uv_left,
                tuple::Point::new(-1.0, -0.5, 0.5),
                0.75,
                0.25,
            ),
            (
                "right",
                uv::cube_uv_right,
                tuple::Point::new(1.0, 0.5, 0.5),
                0.25,
                0.75,
            ),
            (
                "right",
                uv::cube_uv_right,
                tuple::Point::new(1.0, -0.5, -0.5),
                0.75,
                0.25,
            ),
            (
                "up",
                uv::cube_uv_up,
                tuple::Point::new(-0.5, 1.0, -0.5),
                0.25,
                0.75,
            ),
            (
                "up",
                uv::cube_uv_up,
                tuple::Point::new(0.5, 1.0, 0.5),
                0.75,
                0.25,
            ),
            (
                "down",
                uv::cube_uv_down,
                tuple::Point::new(-0.5, -1.0, 0.5),
                0.25,
                0.75,
            ),
            (
                "down",
                uv::cube_uv_down,
                tuple::Point::new(0.5, -1.0, -0.5),
                0.75,
                0.25,
            ),
        ];
        for (face, uv_fn, point, expected_u, expected_v) in cases {
            let (u, v) = uv_fn(&point);
            assert!(
                (u - expected_u).abs() < 1e-5,
                "u on {} for {:?}: {} != {}",
                face,
                point,
                u,
                expected_u
            );
            assert!(
                (v - expected_v).abs() < 1e-5,
                "v on {} for {:?}: {} != {}",
                face,
                point,
                v,
                expected_v
            );
        }
    }

    // Scenario Outline: Checker pattern in 2D (image-based)
    #[test]
    fn test_uv_pattern_from_an_image() {
        let ppm = "P3\n\
                   10 10\n\
                   10\n\
                   0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9\n\
                   1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0\n\
                   2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1\n\
                   3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2\n\
                   4 4 4  5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3\n\
                   5 5 5  6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4\n\
                   6 6 6  7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5\n\
                   7 7 7  8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6\n\
                   8 8 8  9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7\n\
                   9 9 9  0 0 0  1 1 1  2 2 2  3 3 3  4 4 4  5 5 5  6 6 6  7 7 7  8 8 8\n";
        let canvas = canvas::canvas_from_ppm(ppm).unwrap();
        let pattern = uv::UvPattern::image(canvas);

        let cases = [
            (0.0, 0.0, color::color(0.9, 0.9, 0.9)),
            (0.3, 0.0, color::color(0.2, 0.2, 0.2)),
            (0.6, 0.3, color::color(0.1, 0.1, 0.1)),
            (1.0, 1.0, color::color(0.9, 0.9, 0.9)),
        ];
        for (u, v, expected) in cases {
            assert_color_approx_eq!(pattern.uv_pattern_at(u, v), expected);
        }
    }
}
