use crate::matrix;
use crate::matrix::Inverse;
use crate::ray;
use crate::tuple;

pub struct Camera {
    hsize: i32,
    vsize: i32,
    field_of_view: f64,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    transform: matrix::Matrix4,
}

impl Camera {
    pub fn new(hsize: i32, vsize: i32, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = f64::from(hsize) / f64::from(vsize);
        let half_width;
        let half_height;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }

        let pixel_size = (half_width * 2.0) / f64::from(hsize);

        Camera {
            hsize,
            vsize,
            field_of_view,
            half_width,
            half_height,
            pixel_size,
            transform: matrix::Matrix4::IDENTITY,
        }
    }

    pub fn ray_for_pixel(&self, x: i32, y: i32) -> ray::Ray {
        // offset from the edge of the canvas to the pixel's center
        let xoffset = ((x as f64) + 0.5) * self.pixel_size;
        let yoffset = ((y as f64) + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        // (camera looks towards -z, so +x is the left)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inverse_transform = self.transform.inverse().unwrap();
        let pixel = inverse_transform * tuple::point(world_x, world_y, -1.0);
        let origin = inverse_transform * tuple::point(0.0, 0.0, 0.0);
        let direction = tuple::normalize(&(pixel - origin));

        return ray::ray(origin, direction);
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::assert_tuple_approx_eq;
    use crate::camera;
    use crate::matrix;
    use crate::transformation::Transform;
    use crate::tuple;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_camera_constructor() {
        let hsize = 160;
        let vsize = 160;
        let field_of_view = std::f64::consts::PI / 2.0;

        let camera = camera::Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
        assert_eq!(camera.field_of_view, field_of_view);
        assert_eq!(camera.transform, matrix::Matrix4::IDENTITY);
    }

    #[test]
    fn test_pixel_size_for_a_horizontal_canvas() {
        let camera = camera::Camera::new(200, 125, std::f64::consts::PI / 2.0);
        assert_approx_eq!(camera.pixel_size, 0.01);
    }

    #[test]
    fn test_pixel_size_for_a_vertical_canvas() {
        let camera = camera::Camera::new(125, 200, std::f64::consts::PI / 2.0);
        assert_approx_eq!(camera.pixel_size, 0.01);
    }

    #[test]
    fn test_a_ray_through_the_center_of_the_canvas() {
        let camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);

        let ray = camera.ray_for_pixel(100, 50);

        assert_tuple_approx_eq!(ray.origin, tuple::point(0.0, 0.0, 0.0));
        assert_tuple_approx_eq!(ray.direction, tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_a_ray_through_the_corner_of_the_canvas() {
        let camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert_tuple_approx_eq!(ray.origin, tuple::point(0.0, 0.0, 0.0));
        assert_tuple_approx_eq!(ray.direction, tuple::vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn test_a_ray_when_the_camera_is_transformed() {
        let mut camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let transform = matrix::Matrix4::IDENTITY
            .translation(0.0, -2.0, 5.0)
            .rotation_y(std::f64::consts::PI / 4.0);
        // TODO might be reversed
        camera.transform = transform;

        let ray = camera.ray_for_pixel(100, 50);

        assert_tuple_approx_eq!(ray.origin, tuple::point(0.0, 2.0, -5.0));
        assert_tuple_approx_eq!(
            ray.direction,
            tuple::vector(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0,)
        );
    }
}
