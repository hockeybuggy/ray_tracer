use crate::matrix;
use crate::ray;
use crate::tuple;

// An axis-aligned bounding box: the smallest box, with faces parallel to
// the axes, that encloses a shape. Rays that miss the box cannot hit
// anything inside it, so an inexpensive box test can skip whole subtrees
// of expensive shape tests.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min: tuple::Point,
    pub max: tuple::Point,
}

impl BoundingBox {
    // An empty box has its extents inverted, so that the first point added
    // establishes both of them and any containment check fails.
    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: tuple::Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY),
            max: tuple::Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY),
        }
    }

    pub fn new(min: tuple::Point, max: tuple::Point) -> BoundingBox {
        BoundingBox { min, max }
    }

    pub fn add_point(&mut self, point: tuple::Point) {
        self.min.x = self.min.x.min(point.x);
        self.min.y = self.min.y.min(point.y);
        self.min.z = self.min.z.min(point.z);
        self.max.x = self.max.x.max(point.x);
        self.max.y = self.max.y.max(point.y);
        self.max.z = self.max.z.max(point.z);
    }

    pub fn add_box(&mut self, other: &BoundingBox) {
        self.add_point(other.min);
        self.add_point(other.max);
    }

    pub fn contains_point(&self, point: tuple::Point) -> bool {
        (self.min.x..=self.max.x).contains(&point.x)
            && (self.min.y..=self.max.y).contains(&point.y)
            && (self.min.z..=self.max.z).contains(&point.z)
    }

    pub fn contains_box(&self, other: &BoundingBox) -> bool {
        self.contains_point(other.min) && self.contains_point(other.max)
    }

    // The box that encloses this box after a transformation. Transforming
    // only the two extents would be wrong for rotations, so all eight
    // corners are transformed and a new box grown around them.
    pub fn transform(&self, transform: &matrix::Matrix4) -> BoundingBox {
        let corners = [
            tuple::Point::new(self.min.x, self.min.y, self.min.z),
            tuple::Point::new(self.min.x, self.min.y, self.max.z),
            tuple::Point::new(self.min.x, self.max.y, self.min.z),
            tuple::Point::new(self.min.x, self.max.y, self.max.z),
            tuple::Point::new(self.max.x, self.min.y, self.min.z),
            tuple::Point::new(self.max.x, self.min.y, self.max.z),
            tuple::Point::new(self.max.x, self.max.y, self.min.z),
            tuple::Point::new(self.max.x, self.max.y, self.max.z),
        ];

        let mut result = BoundingBox::empty();
        for corner in corners {
            result.add_point(*transform * corner);
        }
        return result;
    }

    // Whether the ray strikes the box. This is the cube intersection
    // generalized to arbitrary extents, except only the yes/no answer is
    // needed, not the `t` values.
    pub fn intersects(&self, ray: &ray::Ray) -> bool {
        let (xtmin, xtmax) = check_axis(ray.origin.x, ray.direction.x, self.min.x, self.max.x);
        let (ytmin, ytmax) = check_axis(ray.origin.y, ray.direction.y, self.min.y, self.max.y);
        let (ztmin, ztmax) = check_axis(ray.origin.z, ray.direction.z, self.min.z, self.max.z);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        return tmin <= tmax;
    }
}

fn check_axis(origin: f64, direction: f64, min: f64, max: f64) -> (f64, f64) {
    let tmin = (min - origin) / direction;
    let tmax = (max - origin) / direction;

    if tmin > tmax {
        return (tmax, tmin);
    }
    return (tmin, tmax);
}

#[cfg(test)]
mod bounds_tests {
    use crate::bounds;
    use crate::matrix;
    use crate::ray;
    use crate::transformation::Transform;
    use crate::tuple;

