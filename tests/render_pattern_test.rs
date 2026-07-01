extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, color, lights, material, matrix, patterns, shape, transformation, tuple, world,
};

const SCALE: u32 = 1;

#[test]
fn test_checkered_sphere() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        // Scaled down so several cells are visible on the sphere, and nudged
        // so cell boundaries don't land exactly on the sphere's poles.
        let mut pattern = patterns::Pattern::checkers(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.4, 0.4, 0.4)
                .translation(0.01, 0.01, 0.01),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let mut pattern = patterns::Pattern::checkers(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.4, 0.4, 0.4)
                .translation(0.01, 0.01, 0.01),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

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
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        // Stretched and shifted so a single smooth black-to-white ramp spans
        // the sphere instead of the pattern wrapping at x = 0.
        let mut pattern = patterns::Pattern::gradient(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(2.02, 2.02, 2.02)
                .translation(-1.01, 0.0, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let mut pattern = patterns::Pattern::gradient(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(2.02, 2.02, 2.02)
                .translation(-1.01, 0.0, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

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
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        // Scaled down so several rings fit on the sphere, and rotated so
        // the rings' axis points at the camera, showing a bullseye
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.2, 0.2, 0.2)
                .rotation_x(std::f64::consts::PI / 2.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(1.5, 1.0, 0.5));
        // A wider bullseye than the sphere on the left
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.25, 0.25, 0.25)
                .rotation_x(std::f64::consts::PI / 2.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

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
fn test_checkered_cube() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a cube to the left. The pattern is scaled and nudged so cell
    // boundaries don't land exactly on the cube faces (float noise there
    // renders differently on macOS and Linux).
    builder.add_shape({
        let mut left = shape::Shape::default_cube();
        left.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_y(std::f64::consts::PI / 6.0)
                .translation(-1.5, 0.7, 0.5),
        );
        let mut pattern = patterns::Pattern::checkers(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.45, 0.45, 0.45)
                .translation(0.01, 0.01, 0.01),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a cube to the right, rotated the other way
    builder.add_shape({
        let mut right = shape::Shape::default_cube();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_y(-std::f64::consts::PI / 5.0)
                .translation(1.5, 0.7, 0.5),
        );
        let mut pattern = patterns::Pattern::checkers(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.55, 0.55, 0.55)
                .translation(0.01, 0.01, 0.01),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    if !std::path::Path::new("tests/fixtures/checkered_cube.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "checkered_cube.png").unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `checkered_cube.png`."
        );
    }

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("checkered_cube").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "checkered_cube.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `checkered_cube.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_ring_cylinder() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene. A plane rather than the
    // squashed sphere the other pattern scenes use: the camera sits high
    // enough here that the sphere's edge would show.
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a cylinder to the left with the ring pattern rotated onto its
    // side, so the rings arc around the barrel
    builder.add_shape({
        let mut left = shape::Shape::cylinder(0.0, 1.5, true);
        left.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.5, 1.0, 0.5)
                .translation(-1.5, 0.0, 0.5),
        );
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.35, 0.35, 0.35)
                .rotation_x(std::f64::consts::PI / 2.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a cylinder to the right with the rings around the cylinder's own
    // axis, showing a bullseye on the top cap. The barrel is a constant
    // distance from the axis so it lands in a single ring; the scale is
    // chosen so that ring is white and no ring boundary sits exactly on
    // the rim of the cap.
    builder.add_shape({
        let mut right = shape::Shape::cylinder(0.0, 1.5, true);
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.5, 1.0, 0.5)
                .translation(1.5, 0.0, 0.5),
        );
        let mut pattern = patterns::Pattern::ring(color::black(), color::white());
        pattern.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.3, 0.3, 0.3));
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    // The camera sits higher than in the other pattern scenes so the top
    // caps of the cylinders are visible
    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.8, -3.8),
        &tuple::Point::new(0.0, 0.75, 0.5),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    if !std::path::Path::new("tests/fixtures/ring_cylinder.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "ring_cylinder.png").unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `ring_cylinder.png`."
        );
    }

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("ring_cylinder").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "ring_cylinder.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `ring_cylinder.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_stripe_cube() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a cube to the left with vertical stripes. The pattern is scaled
    // and nudged so stripe boundaries don't land exactly on the cube faces.
    builder.add_shape({
        let mut left = shape::Shape::default_cube();
        left.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_y(std::f64::consts::PI / 6.0)
                .translation(-1.5, 0.7, 0.5),
        );
        let mut pattern = patterns::Pattern::stripe(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.25, 0.25, 0.25)
                .translation(0.01, 0.0, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a cube to the right with the stripes running horizontally
    builder.add_shape({
        let mut right = shape::Shape::default_cube();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_y(-std::f64::consts::PI / 5.0)
                .translation(1.5, 0.7, 0.5),
        );
        let mut pattern = patterns::Pattern::stripe(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.25, 0.25, 0.25)
                .rotation_z(std::f64::consts::PI / 2.0)
                .translation(0.0, 0.01, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    if !std::path::Path::new("tests/fixtures/stripe_cube.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "stripe_cube.png").unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `stripe_cube.png`."
        );
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file("stripe_cube").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "stripe_cube.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `stripe_cube.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_stripe_sphere() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // Create a floor and add it to the scene
    builder.add_shape({
        let mut floor = shape::Shape::default_sphere();
        floor.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(10.0, 0.01, 10.0));
        let mut material = material::material();
        material.color = color::color(1.0, 0.9, 0.9);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // Add a sphere to the left
    builder.add_shape({
        let mut left = shape::Shape::default_sphere();
        left.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-1.5, 1.0, 0.5));
        // Scaled down so several stripes fit on the sphere, and nudged so
        // stripe boundaries don't land exactly on the sphere's poles.
        let mut pattern = patterns::Pattern::stripe(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.25, 0.25, 0.25)
                .rotation_x(3.0)
                .translation(0.01, 0.0, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        left.material = material;
        left
    });

    // Add a sphere to the right
    builder.add_shape({
        let mut right = shape::Shape::default_sphere();
        right.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_z(-std::f64::consts::PI / 2.0)
                .translation(1.5, 1.0, 0.5),
        );
        let mut pattern = patterns::Pattern::stripe(color::black(), color::white());
        pattern.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.25, 0.25, 0.25)
                .translation(0.01, 0.0, 0.0),
        );
        let mut material = material::material();
        material.color = color::color(0.1, 1.0, 0.5);
        material.diffuse = 0.7;
        material.specular = 0.3;
        material.pattern = Some(pattern);
        right.material = material;
        right
    });

    // Let there be light
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 50 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

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
