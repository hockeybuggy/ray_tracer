extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::{canvas, color, ray, shape, tuple};

#[test]
fn test_simple_circle_test() -> Result<(), std::io::Error> {
    let ray_origin = tuple::Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let canvas_size = 100;
    let pixel_size = wall_size / canvas_size as f64;
    let mut canvas = canvas::canvas(canvas_size, canvas_size);
    let sphere = shape::Shape::default_sphere();

    for y in 0..canvas.height {
        // Top +half, bottom -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            // left -half, right +half
            let world_x = -half + pixel_size * x as f64;

            let position = tuple::Point::new(world_x, world_y, wall_z);

            let ray = ray::ray(ray_origin, tuple::normalize(&(position - ray_origin)));
            let intersections = ray.intersect(&sphere);

            match ray::hit(&intersections) {
                Some(_hit) => canvas.write_pixel(x, y, color::color(1.0, 0.8, 0.6)),
                None => (),
            }
        }
    }

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("simple_circle_test").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "simple_circle_test.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `simple_circle_test.png`."
        );
    }

    return Ok(());
}
