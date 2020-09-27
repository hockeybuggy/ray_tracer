extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{canvas, color, lighting, lights, material, matrix, ray, shape, tuple};

#[test]
fn test_simple_sphere_test() -> Result<(), std::io::Error> {
    let ray_origin = tuple::Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let canvas_size = 100;
    let pixel_size = wall_size / canvas_size as f64;
    let mut canvas = canvas::canvas(canvas_size, canvas_size);
    let mut sphere = shape::Shape::default_sphere();
    let mut pink_material = material::material();
    pink_material.color = color::color(1.0, 0.2, 1.0);
    sphere.material = pink_material;

    let light_position = tuple::Point::new(-10.0, 10.0, -10.0);
    let light_color = color::color(1.0, 1.0, 1.0);
    let light = lights::point_light(light_position, light_color);

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
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let camera = -ray.direction;
                    let color = lighting::lighting(
                        &hit.object.material,
                        &hit.object,
                        &light,
                        &point,
                        &camera,
                        &normal,
                        false,
                    );
                    canvas.write_pixel(x, y, color);
                }
                None => (),
            }
        }
    }

    // Write to the output file
    let output_path = "output_simple_sphere.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(&canvas, "output_simple_sphere.png").unwrap();

    let expected_str = include_str!("fixtures/simple_sphere_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}

#[test]
fn test_translated_sphere_test() -> Result<(), std::io::Error> {
    let ray_origin = tuple::Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let half = wall_size / 2.0;
    let canvas_size = 100;
    let pixel_size = wall_size / canvas_size as f64;
    let mut canvas = canvas::canvas(canvas_size, canvas_size);
    let mut sphere = shape::Shape::default_sphere();
    let mut pink_material = material::material();
    pink_material.color = color::color(1.0, 0.2, 1.0);
    sphere.material = pink_material;
    sphere.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .identity()
            .translation(1.0, 0.0, 0.0),
    );

    let light_position = tuple::Point::new(-10.0, 10.0, -10.0);
    let light_color = color::color(1.0, 1.0, 1.0);
    let light = lights::point_light(light_position, light_color);

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
                Some(hit) => {
                    let point = ray.position(hit.t);
                    let normal = hit.object.normal_at(point);
                    let camera = -ray.direction;
                    let color = lighting::lighting(
                        &hit.object.material,
                        &hit.object,
                        &light,
                        &point,
                        &camera,
                        &normal,
                        false,
                    );
                    canvas.write_pixel(x, y, color);
                }
                None => (),
            }
        }
    }

    // Write to the output file
    let output_path = "output_translated_sphere.ppm";
    let output_ppm_string = shared_test_helpers::get_ppm_string_via_file(&canvas, output_path);
    shared_test_helpers::write_image_to_file(&canvas, "output_translated_sphere.png").unwrap();

    let expected_str = include_str!("fixtures/translated_sphere_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_ppm_string.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}
