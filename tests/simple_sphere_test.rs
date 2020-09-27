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

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("simple_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "simple_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `simple_sphere.png`."
        );
    }
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

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("translated_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "translated_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `translated_sphere.png`."
        );
    }

    return Ok(());
}
