extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, canvas, color, lights, material, matrix, shape, transformation, tuple, world,
};

const SCALE: u32 = 1;

// Compare the render against its fixture. While iterating on a scene there
// is no fixture yet: the render is written to the repo root instead. Inspect
// it and copy it to `tests/fixtures/<name>.png` once the scene looks right.
fn assert_matches_fixture(canvas: &canvas::Canvas, name: &str) {
    let fixture_path = format!("tests/fixtures/{}.png", name);
    let output_path = format!("{}.png", name);

    if !std::path::Path::new(&fixture_path).exists() {
        shared_test_helpers::write_image_to_file(canvas, &output_path).unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `{}`.",
            output_path
        );
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file(name).unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(canvas, &output_path).unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `{}`.",
            output_path
        );
    }
}

#[test]
fn test_simple_cube() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A blue cube, rotated so three faces are visible.
    builder.add_shape({
        let mut cube = shape::Shape::default_cube();
        cube.set_transformation_matrix(
            matrix::Matrix4::IDENTITY.rotation_y(std::f64::consts::PI / 6.0),
        );
        let mut material = material::material();
        material.color = color::color(0.2, 0.4, 1.0);
        cube.material = material;
        cube
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.0, -4.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "simple_cube");
    return Ok(());
}

#[test]
fn test_simple_cylinder() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A capped teal cylinder, seen from slightly above so the top cap shows.
    builder.add_shape({
        let mut cylinder = shape::Shape::cylinder(0.0, 2.0, true);
        let mut material = material::material();
        material.color = color::color(0.2, 0.7, 0.6);
        cylinder.material = material;
        cylinder
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 3.0, -4.5),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "simple_cylinder");
    return Ok(());
}

#[test]
fn test_open_cylinder() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // An uncapped purple cylinder; the camera looks down into it, so the
    // hollow inside is visible.
    builder.add_shape({
        let mut cylinder = shape::Shape::cylinder(0.0, 2.0, false);
        let mut material = material::material();
        material.color = color::color(0.6, 0.3, 0.8);
        cylinder.material = material;
        cylinder
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 3.5, -4.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "open_cylinder");
    return Ok(());
}

#[test]
fn test_open_cone() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // An uncapped red cone with its tip at the bottom, opening upward like
    // a funnel; the camera looks down into the hollow inside. Squeezed in
    // `x` and `z` so the funnel is taller than it is wide.
    builder.add_shape({
        let mut cone = shape::Shape::cone(0.0, 1.0, false);
        cone.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.5, 1.0, 0.5));
        let mut material = material::material();
        material.color = color::color(0.85, 0.3, 0.3);
        cone.material = material;
        cone
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.6, -2.5),
        &tuple::Point::new(0.0, 0.5, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "open_cone");
    return Ok(());
}

#[test]
fn test_simple_cone() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // An orange cone with its wide capped base at the bottom and its tip
    // pointing up.
    builder.add_shape({
        let mut cone = shape::Shape::cone(-1.0, 0.0, true);
        cone.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0));
        let mut material = material::material();
        material.color = color::color(0.9, 0.5, 0.2);
        cone.material = material;
        cone
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -4.0),
        &tuple::Point::new(0.0, 0.5, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "simple_cone");
    return Ok(());
}

#[test]
fn test_simple_plane() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A single matte plane stretching to the horizon.
    builder.add_shape({
        let mut plane = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(0.4, 0.6, 0.9);
        material.specular = 0.0;
        plane.material = material;
        plane
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 1.5, -5.0),
        &tuple::Point::new(0.0, 0.0, 5.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "simple_plane");
    return Ok(());
}