    #[test]
    fn test_creating_an_empty_bounding_box() {
        let bbox = bounds::BoundingBox::empty();

        assert_eq!(
            bbox.min,
            tuple::Point::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)
        );
        assert_eq!(
            bbox.max,
            tuple::Point::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY)
        );
    }

    #[test]
    fn test_creating_a_bounding_box_with_volume() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(-1.0, -2.0, -3.0),
            tuple::Point::new(3.0, 2.0, 1.0),
        );

        assert_eq!(bbox.min, tuple::Point::new(-1.0, -2.0, -3.0));
        assert_eq!(bbox.max, tuple::Point::new(3.0, 2.0, 1.0));
    }

    #[test]
    fn test_adding_points_to_an_empty_bounding_box() {
        let mut bbox = bounds::BoundingBox::empty();

        bbox.add_point(tuple::Point::new(-5.0, 2.0, 0.0));
        bbox.add_point(tuple::Point::new(7.0, 0.0, -3.0));

        assert_eq!(bbox.min, tuple::Point::new(-5.0, 0.0, -3.0));
        assert_eq!(bbox.max, tuple::Point::new(7.0, 2.0, 0.0));
    }

    #[test]
    fn test_adding_one_bounding_box_to_another() {
        let mut box1 = bounds::BoundingBox::new(
            tuple::Point::new(-5.0, -2.0, 0.0),
            tuple::Point::new(7.0, 4.0, 4.0),
        );
        let box2 = bounds::BoundingBox::new(
            tuple::Point::new(8.0, -7.0, -2.0),
            tuple::Point::new(14.0, 2.0, 8.0),
        );

        box1.add_box(&box2);

        assert_eq!(box1.min, tuple::Point::new(-5.0, -7.0, -2.0));
        assert_eq!(box1.max, tuple::Point::new(14.0, 4.0, 8.0));
    }

    #[test]
    fn test_checking_to_see_if_a_box_contains_a_given_point() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(5.0, -2.0, 0.0),
            tuple::Point::new(11.0, 4.0, 7.0),
        );

        let examples = [
            (tuple::Point::new(5.0, -2.0, 0.0), true),
            (tuple::Point::new(11.0, 4.0, 7.0), true),
            (tuple::Point::new(8.0, 1.0, 3.0), true),
            (tuple::Point::new(3.0, 0.0, 3.0), false),
            (tuple::Point::new(8.0, -4.0, 3.0), false),
            (tuple::Point::new(8.0, 1.0, -1.0), false),
            (tuple::Point::new(13.0, 1.0, 3.0), false),
            (tuple::Point::new(8.0, 5.0, 3.0), false),
            (tuple::Point::new(8.0, 1.0, 8.0), false),
        ];
        for (point, expected) in examples {
            assert_eq!(bbox.contains_point(point), expected);
        }
    }

    #[test]
    fn test_checking_to_see_if_a_box_contains_a_given_box() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(5.0, -2.0, 0.0),
            tuple::Point::new(11.0, 4.0, 7.0),
        );

        let examples = [
            (
                tuple::Point::new(5.0, -2.0, 0.0),
                tuple::Point::new(11.0, 4.0, 7.0),
                true,
            ),
            (
                tuple::Point::new(6.0, -1.0, 1.0),
                tuple::Point::new(10.0, 3.0, 6.0),
                true,
            ),
            (
                tuple::Point::new(4.0, -3.0, -1.0),
                tuple::Point::new(10.0, 3.0, 6.0),
                false,
            ),
            (
                tuple::Point::new(6.0, -1.0, 1.0),
                tuple::Point::new(12.0, 5.0, 8.0),
                false,
            ),
        ];
        for (min, max, expected) in examples {
            let other = bounds::BoundingBox::new(min, max);
            assert_eq!(bbox.contains_box(&other), expected);
        }
    }

    #[test]
    fn test_transforming_a_bounding_box() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(-1.0, -1.0, -1.0),
            tuple::Point::new(1.0, 1.0, 1.0),
        );
        let transform = matrix::Matrix4::IDENTITY
            .rotation_y(std::f64::consts::PI / 4.0)
            .rotation_x(std::f64::consts::PI / 4.0);

        let transformed = bbox.transform(&transform);

        crate::assert_tuple_approx_eq!(
            transformed.min,
            tuple::Point::new(-1.41421, -1.70711, -1.70711)
        );
        crate::assert_tuple_approx_eq!(
            transformed.max,
            tuple::Point::new(1.41421, 1.70711, 1.70711)
        );
    }

    #[test]
    fn test_intersecting_a_ray_with_a_bounding_box_at_the_origin() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(-1.0, -1.0, -1.0),
            tuple::Point::new(1.0, 1.0, 1.0),
        );

        let examples = [
            ((5.0, 0.5, 0.0), (-1.0, 0.0, 0.0), true),
            ((-5.0, 0.5, 0.0), (1.0, 0.0, 0.0), true),
            ((0.5, 5.0, 0.0), (0.0, -1.0, 0.0), true),
            ((0.5, -5.0, 0.0), (0.0, 1.0, 0.0), true),
            ((0.5, 0.0, 5.0), (0.0, 0.0, -1.0), true),
            ((0.5, 0.0, -5.0), (0.0, 0.0, 1.0), true),
            ((0.0, 0.5, 0.0), (0.0, 0.0, 1.0), true),
            ((-2.0, 0.0, 0.0), (2.0, 4.0, 6.0), false),
            ((0.0, -2.0, 0.0), (6.0, 2.0, 4.0), false),
            ((0.0, 0.0, -2.0), (4.0, 6.0, 2.0), false),
            ((2.0, 0.0, 2.0), (0.0, 0.0, -1.0), false),
            ((0.0, 2.0, 2.0), (0.0, -1.0, 0.0), false),
            ((2.0, 2.0, 0.0), (-1.0, 0.0, 0.0), false),
        ];
        for ((ox, oy, oz), (dx, dy, dz), expected) in examples {
            let direction = tuple::normalize(&tuple::Vector::new(dx, dy, dz));
            let ray = ray::ray(tuple::Point::new(ox, oy, oz), direction);
            assert_eq!(bbox.intersects(&ray), expected);
        }
    }

    #[test]
    fn test_intersecting_a_ray_with_a_non_cubic_bounding_box() {
        let bbox = bounds::BoundingBox::new(
            tuple::Point::new(5.0, -2.0, 0.0),
            tuple::Point::new(11.0, 4.0, 7.0),
        );

        let examples = [
            ((15.0, 1.0, 2.0), (-1.0, 0.0, 0.0), true),
            ((-5.0, -1.0, 4.0), (1.0, 0.0, 0.0), true),
            ((7.0, 6.0, 5.0), (0.0, -1.0, 0.0), true),
            ((9.0, -5.0, 6.0), (0.0, 1.0, 0.0), true),
            ((8.0, 2.0, 12.0), (0.0, 0.0, -1.0), true),
            ((6.0, 0.0, -5.0), (0.0, 0.0, 1.0), true),
            ((8.0, 1.0, 3.5), (0.0, 0.0, 1.0), true),
            ((9.0, -1.0, -8.0), (2.0, 4.0, 6.0), false),
            ((8.0, 3.0, -4.0), (6.0, 2.0, 4.0), false),
            ((9.0, -1.0, -2.0), (4.0, 6.0, 2.0), false),
            ((4.0, 0.0, 9.0), (0.0, 0.0, -1.0), false),
            ((8.0, 6.0, -1.0), (0.0, -1.0, 0.0), false),
            ((12.0, 5.0, 4.0), (-1.0, 0.0, 0.0), false),
        ];
        for ((ox, oy, oz), (dx, dy, dz), expected) in examples {
            let direction = tuple::normalize(&tuple::Vector::new(dx, dy, dz));
            let ray = ray::ray(tuple::Point::new(ox, oy, oz), direction);
            assert_eq!(bbox.intersects(&ray), expected);
        }
    }

    // Not from the book: an untransformed group is empty, and its inverted
    // extents must reject every ray rather than yielding NaN comparisons
    // that accept them.
    #[test]
    fn test_an_empty_box_intersects_nothing() {
        let bbox = bounds::BoundingBox::empty();

        let ray = ray::ray(
            tuple::Point::new(0.0, 0.0, -5.0),
            tuple::Vector::new(0.0, 0.0, 1.0),
        );
        assert!(!bbox.intersects(&ray));
    }

    #[test]
    fn test_transforming_an_empty_box_leaves_it_empty() {
        let bbox = bounds::BoundingBox::empty();

        let transformed = bbox.transform(&matrix::Matrix4::IDENTITY.translation(1.0, 2.0, 3.0));

        assert!(!transformed.contains_point(tuple::Point::new(1.0, 2.0, 3.0)));
    }

    fn transformed_is_empty(bbox: &bounds::BoundingBox) -> bool {
        bbox.min.x > bbox.max.x
    }

    #[test]
    fn test_transforming_an_empty_box_keeps_extents_inverted() {
        let bbox = bounds::BoundingBox::empty();

        let transformed = bbox.transform(&matrix::Matrix4::IDENTITY.translation(1.0, 2.0, 3.0));

        assert!(transformed_is_empty(&transformed));
    }
}
