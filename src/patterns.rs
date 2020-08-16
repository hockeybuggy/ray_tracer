use crate::color;
use crate::matrix;
use crate::matrix::Inverse;
use crate::shape;
use crate::tuple;

#[derive(Debug, PartialEq)]
enum PatternType {
    Stripe,
    Gradient,
}

#[derive(Debug, PartialEq)]
pub struct Pattern {
    a: color::Color,
    b: color::Color,
    transform: matrix::Matrix4,
    pattern_type: PatternType,
}

impl Pattern {
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

    pub fn pattern_at(&self, point: &tuple::Point) -> color::Color {
        return match self.pattern_type {
            PatternType::Stripe => self.stripe_at(point),
            PatternType::Gradient => self.gradient_at(point),
        };
    }

    pub fn pattern_at_object(
        &self,
        object: &shape::Shape,
        world_point: &tuple::Point,
    ) -> color::Color {
        let object_point = object.transform.inverse().unwrap() * *world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.pattern_at(&pattern_point)
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
            pattern.pattern_at_object(&object, &tuple::Point::new(1.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = shape::Shape::default_sphere();
        let mut pattern = patterns::Pattern::stripe(color::white(), color::black());
        pattern.transform = pattern.transform.scaling(2.0, 2.0, 2.0);
        dbg!(pattern.transform);

        assert_color_approx_eq!(
            pattern.pattern_at_object(&object, &tuple::Point::new(1.5, 0.0, 0.0)),
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
            pattern.pattern_at_object(&object, &tuple::Point::new(2.5, 0.0, 0.0)),
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
}
