extern crate ray_tracer;
use std::io::{Read, Seek};

use ray_tracer::{canvas, color, lighting, lights, material, ray, sphere, tuple};

// fn draw_hit(canvas: &mut canvas::Canvas, x: u64, y: u64, _hit: &ray::Intersection) {
// }

#[test]
fn test_simple_sphere_test() -> Result<(), std::io::Error> {
    let ray_origin = tuple::point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let canvas_size = 100;
    let pixel_size = wall_size / canvas_size as f64;
    let mut canvas = canvas::canvas(canvas_size, canvas_size);
    let mut sphere = sphere::sphere();
    let mut pink_material = material::material();
    pink_material.color = color::color(1.0, 0.2, 1.0);
    sphere.material = pink_material;

    let light_position = tuple::point(-10.0, 10.0, -10.0);
    let light_color = color::color(1.0, 1.0, 1.0);
    let light = lights::point_light(light_position, light_color);

    for y in 0..canvas.height {
        // Top +half, bottom -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas.width {
            // left -half, right +half
            let world_x = -half + pixel_size * x as f64;

            let position = tuple::point(world_x, world_y, wall_z);

            let ray = ray::ray(ray_origin, tuple::normalize(&(position - ray_origin)));

            let intersections = ray.intersect(&sphere);

            match ray::hit(&intersections) {
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let camera = -ray.direction;
                    let color =
                        lighting::lighting(&hit.object.material, &light, &point, &camera, &normal);
                    canvas.write_pixel(x, y, color);
                }
                None => (),
            }
        }
    }

    // Write to the output file
    let output_path = "output_simple_sphere.ppm";
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

    let expected_str = include_str!("fixtures/simple_sphere_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_contents.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}
