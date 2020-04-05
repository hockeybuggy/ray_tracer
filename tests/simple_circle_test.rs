extern crate ray_tracer;
use std::io::{Read, Seek};

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

    // Write to the output file
    let output_path = "output_simple_circle.ppm";
    // Borrowed from https://stackoverflow.com/a/47956654
    let mut output_file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(output_path)?;
    canvas.canvas_to_ppm(&mut output_file)?;
    let mut output_contents = String::new();
    output_file.seek(std::io::SeekFrom::Start(0))?;
    output_file.read_to_string(&mut output_contents)?;

    let expected_str = include_str!("fixtures/simple_circle_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_contents.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}
