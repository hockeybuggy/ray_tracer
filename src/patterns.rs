use crate::color;
use crate::matrix;
use crate::matrix::Inverse;
use crate::shape;
use crate::tuple;

#[derive(Debug, PartialEq)]
pub struct StripePattern {
    a: color::Color,
    b: color::Color,
    transform: matrix::Matrix4,
}

impl StripePattern {
    pub fn new(a: color::Color, b: color::Color) -> StripePattern {
        return StripePattern {
            a,
            b,
            transform: matrix::Matrix4::IDENTITY,
        };
    }

    pub fn stripe_at(&self, point: &tuple::Point) -> color::Color {
        return if point.x.floor() % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        };
    }

    pub fn stripe_at_object(
        &self,
        object: &shape::Shape,
        world_point: &tuple::Point,
    ) -> color::Color {
        let object_point = object.transform.inverse().unwrap() * *world_point;
        let pattern_point = self.transform.inverse().unwrap() * object_point;

        self.stripe_at(&pattern_point)
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
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(pattern.a, color::white());
        assert_color_approx_eq!(pattern.b, color::black());
    }

    #[test]
    fn test_stripe_at_is_constant_in_y() {
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 1.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 2.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripe_at_is_constant_in_z() {
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 0.0, 1.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 0.0, 2.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripe_at_alternates_in_x() {
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(0.9, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(-0.1, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(-1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(&tuple::Point::new(-1.1, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_an_object_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at_object(&object, &tuple::Point::new(1.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_a_pattern_transformation() {
        let object = shape::Shape::default_sphere();
        let mut pattern = patterns::StripePattern::new(color::white(), color::black());
        pattern.transform = pattern.transform.scaling(2.0, 2.0, 2.0);
        dbg!(pattern.transform);

        assert_color_approx_eq!(
            pattern.stripe_at_object(&object, &tuple::Point::new(1.5, 0.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripes_with_both_an_object_and_a_pattern_transformation() {
        let mut object = shape::Shape::default_sphere();
        object.transform = object.transform.scaling(2.0, 2.0, 2.0);
        let mut pattern = patterns::StripePattern::new(color::white(), color::black());
        pattern.transform = pattern.transform.translation(0.5, 0.0, 0.0);

        assert_color_approx_eq!(
            pattern.stripe_at_object(&object, &tuple::Point::new(2.5, 0.0, 0.0)),
            color::white()
        );
    }
}
