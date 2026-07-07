extern crate ray_tracer;

mod shared_test_helpers;

use ray_tracer::transformation::Transform;
use ray_tracer::{camera, color, lights, material, matrix, shape, transformation, tuple, world};

const SCALE: u32 = 1;

// Chapter 16 "Putting It Together": one scene using all three CSG
// operations. A lens (the intersection of two spheres), a cube with an
// oversized sphere subtracted so its faces are carved hollow, and a Saturn
// whose ring is a nested difference of two flat cylinders joined to the
// planet by a union.
//
// Each surviving intersection references the primitive child that was hit,
// so every surface keeps its own material: the two lens faces, the red
// carving inside the cube, and the ring around the planet are all colored
// by their source shape.
#[test]
fn test_csg_scene() -> Result<(), std::io::Error> {
    let mut builder = world::WorldBuilder::new();

    // The floor.
    builder.add_shape({
        let mut floor = shape::Shape::default_plane();
        let mut material = material::material();
        material.color = color::color(0.9, 0.85, 0.8);
        material.specular = 0.0;
        floor.material = material;
        floor
    });

    // The lens: the intersection of two overlapping spheres, turned so both
    // faces are visible. Each face keeps the color of its sphere.
    builder.add_shape({
        let mut left_half = shape::Shape::default_sphere();
        left_half.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(-0.4, 0.0, 0.0));
        let mut left_material = material::material();
        left_material.color = color::color(0.2, 0.6, 0.4);
        left_material.specular = 0.4;
        left_half.material = left_material;

        let mut right_half = shape::Shape::default_sphere();
        right_half.set_transformation_matrix(matrix::Matrix4::IDENTITY.translation(0.4, 0.0, 0.0));
        let mut right_material = material::material();
        right_material.color = color::color(0.3, 0.5, 0.8);
        right_material.specular = 0.4;
        right_half.material = right_material;

        let mut lens = shape::Shape::csg(shape::CsgOperation::Intersection, left_half, right_half);
        lens.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_y(0.1)
                .translation(-2.2, 0.92, 0.8),
        );
        lens
    });

    // The carved cube: an oversized red sphere subtracted from a yellow
    // cube, hollowing a red dish out of each face.
    builder.add_shape({
        let mut cube = shape::Shape::default_cube();
        let mut cube_material = material::material();
        cube_material.color = color::color(1.0, 0.85, 0.2);
        cube_material.specular = 0.3;
        cube.material = cube_material;

        let mut scoop = shape::Shape::default_sphere();
        scoop.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(1.35, 1.35, 1.35));
        let mut scoop_material = material::material();
        scoop_material.color = color::color(0.9, 0.2, 0.2);
        scoop_material.specular = 0.3;
        scoop.material = scoop_material;

        let mut carved = shape::Shape::csg(shape::CsgOperation::Difference, cube, scoop);
        carved.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_y(0.6)
                .translation(0.0, 1.0, 1.5),
        );
        carved
    });

    // Saturn: the ring is one flat capped cylinder minus a slightly wider
    // one punched through its middle, and a union joins it to the planet so
    // the whole thing is a single tilted shape.
    builder.add_shape({
        let mut planet = shape::Shape::default_sphere();
        planet.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(0.8, 0.8, 0.8));
        let mut planet_material = material::material();
        planet_material.color = color::color(0.9, 0.7, 0.4);
        planet_material.specular = 0.2;
        planet.material = planet_material;

        fn ring_material() -> material::Material {
            let mut material = material::material();
            material.color = color::color(0.7, 0.7, 0.8);
            material.specular = 0.1;
            material
        }

        let mut disc = shape::Shape::cylinder(-0.5, 0.5, true);
        disc.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(1.6, 0.06, 1.6));
        disc.material = ring_material();

        let mut hole = shape::Shape::cylinder(-0.5, 0.5, true);
        hole.set_transformation_matrix(matrix::Matrix4::IDENTITY.scaling(1.1, 0.3, 1.1));
        hole.material = ring_material();

        let ring = shape::Shape::csg(shape::CsgOperation::Difference, disc, hole);

        let mut saturn = shape::Shape::csg(shape::CsgOperation::Union, planet, ring);
        saturn.set_transformation_matrix(
            matrix::Matrix4::IDENTITY
                .rotation_x(-0.5)
                .rotation_z(-0.3)
                .translation(2.1, 1.5, 1.5),
        );
        saturn
    });

    builder.add_light_source(lights::point_light(
        tuple::Point::new(-6.0, 8.0, -8.0),
        color::white(),
    ));

    let mut camera = camera::Camera::new(150 * SCALE, 100 * SCALE, std::f64::consts::PI / 3.0);
    camera.transform = transformation::view_transform(
        &tuple::Point::new(0.0, 2.5, -5.5),
        &tuple::Point::new(0.0, 1.0, 0.0),
        &tuple::Vector::new(0.0, 1.0, 0.0),
    );

    let canvas = camera.render(&builder.world);

    // While iterating on the scene there is no fixture yet: the render is
    // written to the repo root instead. Inspect it and copy it to
    // `tests/fixtures/csg_shapes.png` once the scene looks right.
    if !std::path::Path::new("tests/fixtures/csg_shapes.png").exists() {
        shared_test_helpers::write_image_to_file(&canvas, "csg_shapes.png").unwrap();
        assert!(false, "No fixture yet. Written canvas to `csg_shapes.png`.");
    }

    let expected_image = shared_test_helpers::read_image_from_fixture_file("csg_shapes").unwrap();

    if expected_image != canvas.canvas_to_image() {
        shared_test_helpers::write_image_to_file(&canvas, "csg_shapes.png").unwrap();
        assert!(
            false,
            "Result differed from fixture. Written canvas to `csg_shapes.png`."
        );
    }
    return Ok(());
}
