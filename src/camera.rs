use crate::canvas;
use crate::matrix;
use crate::matrix::Inverse;
use crate::ray;
use crate::tuple;
use crate::world;

pub struct Camera {
    hsize: u32,
    vsize: u32,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
    pub transform: matrix::Matrix4,
}

impl Camera {
    pub fn new(hsize: u32, vsize: u32, field_of_view: f64) -> Camera {
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
            half_width,
            half_height,
            pixel_size,
            transform: matrix::Matrix4::IDENTITY,
        }
    }

    pub fn ray_for_pixel(&self, x: u32, y: u32) -> ray::Ray {
        // offset from the edge of the canvas to the pixel's center
        let xoffset = ((x as f64) + 0.5) * self.pixel_size;
        let yoffset = ((y as f64) + 0.5) * self.pixel_size;

        // untransformed coordinates of the pixel in world space
        // (camera looks towards -z, so +x is the left)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let inverse_transform = self.transform.inverse().unwrap();
        let pixel = inverse_transform * tuple::Point::new(world_x, world_y, -1.0);
        let origin = inverse_transform * tuple::Point::new(0.0, 0.0, 0.0);
        let direction = tuple::normalize(&(pixel - origin));

        return ray::ray(origin, direction);
    }

    pub fn render(&self, world: &world::World) -> canvas::Canvas {
        let mut image = canvas::canvas(self.hsize, self.vsize);
        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }
        return image;
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera;
    use crate::color;
    use crate::matrix;
    use crate::transformation;
    use crate::transformation::Transform;
    use crate::tuple;
    use crate::world;
    use crate::{assert_color_approx_eq, assert_tuple_approx_eq};
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_camera_constructor() {
        let hsize = 160;
        let vsize = 160;
        let field_of_view = std::f64::consts::PI / 2.0;

        let camera = camera::Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
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

        assert_tuple_approx_eq!(ray.origin, tuple::Point::new(0.0, 0.0, 0.0));
        assert_tuple_approx_eq!(ray.direction, tuple::Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_a_ray_through_the_near_corner_of_the_canvas() {
        let camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);

        let ray = camera.ray_for_pixel(0, 0);

        assert_tuple_approx_eq!(ray.origin, tuple::Point::new(0.0, 0.0, 0.0));
        assert_tuple_approx_eq!(
            ray.direction,
            tuple::Vector::new(0.66519, 0.33259, -0.66851)
        );
    }

    #[test]
    fn test_a_ray_through_the_far_corner_of_the_canvas() {
        let camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);

        let ray = camera.ray_for_pixel(200, 100);

        assert_tuple_approx_eq!(ray.origin, tuple::Point::new(0.0, 0.0, 0.0));
        assert_tuple_approx_eq!(
            ray.direction,
            tuple::Vector::new(-0.66519, -0.33259, -0.66851)
        );
    }

    #[test]
    fn test_a_ray_when_the_camera_is_transformed() {
        let mut camera = camera::Camera::new(201, 101, std::f64::consts::PI / 2.0);
        let transform = matrix::Matrix4::IDENTITY
            .translation(0.0, -2.0, 5.0)
            .rotation_y(std::f64::consts::PI / 4.0);
        camera.transform = transform;

        let ray = camera.ray_for_pixel(100, 50);

        assert_tuple_approx_eq!(ray.origin, tuple::Point::new(0.0, 2.0, -5.0));
        assert_tuple_approx_eq!(
            ray.direction,
            tuple::Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0,)
        );
    }

    #[test]
    fn test_rendering_a_world_with_a_camera() {
        let world = world::default_world();
        let mut camera = camera::Camera::new(11, 11, std::f64::consts::PI / 2.0);
        let from = tuple::Point::new(0.0, 0.0, -5.0);
        let to = tuple::Point::new(0.0, 0.0, 0.0);
        let up = tuple::Vector::new(0.0, 1.0, 0.0);
        camera.transform = transformation::view_transform(&from, &to, &up);

        let image = camera.render(&world);

        assert_color_approx_eq!(image.pixel_at(5, 5), color::color(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn test_rendering_a_world_with_a_camera_bounds() {
        // I added this test because the edges of the camera were black.
        use crate::lights;
        use crate::material;
        use crate::shape::Shape;
        use crate::sphere;

        let mut world = world::world();
        // Add  big sphere to the center so that the whole image is full of something
        {
            let mut big_guy = sphere::Sphere::default();
            big_guy.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(5.0, 5.0, 5.0));
            let mut material = material::material();
            material.color = color::color(0.5, 1.0, 0.5);
            material.diffuse = 0.7;
            material.specular = 0.3;
            big_guy.material = material;
            world.shapes.push(big_guy);
        }
        // Let there be light
        let white_point_light =
            lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
        world.light = Some(white_point_light);

        let mut camera = camera::Camera::new(11, 11, std::f64::consts::PI / 2.0);
        let from = tuple::Point::new(0.0, 0.0, -5.0);
        let to = tuple::Point::new(0.0, 0.0, 0.0);
        let up = tuple::Vector::new(0.0, 1.0, 0.0);
        camera.transform = transformation::view_transform(&from, &to, &up);

        let image = camera.render(&world);

        // Check the corners
        assert_color_approx_eq!(
            image.pixel_at(0, 0),
            color::color(0.166675, 0.33334, 0.166675)
        );
        assert_color_approx_eq!(
            image.pixel_at(10, 10),
            color::color(0.166675, 0.33334, 0.166675)
        );
    }
}
