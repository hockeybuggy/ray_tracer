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

#[test]
fn test_uv_mapped_cube() -> Result<(), std::io::Error> {
    let red = color::color(1.0, 0.0, 0.0);
    let yellow = color::color(1.0, 1.0, 0.0);
    let brown = color::color(1.0, 0.5, 0.0);
    let green = color::color(0.0, 1.0, 0.0);
    let cyan = color::color(0.0, 1.0, 1.0);
    let blue = color::color(0.0, 0.0, 1.0);
    let purple = color::color(1.0, 0.0, 1.0);
    let white = color::color(1.0, 1.0, 1.0);

    let mapped_cube_material = || {
        let pattern = ray_tracer::patterns::Pattern::cube_map(uv::CubeFaces {
            left: uv::UvPattern::align_check(yellow, cyan, red, blue, brown),
            front: uv::UvPattern::align_check(cyan, red, yellow, brown, green),
            right: uv::UvPattern::align_check(red, yellow, purple, green, white),
            back: uv::UvPattern::align_check(green, purple, cyan, white, blue),
            up: uv::UvPattern::align_check(brown, cyan, purple, red, yellow),
            down: uv::UvPattern::align_check(purple, brown, green, blue, white),
        });
        let mut material = material::material();
        material.pattern = Some(pattern);
        material.ambient = 0.2;
        material.specular = 0.0;
        material.diffuse = 0.8;
        material
    };

    let mut builder = world::WorldBuilder::new();

    let placements = [
        (0.7854, 0.7854, -6.0, 2.0),
        (2.3562, 0.7854, -2.0, 2.0),
        (3.927, 0.7854, 2.0, 2.0),
        (5.4978, 0.7854, 6.0, 2.0),
        (0.7854, -0.7854, -6.0, -2.0),
        (2.3562, -0.7854, -2.0, -2.0),
        (3.927, -0.7854, 2.0, -2.0),
        (5.4978, -0.7854, 6.0, -2.0),
    ];
    for (rot_y, rot_x, tx, ty) in placements {
        builder.add_shape(
            shape::ShapeBuilder::cube()
                .set_transform(
                    matrix::Matrix4::IDENTITY
                        .rotation_y(rot_y)
                        .rotation_x(rot_x)
                        .translation(tx, ty, 0.0),
                )
                .set_material(mapped_cube_material())
                .build(),
        );
    }

    for (x, y) in [(0.0, 100.0), (0.0, -100.0), (-100.0, 0.0), (100.0, 0.0)] {
        builder.add_light_source(lights::point_light(
            tuple::Point::new(x, y, -100.0),
            color::color(0.25, 0.25, 0.25),
        ));
    }

    let mut camera = camera::Camera::new(200 * SCALE, 100 * SCALE, 0.8);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 0.0, -20.0),
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image =
        shared_test_helpers::read_image_from_fixture_file("uv_mapped_cube").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "uv_mapped_cube.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `uv_mapped_cube.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_earth() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::plane()
            .set_material({
                let mut material = material::material();
                material.color = color::color(1.0, 1.0, 1.0);
                material.diffuse = 0.1;
                material.specular = 0.0;
                material.ambient = 0.0;
                material.reflective = 0.4;
                material
            })
            .build(),
    );

    builder.add_shape(
        shape::ShapeBuilder::cylinder(0.0, 0.1, true)
            .set_material({
                let mut material = material::material();
                material.color = color::color(1.0, 1.0, 1.0);
                material.diffuse = 0.2;
                material.specular = 0.0;
                material.ambient = 0.0;
                material.reflective = 0.1;
                material
            })
            .build(),
    );

    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .rotation_y(1.9)
                    .translation(0.0, 1.1, 0.0),
            )
            .set_material({
                let ppm = std::fs::read_to_string("textures/earth.ppm").unwrap();
                let earth_canvas = ray_tracer::canvas::canvas_from_ppm(&ppm).unwrap();
                let pattern = ray_tracer::patterns::Pattern::texture_map(
                    uv::UvPattern::image(earth_canvas),
                    uv::UvMap::Spherical,
                );
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.diffuse = 0.9;
                material.specular = 0.1;
                material.shininess = 10.0;
                material.ambient = 0.1;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-100.0, 100.0, -100.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(200 * SCALE, 100 * SCALE, 0.8);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(1.0, 2.0, -10.0),
        &tuple::Point::new(0.0, 1.1, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("earth").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "earth.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `earth.png`."
        );
    }
    return Ok(());
}

#[test]
fn test_skybox() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    builder.add_shape(
        shape::ShapeBuilder::sphere()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.75, 0.75, 0.75)
                    .translation(0.0, 0.0, 5.0),
            )
            .set_material({
                let mut material = material::material();
                material.diffuse = 0.4;
                material.specular = 0.6;
                material.shininess = 20.0;
                material.reflective = 0.6;
                material.ambient = 0.0;
                material
            })
            .build(),
    );

    let face_from_ppm = |name: &str| {
        let path = format!("textures/skybox/{}.ppm", name);
        let ppm = std::fs::read_to_string(&path).unwrap();
        uv::UvPattern::image(ray_tracer::canvas::canvas_from_ppm(&ppm).unwrap())
    };

    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(matrix::Matrix4::IDENTITY.scaling(1000.0, 1000.0, 1000.0))
            .set_material({
                let pattern = ray_tracer::patterns::Pattern::cube_map(uv::CubeFaces {
                    left: face_from_ppm("negx"),
                    right: face_from_ppm("posx"),
                    front: face_from_ppm("posz"),
                    back: face_from_ppm("negz"),
                    up: face_from_ppm("posy"),
                    down: face_from_ppm("negy"),
                });
                let mut material = material::material();
                material.pattern = Some(pattern);
                material.diffuse = 0.0;
                material.specular = 0.0;
                material.ambient = 1.0;
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(0.0, 100.0, 0.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(200 * SCALE, 100 * SCALE, 1.2);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 0.0, 0.0),
        &tuple::Point::new(0.0, 0.0, 5.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    let expected_image = shared_test_helpers::read_image_from_fixture_file("skybox").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "skybox.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `skybox.png`."
        );
    }
    return Ok(());
}
