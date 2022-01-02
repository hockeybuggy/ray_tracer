extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, color, lights, material, matrix, patterns, shape, transformation, tuple, world,
};

const SCALE: u32 = 1;

#[test]
fn test_reflective_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 1.0, 1.0);
        material.specular = 0.0;
        material.reflective = 0.5;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(-2.5, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(1.0, 0.0, 0.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        left.material = material;
        left
    });

    // Add a sphere in the middle
    builder.add_shape({
        let mut middle = shape::Shape::default_sphere();
        middle.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(0.0, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(0.0, 1.0, 0.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        middle.material = material;
        middle
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(2.5, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(0.0, 0.0, 1.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        right.material = material;
        right
    });
    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 150 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("reflection").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "reflection.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `reflection.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_very_reflective_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 1.0, 1.0);
        material.specular = 0.0;
        material.reflective = 0.5;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(-2.5, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(1.0, 0.0, 0.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.reflective = 0.5;
        left.material = material;
        left
    });

    // Add a sphere in the middle
    builder.add_shape({
        let mut middle = shape::Shape::default_sphere();
        middle.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(0.0, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(0.0, 1.0, 0.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.reflective = 0.5;
        middle.material = material;
        middle
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .translation(2.5, 1.0, 0.5)
                .scaling(0.7, 0.7, 0.7),
        );
        let mut material = material::material();
        material.color = color::color(0.0, 0.0, 1.0);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.reflective = 0.5;
        right.material = material;
        right
    });
    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-5.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 150 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("very_reflection").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "very_reflection.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `very_reflection.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_glass_sphere_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(100.0, 0.01, 100.0));
        let mut material = material::material();
        let mut pattern =
            patterns::Pattern::checkers(color::color(0.5, 0.5, 0.5), color::color(0.7, 0.7, 0.7));
        pattern.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.005, 0.005, 0.005));
        material.pattern = Some(pattern);
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut sphere = shape::Shape::glass_sphere();
        sphere.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0));
        sphere
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 150 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 5.0, 0.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 0.0, 1.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("glass_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "glass_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `glass_sphere.png`."
        );
    }
    return Ok(());
}
