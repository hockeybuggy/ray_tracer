extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, shape, transformation, tuple, world};

const SCALE: u32 = 1;

// The appendix materials are all tints of the same matte, slightly
// reflective base.
fn cover_material(color: color::Color) -> material::Material {
    let mut material = material::material();
    material.color = color;
    material.diffuse = 0.7;
    material.ambient = 0.1;
    material.specular = 0.0;
    material.reflective = 0.1;
    material
}

fn white() -> color::Color {
    color::color(1.0, 1.0, 1.0)
}

fn blue() -> color::Color {
    color::color(0.537, 0.831, 0.914)
}

fn red() -> color::Color {
    color::color(0.941, 0.322, 0.388)
}

fn purple() -> color::Color {
    color::color(0.373, 0.404, 0.550)
}

// The scene is built from one cube nudged off-center and shrunk, at
// three different scales.
fn standard_transform() -> matrix::Matrix4 {
    matrix::Matrix4::IDENTITY
        .translation(1.0, -1.0, 1.0)
        .scaling(0.5, 0.5, 0.5)
}

fn large_object() -> matrix::Matrix4 {
    standard_transform().scaling(3.5, 3.5, 3.5)
}

fn medium_object() -> matrix::Matrix4 {
    standard_transform().scaling(3.0, 3.0, 3.0)
}

fn small_object() -> matrix::Matrix4 {
    standard_transform().scaling(2.0, 2.0, 2.0)
}

fn cube(color: color::Color, transform: matrix::Matrix4) -> shape::Shape {
    shape::ShapeBuilder::cube()
        .set_material(cover_material(color))
        .set_transform(transform)
        .build()
}

// The book's cover image, translated from the YAML scene description in
// appendix 1: a reflective glassy sphere resting on a cascade of cubes,
// lit against a white backdrop.
#[test]
fn test_cover_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // A white backdrop far behind the scene, lit only by its ambient term.
    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let mut material = material::material();
                material.color = white();
                material.ambient = 1.0;
                material.diffuse = 0.0;
                material.specular = 0.0;
                material
            })
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .rotation_x(std::f64::consts::FRAC_PI_2)
                    .translation(0.0, 0.0, 500.0),
            )
            .build(),
    );

    // The glassy purple sphere sitting on top of the stack.
    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_material({
                let mut material = material::material();
                material.color = purple();
                material.diffuse = 0.2;
                material.ambient = 0.0;
                material.specular = 1.0;
                material.shininess = 200.0;
                material.reflective = 0.7;
                material.transparency = 0.7;
                material.refractive_index = 1.5;
                material
            })
            .set_transform(large_object())
            .build(),
    );

    // The cascade of cubes, listed in the same order as the book.
    builder.add_shape(cube(white(), medium_object().translation(4.0, 0.0, 0.0)));
    builder.add_shape(cube(blue(), large_object().translation(8.5, 1.5, -0.5)));
    builder.add_shape(cube(red(), large_object().translation(0.0, 0.0, 4.0)));
    builder.add_shape(cube(white(), small_object().translation(4.0, 0.0, 4.0)));
    builder.add_shape(cube(purple(), medium_object().translation(7.5, 0.5, 4.0)));
    builder.add_shape(cube(white(), medium_object().translation(-0.25, 0.25, 8.0)));
    builder.add_shape(cube(blue(), large_object().translation(4.0, 1.0, 7.5)));
    builder.add_shape(cube(red(), medium_object().translation(10.0, 2.0, 7.5)));
    builder.add_shape(cube(white(), small_object().translation(8.0, 2.0, 12.0)));
    builder.add_shape(cube(white(), small_object().translation(20.0, 1.0, 9.0)));
    builder.add_shape(cube(blue(), large_object().translation(-0.5, -5.0, 0.25)));
    builder.add_shape(cube(red(), large_object().translation(4.0, -4.0, 0.0)));
    builder.add_shape(cube(white(), large_object().translation(8.5, -4.0, 0.0)));
    builder.add_shape(cube(white(), large_object().translation(0.0, -4.0, 4.0)));
    builder.add_shape(cube(purple(), large_object().translation(-0.5, -4.5, 8.0)));
    builder.add_shape(cube(white(), large_object().translation(0.0, -8.0, 4.0)));
    builder.add_shape(cube(white(), large_object().translation(-0.5, -8.5, 8.0)));

    builder.add_light_source(lights::point_light(
        tuple::Point::new(50.0, 100.0, -50.0),
        color::white(),
    ));
    // An optional second light for additional illumination.
    builder.add_light_source(lights::point_light(
        tuple::Point::new(-400.0, 50.0, -10.0),
        color::color(0.2, 0.2, 0.2),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.785);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(-6.0, 6.0, -10.0),
        &tuple::Point::new(6.0, 0.0, 6.0),
        &tuple::Vector::new(-0.45, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    // While iterating on the scene there is no fixture yet: the render is
    // written to the repo root instead. Inspect it and copy it to
    // `tests/fixtures/cover.png` once the scene looks right.
    if !std::path::Path::new("tests/fixtures/cover.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "cover.png").unwrap();
        assert!(false, "No fixture yet. Written canvas to `cover.png`.");
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file("cover").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "cover.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `cover.png`."
        );
    }
    return Ok(());
}
