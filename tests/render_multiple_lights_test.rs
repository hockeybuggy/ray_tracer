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
fn test_multiple_lights() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A matte white floor to catch the shadows.
    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let mut material = material::material();
                material.color = color::color(1.0, 1.0, 1.0);
                material.specular = 0.0;
                material
            })
            .build(),
    );

    // A white sphere lit from both sides.
    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_transform(matrix::Matrix4::IDENTITY.translation(0.0, 1.0, 0.0))
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.9, 0.9, 0.9);
                material
            })
            .build(),
    );

    // A warm light from the left and a cool light from the right. Each casts
    // its own shadow, tinted by the opposite light's color.
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-8.0, 6.0, -6.0),
        color::color(0.8, 0.4, 0.3),
    ));
    builder.add_light_source(lights::point_light(
        tuple::Point::new(8.0, 6.0, -6.0),
        color::color(0.3, 0.4, 0.8),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.5, -7.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "multiple_lights");
    return Ok(());
}
