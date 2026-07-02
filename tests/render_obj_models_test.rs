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

fn load_model(path: &str) -> Result<shape::Shape, std::io::Error> {
    let source = std::fs::read_to_string(path)?;
    return Ok(obj_file::parse_obj(&source).into_group());
}

// Every model stands on the same matte floor, lit from the upper left, with
// the camera looking slightly down at it from -z.
fn render_model(model: shape::Shape, from: tuple::Point, to: tuple::Point) -> canvas::Canvas {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(0.55, 0.6, 0.65);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    builder.add_shape(model);

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::color(1.0, 1.0, 1.0),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 75 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform =
        transformation::view_transform(&from, &to, &tuple::Vector::new(0.0, 1.0, 0.0));

    return camera.render(&builder.world);
}

// The Utah teapot (low resolution version, 128 quad faces), exercising the
// whole OBJ pipeline in one scene: `v`/`vn` statements, `v/t/n` face
// corners, quad faces that need fan triangulation, and smooth triangles
// interpolating the vertex normals to hide the low polygon count.
#[test]
fn test_low_poly_teapot() -> Result<(), std::io::Error> {
    let mut teapot = load_model("object_files/teapot-low.obj")?;
    // The model is built z-up and roughly 32 units wide, so stand it up on
    // the y axis and scale it down to about three units across.
    teapot.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .rotation_x(-std::f64::consts::PI / 2.0)
            .scaling(0.1, 0.1, 0.1),
    );

    let canvas = render_model(
        teapot,
        tuple::Point::new(0.0, 2.0, -4.0),
        tuple::Point::new(0.0, 0.7, 0.0),
    );

    assert_matches_fixture(&canvas, "teapot");
    return Ok(());
}

// The same teapot at 3,200 quad faces: the smooth shading barely changes,
// but the silhouette and the lid seam get much cleaner.
#[test]
fn test_high_poly_teapot() -> Result<(), std::io::Error> {
    let mut teapot = load_model("object_files/teapot.obj")?;
    teapot.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .rotation_x(-std::f64::consts::PI / 2.0)
            .scaling(0.1, 0.1, 0.1),
    );

    let canvas = render_model(
        teapot,
        tuple::Point::new(0.0, 2.0, -4.0),
        tuple::Point::new(0.0, 0.7, 0.0),
    );

    assert_matches_fixture(&canvas, "teapot_high");
    return Ok(());
}

// A cow model of 5,804 flat triangles without vertex normals, so unlike the
// teapots every facet is visible.
#[test]
fn test_cow() -> Result<(), std::io::Error> {
    let mut cow = load_model("object_files/cow-nonormals.obj")?;
    // The model is y-up, about 10 units long, facing +x, with its hooves at
    // y=-3.64: shrink it and lift it onto the floor.
    cow.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .scaling(0.25, 0.25, 0.25)
            .translation(-0.2, 0.91, 0.0),
    );

    let canvas = render_model(
        cow,
        tuple::Point::new(0.0, 1.6, -3.0),
        tuple::Point::new(0.0, 0.75, 0.0),
    );

    assert_matches_fixture(&canvas, "cow");
    return Ok(());
}

// A teddy bear of 3,192 flat triangles without vertex normals.
#[test]
fn test_teddy() -> Result<(), std::io::Error> {
    let mut teddy = load_model("object_files/teddy.obj")?;
    // The model is y-up and centered on the origin, about 42 units tall,
    // facing -z: turn it around to face the camera, then shrink it and
    // lift it onto the floor.
    teddy.set_transformation_matrix(
        matrix::Matrix4::IDENTITY
            .rotation_y(std::f64::consts::PI)
            .scaling(0.05, 0.05, 0.05)
            .translation(0.0, 1.05, 0.0),
    );

    let canvas = render_model(
        teddy,
        tuple::Point::new(0.0, 2.0, -4.0),
        tuple::Point::new(0.0, 1.0, 0.0),
    );

    assert_matches_fixture(&canvas, "teddy");
    return Ok(());
}
