extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, shape, transformation, tuple, world};

const SCALE: u32 = 1;

// Chapter 12 "Putting It Together": a room built from one large cube, a table
// made of five stretched cubes, and some boxes scattered around.
//
// Transforms chain back to front: `.scaling(..).translation(..)` scales the
// unit cube first, then moves it into place.
#[test]
fn test_cube_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // The room: one big cube. The floor is at y = 0 because the cube is
    // scaled to 15 and lifted by 15.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(15.0, 15.0, 15.0)
                    .translation(0.0, 15.0, 0.0),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.9, 0.85, 0.8);
                material.specular = 0.0;
                material
            })
            .build(),
    );

    // The table top.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(1.5, 0.05, 1.0)
                    .translation(0.0, 1.0, 0.0),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.55, 0.35, 0.2);
                material.specular = 0.3;
                material
            })
            .build(),
    );

    // The table legs: tall thin cubes under each corner of the top.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.05, 0.5, 0.05)
                    .translation(-1.3, 0.5, -0.8),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.55, 0.35, 0.2);
                material.specular = 0.3;
                material
            })
            .build(),
    );

    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.05, 0.5, 0.05)
                    .translation(-1.3, 0.5, 0.8),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.55, 0.35, 0.2);
                material.specular = 0.3;
                material
            })
            .build(),
    );

    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.05, 0.5, 0.05)
                    .translation(1.3, 0.5, -0.8),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.55, 0.35, 0.2);
                material.specular = 0.3;
                material
            })
            .build(),
    );

    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.05, 0.5, 0.05)
                    .translation(1.3, 0.5, 0.8),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.55, 0.35, 0.2);
                material.specular = 0.3;
                material
            })
            .build(),
    );

    // A small red box sitting on the table, turned slightly.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.15, 0.15, 0.15)
                    .rotation_y(0.5)
                    .translation(0.4, 1.2, 0.2),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.8, 0.2, 0.2);
                material
            })
            .build(),
    );

    // A green box on the floor, left of the table.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.35, 0.35, 0.35)
                    .rotation_y(0.3)
                    .translation(-2.7, 0.35, -1.4),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.2, 0.6, 0.3);
                material
            })
            .build(),
    );

    // A blue box on the floor, in front of the table.
    builder.add_shape(
        shape::ShapeBuilder::cube()
            .set_transform(
                matrix::Matrix4::IDENTITY
                    .scaling(0.2, 0.2, 0.2)
                    .rotation_y(-0.4)
                    .translation(1.2, 0.2, -1.8),
            )
            .set_material({
                let mut material = material::material();
                material.color = color::color(0.25, 0.35, 0.8);
                material
            })
            .build(),
    );

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(4.0, 3.0, -5.0),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    // While iterating on the scene there is no fixture yet: the render is
    // written to the repo root instead. Inspect it and copy it to
    // `tests/fixtures/cubes.png` once the scene looks right.
    if !std::path::Path::new("tests/fixtures/cubes.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "cubes.png").unwrap();
        assert!(false, "No fixture yet. Written canvas to `cubes.png`.");
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file("cubes").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "cubes.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `cubes.png`."
        );
    }
    return Ok(());
}
