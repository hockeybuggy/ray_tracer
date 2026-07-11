extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{
    camera, color, lights, material, matrix, shape, transformation, tuple, uv, world,
};

const SCALE: u32 = 1;

#[test]
fn test_uv_checkered_sphere() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_material({
                let checkers = uv::UvPattern::checkers(
                    20,
                    10,
                    color::color(0.0, 0.5, 0.0),
                    color::color(1.0, 1.0, 1.0),
                );
                let pattern =
                    ray_tracer::patterns::Pattern::texture_map(checkers, uv::UvMap::Spherical);
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.ambient = 0.1;
                material.specular = 0.4;
                material.shininess = 10.0;
                material.diffuse = 0.6;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.5);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 0.0, -5.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("uv_checkered_sphere").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "uv_checkered_sphere.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `uv_checkered_sphere.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_uv_checkered_plane() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let checkers = uv::UvPattern::checkers(
                    2,
                    2,
                    color::color(0.0, 0.5, 0.0),
                    color::color(1.0, 1.0, 1.0),
                );
                let pattern =
                    ray_tracer::patterns::Pattern::texture_map(checkers, uv::UvMap::Planar);
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.ambient = 0.1;
                material.specular = 0.0;
                material.diffuse = 0.9;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.5);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(1.0, 2.0, -5.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("uv_checkered_plane").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "uv_checkered_plane.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `uv_checkered_plane.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_uv_checkered_cylinder() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::cylinder(0.0, 1.0, false)
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .translation(0.0, -0.5, 0.0)
                    .scaling(1.0, 3.1415, 1.0),
            )
            .set_material({
                let checkers = uv::UvPattern::checkers(
                    16,
                    8,
                    color::color(0.0, 0.5, 0.0),
                    color::color(1.0, 1.0, 1.0),
                );
                let pattern =
                    ray_tracer::patterns::Pattern::texture_map(checkers, uv::UvMap::Cylindrical);
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.ambient = 0.1;
                material.specular = 0.6;
                material.shininess = 15.0;
                material.diffuse = 0.8;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.5);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 0.0, -10.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("uv_checkered_cylinder").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "uv_checkered_cylinder.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `uv_checkered_cylinder.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_align_check_plane() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let align_check = uv::UvPattern::align_check(
                    color::color(1.0, 1.0, 1.0), // main: white
                    color::color(1.0, 0.0, 0.0), // ul: red
                    color::color(1.0, 1.0, 0.0), // ur: yellow
                    color::color(0.0, 1.0, 0.0), // bl: green
                    color::color(0.0, 1.0, 1.0), // br: cyan
                );
                let pattern =
                    ray_tracer::patterns::Pattern::texture_map(align_check, uv::UvMap::Planar);
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.ambient = 0.1;
                material.diffuse = 0.8;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-10.0, 10.0, -10.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(100 * SCALE, 100 * SCALE, 0.5);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(1.0, 2.0, -5.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("align_check_plane").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "align_check_plane.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `align_check_plane.png`."
        );
    }
    return Ok(());
}
