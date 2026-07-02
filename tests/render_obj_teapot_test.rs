extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, canvas, color, lights, material, matrix, obj_file, shape, transformation, tuple, world,
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

// The Utah teapot (low resolution version, 128 faces), exercising the whole
// OBJ pipeline: `v`/`vn` statements, `v/t/n` face corners, quad faces that
// need fan triangulation, and smooth triangles interpolating the vertex
// normals to hide the low polygon count.
#[test]
fn test_low_poly_teapot() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A matte floor to catch the teapot's shadow.
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(0.55, 0.6, 0.65);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    builder.add_shape({
        let source = std::fs::read_to_string("object_files/teapot-low.obj")?;
        let mut teapot = obj_file::parse_obj(&source).into_group();
        // The model is built z-up and roughly 32 units wide, so stand it up
        // on the y axis and scale it down to about three units across.
        teapot.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_x(-std::f64::consts::PI / 2.0)
                .scaling(0.1, 0.1, 0.1),
        );
        teapot
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::color(1.0, 1.0, 1.0),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 75 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.0, -4.0),
        &tuple::Point::new(0.0, 0.7, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    assert_matches_fixture(&canvas, "teapot");
    return Ok(());
}
