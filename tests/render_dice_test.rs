extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, shape, transformation, tuple, world};

const SCALE: u32 = 1;

fn colored_material(color: color::Color) -> material::Material {
    let mut material = material::material();
    material.color = color;
    material.specular = 0.3;
    material
}

// Painted matte pips: a specular highlight inside the carved dish makes it
// read as a protruding bead, so the pips get none.
fn pip_material(color: color::Color) -> material::Material {
    let mut material = material::material();
    material.color = color;
    material.specular = 0.0;
    material
}

// The centers of all 21 pips on a cube spanning -1..1, laid out like a
// real die: opposite faces sum to seven, with one on top, two facing the
// camera, and three on the +x side. The centers sit slightly outside the
// faces so the subtracted spheres carve shallow dishes rather than deep
// hemispheres.
fn pip_positions() -> Vec<(f64, f64, f64)> {
    const D: f64 = 0.55;
    const F: f64 = 1.08;
    let mut positions = Vec::new();

    // One, on top.
    positions.push((0.0, F, 0.0));
    // Two, on the front.
    for (a, b) in [(-D, -D), (D, D)] {
        positions.push((a, b, -F));
    }
    // Three, on the right.
    for (a, b) in [(-D, -D), (0.0, 0.0), (D, D)] {
        positions.push((F, a, b));
    }
    // Four, on the left.
    for (a, b) in [(-D, -D), (-D, D), (D, -D), (D, D)] {
        positions.push((-F, a, b));
    }
    // Five, on the back.
    for (a, b) in [(-D, -D), (-D, D), (0.0, 0.0), (D, -D), (D, D)] {
        positions.push((a, b, F));
    }
    // Six, on the bottom.
    for (a, b) in [(-D, -D), (-D, 0.0), (-D, D), (D, -D), (D, 0.0), (D, D)] {
        positions.push((a, -F, b));
    }
    positions
}

// A die is a chain of CSG operations: a cube intersected with a sphere to
// round the corners off, then a difference for each pip, carving a dish
// that keeps the pip's own color.
fn die(body_color: color::Color, pip_color: color::Color) -> shape::Shape {
    let cube = shape::ShapeBuilder::cube()
        .set_material(colored_material(body_color))
        .build();

    let corners = shape::ShapeBuilder::sphere()
        .set_transform(matrix::Matrix4::IDENTITY.scaling(1.6, 1.6, 1.6))
        .set_material(colored_material(body_color))
        .build();

    let mut die = shape::Shape::csg(shape::CsgOperation::Intersection, cube, corners);

    for (x, y, z) in pip_positions() {
        let pip = shape::ShapeBuilder::sphere()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.22, 0.22, 0.22)
                    .translation(x, y, z),
            )
            .set_material(pip_material(pip_color))
            .build();
        die = shape::Shape::csg(shape::CsgOperation::Difference, die, pip);
    }
    return die;
}

// A pair of dice built entirely from CSG: 22 primitives and 22 nested set
// operations each. The red die is tipped a quarter turn so a different
// face is up, exercising a transform on a deeply nested CSG tree.
#[test]
fn test_dice_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // The felt.
    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.15, 0.45, 0.25);
                material.specular = 0.0;
                material
            })
            .build(),
    );

    // An ivory die with dark pips, showing one on top.
    builder.add_shape(
        shape::ShapeBuilder::from(die(
            color::color(0.95, 0.93, 0.85),
            color::color(0.15, 0.15, 0.18),
        ))
        .set_transform(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_y(-0.4)
                .translation(-1.3, 0.7, 0.2),
        )
        .build(),
    );

    // A red die with white pips, tipped a quarter turn onto its side so
    // three is up, then spun a little.
    builder.add_shape(
        shape::ShapeBuilder::from(die(
            color::color(0.75, 0.15, 0.18),
            color::color(0.95, 0.95, 0.95),
        ))
        .set_transform(
            matrix::Matrix4::IDENTITY
                .scaling(0.7, 0.7, 0.7)
                .rotation_z(std::f64::consts::FRAC_PI_2)
                .rotation_y(0.5)
                .translation(1.2, 0.7, -0.4),
        )
        .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.4, -4.6),
        &tuple::Point::new(0.0, 0.6, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    // While iterating on the scene there is no fixture yet: the render is
    // written to the repo root instead. Inspect it and copy it to
    // `tests/fixtures/dice.png` once the scene looks right.
    if !std::path::Path::new("tests/fixtures/dice.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "dice.png").unwrap();
        assert!(false, "No fixture yet. Written canvas to `dice.png`.");
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file("dice").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "dice.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `dice.png`."
        );
    }
    return Ok(());
}
