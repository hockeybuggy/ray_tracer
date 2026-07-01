extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, shape, transformation, tuple, world};

const SCALE: u32 = 1;

// Chapter 13 "Putting It Together": a capped cylinder, an open pipe seen
// from above, and an ice cream cone built from a truncated cone with a
// sphere scoop.
//
// The floor is a plain matte plane rather than a checkered one to avoid the
// macOS/Linux float noise on pattern cell boundaries (see `AGENTS.md`).
#[test]
fn test_cylinder_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // The floor.
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(0.9, 0.85, 0.8);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // A solid green column: a capped unit-height cylinder.
    builder.add_shape({
        let mut column = shape::Shape::cylinder(0.0, 1.0, true);
        column.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.5, 1.0, 0.5)
                .translation(-1.5, 0.0, 0.5),
        );
        let mut material = material::material();
        material.color = color::color(0.2, 0.6, 0.4);
        material.specular = 0.3;
        column.material = material;
        column
    });

    // An open yellow pipe; the camera looks down into it, so the hollow
    // inside is visible.
    builder.add_shape({
        let mut pipe = shape::Shape::cylinder(0.0, 1.0, false);
        pipe.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.35, 1.4, 0.35)
                .translation(1.6, 0.0, 1.0),
        );
        let mut material = material::material();
        material.color = color::color(0.8, 0.7, 0.3);
        material.specular = 0.3;
        pipe.material = material;
        pipe
    });

    // The waffle cone: the upper half of a cone opens upward, leaving the
    // tip resting on the floor.
    builder.add_shape({
        let mut waffle_cone = shape::Shape::cone(0.0, 1.0, false);
        waffle_cone.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.5, 1.5, 0.5)
                .translation(0.0, 0.0, -0.5),
        );
        let mut material = material::material();
        material.color = color::color(0.8, 0.6, 0.4);
        material.specular = 0.2;
        waffle_cone.material = material;
        waffle_cone
    });

    // The strawberry scoop, overlapping the cone's rim.
    builder.add_shape({
        let mut scoop = shape::Shape::default_sphere();
        scoop.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .scaling(0.6, 0.6, 0.6)
                .translation(0.0, 1.6, -0.5),
        );
        let mut material = material::material();
        material.color = color::color(0.9, 0.5, 0.6);
        material.specular = 0.4;
        scoop.material = material;
        scoop
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.5, -5.5),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    // While iterating on the scene there is no fixture yet: the render is
    // written to the repo root instead. Inspect it and copy it to
    // `tests/fixtures/cylinders_and_cones.png` once the scene looks right.
    if !std::path::Path::new("tests/fixtures/cylinders_and_cones.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "cylinders_and_cones.png").unwrap();
        assert!(
            false,
            "No fixture yet. Written canvas to `cylinders_and_cones.png`."
        );
    }

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("cylinders_and_cones").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "cylinders_and_cones.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `cylinders_and_cones.png`."
        );
    }
    return Ok(());
}
