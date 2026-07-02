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

// The book's hexagon model: six sides, each a group holding one corner
// sphere and one edge cylinder, rotated into place around the y axis. Every
// component is built at the origin and only the transforms place it, so the
// finished hexagon can be posed as a single unit.

fn hexagon_material() -> material::Material {
    let mut material = material::material();
    material.color = color::color(0.2, 0.65, 0.55);
    return material;
}

fn hexagon_corner() -> shape::Shape {
    let mut corner = shape::Shape::default_sphere();
    corner.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .scaling(0.25, 0.25, 0.25)
            .translation(0.0, 0.0, -1.0),
    );
    corner.material = hexagon_material();
    return corner;
}

fn hexagon_edge() -> shape::Shape {
    let mut edge = shape::Shape::cylinder(0.0, 1.0, false);
    edge.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .scaling(0.25, 1.0, 0.25)
            .rotation_z(-std::f64::consts::PI / 2.0)
            .rotation_y(-std::f64::consts::PI / 6.0)
            .translation(0.0, 0.0, -1.0),
    );
    edge.material = hexagon_material();
    return edge;
}

fn hexagon_side() -> shape::Shape {
    let mut side = shape::Shape::default_group();
    side.add_child(hexagon_corner());
    side.add_child(hexagon_edge());
    return side;
}

fn hexagon() -> shape::Shape {
    let mut hexagon = shape::Shape::default_group();
    for n in 0..6 {
        let mut side = hexagon_side();
        side.set_transformation_matrix(
            matrix::Matrix4::IDENTITY.rotation_y(n as f64 * std::f64::consts::PI / 3.0),
        );
        hexagon.add_child(side);
    }
    return hexagon;
}

#[test]
fn test_hexagon() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A matte white floor to catch the hexagon's shadow.
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(1.0, 1.0, 1.0);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // The whole hexagon is lifted and tipped forward with a single
    // transform on the outermost group.
    builder.add_shape({
        let mut hexagon = hexagon();
        hexagon.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_x(-std::f64::consts::PI / 6.0)
                .translation(0.0, 1.2, 0.0),
        );
        hexagon
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::color(1.0, 1.0, 1.0),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.8, -3.6),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "hexagon");
    return Ok(());
}
