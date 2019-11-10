extern crate ray_tracer;
use std::io::{Read, Seek};

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, sphere, transformation, tuple, world};

#[test]
fn test_simple_world() -> Result<(), std::io::Error> {
    let mut world = world::world();

    // Create a floor and add it to the scene
    {
        let mut floor = sphere::sphere();
        floor.transform = matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0);
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        world.shapes.push(floor);
    }

    // Create a wall and add it to the scene
    {
        let mut left_wall = sphere::sphere();
        left_wall.transform = matrix::Matrix4::IDENTITY
            .scaling(10.0, 0.01, 10.0)
            .rotation_x(std::f64::consts::PI / 2.0)
            .rotation_y(-std::f64::consts::PI / 4.0)
            .translation(0.0, 0.0, 5.0);
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        left_wall.material = material;
        world.shapes.push(left_wall);
    }

    // Create another wall and add it to the scene
    {
        let mut right_wall = sphere::sphere();
        right_wall.transform = matrix::Matrix4::IDENTITY
            .scaling(10.0, 0.01, 10.0)
            .rotation_x(std::f64::consts::PI / 2.0)
            .rotation_y(std::f64::consts::PI / 4.0)
            .translation(0.0, 0.0, 5.0);
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        right_wall.material = material;
        world.shapes.push(right_wall);
    }

    // Add a sphere to the center
    {
        let mut middle = sphere::sphere();
        middle.transform = matrix::Matrix4::IDENTITY.translation(-0.5, 1.0, 0.5);
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        middle.material = material;
        world.shapes.push(middle);
    }

    // Add a small green sphere on the right
    {
        let mut right = sphere::sphere();
        right.transform = matrix::Matrix4::IDENTITY
            .scaling(0.5, 0.5, 0.5)
            .translation(1.5, 0.5, 0.5);
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        right.material = material;
        world.shapes.push(right);
    }

    // Add a smaller green sphere on the left
    {
        let mut left = sphere::sphere();
        left.transform = matrix::Matrix4::IDENTITY
            .scaling(0.3333, 0.3333, 0.3333)
            .translation(-1.5, 0.33, -0.75);
        let mut material = material::material();
        material.color = color::color(1.0, 0.8, 0.1);
        material.diffuse = 0.7;
        material.specular = 0.3;
        left.material = material;
        world.shapes.push(left);
    }

    // Let there be light
    let white_point_light = lights::point_light(tuple::point(-10.0, 10.0, -10.0), color::white());
    world.light = Some(white_point_light);

    let mut camera = camera::Camera::new(100, 50, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::point(0.0, 1.5, -5.0),
        &tuple::point(0.0, 1.0, 0.0),
        &tuple::vector(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    // Write to the output file
    let output_path = "output_simple_world.ppm";
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

    let expected_str = include_str!("fixtures/simple_world_test.ppm");

    // TODO consider if this would be better as a line by line check
    assert!(output_contents.contains(expected_str));

    std::fs::remove_file(output_path)?;
    return Ok(());
}
