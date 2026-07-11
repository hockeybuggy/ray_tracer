use crate::color;
use crate::matrix;
use crate::matrix::Inverse;
use crate::tuple;
use crate::uv;

#[derive(Debug, PartialEq)]
enum PatternType {
    Stripe,
    Gradient,
    Ring,
    Checkers,
    TestPattern,
    TextureMap {
        uv_pattern: uv::UvPattern,
        uv_map: uv::UvMap,
    },
    CubeMap {
        faces: Box<uv::CubeFaces>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    a: color::Color,
    b: color::Color,
    transform: matrix::Matrix4,
    pattern_type: PatternType,
}

impl Pattern {
    pub fn test_pattern() -> Pattern {
        return Pattern {
            a: color::black(),
            b: color::black(),
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::TestPattern,
        };
    }

    pub fn stripe(a: color::Color, b: color::Color) -> Pattern {
        return Pattern {
            a,
            b,
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::Stripe,
        };
    }

    pub fn gradient(a: color::Color, b: color::Color) -> Pattern {
        return Pattern {
            a,
            b,
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::Gradient,
        };
    }

    pub fn ring(a: color::Color, b: color::Color) -> Pattern {
        return Pattern {
            a,
            b,
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::Ring,
        };
    }

    pub fn checkers(a: color::Color, b: color::Color) -> Pattern {
        return Pattern {
            a,
            b,
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::Checkers,
        };
    }

    pub fn texture_map(uv_pattern: uv::UvPattern, uv_map: uv::UvMap) -> Pattern {
        return Pattern {
            a: color::black(),
            b: color::black(),
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::TextureMap { uv_pattern, uv_map },
        };
    }

    pub fn cube_map(faces: uv::CubeFaces) -> Pattern {
        return Pattern {
            a: color::black(),
            b: color::black(),
            transform: matrix::Matrix4::IDENTITY,
            pattern_type: PatternType::CubeMap {
                faces: Box::new(faces),
            },
        };
    }

    pub fn stripe_at(&self, point: &tuple::Point) -> color::Color {
        return if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        };
    }

    pub fn gradient_at(&self, point: &tuple::Point) -> color::Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        return self.a + distance * fraction;
    }

    pub fn ring_at(&self, point: &tuple::Point) -> color::Color {
        return if ((point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() as i64) % 2 == 0 {
            self.a
        } else {
            self.b
        };
    }

    pub fn checkers_at(&self, point: &tuple::Point) -> color::Color {
        return if ((point.x.floor() + point.y.floor() + point.z.floor()) as i64) % 2 == 0 {
            self.a
        } else {
            self.b
        };
    }

    pub fn pattern_at(&self, point: &tuple::Point) -> color::Color {
        return match &self.pattern_type {
            PatternType::Stripe => self.stripe_at(point),
            PatternType::Gradient => self.gradient_at(point),
            PatternType::Ring => self.ring_at(point),
            PatternType::Checkers => self.checkers_at(point),
            PatternType::TestPattern => color::color(point.x, point.y, point.z),
            PatternType::TextureMap { uv_pattern, uv_map } => {
                let (u, v) = uv_map.map(point);
                uv_pattern.uv_pattern_at(u, v)
            }
            PatternType::CubeMap { faces } => {
                let (face_pattern, (u, v)) = match uv::face_from_point(point) {
                    uv::Face::Left => (&faces.left, uv::cube_uv_left(point)),
                    uv::Face::Right => (&faces.right, uv::cube_uv_right(point)),
                    uv::Face::Front => (&faces.front, uv::cube_uv_front(point)),
                    uv::Face::Back => (&faces.back, uv::cube_uv_back(point)),
                    uv::Face::Up => (&faces.up, uv::cube_uv_up(point)),
                    uv::Face::Down => (&faces.down, uv::cube_uv_down(point)),
                };
                face_pattern.uv_pattern_at(u, v)
            }
        };
    }

    // `object_to_world` is the object's transform with any enclosing group
    // transforms composed on, so patterns follow shapes into groups.
    pub fn pattern_at_object(
        &self,
        object_to_world: &matrix::Matrix4,
        world_point: &tuple::Point,
    ) -> color::Color {
        let object_point = object_to_world.inverse().unwrap() * *world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.pattern_at(&pattern_point)
    }

    pub fn set_transformation_matrix(&mut self, new_transform: matrix::Matrix4) {
        self.transform = new_transform;
    }
}

