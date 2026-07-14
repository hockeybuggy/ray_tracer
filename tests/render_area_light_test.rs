extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, canvas, color, lights, material, matrix, obj_file, sequences, shape, transformation,
    tuple, world,
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

// The `multiple_lights` scene with its two point lights replaced by a
// single rectangular area light: the same white sphere now casts one soft
// shadow with a penumbra, instead of two crisp ones.
#[test]
fn test_area_light() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

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

    let mut light = lights::area_light(
        tuple::Point::new(-2.0, 5.0, -6.0),
        tuple::Vector::new(4.0, 0.0, 0.0),
        8,
        tuple::Vector::new(0.0, 0.0, 3.0),
        8,
        color::color(1.5, 1.5, 1.5),
    );
    light.set_jitter(sequences::Sequence::random(256, 0x5EED));
    builder.add_light_source(light);

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.5, -7.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "area_light");
    return Ok(());
}

// The low-poly Utah teapot under the same kind of rectangular area light:
// the spout and handle cast soft shadows whose penumbras widen the further
// the shadow falls from the body.
#[test]
fn test_soft_shadow_teapot() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

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

    // The model is built z-up and roughly 32 units wide, so stand it up on
    // the y axis and scale it down to about three units across.
    let source = std::fs::read_to_string("object_files/teapot-low.obj")?;
    builder.add_shape(
        shape::ShapeBuilder::from(obj_file::parse_obj(&source).into_group())
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .rotation_x(-std::f64::consts::PI / 2.0)
                    .scaling(0.1, 0.1, 0.1),
            )
            .build(),
    );

    // Up and to the left, so the shadow stretches to the right of the
    // teapot where the camera can see it.
    let mut light = lights::area_light(
        tuple::Point::new(-7.0, 5.0, -2.0),
        tuple::Vector::new(3.0, 0.0, 0.0),
        8,
        tuple::Vector::new(0.0, 0.0, 3.0),
        8,
        color::color(1.5, 1.5, 1.5),
    );
    light.set_jitter(sequences::Sequence::random(256, 0x5EED));
    builder.add_light_source(light);

    let mut camera = camera::Camera::new(100 * SCALE, 75 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.5, 2.5, -5.0),
        &tuple::Point::new(0.5, 0.6, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "soft_shadow_teapot");
    return Ok(());
}
