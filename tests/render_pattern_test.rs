extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, color, lights, material, matrix, patterns, shape, transformation, tuple, world,
};

const SCALE: u32 = 1;

#[test]
fn test_checkered_sphere() -> Result<(), std::io::Error> {
    let mut world = world::world();

    // Create a floor and add it to the scene
    {
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        world.shapes.push(floor);
    }

    // Add a sphere to the left
    {
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        let pattern = patterns::Pattern::checkers(color::black(), color::white());
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        world.shapes.push(left);
    }

    // Add a sphere to the right
    {
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let pattern = patterns::Pattern::checkers(color::black(), color::white());
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        world.shapes.push(right);
    }

    // Let there be light
    let white_point_light =
        lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
    world.light = Some(white_point_light);

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("checkered_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "checkered_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `checkered_sphere.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_gradient_sphere() -> Result<(), std::io::Error> {
    let mut world = world::world();

    // Create a floor and add it to the scene
    {
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        world.shapes.push(floor);
    }

    // Add a sphere to the left
    {
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        let pattern = patterns::Pattern::gradient(color::black(), color::white());
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        world.shapes.push(left);
    }

    // Add a sphere to the right
    {
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let pattern = patterns::Pattern::gradient(color::black(), color::white());
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        world.shapes.push(right);
    }

    // Let there be light
    let white_point_light =
        lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
    world.light = Some(white_point_light);

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("gradient_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "gradient_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `gradient_sphere.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_ring_sphere() -> Result<(), std::io::Error> {
    let mut world = world::world();

    // Create a floor and add it to the scene
    {
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        world.shapes.push(floor);
    }

    // Add a sphere to the left
    {
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(2.0, 2.0, 2.0)
                .rotation_x(3.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        world.shapes.push(left);
    }

    // Add a sphere to the right
    {
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_x(std::f64::consts::PI / 1.0)
                .rotation_y(std::f64::consts::PI / 2.0)
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(1.0, 1.0, 1.0));
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        world.shapes.push(right);
    }

    // Let there be light
    let white_point_light =
        lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
    world.light = Some(white_point_light);

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("ring_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "ring_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `ring_sphere.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_stripe_sphere() -> Result<(), std::io::Error> {
    let mut world = world::world();

    // Create a floor and add it to the scene
    {
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        world.shapes.push(floor);
    }

    // Add a sphere to the left
    {
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        let mut pattern = patterns::Pattern::stripe(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(2.0, 2.0, 2.0)
                .rotation_x(3.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        world.shapes.push(left);
    }

    // Add a sphere to the right
    {
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(-std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let pattern = patterns::Pattern::stripe(color::black(), color::white());
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        world.shapes.push(right);
    }

    // Let there be light
    let white_point_light =
        lights::point_light(tuple::Point::new(-10.0, 10.0, -10.0), color::white());
    world.light = Some(white_point_light);

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("stripe_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "stripe_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `stripe_sphere.png`."
        );
    }
    return Ok(());
}