#[cfg(test)]
mod patterns_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::patterns;
    use crate::shape;
    use crate::transformation::Transform;
    use crate::tuple;
    use crate::uv;

    #[test]
    fn test_stripe_pattern_can_be_created() {
        let pattern = patterns::Pattern::stripe(color::white(), color::black());

        assert_color_approx_eq!(pattern.a, color::white());
        assert_color_approx_eq!(pattern.b, color::black());
    }

    #[test]
    fn test_pattern_at_is_constant_in_y() {
        let pattern = patterns::Pattern::stripe(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 1.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 2.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_pattern_at_is_constant_in_z() {
        let pattern = patterns::Pattern::stripe(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 1.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 2.0)),
            color::white()
        );
    }

    #[test]
    fn test_pattern_at_alternates_in_x() {
        let pattern = patterns::Pattern::stripe(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.9, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(-0.1, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(-1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(-1.1, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let pattern = patterns::Pattern::stripe(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(1.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = shape::Shape::default_sphere();
        let mut pattern = patterns::Pattern::stripe(color::white(), color::black());
        pattern.transform = pattern.transform.scaling(2.0, 2.0, 2.0);

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(1.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let mut pattern = patterns::Pattern::stripe(color::white(), color::black());
        pattern.transform = pattern.transform.translation(0.5, 0.0, 0.0);

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(2.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_a_gradient_linerly_interpolates_between_colors() {
        let pattern = patterns::Pattern::gradient(color::white(), color::black());
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.25, 0.0, 0.0)),
            color::color(0.75, 0.75, 0.75)
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.5, 0.0, 0.0)),
            color::color(0.5, 0.5, 0.5)
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.75, 0.0, 0.0)),
            color::color(0.25, 0.25, 0.25)
        );
    }

    #[test]
    fn test_a_ring_should_extend_in_both_x_and_z() {
        let pattern = patterns::Pattern::ring(color::white(), color::black());
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 1.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.708, 0.0, 0.708)),
            // 0.708 is more than 2.sqrt() / 2
            color::black()
        );
        // The radius is the euclidean distance sqrt(x^2 + z^2), so this
        // point is inside the first ring even though x + z > 1
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.6, 0.0, 0.6)),
            color::white()
        );
        // Rings extend to negative coordinates too
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(-1.5, 0.0, 0.0)),
            color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_x() {
        let pattern = patterns::Pattern::checkers(color::white(), color::black());
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.99, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(1.01, 0.0, 0.0)),
            color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_y() {
        let pattern = patterns::Pattern::checkers(color::white(), color::black());
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.99, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 1.01, 0.0)),
            color::black()
        );
    }

    #[test]
    fn test_checkers_should_repeat_in_z() {
        let pattern = patterns::Pattern::checkers(color::white(), color::black());
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 0.99)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.pattern_at(&tuple::Point::new(0.0, 0.0, 1.01)),
            color::black()
        );
    }

    #[test]
    fn test_a_pattern_with_object_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let pattern = patterns::Pattern::test_pattern();

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(2.0, 3.0, 4.0)),
            color::color(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn test_a_pattern_with_pattern_transformation() {
        let object = shape::Shape::default_sphere();
        let mut pattern = patterns::Pattern::test_pattern();
        pattern.transform = pattern.transform.scaling(2.0, 2.0, 2.0);

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(2.0, 3.0, 4.0)),
            color::color(1.0, 1.5, 2.0)
        );
    }

    #[test]
    fn test_a_pattern_with_both_object_and_pattern_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let mut pattern = patterns::Pattern::test_pattern();
        pattern.transform = pattern.transform.translation(0.5, 1.0, 1.5);

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object.transform, &tuple::Point::new(2.5, 3.0, 3.5)),
            color::color(0.75, 0.5, 0.25)
        );
    }

    // Scenario Outline: Using a texture map pattern with a spherical map
    #[test]
    fn test_texture_map_pattern_with_a_spherical_map() {
        let checkers = uv::UvPattern::checkers(16, 8, color::black(), color::white());
        let pattern = patterns::Pattern::texture_map(checkers, uv::UvMap::Spherical);

        let cases = [
            (tuple::Point::new(0.4315, 0.4670, 0.7719), color::white()),
            (tuple::Point::new(-0.9654, 0.2552, -0.0534), color::black()),
            (tuple::Point::new(0.1039, 0.7090, 0.6975), color::white()),
            (tuple::Point::new(-0.4986, -0.7856, -0.3663), color::black()),
            (tuple::Point::new(-0.0317, -0.9395, 0.3411), color::black()),
            (tuple::Point::new(0.4809, -0.7721, 0.4154), color::black()),
            (tuple::Point::new(0.0285, -0.9612, -0.2745), color::black()),
            (tuple::Point::new(-0.5734, -0.2162, -0.7903), color::white()),
            (tuple::Point::new(0.7688, -0.1470, 0.6223), color::black()),
            (tuple::Point::new(-0.7652, 0.2175, 0.6060), color::black()),
        ];
        for (point, expected) in cases {
            assert_color_approx_eq!(pattern.pattern_at(&point), expected);
        }
    }

    #[test]
    fn test_finding_the_colors_on_a_mapped_cube() {
        let red = color::color(1.0, 0.0, 0.0);
        let yellow = color::color(1.0, 1.0, 0.0);
        let brown = color::color(1.0, 0.5, 0.0);
        let green = color::color(0.0, 1.0, 0.0);
        let cyan = color::color(0.0, 1.0, 1.0);
        let blue = color::color(0.0, 0.0, 1.0);
        let purple = color::color(1.0, 0.0, 1.0);
        let white = color::color(1.0, 1.0, 1.0);

        let pattern = patterns::Pattern::cube_map(uv::CubeFaces {
            left: uv::UvPattern::align_check(yellow, cyan, red, blue, brown),
            front: uv::UvPattern::align_check(cyan, red, yellow, brown, green),
            right: uv::UvPattern::align_check(red, yellow, purple, green, white),
            back: uv::UvPattern::align_check(green, purple, cyan, white, blue),
            up: uv::UvPattern::align_check(brown, cyan, purple, red, yellow),
            down: uv::UvPattern::align_check(purple, brown, green, blue, white),
        });

        let cases = [
            // left face
            (tuple::Point::new(-1.0, 0.0, 0.0), yellow),
            (tuple::Point::new(-1.0, 0.9, -0.9), cyan),
            (tuple::Point::new(-1.0, 0.9, 0.9), red),
            (tuple::Point::new(-1.0, -0.9, -0.9), blue),
            (tuple::Point::new(-1.0, -0.9, 0.9), brown),
            // front face
            (tuple::Point::new(0.0, 0.0, 1.0), cyan),
            (tuple::Point::new(-0.9, 0.9, 1.0), red),
            (tuple::Point::new(0.9, 0.9, 1.0), yellow),
            (tuple::Point::new(-0.9, -0.9, 1.0), brown),
            (tuple::Point::new(0.9, -0.9, 1.0), green),
            // right face
            (tuple::Point::new(1.0, 0.0, 0.0), red),
            (tuple::Point::new(1.0, 0.9, 0.9), yellow),
            (tuple::Point::new(1.0, 0.9, -0.9), purple),
            (tuple::Point::new(1.0, -0.9, 0.9), green),
            (tuple::Point::new(1.0, -0.9, -0.9), white),
            // back face
            (tuple::Point::new(0.0, 0.0, -1.0), green),
            (tuple::Point::new(0.9, 0.9, -1.0), purple),
            (tuple::Point::new(-0.9, 0.9, -1.0), cyan),
            (tuple::Point::new(0.9, -0.9, -1.0), white),
            (tuple::Point::new(-0.9, -0.9, -1.0), blue),
            // up face
            (tuple::Point::new(0.0, 1.0, 0.0), brown),
            (tuple::Point::new(-0.9, 1.0, -0.9), cyan),
            (tuple::Point::new(0.9, 1.0, -0.9), purple),
            (tuple::Point::new(-0.9, 1.0, 0.9), red),
            (tuple::Point::new(0.9, 1.0, 0.9), yellow),
            // down face
            (tuple::Point::new(0.0, -1.0, 0.0), purple),
            (tuple::Point::new(-0.9, -1.0, 0.9), brown),
            (tuple::Point::new(0.9, -1.0, 0.9), green),
            (tuple::Point::new(-0.9, -1.0, -0.9), blue),
            (tuple::Point::new(0.9, -1.0, -0.9), white),
        ];
        for (point, expected) in cases {
            assert_color_approx_eq!(pattern.pattern_at(&point), expected);
        }
    }
}
