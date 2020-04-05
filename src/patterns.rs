// TODO remove allow dead code later on
use crate::color;
use crate::tuple;

#[allow(dead_code)]
struct StripePattern {
    a: color::Color,
    b: color::Color,
}

impl StripePattern {
    #[allow(dead_code)]
    fn new(a: color::Color, b: color::Color) -> StripePattern {
        return StripePattern { a, b };
    }

    #[allow(dead_code)]
    fn stripe_at(&self, point: tuple::Point) -> color::Color {
        return if point.x.floor() % 2.0 == 0.0 {
            color::white()
        } else {
            color::black()
        };
    }
}

#[cfg(test)]
mod patterns_tests {
    use crate::assert_color_approx_eq;
    use crate::color;
    use crate::patterns;
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
            pattern.stripe_at(tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 1.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 2.0, 0.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripe_at_is_constant_in_z() {
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 0.0, 1.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 0.0, 2.0)),
            color::white()
        );
    }

    #[test]
    fn test_stripe_at_alternates_in_x() {
        let pattern = patterns::StripePattern::new(color::white(), color::black());

        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.0, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(0.9, 0.0, 0.0)),
            color::white()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(-0.1, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(-1.0, 0.0, 0.0)),
            color::black()
        );
        assert_color_approx_eq!(
            pattern.stripe_at(tuple::Point::new(-1.1, 0.0, 0.0)),
            color::white()
        );
    }
}
